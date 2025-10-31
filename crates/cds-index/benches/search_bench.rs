//! Search latency benchmarks for the sparse index.

use std::{hint::black_box, path::PathBuf};

use cds_index::graph::{DependencyGraph, GraphNode, GraphNodeIndex, NodeKind, SourceRange};
use cds_index::index::{NameIndex, NameIndexBuilder};
use criterion::{criterion_group, criterion_main, Criterion};

fn build_index(entity_count: usize) -> NameIndex {
    let builder = NameIndexBuilder::with_capacity(entity_count);
    let mut graph = DependencyGraph::new();

    for i in 0..entity_count {
        let dir_id = GraphNodeIndex::new(i * 3);
        let file_id = GraphNodeIndex::new(i * 3 + 1);
        let func_id = GraphNodeIndex::new(i * 3 + 2);

        builder.insert(
            format!("module{i:04}"),
            dir_id,
            NodeKind::Directory,
            Some(format!("pkg::module{i:04}")),
        );
        builder.insert(
            format!("module{i:04}.rs"),
            file_id,
            NodeKind::File,
            Some(format!("src/module_{i:04}.rs")),
        );
        builder.insert(
            format!("handle_module_{i:04}"),
            func_id,
            NodeKind::Function,
            Some(format!("pkg::module{i:04}::handle")),
        );

        // Maintain graph parity for future expansions that require it.
        let _ = graph.add_node(GraphNode::directory(
            format!("pkg::{i:04}"),
            format!("module{i:04}"),
            None,
        ));
        let _ = graph.add_node(GraphNode::file(
            format!("pkg::{i:04}::file"),
            format!("module{i:04}.rs"),
            PathBuf::from(format!("src/module_{i:04}.rs")),
        ));
        let _ = graph.add_node(GraphNode::entity(
            format!("pkg::{i:04}::handler"),
            NodeKind::Function,
            format!("handle_module_{i:04}"),
            PathBuf::from(format!("src/module_{i:04}.rs")),
            Some(SourceRange::new(10, 20)),
        ));
    }

    builder.finish()
}

fn name_index_query_benchmarks(c: &mut Criterion) {
    let index = build_index(4_096);

    c.bench_function("name_index_exact_match", |b| {
        b.iter(|| {
            let hits = index.exact_match("module2048.rs", None, 8);
            black_box(hits);
        });
    });

    c.bench_function("name_index_prefix_match", |b| {
        b.iter(|| {
            let hits = index.prefix_match("handle_module_2", Some(NodeKind::Function), 16);
            black_box(hits);
        });
    });
}

fn name_index_build_benchmark(c: &mut Criterion) {
    c.bench_function("name_index_build_1024", |b| {
        b.iter(|| {
            let index = build_index(1_024);
            black_box(index.stats());
        });
    });
}

criterion_group!(
    benches,
    name_index_query_benchmarks,
    name_index_build_benchmark
);
criterion_main!(benches);
