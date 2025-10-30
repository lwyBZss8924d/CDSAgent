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
        matches!(
            graph.graph().node_weight(*idx),
            Some(node) if predicate(node)
        )
    })
}

fn has_edge(
    graph: &cds_index::graph::DependencyGraph,
    source: cds_index::graph::GraphNodeIndex,
    target: cds_index::graph::GraphNodeIndex,
    kind: EdgeKind,
) -> bool {
    graph
        .graph()
        .edges(source)
        .any(|edge| edge.weight().kind == kind && edge.target() == target)
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
fn import_edges_follow_package_reexports() {
    let files = [
        (
            "pkg/__init__.py",
            r#"
from pkg.core import Service

__all__ = ["Service"]
"#,
        ),
        (
            "pkg/core.py",
            r#"
class Service:
    def perform(self):
        pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg import Service

def handler():
    svc = Service()
    svc.perform()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx = find_node(&graph, |node| {
        node.kind == NodeKind::File && node.display_name == "main.py"
    })
    .expect("main.py file node");
    let service_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("pkg/core.py::Service")
    })
    .expect("Service class");

    let mut found_edge = false;
    for edge in graph.graph().edges(main_idx) {
        if edge.weight().kind == EdgeKind::Import && edge.target() == service_idx {
            found_edge = true;
            break;
        }
    }

    assert!(
        found_edge,
        "re-exported import should resolve to Service class in pkg/core.py"
    );
}

#[test]
fn wildcard_imports_expand_all_exports() {
    let files = [
        (
            "pkg/__init__.py",
            r#"
from pkg.core import Service, Hidden
__all__ = ["Service"]
"#,
        ),
        (
            "pkg/core.py",
            r#"
class Service:
    def action(self):
        pass

class Hidden:
    def secret(self):
        pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg import *

def build():
    return Service()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx = find_node(&graph, |node| {
        node.kind == NodeKind::File && node.display_name == "main.py"
    })
    .expect("main file node");
    let service_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("pkg/core.py::Service")
    })
    .expect("Service class");
    let hidden_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Class && node.id.ends_with("pkg/core.py::Hidden")
    })
    .expect("Hidden class");

    let mut imports_service = false;
    let mut imports_hidden = false;
    for edge in graph.graph().edges(main_idx) {
        if edge.weight().kind != EdgeKind::Import {
            continue;
        }
        if edge.target() == service_idx {
            imports_service = true;
        }
        if edge.target() == hidden_idx {
            imports_hidden = true;
        }
    }

    assert!(
        imports_service,
        "wildcard import should include Service from __all__"
    );
    assert!(
        !imports_hidden,
        "__all__ constraints should prevent Hidden from being imported"
    );
}

#[test]
fn exports_follow_module_all_aliases() {
    let files = [
        (
            "pkg/repo_ops.py",
            r#"
__all__ = ["run"]

def run():
    return True
"#,
        ),
        (
            "pkg/locationtools.py",
            r#"
from pkg import repo_ops

__all__ = repo_ops.__all__
"#,
        ),
        ("pkg/__init__.py", ""),
        (
            "main.py",
            r#"
from pkg.locationtools import *

def use():
    return run()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx = find_node(&graph, |node| {
        node.kind == NodeKind::File && node.display_name == "main.py"
    })
    .expect("main file node");
    let run_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.id.ends_with("pkg/repo_ops.py::run")
    })
    .expect("run function");

    let mut found = false;
    for edge in graph.graph().edges(main_idx) {
        if edge.weight().kind == EdgeKind::Import && edge.target() == run_idx {
            found = true;
            break;
        }
    }

    assert!(
        found,
        "re-exported __all__ via module alias should surface run()"
    );
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

#[test]
fn invoke_edges_include_all_alias_candidates() {
    let files = [
        ("pkg/__init__.py", ""),
        (
            "pkg/alpha.py",
            r#"
def handler():
    return "alpha"
"#,
        ),
        (
            "pkg/beta.py",
            r#"
def handler():
    return "beta"
"#,
        ),
        (
            "main.py",
            r#"
from pkg.alpha import handler as shared
from pkg.beta import handler as shared

def caller():
    return shared()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let caller_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.id.ends_with("main.py::caller")
    })
    .expect("caller function");
    let alpha_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.id.ends_with("pkg/alpha.py::handler")
    })
    .expect("alpha handler");
    let beta_idx = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.id.ends_with("pkg/beta.py::handler")
    })
    .expect("beta handler");

    let mut targets = Vec::new();
    for edge in graph.graph().edges(caller_idx) {
        if edge.weight().kind == EdgeKind::Invoke {
            targets.push(edge.target());
        }
    }

    assert!(
        targets.contains(&alpha_idx),
        "caller() should include alpha handler as possible callee"
    );
    assert!(
        targets.contains(&beta_idx),
        "caller() should include beta handler as possible callee"
    );
}

#[test]
fn nested_classes_create_containment_hierarchy() {
    let files = [(
        "pkg.py",
        r#"
class Outer:
    class Inner:
        class Deep:
            pass
"#,
    )];

    let (_dir, graph) = build_graph_with_files(&files);
    let outer_idx = find_node(&graph, |node| node.id.ends_with("pkg.py::Outer")).expect("Outer class");
    let inner_idx = find_node(&graph, |node| node.id.ends_with("pkg.py::Outer::Inner"))
        .expect("Inner class");
    let deep_idx = find_node(&graph, |node| node.id.ends_with("pkg.py::Outer::Inner::Deep"))
        .expect("Deep class");

    assert!(
        has_edge(&graph, outer_idx, inner_idx, EdgeKind::Contain),
        "Outer should contain Inner"
    );
    assert!(
        has_edge(&graph, inner_idx, deep_idx, EdgeKind::Contain),
        "Inner should contain Deep"
    );
}

#[test]
fn nested_functions_create_containment_hierarchy() {
    let files = [(
        "module.py",
        r#"
def outer():
    def inner():
        def deep():
            return 42
        return deep()
    return inner()
"#,
    )];

    let (_dir, graph) = build_graph_with_files(&files);
    let outer_idx = find_node(&graph, |node| node.id.ends_with("module.py::outer")).expect("outer()");
    let inner_idx =
        find_node(&graph, |node| node.id.ends_with("module.py::outer::inner")).expect("inner()");
    let deep_idx = find_node(&graph, |node| node.id.ends_with("module.py::outer::inner::deep"))
        .expect("deep()");

    assert!(
        has_edge(&graph, outer_idx, inner_idx, EdgeKind::Contain),
        "outer() should contain inner()"
    );
    assert!(
        has_edge(&graph, inner_idx, deep_idx, EdgeKind::Contain),
        "inner() should contain deep()"
    );
}

#[test]
fn async_functions_recorded_as_function_nodes() {
    let files = [(
        "mod.py",
        r#"
async def fetch(session):
    await session.request()
"#,
    )];

    let (_dir, graph) = build_graph_with_files(&files);
    let fetch_idx = find_node(&graph, |node| node.id.ends_with("mod.py::fetch")).expect("fetch()");
    let fetch_node = graph.graph().node_weight(fetch_idx).expect("fetch node");

    assert_eq!(fetch_node.kind, NodeKind::Function);
}

#[test]
fn type_checking_imports_are_resolved() {
    let files = [
        ("pkg/__init__.py", ""),
        (
            "pkg/core.py",
            r#"
class Service:
    def perform(self):
        pass
"#,
        ),
        (
            "main.py",
            r#"
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pkg.core import Service

def factory(svc: "Service") -> "Service":
    return svc
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx = find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "main.py")
        .expect("main.py node");
    let service_idx = find_node(&graph, |node| node.id.ends_with("pkg/core.py::Service"))
        .expect("Service class");

    assert!(
        has_edge(&graph, main_idx, service_idx, EdgeKind::Import),
        "TYPE_CHECKING import should resolve to Service class"
    );
}

#[test]
fn relative_imports_single_dot_resolve() {
    let files = [
        ("pkg/__init__.py", ""),
        (
            "pkg/core.py",
            r#"
class Helper:
    pass
"#,
        ),
        (
            "pkg/util.py",
            r#"
from .core import Helper

def build():
    return Helper()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let util_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "util.py")
            .expect("util.py");
    let helper_idx = find_node(&graph, |node| node.id.ends_with("pkg/core.py::Helper"))
        .expect("Helper class");

    assert!(
        has_edge(&graph, util_idx, helper_idx, EdgeKind::Import),
        "relative import .core should resolve to Helper"
    );
}

#[test]
fn relative_imports_parent_resolve() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/sub/__init__.py", ""),
        (
            "pkg/core.py",
            r#"
class Service:
    pass
"#,
        ),
        (
            "pkg/sub/module.py",
            r#"
from ..core import Service

def make():
    return Service()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let module_idx = find_node(&graph, |node| {
        node.kind == NodeKind::File && node.display_name == "module.py"
    })
    .expect("module.py");
    let service_idx = find_node(&graph, |node| node.id.ends_with("pkg/core.py::Service"))
        .expect("Service class");

    assert!(
        has_edge(&graph, module_idx, service_idx, EdgeKind::Import),
        "relative parent import ..core should resolve to Service"
    );
}

#[test]
fn circular_imports_emit_edges_both_directions() {
    let files = [
        (
            "alpha.py",
            r#"
from beta import worker

def make():
    return worker()
"#,
        ),
        (
            "beta.py",
            r#"
from alpha import make

def worker():
    return make
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let alpha_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "alpha.py")
            .expect("alpha.py");
    let beta_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "beta.py")
            .expect("beta.py");
    let worker_idx =
        find_node(&graph, |node| node.id.ends_with("beta.py::worker")).expect("worker()");
    let make_idx = find_node(&graph, |node| node.id.ends_with("alpha.py::make")).expect("make()");

    assert!(
        has_edge(&graph, alpha_idx, worker_idx, EdgeKind::Import),
        "alpha.py should import beta.worker"
    );
    assert!(
        has_edge(&graph, beta_idx, make_idx, EdgeKind::Import),
        "beta.py should import alpha.make"
    );
}

#[test]
fn multiple_inheritance_generates_multiple_edges() {
    let files = [(
        "models.py",
        r#"
class BaseA:
    pass

class BaseB:
    pass

class Child(BaseA, BaseB):
    pass
"#,
    )];

    let (_dir, graph) = build_graph_with_files(&files);
    let child_idx =
        find_node(&graph, |node| node.id.ends_with("models.py::Child")).expect("Child");
    let base_a_idx =
        find_node(&graph, |node| node.id.ends_with("models.py::BaseA")).expect("BaseA");
    let base_b_idx =
        find_node(&graph, |node| node.id.ends_with("models.py::BaseB")).expect("BaseB");

    assert!(
        has_edge(&graph, child_idx, base_a_idx, EdgeKind::Inherit),
        "Child should inherit BaseA"
    );
    assert!(
        has_edge(&graph, child_idx, base_b_idx, EdgeKind::Inherit),
        "Child should inherit BaseB"
    );
}

#[test]
fn set_based_all_exports_surface_symbols() {
    let files = [
        (
            "pkg/__init__.py",
            r#"
from pkg.core import Service
__all__ = {"Service"}
"#,
        ),
        (
            "pkg/core.py",
            r#"
class Service:
    pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg import Service

def make():
    return Service()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "main.py")
            .expect("main.py");
    let service_idx = find_node(&graph, |node| node.id.ends_with("pkg/core.py::Service"))
        .expect("Service class");

    assert!(
        has_edge(&graph, main_idx, service_idx, EdgeKind::Import),
        "__all__ defined as set should export Service"
    );
}

#[test]
fn tuple_based_all_exports_surface_symbols() {
    let files = [
        (
            "pkg/__init__.py",
            r#"
from pkg.core import Service
__all__ = ("Service",)
"#,
        ),
        (
            "pkg/core.py",
            r#"
class Service:
    pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg import Service

Service
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "main.py")
            .expect("main.py");
    let service_idx = find_node(&graph, |node| node.id.ends_with("pkg/core.py::Service"))
        .expect("Service class");

    assert!(
        has_edge(&graph, main_idx, service_idx, EdgeKind::Import),
        "__all__ defined as tuple should export Service"
    );
}

#[test]
fn augassign_all_exports_append_symbols() {
    let files = [
        (
            "pkg/__init__.py",
            r#"
from pkg.core import Service, Helper
__all__ = ["Service"]
__all__ += ["Helper"]
"#,
        ),
        (
            "pkg/core.py",
            r#"
class Service:
    pass

class Helper:
    pass
"#,
        ),
        (
            "main.py",
            r#"
from pkg import Helper

Helper
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "main.py")
            .expect("main.py");
    let helper_idx = find_node(&graph, |node| node.id.ends_with("pkg/core.py::Helper"))
        .expect("Helper class");

    assert!(
        has_edge(&graph, main_idx, helper_idx, EdgeKind::Import),
        "__all__ += should retain newly appended Helper symbol"
    );
}

#[test]
fn attribute_all_exports_follow_chain() {
    let files = [
        (
            "pkg/base.py",
            r#"
__all__ = ["Service"]

class Service:
    pass
"#,
        ),
        (
            "pkg/__init__.py",
            r#"
from pkg import base
__all__ = base.__all__
"#,
        ),
        (
            "main.py",
            r#"
from pkg import Service

Service
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "main.py")
            .expect("main.py");
    let service_idx = find_node(&graph, |node| node.id.ends_with("pkg/base.py::Service"))
        .expect("Service class");

    assert!(
        has_edge(&graph, main_idx, service_idx, EdgeKind::Import),
        "__all__ = base.__all__ should expose Service"
    );
}

#[test]
fn reexport_chain_through_intermediate_module() {
    let files = [
        (
            "pkg/a.py",
            r#"
from pkg.core import Service
__all__ = ["Service"]
"#,
        ),
        (
            "pkg/core.py",
            r#"
class Service:
    pass
"#,
        ),
        (
            "pkg/__init__.py",
            r#"
from pkg.a import *
"#,
        ),
        (
            "main.py",
            r#"
from pkg import Service

Service
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let main_idx =
        find_node(&graph, |node| node.kind == NodeKind::File && node.display_name == "main.py")
            .expect("main.py");
    let service_idx = find_node(&graph, |node| node.id.ends_with("pkg/core.py::Service"))
        .expect("Service class");

    assert!(
        has_edge(&graph, main_idx, service_idx, EdgeKind::Import),
        "re-export chain pkg.a -> pkg should expose Service"
    );
}

#[test]
fn lambda_expressions_do_not_create_function_nodes() {
    let files = [(
        "util.py",
        r#"
callback = lambda value: value + 1
"#,
    )];

    let (_dir, graph) = build_graph_with_files(&files);
    let lambda_node = find_node(&graph, |node| {
        node.kind == NodeKind::Function && node.display_name == "<lambda>"
    });

    assert!(
        lambda_node.is_none(),
        "lambdas should not produce standalone function nodes"
    );
}

#[test]
fn async_class_initializers_collect_invokes() {
    let files = [
        (
            "pkg/helpers.py",
            r#"
async def setup():
    return True
"#,
        ),
        (
            "main.py",
            r#"
from pkg.helpers import setup

class Runner:
    async def __init__(self):
        await setup()
"#,
        ),
    ];

    let (_dir, graph) = build_graph_with_files(&files);
    let runner_idx =
        find_node(&graph, |node| node.id.ends_with("main.py::Runner")).expect("Runner class");
    let setup_idx = find_node(&graph, |node| node.id.ends_with("pkg/helpers.py::setup"))
        .expect("setup function");

    assert!(
        has_edge(&graph, runner_idx, setup_idx, EdgeKind::Invoke),
        "async __init__ should record invoke edge to setup()"
    );
}
