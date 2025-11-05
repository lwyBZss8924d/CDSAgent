use std::path::Path;

use cds_index::graph::builder::{GraphBuilder, GraphBuilderConfig};
use cds_index::index::{AnalyzerConfig, SparseIndex};
use tempfile::TempDir;

#[test]
#[ignore = "Requires SMOKE_REPO_PATHS env var with comma-separated repo roots"]
fn smoke_sparse_index_builds_for_external_repos() {
    let paths = std::env::var("SMOKE_REPO_PATHS").expect(
        "Set SMOKE_REPO_PATHS=/abs/path/to/repo1,/abs/path/to/repo2 to run this smoke test",
    );

    for raw_path in paths.split(',') {
        let trimmed = raw_path.trim();
        if trimmed.is_empty() {
            continue;
        }
        let repo_path = Path::new(trimmed);
        assert!(
            repo_path.exists(),
            "Smoke repo path {} is missing",
            repo_path.display()
        );

        let builder = GraphBuilder::with_config(repo_path, GraphBuilderConfig::default());
        let graph = builder
            .build()
            .expect("failed to build graph for smoke repo")
            .graph;

        let temp_dir = TempDir::new().expect("failed to create temp dir for smoke index");
        SparseIndex::from_graph(graph, temp_dir.path(), AnalyzerConfig::default())
            .expect("failed to build sparse index for smoke repo");
    }
}
