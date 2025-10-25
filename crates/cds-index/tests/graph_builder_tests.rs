use cds_index::graph::{EdgeKind, GraphBuilder, NodeKind};
use petgraph::visit::EdgeRef;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn write_file(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create directories");
    }
    fs::write(&path, contents).expect("failed to write file");
}

fn build_graph_with_files(files: &[(&str, &str)]) -> (TempDir, cds_index::graph::DependencyGraph) {
    let temp = TempDir::new().expect("tempdir");
    for (path, contents) in files {
        write_file(temp.path(), path, contents);
    }

    let builder = GraphBuilder::new(temp.path());
    let result = builder.build().expect("graph build");
    (temp, result.graph)
}

fn find_node<F>(
    graph: &cds_index::graph::DependencyGraph,
    predicate: F,
) -> Option<cds_index::graph::GraphNodeIndex>
where
    F: Fn(&cds_index::graph::GraphNode) -> bool,
{
    graph.graph().node_indices().find(|idx| {
        graph
            .graph()
            .node_weight(*idx)
            .map_or(false, |node| predicate(node))
    })
}

#[test]
fn import_edges_capture_aliases() {
    let files = [
        ("pkg/__init__.py", ""),
        (
            "pkg/util.py",
            r#"
class Helper:
    def action(self):
        pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg.util import Helper as Renamed

def runner():
    Renamed().action()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let file_idx = find_node(&graph, |node| {
        node.kind == NodeKind::File && node.display_name == "main.py"
    })
    .expect("main.py node");
    let helper_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("pkg/util.py::Helper")
    })
    .expect("helper class");

    let mut import_alias = None;
    for edge in graph.graph().edges(file_idx) {
        if edge.weight().kind == EdgeKind::Import && edge.target() == helper_idx {
            import_alias = edge.weight().alias.clone();
            break;
        }
    }

    assert_eq!(import_alias.as_deref(), Some("Renamed"));
}

#[test]
fn behavior_edges_detect_invokes_and_inherits() {
    let files = [
        ("pkg/__init__.py", ""),
        (
            "pkg/util.py",
            r#"
class Helper:
    def action(self):
        pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg.util import Helper

class Base:
    def ping(self):
        pass

class Child(Base):
    def __init__(self):
        Helper().action()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let child_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("main.py::Child")
    })
    .expect("child class");
    let base_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("main.py::Base")
    })
    .expect("base class");
    let helper_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("pkg/util.py::Helper")
    })
    .expect("helper class");

    let mut inherits_base = false;
    let mut invokes_helper = false;
    for edge in graph.graph().edges(child_idx) {
        if edge.weight().kind == EdgeKind::Inherit && edge.target() == base_idx {
            inherits_base = true;
        }
        if edge.weight().kind == EdgeKind::Invoke && edge.target() == helper_idx {
            invokes_helper = true;
        }
    }

    assert!(inherits_base, "Child should inherit Base");
    assert!(invokes_helper, "Child __init__ should invoke Helper");
}

#[test]
fn invoke_edges_follow_import_aliases() {
    let files = [
        ("pkg/__init__.py", ""),
        (
            "pkg/core.py",
            r#"
class Service:
    def __init__(self):
        pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg.core import Service as Engine

def run():
    thing = Engine()
    return thing
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let run_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.id.ends_with("main.py::run")
    })
    .expect("run function");
    let service_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("pkg/core.py::Service")
    })
    .expect("Service class");

    let mut found_invoke = false;
    for edge in graph.graph().edges(run_idx) {
        if edge.weight().kind == EdgeKind::Invoke && edge.target() == service_idx {
            found_invoke = true;
            break;
        }
    }

    assert!(
        found_invoke,
        "alias Engine() should resolve to Service invoke edge"
    );
}

#[test]
fn decorator_aliases_emit_invoke_edges() {
    let files = [
        ("pkg/__init__.py", ""),
        (
            "pkg/decorators.py",
            r#"
def audit(func):
    return func
"#,
        ),
        (
            "main.py",
            r#"
from pkg.decorators import audit as wrapped

@wrapped
def handler():
    pass
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let handler_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.id.ends_with("main.py::handler")
    })
    .expect("handler function");
    let audit_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.id.ends_with("pkg/decorators.py::audit")
    })
    .expect("audit decorator");

    let mut invokes_decorator = false;
    for edge in graph.graph().edges(handler_idx) {
        if edge.weight().kind == EdgeKind::Invoke && edge.target() == audit_idx {
            invokes_decorator = true;
            break;
        }
    }

    assert!(
        invokes_decorator,
        "decorator alias should record invoke edge to audit()"
    );
}
