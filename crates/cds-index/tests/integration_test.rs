//! Integration tests for cds-index

use cds_index::graph::GraphBuilder;
use tempfile::TempDir;

#[test]
fn builds_empty_repository_graph() {
    let repo = TempDir::new().expect("temp directory");
    let builder = GraphBuilder::new(repo.path());
    let result = builder.build().expect("graph build");
    assert_eq!(result.graph.node_count(), 1);
}
