mod support;

use cds_index::graph::builder::{GraphBuilder, GraphBuilderConfig};
use cds_index::index::{AnalyzerConfig, SparseIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use support::GoldenSearchQuery;

#[derive(Debug, Serialize, Deserialize)]
struct QueryDiagnostic {
    query_id: usize,
    query_text: String,
    overlap_at_10: f32,
    overlap_at_20: f32,
    overlap_at_50: f32,
    expected_top10: Vec<String>,
    cds_top10: Vec<String>,
    cds_top20: Vec<String>,
    cds_top50: Vec<String>,
    loc_only_top10: Vec<String>,
    cds_only_top10: Vec<String>,
    cds_scores: Vec<(String, f32)>,
}

#[test]
#[ignore = "Requires SMOKE_REPO_PATHS env var and golden fixtures"]
fn smoke_sparse_index_overlap_report() {
    let repo_paths = env::var("SMOKE_REPO_PATHS").expect(
        "Set SMOKE_REPO_PATHS=/abs/path/to/repo1,/abs/path/to/repo2 to run this smoke test",
    );
    let diag = env::var("SMOKE_OVERLAP_DIAG")
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false);

    let workspace_root = repo_root();
    let golden_dir = workspace_root.join("tests/fixtures/parity/golden_outputs");
    let locagent_queries_path = golden_dir.join("search_queries.jsonl");
    let locagent_queries = if locagent_queries_path.exists() {
        support::load_locagent_search_queries(&locagent_queries_path)
    } else {
        Vec::new()
    };

    let mut overlap_accumulator = 0.0f32;
    let mut overlap_repos = 0usize;

    for raw in repo_paths.split(',') {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }

        let repo_path = Path::new(trimmed);
        assert!(
            repo_path.exists(),
            "Smoke repo path {} is missing",
            repo_path.display()
        );

        let repo_name = repo_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("repo")
            .to_string();

        let builder = GraphBuilder::with_config(repo_path, GraphBuilderConfig::default());
        let graph_result = builder.build().unwrap_or_else(|err| {
            panic!("failed to build graph for {}: {err}", repo_path.display())
        });

        let temp_dir = TempDir::new().expect("failed to create temp directory for sparse index");
        let sparse_index = SparseIndex::from_graph(
            graph_result.graph,
            temp_dir.path(),
            AnalyzerConfig::default(),
        )
        .unwrap_or_else(|err| {
            panic!(
                "failed to build sparse index for {}: {err}",
                repo_path.display()
            )
        });

        if let Some(queries) = golden_queries_for_repo(&repo_name, &golden_dir, &locagent_queries) {
            let canonical_repo =
                std::fs::canonicalize(repo_path).unwrap_or_else(|_| repo_path.to_path_buf());
            let mut repo_overlaps = Vec::new();
            let mut diagnostics = Vec::new();

            for (query_id, GoldenSearchQuery { query, top10_files }) in queries.iter().enumerate() {
                // Fetch top-50 for overlap@20 and overlap@50 analysis
                let results = sparse_index.search(query, 50, None).unwrap_or_else(|err| {
                    panic!("search failed for '{query}' on {repo_name}: {err}")
                });

                let collected: Vec<_> = results.into_iter().collect();

                // Extract paths at different cutoffs
                let extract_paths = |limit: usize| -> HashSet<PathBuf> {
                    collected
                        .iter()
                        .take(limit)
                        .filter_map(|result| {
                            relative_result_path(repo_path, &canonical_repo, &result.path)
                        })
                        .collect()
                };

                let our_top10 = extract_paths(10);
                let our_top20 = extract_paths(20);
                let our_top50 = extract_paths(50);

                if our_top10.is_empty() || top10_files.is_empty() {
                    continue;
                }

                let expected: HashSet<PathBuf> = top10_files.iter().cloned().collect();

                // Calculate overlaps at different cutoffs
                let overlap_10 = our_top10.intersection(&expected).count();
                let overlap_20 = our_top20.intersection(&expected).count();
                let overlap_50 = our_top50.intersection(&expected).count();

                let overlap_pct_10 = (overlap_10 as f32 / expected.len() as f32) * 100.0;
                let overlap_pct_20 = (overlap_20 as f32 / expected.len() as f32) * 100.0;
                let overlap_pct_50 = (overlap_50 as f32 / expected.len() as f32) * 100.0;

                // Compute loc_only and cds_only for diagnostics
                let loc_only: Vec<String> = expected
                    .difference(&our_top10)
                    .map(|p| p.display().to_string())
                    .collect();
                let cds_only: Vec<String> = our_top10
                    .difference(&expected)
                    .map(|p| p.display().to_string())
                    .collect();

                if diag && overlap_pct_10 < 90.0 {
                    println!(
                        "[SMOKE-OVERLAP][DETAIL] repo={repo_name} overlap@10={overlap_pct_10:.2}% overlap@20={overlap_pct_20:.2}% overlap@50={overlap_pct_50:.2}% query=\"{query}\" expected_top10={} hits={}",
                        expected.len(),
                        our_top10.len()
                    );
                    for result in collected.iter().take(10) {
                        println!(
                            "[SMOKE-OVERLAP][HIT] repo={repo_name} score={:.4} path={}",
                            result.score, result.path
                        );
                    }
                    for path in &loc_only {
                        println!(
                            "[SMOKE-OVERLAP][LOC_ONLY] repo={repo_name} missing={path}"
                        );
                    }
                    for path in &cds_only {
                        println!(
                            "[SMOKE-OVERLAP][CDS_ONLY] repo={repo_name} extra={path}"
                        );
                    }
                }

                // Build diagnostic record
                if diag {
                    diagnostics.push(QueryDiagnostic {
                        query_id,
                        query_text: query.clone(),
                        overlap_at_10: overlap_pct_10,
                        overlap_at_20: overlap_pct_20,
                        overlap_at_50: overlap_pct_50,
                        expected_top10: expected.iter().map(|p| p.display().to_string()).collect(),
                        cds_top10: our_top10.iter().map(|p| p.display().to_string()).collect(),
                        cds_top20: our_top20.iter().map(|p| p.display().to_string()).collect(),
                        cds_top50: our_top50.iter().map(|p| p.display().to_string()).collect(),
                        loc_only_top10: loc_only,
                        cds_only_top10: cds_only,
                        cds_scores: collected.iter().take(10).map(|r| (r.path.clone(), r.score)).collect(),
                    });
                }

                repo_overlaps.push(overlap_pct_10);
            }

            // Write JSON diagnostics if enabled
            if diag && !diagnostics.is_empty() {
                let diag_dir = workspace_root.join(".artifacts/spec-tasks-T-02-02-sparse-index/diag");
                fs::create_dir_all(&diag_dir).ok();
                let diag_file = diag_dir.join(format!("{repo_name}_query_diagnostics.json"));
                if let Ok(json) = serde_json::to_string_pretty(&diagnostics) {
                    fs::write(&diag_file, json).ok();
                    println!("[SMOKE-OVERLAP][JSON] repo={repo_name} diagnostics written to {}", diag_file.display());
                }
            }

            if !repo_overlaps.is_empty() {
                let average =
                    repo_overlaps.iter().copied().sum::<f32>() / repo_overlaps.len() as f32;
                println!(
                    "[SMOKE-OVERLAP] repo={repo_name} queries={} avg_overlap={average:.2}%",
                    repo_overlaps.len()
                );
                overlap_accumulator += average;
                overlap_repos += 1;
            } else {
                println!(
                    "[SMOKE-OVERLAP] repo={repo_name} has golden fixtures but produced no overlap scores"
                );
            }
        } else {
            println!(
                "[SMOKE-OVERLAP] repo={repo_name} has no golden fixtures; skipping overlap computation"
            );
        }
    }

    if overlap_repos > 0 {
        let global_average = overlap_accumulator / overlap_repos as f32;
        println!(
            "[SMOKE-OVERLAP] global_average_overlap={global_average:.2}% across {overlap_repos} repo(s)"
        );
        if global_average < 75.0 {
            println!(
                "[SMOKE-OVERLAP][WARN] global average {:.2}% is below the 75% target",
                global_average
            );
        }
    } else {
        panic!(
            "No repositories with golden fixtures were evaluated. Provide fixtures or update harness."
        );
    }
}

fn golden_queries_for_repo(
    repo_name: &str,
    golden_dir: &Path,
    locagent_queries: &[GoldenSearchQuery],
) -> Option<Vec<GoldenSearchQuery>> {
    if repo_name.eq_ignore_ascii_case("locagent") && !locagent_queries.is_empty() {
        return Some(locagent_queries.to_vec());
    }

    repo_specific_golden_queries(repo_name, golden_dir)
}

fn repo_specific_golden_queries(
    repo_name: &str,
    golden_dir: &Path,
) -> Option<Vec<GoldenSearchQuery>> {
    let sanitized = repo_name.trim().to_ascii_lowercase().replace(' ', "_");
    let candidate = golden_dir.join(format!("{sanitized}.search_queries.jsonl"));
    if candidate.exists() {
        Some(support::load_locagent_search_queries(candidate))
    } else {
        None
    }
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
