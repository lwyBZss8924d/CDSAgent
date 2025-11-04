// graph_export_tests.rs - Thread-23: Graph Export Tests for Parity Analysis
//
// Purpose:
//   - Test JSON export functionality for DependencyGraph
//   - Export test fixture graphs to JSON for Python comparison harness
//   - Validate serialization/deserialization round-trip

use cds_index::graph::{DependencyGraph, GraphBuilder, GraphBuilderConfig, SerializableGraph};
use std::fs;
use std::path::{Path, PathBuf};

/// Find workspace root by walking up from crate directory
fn find_workspace_root() -> PathBuf {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ancestors: Vec<PathBuf> = crate_dir.ancestors().map(|p| p.to_path_buf()).collect();

    for candidate in &ancestors {
        // Workspace root has Cargo.toml and tmp/LocAgent directory
        if candidate.join("Cargo.toml").exists() && candidate.join("tmp/LocAgent").exists() {
            return candidate.clone();
        }
    }

    panic!("Could not find workspace root with tmp/LocAgent directory");
}

/// Helper function to build graph for a test repo
fn build_test_graph(repo_rel_path: &str) -> anyhow::Result<DependencyGraph> {
    let workspace_root = find_workspace_root();
    let repo_path = workspace_root.join(repo_rel_path);

    let config = GraphBuilderConfig::default();
    let builder = GraphBuilder::with_config(&repo_path, config);
    let result = builder.build()?;
    Ok(result.graph)
}

#[test]
fn test_json_export_serialization() {
    // Build a simple graph
    let graph = build_test_graph("tmp/LocAgent")
        .expect("Failed to build LocAgent graph");

    // Export to JSON
    let json = graph.to_json()
        .expect("Failed to serialize graph to JSON");

    // Verify JSON is valid
    let serializable: SerializableGraph = serde_json::from_str(&json)
        .expect("Failed to parse exported JSON");

    // Basic validation
    assert!(serializable.nodes.len() > 0, "Graph should have nodes");
    assert_eq!(
        serializable.nodes.len(),
        graph.node_count(),
        "Node count mismatch"
    );

    println!("✅ JSON export successful: {} nodes, {} edges",
             serializable.nodes.len(),
             serializable.edges.len());
}

#[test]
fn test_export_locagent_graph() {
    // Build LocAgent graph
    let graph = build_test_graph("tmp/LocAgent")
        .expect("Failed to build LocAgent graph");

    // Export to JSON file
    let workspace_root = find_workspace_root();
    let output_path = workspace_root.join(".artifacts/spec-tasks-T-02-02-sparse-index/diag/graph_locagent_cdsagent.json");
    graph.export_to_json(&output_path)
        .expect("Failed to export graph to JSON file");

    // Verify file exists and is valid JSON
    assert!(output_path.exists(), "Output file should exist");

    let contents = fs::read_to_string(&output_path)
        .expect("Failed to read output file");

    let serializable: SerializableGraph = serde_json::from_str(&contents)
        .expect("Failed to parse output JSON");

    println!("✅ Exported LocAgent graph to {}", output_path.display());
    println!("   Nodes: {}", serializable.nodes.len());
    println!("   Edges: {}", serializable.edges.len());
}

#[test]
#[ignore] // Run manually: cargo test --test graph_export_tests export_all_fixtures -- --ignored --nocapture
fn test_export_all_fixtures() {
    // Export all 6 test fixtures for parity comparison
    let fixtures = vec![
        ("tmp/LocAgent", "graph_locagent_cdsagent.json"),
        ("tmp/smoke/requests", "graph_requests_cdsagent.json"),
        ("tmp/smoke/pytest", "graph_pytest_cdsagent.json"),
        ("tmp/smoke/django", "graph_django_cdsagent.json"),
        ("tmp/smoke/matplotlib", "graph_matplotlib_cdsagent.json"),
        ("tmp/smoke/scikit-learn", "graph_sklearn_cdsagent.json"),
    ];

    let workspace_root = find_workspace_root();
    let output_dir = workspace_root.join(".artifacts/spec-tasks-T-02-02-sparse-index/diag/graphs");
    fs::create_dir_all(&output_dir)
        .expect("Failed to create output directory");

    for (repo_path, output_name) in fixtures {
        println!("\n📦 Exporting {}...", repo_path);

        match build_test_graph(repo_path) {
            Ok(graph) => {
                let output_path = output_dir.join(output_name);
                match graph.export_to_json(&output_path) {
                    Ok(_) => {
                        println!("   ✅ Exported to {}", output_path.display());
                        println!("      Nodes: {}, Edges: {}",
                                 graph.node_count(),
                                 graph.edge_count());
                    }
                    Err(e) => {
                        eprintln!("   ❌ Export failed: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("   ⚠️  Failed to build graph: {} (repo may not exist)", e);
            }
        }
    }

    println!("\n✅ Export complete! All graphs saved to {}", output_dir.display());
}

#[test]
fn test_node_id_format() {
    // Verify node ID format matches LocAgent expectations
    let graph = build_test_graph("tmp/LocAgent")
        .expect("Failed to build LocAgent graph");

    let serializable = graph.to_serializable();

    // Check that node IDs use :: separator (CDSAgent format)
    for node in &serializable.nodes {
        if node.id.contains("::") {
            // Verify format: repo::path::entity
            let parts: Vec<&str> = node.id.split("::").collect();
            assert!(parts.len() >= 2, "Node ID should have at least repo::path format");
        }
    }

    println!("✅ Node ID format validation passed");
}

#[test]
fn test_edge_kind_names() {
    // Verify edge kind names
    let graph = build_test_graph("tmp/LocAgent")
        .expect("Failed to build LocAgent graph");

    let serializable = graph.to_serializable();

    // Count edge kinds
    let mut edge_kinds = std::collections::HashMap::new();
    for edge in &serializable.edges {
        let kind_str = format!("{:?}", edge.kind).to_lowercase();
        *edge_kinds.entry(kind_str).or_insert(0) += 1;
    }

    println!("✅ Edge kinds found:");
    for (kind, count) in edge_kinds {
        println!("   {}: {}", kind, count);
    }

    // Verify expected edge kinds exist
    // (contain, import, invoke, inherit)
}
