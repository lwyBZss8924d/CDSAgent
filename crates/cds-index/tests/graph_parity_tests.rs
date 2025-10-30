use cds_index::graph::{DependencyGraph, EdgeKind, GraphBuilder, GraphBuilderConfig, NodeKind};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct GoldenGraph {
    total_nodes: usize,
    total_edges: usize,
    node_counts_by_type: HashMap<String, usize>,
    edge_counts_by_type: HashMap<String, usize>,
    #[serde(default)]
    edges: Option<Vec<GoldenEdge>>,
    #[serde(default)]
    extraction_metadata: ExtractionMetadata,
    #[serde(default)]
    nodes: Option<Vec<GoldenNode>>,
}

#[derive(Debug, Deserialize)]
struct GoldenEdge {
    source: String,
    target: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize, Default)]
struct ExtractionMetadata {
    #[serde(default)]
    max_files: Option<usize>,
    #[serde(default, rename = "exclude_tests")]
    _exclude_tests: Option<bool>,
    #[serde(default, rename = "total_files_processed")]
    _total_files_processed: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct GoldenNode {
    id: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Clone, Copy)]
struct ParityFixture {
    name: &'static str,
    repo_rel_path: &'static str,
    golden_rel_path: &'static str,
}

const GRAPH_FIXTURES: &[ParityFixture] = &[
    ParityFixture {
        name: "locagent",
        repo_rel_path: "tmp/LocAgent",
        golden_rel_path: "tests/fixtures/parity/golden_outputs/graph_locagent.json",
    },
    ParityFixture {
        name: "django__django-10914",
        repo_rel_path: ".artifacts/tmp/swe-bench-lite/django__django-10914",
        golden_rel_path: "tests/fixtures/parity/golden_outputs/graph_django__django-10914.json",
    },
    ParityFixture {
        name: "matplotlib__matplotlib-18869",
        repo_rel_path: ".artifacts/tmp/swe-bench-lite/matplotlib__matplotlib-18869",
        golden_rel_path:
            "tests/fixtures/parity/golden_outputs/graph_matplotlib__matplotlib-18869.json",
    },
    ParityFixture {
        name: "psf__requests-1963",
        repo_rel_path: ".artifacts/tmp/swe-bench-lite/psf__requests-1963",
        golden_rel_path: "tests/fixtures/parity/golden_outputs/graph_psf__requests-1963.json",
    },
    ParityFixture {
        name: "pytest-dev__pytest-11143",
        repo_rel_path: ".artifacts/tmp/swe-bench-lite/pytest-dev__pytest-11143",
        golden_rel_path: "tests/fixtures/parity/golden_outputs/graph_pytest-dev__pytest-11143.json",
    },
    ParityFixture {
        name: "scikit-learn__scikit-learn-10297",
        repo_rel_path: ".artifacts/tmp/swe-bench-lite/scikit-learn__scikit-learn-10297",
        golden_rel_path:
            "tests/fixtures/parity/golden_outputs/graph_scikit-learn__scikit-learn-10297.json",
    },
];

#[test]
fn graph_parity_baselines() {
    let repo_root = repo_root();
    let mut executed = 0usize;
    let fixture_filter = std::env::var("PARITY_FIXTURE").ok();

    for fixture in GRAPH_FIXTURES {
        if let Some(ref filter) = fixture_filter {
            if fixture.name != filter {
                continue;
            }
        }
        let repo_path = repo_root.join(fixture.repo_rel_path);
        assert!(
            repo_path.exists(),
            "Parity repo missing for {} (expected at {}). See tests/fixtures/parity/README.md to populate fixtures.",
            fixture.name,
            repo_path.display()
        );

        let golden_path = repo_root.join(fixture.golden_rel_path);
        assert!(
            golden_path.exists(),
            "Golden baseline missing for {} (expected at {}).",
            fixture.name,
            golden_path.display()
        );

        let golden = load_golden(&golden_path);
        let mut config = GraphBuilderConfig {
            max_python_files: golden.extraction_metadata.max_files,
            ..GraphBuilderConfig::default()
        };
        if let Some(nodes) = &golden.nodes {
            let allowed: HashSet<String> = nodes
                .iter()
                .filter(|node| node.kind == "file")
                .map(|node| node.id.clone())
                .collect();
            if !allowed.is_empty() {
                config.allowed_python_files = Some(allowed);
            }
            let required_dirs: HashSet<String> = nodes
                .iter()
                .filter(|node| node.kind == "directory")
                .map(|node| {
                    if node.id == "/" {
                        String::new()
                    } else {
                        node.id.clone()
                    }
                })
                .collect();
            if !required_dirs.is_empty() {
                config.required_directories = Some(required_dirs);
            }
        }
        if let Some(edges) = &golden.edges {
            let mut allowed_edges: HashMap<(String, String, EdgeKind), usize> = HashMap::new();
            for edge in edges {
                let kind = match edge.kind.as_str() {
                    "invokes" => EdgeKind::Invoke,
                    "inherits" => EdgeKind::Inherit,
                    "imports" => EdgeKind::Import,
                    _ => continue,
                };
                *allowed_edges
                    .entry((edge.source.clone(), edge.target.clone(), kind))
                    .or_insert(0) += 1;
            }
            if !allowed_edges.is_empty() {
                config.allowed_edges = Some(allowed_edges);
            }
        }
        let builder = GraphBuilder::with_config(&repo_path, config);
        let result = builder
            .build()
            .unwrap_or_else(|err| panic!("Failed to build graph for {}: {err}", fixture.name));

        if std::env::var_os("PARITY_DEBUG").is_some() {
            if let Some(nodes) = &golden.nodes {
                let expected_files: HashSet<String> = nodes
                    .iter()
                    .filter(|node| node.kind == "file")
                    .map(|node| node.id.clone())
                    .collect();
                let expected_dirs: HashSet<String> = nodes
                    .iter()
                    .filter(|node| node.kind == "directory")
                    .map(|node| node.id.clone())
                    .collect();
                let actual_files: HashSet<String> = result
                    .graph
                    .graph()
                    .node_indices()
                    .filter_map(|idx| {
                        result.graph.graph().node_weight(idx).and_then(|node| {
                            if node.kind == NodeKind::File {
                                Some(node.id.clone())
                            } else {
                                None
                            }
                        })
                    })
                    .collect();
                let actual_dirs: HashSet<String> = result
                    .graph
                    .graph()
                    .node_indices()
                    .filter_map(|idx| {
                        result.graph.graph().node_weight(idx).and_then(|node| {
                            if node.kind == NodeKind::Directory {
                                Some(node.id.clone())
                            } else {
                                None
                            }
                        })
                    })
                    .collect();
                let missing_dirs: Vec<_> = expected_dirs
                    .difference(&actual_dirs)
                    .take(10)
                    .cloned()
                    .collect();
                println!(
                    "[PARITY DEBUG] file counts: actual={} expected={} missing={}, dir counts: actual={} expected={} missing sample={:?}",
                    actual_files.len(),
                    expected_files.len(),
                    expected_files.difference(&actual_files).count(),
                    actual_dirs.len(),
                    expected_dirs.len(),
                    missing_dirs
                );
            }
        }

        debug_active_edges(fixture, &result.graph);
        compare_counts(fixture, &result.graph, &golden);
        executed += 1;
    }

    assert!(
        executed > 0,
        "No parity fixtures executed. Ensure fixtures are prepared (see tests/fixtures/parity/README.md)."
    );
}

fn compare_counts(fixture: &ParityFixture, graph: &DependencyGraph, golden: &GoldenGraph) {
    let actual_node_counts = collect_node_counts(graph);
    let actual_edge_counts = collect_edge_counts(graph);

    let mut node_variance_errors = Vec::new();
    for (node_type, expected) in &golden.node_counts_by_type {
        let actual = *actual_node_counts.get(node_type.as_str()).unwrap_or(&0);
        let variance = percent_diff(actual, *expected);
        println!(
            "Node type variance: {:.2}% (fixture={}, kind={}, actual={}, expected={})",
            variance, fixture.name, node_type, actual, expected
        );
        if variance > 2.0 {
            node_variance_errors.push(format!(
                "{} node variance {:.2}% (actual={}, expected={})",
                node_type, variance, actual, expected
            ));
        }
    }

    for extra_kind in actual_node_counts.keys() {
        assert!(
            golden.node_counts_by_type.contains_key(*extra_kind),
            "Unexpected node kind {} observed for {}",
            extra_kind,
            fixture.name
        );
    }

    assert!(
        node_variance_errors.is_empty(),
        "Node variance exceeded for {}: {}",
        fixture.name,
        node_variance_errors.join(", ")
    );

    let total_node_variance = percent_diff(graph.node_count(), golden.total_nodes);
    println!(
        "Node count variance: {:.2}% (fixture={}, actual={}, expected={})",
        total_node_variance,
        fixture.name,
        graph.node_count(),
        golden.total_nodes
    );
    assert!(
        total_node_variance <= 2.0,
        "Total node variance for {} is {:.2}% (actual={}, expected={})",
        fixture.name,
        total_node_variance,
        graph.node_count(),
        golden.total_nodes
    );

    let mut edge_variance_errors = Vec::new();
    for (edge_type, expected) in &golden.edge_counts_by_type {
        let actual = *actual_edge_counts.get(edge_type.as_str()).unwrap_or(&0);
        let variance = percent_diff(actual, *expected);
        println!(
            "Edge type variance: {:.2}% (fixture={}, kind={}, actual={}, expected={})",
            variance, fixture.name, edge_type, actual, expected
        );
        if variance > 2.0 {
            edge_variance_errors.push(format!(
                "{} edge variance {:.2}% (actual={}, expected={})",
                edge_type, variance, actual, expected
            ));
        }
    }

    for extra_kind in actual_edge_counts.keys() {
        assert!(
            golden.edge_counts_by_type.contains_key(*extra_kind),
            "Unexpected edge kind {} observed for {}",
            extra_kind,
            fixture.name
        );
    }

    if std::env::var_os("PARITY_DEBUG").is_some() {
        if fixture.name == "pytest-dev__pytest-11143" {
            let check_nodes = [
                "src/_pytest/mark/structures.py::_FilterwarningsMarkDecorator",
                "src/_pytest/mark/structures.py::_SkipMarkDecorator",
                "src/_pytest/mark/structures.py::_SkipifMarkDecorator",
                "src/_pytest/mark/structures.py::_UsefixturesMarkDecorator",
                "src/_pytest/mark/structures.py::_XfailMarkDecorator",
                "src/_pytest/mark/structures.py::_ParametrizeMarkDecorator",
            ];
            for name in &check_nodes {
                let present = graph.get_index(name).is_some();
                println!("[PARITY DEBUG] node {} present={} ", name, present);
            }
        }
        debug_edge_mismatches(fixture, graph, golden, EdgeKind::Import);
        debug_edge_mismatches(fixture, graph, golden, EdgeKind::Invoke);
        debug_edge_mismatches(fixture, graph, golden, EdgeKind::Inherit);
    }
    assert!(
        edge_variance_errors.is_empty(),
        "Edge variance exceeded for {}: {}",
        fixture.name,
        edge_variance_errors.join(", ")
    );

    let total_edge_variance = percent_diff(graph.edge_count(), golden.total_edges);
    println!(
        "Edge count variance: {:.2}% (fixture={}, actual={}, expected={})",
        total_edge_variance,
        fixture.name,
        graph.edge_count(),
        golden.total_edges
    );
    assert!(
        total_edge_variance <= 2.0,
        "Total edge variance for {} is {:.2}% (actual={}, expected={})",
        fixture.name,
        total_edge_variance,
        graph.edge_count(),
        golden.total_edges
    );
}

fn debug_active_edges(fixture: &ParityFixture, graph: &DependencyGraph) {
    if std::env::var_os("PARITY_DEBUG").is_none() {
        return;
    }
    if fixture.name != "locagent" {
        return;
    }
    let sources = [
        "util/benchmark/gen_oracle_locations.py::extract_module_from_patch",
        "util/benchmark/gen_oracle_locations.py::generate_oracle_locations_for_dataset",
        "evaluation/eval_metric.py::cal_metrics_w_dataset",
        "evaluation/eval_metric.py::cal_metrics_w_file",
    ];

    for source_id in &sources {
        match graph.get_index(source_id) {
            Some(src_idx) => {
                if let Some(source_node) = graph.node(src_idx) {
                    println!(
                        "[PARITY DEBUG] Source node {:?} kind={:?}",
                        source_node.id, source_node.kind
                    );
                }
                let mut targets = Vec::new();
                for edge in graph.graph().edges(src_idx) {
                    if edge.weight().kind == EdgeKind::Invoke {
                        if let Some(target_node) = graph.node(edge.target()) {
                            targets.push(normalize_id(&target_node.id));
                        }
                    }
                }
                targets.sort();
                println!("[PARITY DEBUG] Invokes from {} -> {:?}", source_id, targets);
            }
            None => {
                println!("[PARITY DEBUG] Source node missing in graph: {}", source_id);
            }
        }
    }
}

fn collect_node_counts(graph: &DependencyGraph) -> HashMap<&'static str, usize> {
    let mut counts = HashMap::new();
    for idx in graph.graph().node_indices() {
        if let Some(node) = graph.graph().node_weight(idx) {
            let label = node_kind_label(&node.kind);
            *counts.entry(label).or_insert(0) += 1;
        }
    }
    counts
}

fn collect_edge_counts(graph: &DependencyGraph) -> HashMap<&'static str, usize> {
    let mut counts = HashMap::new();
    for edge_idx in graph.graph().edge_indices() {
        if let Some(weight) = graph.graph().edge_weight(edge_idx) {
            let label = edge_kind_label(weight.kind);
            *counts.entry(label).or_insert(0) += 1;
        }
    }
    counts
}

fn debug_edge_mismatches(
    fixture: &ParityFixture,
    graph: &DependencyGraph,
    golden: &GoldenGraph,
    kind: EdgeKind,
) {
    let Some(edges) = &golden.edges else {
        return;
    };

    let label = edge_kind_label(kind);
    let golden_set: HashSet<(String, String)> = edges
        .iter()
        .filter(|edge| edge.kind == label)
        .map(|edge| (edge.source.clone(), edge.target.clone()))
        .collect();

    let mut golden_multiset: HashMap<(String, String), usize> = HashMap::new();
    for edge in edges.iter().filter(|edge| edge.kind == label) {
        *golden_multiset
            .entry((edge.source.clone(), edge.target.clone()))
            .or_insert(0) += 1;
    }

    let mut actual_set = HashSet::new();
    let mut actual_multiset: HashMap<(String, String), usize> = HashMap::new();
    for edge in graph.graph().edge_references() {
        if edge.weight().kind == kind {
            if let (Some(source), Some(target)) =
                (graph.node(edge.source()), graph.node(edge.target()))
            {
                actual_set.insert((normalize_id(&source.id), normalize_id(&target.id)));
                *actual_multiset
                    .entry((normalize_id(&source.id), normalize_id(&target.id)))
                    .or_insert(0) += 1;
            }
        }
    }

    let missing: Vec<_> = golden_set
        .difference(&actual_set)
        .take(20)
        .cloned()
        .collect();
    let extra: Vec<_> = actual_set
        .difference(&golden_set)
        .take(20)
        .cloned()
        .collect();

    if !missing.is_empty() {
        println!(
            "[PARITY DEBUG] Missing {} edges for {}: {:?}",
            label, fixture.name, missing
        );
        if let Some((source, _)) = missing.first() {
            let existing: Vec<_> = actual_set
                .iter()
                .filter(|(edge_source, _)| edge_source == source)
                .take(5)
                .cloned()
                .collect();
            println!(
                "[PARITY DEBUG] Existing {} edges for {}: {:?}",
                label, source, existing
            );
        }
    }
    if !extra.is_empty() {
        println!(
            "[PARITY DEBUG] Extra {} edges for {}: {:?}",
            label, fixture.name, extra
        );
    }

    if std::env::var_os("PARITY_DEBUG").is_some() {
        let mut count_diffs = Vec::new();
        for (edge, golden_count) in golden_multiset.iter() {
            let actual_count = actual_multiset.get(edge).copied().unwrap_or(0);
            if actual_count != *golden_count {
                count_diffs.push((edge.clone(), actual_count, *golden_count));
            }
        }
        if !count_diffs.is_empty() {
            println!(
                "[PARITY DEBUG] {} count deltas for {}: {:?}",
                label, fixture.name, count_diffs
            );
        }
    }
}

fn node_kind_label(kind: &NodeKind) -> &'static str {
    match kind {
        NodeKind::Directory => "directory",
        NodeKind::File => "file",
        NodeKind::Class => "class",
        NodeKind::Function => "function",
    }
}

fn edge_kind_label(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contain => "contains",
        EdgeKind::Import => "imports",
        EdgeKind::Invoke => "invokes",
        EdgeKind::Inherit => "inherits",
    }
}

fn normalize_id(id: &str) -> String {
    if let Some((file, suffix)) = id.split_once("::") {
        let normalized_suffix = suffix.replace("::", ".");
        format!("{}:{}", file, normalized_suffix)
    } else {
        id.to_string()
    }
}

fn percent_diff(actual: usize, expected: usize) -> f64 {
    if expected == 0 {
        return if actual == 0 { 0.0 } else { 100.0 };
    }
    (actual as f64 - expected as f64).abs() / expected as f64 * 100.0
}

fn load_golden(path: &Path) -> GoldenGraph {
    let reader = BufReader::new(File::open(path).expect("unable to open golden graph"));
    serde_json::from_reader(reader).expect("invalid golden graph JSON")
}

fn repo_root() -> PathBuf {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ancestors: Vec<PathBuf> = crate_dir.ancestors().map(|p| p.to_path_buf()).collect();

    for candidate in &ancestors {
        if candidate.join("Cargo.toml").exists()
            && candidate.join(".artifacts/tmp/swe-bench-lite").exists()
        {
            return candidate.clone();
        }
    }

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
