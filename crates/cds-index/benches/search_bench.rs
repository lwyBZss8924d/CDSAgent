//! Search latency benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn search_benchmark(c: &mut Criterion) {
    c.bench_function("search placeholder", |b| {
        b.iter(|| {
            // TODO: Benchmark actual search once implemented
            black_box(42)
        })
    });
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);
