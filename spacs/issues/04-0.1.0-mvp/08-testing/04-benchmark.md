# Sub-Issue 08.04: Benchmark Testing - Performance & SWE-bench Evaluation

**Priority**: P1
**Owner**: QA Lead + Rust Dev 2
**Timing**: Phase 4, Week 10
**PRD Reference**: [PRD-08 §2.4, §4.4](../../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

## Objective

Validate performance targets (search latency p95 <500ms, index build <5min, File Acc@5 ≥75%) with SWE-bench Lite evaluation and comprehensive benchmarking.

## Key Implementations

### SWE-bench Lite Evaluation

```shell
#!/bin/bash
# tests/benchmarks/swe-bench/run-eval.sh

set -e

DATASET="czlll/SWE-bench_Lite"
SPLIT="test"
NUM_INSTANCES=300  # Full dataset, or 50 for subset
OUTPUT_DIR="tests/benchmarks/swe-bench/results"

mkdir -p "$OUTPUT_DIR"

echo "Running SWE-bench Lite evaluation on $NUM_INSTANCES instances..."

# Download dataset (cached)
python -c "from datasets import load_dataset; load_dataset('$DATASET', split='$SPLIT[:$NUM_INSTANCES]')"

# Run CDSAgent on all instances
bun run cds-agent/src/eval/swe_bench_runner.ts \
  --dataset "$DATASET" \
  --split "$SPLIT" \
  --limit "$NUM_INSTANCES" \
  --output "$OUTPUT_DIR/predictions.jsonl"

# Calculate metrics
python tests/benchmarks/swe-bench/calculate_metrics.py \
  --predictions "$OUTPUT_DIR/predictions.jsonl" \
  --output "$OUTPUT_DIR/metrics.json"

# Display results
cat "$OUTPUT_DIR/metrics.json" | jq '.'

echo "Evaluation complete! Results saved to $OUTPUT_DIR"
```

### SWE-bench Runner (TypeScript)

```typescript
// cds-agent/src/eval/swe_bench_runner.ts
import { loadDataset } from './dataset_loader';
import { AgentRunner } from '../../tests/test-utils/agent-runner';
import * as fs from 'fs';

interface SWEBenchInstance {
  instance_id: string;
  problem_statement: string;
  repo: string;
  base_commit: string;
  gold_files: string[];
}

async function runEvaluation(options: {
  dataset: string;
  split: string;
  limit: number;
  output: string;
}) {
  // Load dataset
  const instances: SWEBenchInstance[] = await loadDataset(
    options.dataset,
    options.split,
    options.limit
  );

  const agent = new AgentRunner({
    indexServiceUrl: process.env.INDEX_SERVICE_URL || 'http://localhost:3030',
    cliPath: '/usr/local/bin/cds',
  });

  const predictions = [];

  for (const [idx, instance] of instances.entries()) {
    console.log(`[${idx + 1}/${instances.length}] ${instance.instance_id}`);

    try {
      // Build index for this repo (if not cached)
      await buildIndexForRepo(instance.repo, instance.base_commit);

      // Run agent
      const result = await agent.runTask(instance.problem_statement, {
        maxIterations: 10,
        timeout: 300000,  // 5 minutes
      });

      predictions.push({
        instance_id: instance.instance_id,
        predicted_files: result.locations.map(loc => loc.file),
        gold_files: instance.gold_files,
      });

    } catch (error) {
      console.error(`Error on ${instance.instance_id}:`, error);
      predictions.push({
        instance_id: instance.instance_id,
        predicted_files: [],
        gold_files: instance.gold_files,
        error: error.message,
      });
    }
  }

  // Save predictions
  const output = predictions.map(p => JSON.stringify(p)).join('\n');
  fs.writeFileSync(options.output, output);

  console.log(`Predictions saved to ${options.output}`);
}

async function buildIndexForRepo(repo: string, commit: string) {
  // Clone repo and build index (with caching)
  // Implementation depends on Index Service API
}

// CLI entry point
if (import.meta.main) {
  const args = parseArgs(process.argv.slice(2));
  await runEvaluation(args);
}
```

### Metrics Calculation

```python
# tests/benchmarks/swe-bench/calculate_metrics.py
import json
from collections import defaultdict

def calculate_file_acc_at_k(predictions, k=5):
    """Calculate File Acc@k metric"""
    hits = 0
    total = 0

    for pred in predictions:
        if pred.get('error'):
            continue  # Skip failed instances

        pred_files = set(pred['predicted_files'][:k])
        gold_files = set(pred['gold_files'])

        if pred_files & gold_files:  # Intersection non-empty
            hits += 1
        total += 1

    return hits / total if total > 0 else 0.0

def calculate_line_acc_at_k(predictions, k=5):
    """Calculate Line Acc@k (if line ranges available)"""
    # v0.2.0 feature
    pass

def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--predictions', required=True)
    parser.add_argument('--output', required=True)
    args = parser.parse_args()

    # Load predictions
    predictions = []
    with open(args.predictions) as f:
        for line in f:
            predictions.append(json.loads(line))

    # Calculate metrics
    metrics = {
        'file_acc@1': calculate_file_acc_at_k(predictions, k=1),
        'file_acc@3': calculate_file_acc_at_k(predictions, k=3),
        'file_acc@5': calculate_file_acc_at_k(predictions, k=5),
        'total_instances': len(predictions),
        'successful_instances': sum(1 for p in predictions if not p.get('error')),
        'failed_instances': sum(1 for p in predictions if p.get('error')),
    }

    # Save metrics
    with open(args.output, 'w') as f:
        json.dump(metrics, f, indent=2)

    # Print summary
    print("\n=== SWE-bench Lite Results ===")
    print(f"Total instances: {metrics['total_instances']}")
    print(f"Successful: {metrics['successful_instances']}")
    print(f"Failed: {metrics['failed_instances']}")
    print(f"File Acc@1: {metrics['file_acc@1']:.2%}")
    print(f"File Acc@3: {metrics['file_acc@3']:.2%}")
    print(f"File Acc@5: {metrics['file_acc@5']:.2%}")

    # Check target
    if metrics['file_acc@5'] >= 0.75:
        print("\n✓ Target met: File Acc@5 ≥75%")
    else:
        print(f"\n✗ Target missed: File Acc@5 {metrics['file_acc@5']:.2%} < 75%")

if __name__ == '__main__':
    main()
```

### Search Latency Benchmark (Rust)

```rust
// tests/benchmarks/search_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn search_benchmark(c: &mut Criterion) {
    let index = setup_index();  // Pre-built index

    let queries = vec![
        ("simple", "function"),
        ("medium", "calculate sum average"),
        ("complex", "class inheritance polymorphism method override"),
    ];

    for (name, query) in queries {
        c.bench_with_input(
            BenchmarkId::new("search", name),
            &query,
            |b, query| {
                b.iter(|| {
                    index.search(black_box(query), 10)
                })
            },
        );
    }
}

fn traverse_benchmark(c: &mut Criterion) {
    let graph = setup_graph();

    c.bench_function("traverse_depth_2", |b| {
        b.iter(|| {
            graph.traverse(
                black_box("test.py::Calculator::add"),
                Direction::Outgoing,
                vec![EdgeType::Invoke],
                2  // max_depth
            )
        })
    });
}

fn setup_index() -> BM25Index {
    // Load pre-built index
    unimplemented!()
}

fn setup_graph() -> DependencyGraph {
    // Load pre-built graph
    unimplemented!()
}

criterion_group!(benches, search_benchmark, traverse_benchmark);
criterion_main!(benches);
```

### Memory Profiling

```rust
// tests/benchmarks/memory_bench.rs
use std::alloc::{GlobalAlloc, System, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

#[test]
fn test_index_memory_usage() {
    let before = ALLOCATED.load(Ordering::SeqCst);

    // Build index for medium-sized repo
    let repo_path = PathBuf::from("tests/fixtures/medium_repo");
    let builder = GraphBuilder::new();
    let graph = builder.build(&repo_path).unwrap();

    let after = ALLOCATED.load(Ordering::SeqCst);
    let used_mb = (after - before) as f64 / 1024.0 / 1024.0;

    println!("Memory usage: {:.2} MB", used_mb);

    // Target: <4GB for typical codebase
    assert!(used_mb < 4096.0, "Memory usage exceeds 4GB");
}
```

### Index Build Performance

```shell
#!/bin/bash
# tests/benchmarks/index_build_bench.sh

set -e

REPOS=(
  "tests/fixtures/small_repo"   # ~100 files
  "tests/fixtures/medium_repo"  # ~10K files
  "tests/fixtures/large_repo"   # ~50K files (optional)
)

for repo in "${REPOS[@]}"; do
  echo "Building index for $repo..."

  start=$(date +%s)

  cargo run --release --bin cds-index-build -- \
    --repo-path "$repo" \
    --language python

  end=$(date +%s)
  duration=$((end - start))

  echo "Index build completed in ${duration}s"

  # Target: <5 minutes (300s) for medium repo
  if [ "$repo" = "tests/fixtures/medium_repo" ] && [ $duration -gt 300 ]; then
    echo "ERROR: Index build took longer than 5 minutes"
    exit 1
  fi
done

echo "All index build benchmarks passed!"
```

### Throughput Testing

```rust
// tests/benchmarks/throughput_bench.rs
use std::time::Instant;

#[tokio::test]
async fn test_queries_per_second() {
    let index = setup_index();
    let num_queries = 1000;
    let queries: Vec<_> = (0..num_queries)
        .map(|i| format!("function_{}", i % 100))
        .collect();

    let start = Instant::now();

    for query in &queries {
        let _ = index.search(query, 10);
    }

    let duration = start.elapsed();
    let qps = num_queries as f64 / duration.as_secs_f64();

    println!("Queries per second: {:.2}", qps);

    // Target: >100 QPS
    assert!(qps > 100.0, "QPS {} is below 100", qps);
}
```

## CI/CD Integration

```yaml
# .github/workflows/benchmark.yml
name: Benchmark Testing

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 2 * * *'  # Nightly at 2 AM

jobs:
  benchmark:
    runs-on: ubuntu-latest
    timeout-minutes: 180

    steps:
      - uses: actions/checkout@v3

      - name: Download SWE-bench dataset
        run: |
          python -c "from datasets import load_dataset; load_dataset('czlll/SWE-bench_Lite', split='test[:50]')"

      - name: Run SWE-bench evaluation
        run: bash tests/benchmarks/swe-bench/run-eval.sh

      - name: Check File Acc@5 target
        run: |
          acc=$(jq -r '.file_acc@5' tests/benchmarks/swe-bench/results/metrics.json)
          echo "File Acc@5: $acc"
          if (( $(echo "$acc < 0.75" | bc -l) )); then
            echo "ERROR: File Acc@5 below 75% threshold"
            exit 1
          fi

      - name: Run latency benchmarks
        run: cargo bench --bench search_bench

      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: tests/benchmarks/swe-bench/results/
```

## Acceptance Criteria

- [ ] File Acc@5 ≥75% on SWE-bench Lite subset (50 instances minimum)
- [ ] Search latency p95 <500ms (measured with criterion)
- [ ] Index build <5 minutes for medium repo (~10K files)
- [ ] Memory usage <4GB for typical codebase
- [ ] Throughput >100 queries per second
- [ ] Benchmark suite runs in CI/CD nightly

**Related**: [00-overview.md](00-overview.md), [03-parity.md](03-parity.md), [../02-index-core/](../02-index-core/)
