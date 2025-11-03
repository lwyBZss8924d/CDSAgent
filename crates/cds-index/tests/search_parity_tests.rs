mod support;

use cds_index::graph::builder::{GraphBuilder, GraphBuilderConfig};
use cds_index::index::{AnalyzerConfig, SparseIndex};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use support::{load_locagent_search_queries, GoldenSearchQuery};

#[test]
#[ignore = "Requires LocAgent parity fixtures (target ≥75% overlap@10)"]
fn sparse_index_matches_locagent_top10_overlap() {
    let workspace = repo_root();
    let locagent_repo = workspace.join("tmp/LocAgent");
    assert!(
        locagent_repo.exists(),
        "LocAgent repository missing at {}. See tests/fixtures/parity/README.md for setup instructions.",
        locagent_repo.display()
    );

    let queries_path = workspace.join("tests/fixtures/parity/golden_outputs/search_queries.jsonl");
    assert!(
        queries_path.exists(),
        "Golden search queries missing at {}.",
        queries_path.display()
    );

    let queries = load_locagent_search_queries(&queries_path);
    assert!(
        !queries.is_empty(),
        "No golden queries loaded from {}.",
        queries_path.display()
    );

    let graph = build_locagent_graph(&locagent_repo);
    let temp_dir = TempDir::new().expect("unable to create temporary index directory");
    let sparse_index = SparseIndex::from_graph(graph, temp_dir.path(), AnalyzerConfig::default())
        .expect("failed to build sparse index from graph");

    let mut overlaps = Vec::new();
    let canonical_repo =
        std::fs::canonicalize(&locagent_repo).unwrap_or_else(|_| locagent_repo.clone());

    for GoldenSearchQuery { query, top10_files } in &queries {
        let results = sparse_index
            .search(query, 10, None)
            .expect("search execution failed");
        let our_results: Vec<(PathBuf, f32)> = results
            .into_iter()
            .take(10)
            .filter_map(|result| {
                relative_result_path(&locagent_repo, &canonical_repo, &result.path)
                    .map(|path| (path, result.score))
            })
            .collect();
        let our_paths: HashSet<PathBuf> =
            our_results.iter().map(|(path, _)| path.clone()).collect();

        let expected: HashSet<PathBuf> = top10_files.iter().cloned().collect();
        if expected.is_empty() {
            continue;
        }

        let overlap = our_paths.intersection(&expected).count();
        let overlap_pct = (overlap as f32 / expected.len() as f32) * 100.0;
        if overlap_pct < 75.0 {
            println!(
                "[PARITY] query='{query}' overlap={overlap_pct:.2}%\n  ours: {:?}\n  expected: {:?}",
                our_results, top10_files
            );
        }
        overlaps.push(overlap_pct);
    }

    assert!(
        !overlaps.is_empty(),
        "No overlap scores computed; verify golden fixtures contain results."
    );

    let avg_overlap = overlaps.iter().sum::<f32>() / overlaps.len() as f32;
    assert!(
        avg_overlap >= 75.0,
        "Average overlap {:.2}% < 75%",
        avg_overlap
    );
}

fn build_locagent_graph(repo_path: &Path) -> cds_index::graph::DependencyGraph {
    let config = GraphBuilderConfig::default();
    let builder = GraphBuilder::with_config(repo_path, config);
    let result = builder.build().expect("failed to build LocAgent graph");
    result.graph
}

fn relative_result_path(repo_root: &Path, canonical_root: &Path, path: &str) -> Option<PathBuf> {
    if path.is_empty() {
        return None;
    }
    let candidate = PathBuf::from(path);
    if let Ok(rel) = candidate.strip_prefix(canonical_root) {
        return Some(rel.to_path_buf());
    }
    if let Ok(rel) = candidate.strip_prefix(repo_root) {
        return Some(rel.to_path_buf());
    }
    if let Ok(canonical_candidate) = std::fs::canonicalize(&candidate) {
        if let Ok(rel) = canonical_candidate.strip_prefix(canonical_root) {
            return Some(rel.to_path_buf());
        }
        return Some(canonical_candidate);
    }
    Some(candidate)
}

fn repo_root() -> PathBuf {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ancestors: Vec<PathBuf> = crate_dir.ancestors().map(|p| p.to_path_buf()).collect();

    for candidate in &ancestors {
        if candidate.join("Cargo.toml").exists() && candidate.join("tmp/LocAgent").exists() {
            return candidate.clone();
        }
    }

    for candidate in &ancestors {
        if candidate.join("Cargo.toml").exists() {
            return candidate.clone();
        }
    }

    panic!(
        "Unable to locate workspace root from {}",
        crate_dir.display()
    );
}
