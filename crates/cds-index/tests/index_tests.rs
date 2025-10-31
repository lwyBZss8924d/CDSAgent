use std::path::PathBuf;

use cds_index::graph::{DependencyGraph, GraphNode, GraphNodeIndex, NodeKind, SourceRange};
use cds_index::index::NameIndex;

#[test]
fn name_index_exact_and_prefix_match_behaviour() {
    let builder = NameIndex::builder();
    let dir_idx = GraphNodeIndex::new(1);
    let file_idx = GraphNodeIndex::new(2);
    let func_idx = GraphNodeIndex::new(3);

    builder.insert(
        "Agents",
        dir_idx,
        NodeKind::Directory,
        Some("agents".to_string()),
    );
    builder.insert(
        "agents.rs",
        file_idx,
        NodeKind::File,
        Some("src/agents.rs".to_string()),
    );
    builder.insert(
        "agents_resolve",
        func_idx,
        NodeKind::Function,
        Some("pkg::agents::resolve".to_string()),
    );

    let index = builder.finish();

    let exact = index.exact_match("AGENTS.RS", None, 10);
    assert_eq!(exact.len(), 1);
    assert_eq!(exact[0].node_id, file_idx);

    let prefix_all = index.prefix_match("agents", None, 10);
    assert_eq!(prefix_all.len(), 3);

    let prefix_filtered = index.prefix_match("agents", Some(NodeKind::Function), 5);
    assert_eq!(prefix_filtered.len(), 1);
    assert_eq!(prefix_filtered[0].node_id, func_idx);

    let limited = index.prefix_match("agents", None, 2);
    assert_eq!(limited.len(), 2);
}

#[test]
fn name_index_from_graph_ingests_metadata() {
    let mut graph = DependencyGraph::new();

    let dir = graph.add_node(GraphNode::directory("pkg".into(), "pkg".into(), None));
    let _file = graph.add_node(GraphNode::file(
        "pkg::module".into(),
        "module.py".into(),
        PathBuf::from("pkg/module.py"),
    ));
    let function = graph.add_node(GraphNode::entity(
        "pkg::module::handler".into(),
        NodeKind::Function,
        "handle_request".into(),
        PathBuf::from("pkg/module.py"),
        Some(SourceRange::new(10, 42)),
    ));
    let _ignored = graph.add_node(GraphNode::directory("hidden".into(), "".into(), None));

    let index = NameIndex::from_graph(&graph);
    let stats = index.stats();

    assert_eq!(stats.unique_names, 3);
    assert_eq!(stats.total_entities, 3);

    let dir_hits = index.exact_match("pkg", None, 5);
    assert_eq!(dir_hits.len(), 1);
    assert_eq!(dir_hits[0].node_id, dir);

    let func_hits = index.exact_match("handle_request", Some(NodeKind::Function), 5);
    assert_eq!(func_hits.len(), 1);
    assert_eq!(func_hits[0].node_id, function);

    assert!(index.exact_match("hidden", None, 5).is_empty());
}
