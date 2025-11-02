# PRD-08: Testing & Quality Assurance

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Document Overview

### 1.1 Purpose

Define comprehensive testing strategy, quality metrics, and validation procedures to ensure CDSAgent maintains LocAgent's research fidelity while achieving performance and reliability targets.

### 1.2 Scope

- Unit testing (Rust crates, TypeScript modules)
- Integration testing (CLI, agent workflows, API contracts)
- Benchmark validation against LocAgent results
- Performance testing and profiling
- Quality metrics and acceptance criteria

### 1.3 LocAgent Validation Baseline

- **Benchmark**: SWE-bench Lite (300 instances)
- **Metrics**: File-level accuracy, function-level accuracy, NDCG@5
- **Reference**: LocAgent paper Table 2 (published results)

---

## 2. Testing Strategy

### 2.1 Test Pyramid

```text
         ┌─────────────┐
         │   E2E Tests │  10% - Agent workflows on real repos
         ├─────────────┤
         │  Integration│  30% - CLI + Index Service + API
         ├─────────────┤
         │   Unit Tests│  60% - Individual functions/modules
         └─────────────┘
```

**Coverage Targets**:

- Unit tests: >80% line coverage
- Integration tests: All happy paths + critical error cases
- E2E tests: ≥10 representative scenarios

---

## 3. Unit Testing

### 3.1 Rust Crate Tests

#### cds_graph (Graph Construction)

**Test Cases**:

```rust
// tests/graph_builder_test.rs

#[test]
fn test_parse_python_file_extracts_classes() {
    let source = r#"
class MyClass:
    def method(self):
        pass
"#;
    let entities = parse_python_file(source).unwrap();
    assert_eq!(entities.len(), 2); // Class + method
    assert_eq!(entities[0].entity_type, EntityType::Class);
    assert_eq!(entities[0].name, "MyClass");
}

#[test]
fn test_build_graph_creates_contain_edges() {
    let graph = build_repo_graph("tests/fixtures/sample_repo").unwrap();
    let contain_edges: Vec<_> = graph.edges.iter()
        .filter(|e| e.relation == RelationType::Contain)
        .collect();
    assert!(contain_edges.len() > 0);
}

#[test]
fn test_extract_invoke_edges() {
    let source = r#"
def foo():
    bar()
    baz()
"#;
    let graph = parse_and_build_graph(source).unwrap();
    let invoke_edges: Vec<_> = graph.edges.iter()
        .filter(|e| e.relation == RelationType::Invoke)
        .collect();
    assert_eq!(invoke_edges.len(), 2); // foo → bar, foo → baz
}
```

**Validation**:

- Compare entity counts with LocAgent on same files
- Verify edge types match expectations

#### cds_sparse_index (Search & Indexing)

**Test Cases**:

```rust
// tests/search_test.rs

#[test]
fn test_bm25_search_ranks_correctly() {
    let index = build_test_index();
    let results = index.search("sanitize input");

    // Expect entity with "sanitize_input" in name to rank higher
    assert_eq!(results[0].name, "sanitize_input");
    assert!(results[0].score > results[1].score);
}

#[test]
fn test_hierarchical_search_prefers_name_match() {
    let results = search_entity("AuthService", 10);

    // Upper index (name match) should score 1.0
    let exact_match = results.iter().find(|r| r.name == "AuthService").unwrap();
    assert_eq!(exact_match.score, 1.0);
}

#[test]
fn test_tokenizer_splits_camel_case() {
    let tokens = tokenize_code("getUserName");
    assert_eq!(tokens, vec!["get", "user", "name"]);
}
```

#### cds_traversal (Graph Navigation)

**Test Cases**:

```rust
// tests/bfs_test.rs

#[test]
fn test_bfs_respects_depth_limit() {
    let graph = build_test_graph();
    let result = traverse_graph(&graph, "root", 2, &[], &[]);

    // Nodes at depth 3 should not be included
    for node in result {
        assert!(graph.depth(&node) <= 2);
    }
}

#[test]
fn test_bfs_filters_by_relation_type() {
    let result = traverse_graph(&graph, "func_a", 1, &[RelationType::Invoke], &[]);

    // Only invoke edges should be followed
    for node in result {
        let edge = graph.find_edge("func_a", &node).unwrap();
        assert_eq!(edge.relation, RelationType::Invoke);
    }
}
```

### 3.2 TypeScript Module Tests

#### Agent Integration

**Test Cases**:

```typescript
// cds-agent/src/__tests__/agent-config.test.ts

import { createCDSAgent } from '../agent-config';

test('createCDSAgent initializes with correct config', async () => {
  const agent = await createCDSAgent({
    indexPath: '/tmp/test_index',
    allowedCommands: ['cds'],
    maxSearchResults: 10,
    maxTraverseDepth: 2,
  });

  expect(agent).toBeDefined();
  expect(agent.config.model).toBe('claude-sonnet-4-5');
});

test('preToolUse hook injects index path', async () => {
  const context = { toolName: 'bash', toolInput: { command: 'cds search "test"' } };
  const result = await preToolUse(context);

  expect(result.allow).toBe(true);
  expect(result.toolInput.env.GRAPH_INDEX_DIR).toBeDefined();
});
```

---

## 4. Integration Testing

### 4.1 CLI Integration Tests

**Test Scenarios**:

```shell
# tests/integration/cli_test.sh

# Test 1: Search returns valid JSON
output=$(cds search "sanitize" --format json)
echo "$output" | jq empty || exit 1  # Validate JSON

# Test 2: Traverse from search results
entity_id=$(echo "$output" | jq -r '.results[0].entity_id')
cds traverse "$entity_id" --depth 1 --format json | jq empty || exit 1

# Test 3: Retrieve entity
cds retrieve "$entity_id" --format json | jq '.entities[0].code' | grep -q "def" || exit 1

# Test 4: Piping workflow
cds search "auth" | jq -r '.results[].entity_id' | xargs cds retrieve | jq '.entities | length' | grep -q "[0-9]" || exit 1
```

### 4.2 API Contract Tests

**Test API Stability**:

```typescript
// cds-agent/src/__tests__/api-contract.test.ts

import { z } from 'zod';
import { EntitySchema } from '../schemas';

test('search API returns valid schema', async () => {
  const response = await fetch('http://localhost:9876/rpc', {
    method: 'POST',
    body: JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      method: 'search_entities',
      params: { query: 'test', limit: 10 }
    })
  });

  const data = await response.json();

  // Validate with Zod schema
  const parsed = EntitySchema.array().parse(data.result.entities);
  expect(parsed.length).toBeGreaterThan(0);
});
```

### 4.3 Agent Workflow Tests

**End-to-End Agent Scenarios**:

```typescript
// cds-agent/src/__tests__/workflow.test.ts

test('agent can locate XSS vulnerability', async () => {
  const agent = await createCDSAgent(testConfig);

  const response = await agent.query({
    userMessage: 'Find code related to XSS sanitization in user profile',
    maxIterations: 10
  });

  // Verify agent used tools correctly
  expect(response.toolCalls).toContainEqual(
    expect.objectContaining({ toolName: 'bash', command: expect.stringContaining('cds search') })
  );

  // Verify final answer contains file paths
  expect(response.finalAnswer).toMatch(/\.py:\d+/);
});
```

---

## 5. Benchmark Validation

### 5.1 LocAgent Comparison Tests

**Objective**: Ensure CDSAgent reproduces LocAgent's published results.

**Dataset**: SWE-bench Lite (300 instances)

**Metrics** (from LocAgent Table 2):

- File-level Acc@5: ≥80%
- Function-level Acc@10: ≥65%
- NDCG@5: ≥0.70

**Test Procedure**:

```shell
# 1. Index SWE-bench repos
for repo in $(cat swebench_lite_repos.txt); do
  cds init $repo --output /data/index/$repo
done

# 2. Run localization on all instances
python eval/run_cdsagent.py --dataset swebench_lite --output results/cdsagent.jsonl

# 3. Compute metrics
python eval/compute_metrics.py --predictions results/cdsagent.jsonl --ground-truth swebench_lite_gold.jsonl

# Expected output:
# File Acc@5: 0.82 (target: ≥0.80)
# Func Acc@10: 0.68 (target: ≥0.65)
# NDCG@5: 0.72 (target: ≥0.70)
```

**Validation Scripts** (Python):

```python
# eval/compute_metrics.py
def compute_accuracy_at_k(predictions, ground_truth, k=5):
    """Compare with LocAgent's evaluation script."""
    correct = 0
    for pred, gold in zip(predictions, ground_truth):
        if any(loc in gold.locations for loc in pred.top_k_locations[:k]):
            correct += 1
    return correct / len(predictions)

def compute_ndcg(predictions, ground_truth, k=5):
    """NDCG metric from LocAgent paper."""
    # Implementation from LocAgent eval/eval_metric.py
```

### 5.2 Ablation Studies

**Validate Each Component**:

| Component Disabled | Expected Impact | Pass Criteria |
|-------------------|----------------|---------------|
| Upper index (name/ID) | -15% Acc@5 | Matches LocAgent ablation |
| BM25 lower index | -25% Acc@5 | Matches LocAgent ablation |
| Traverse tool | -20% Func Acc@10 | Matches LocAgent ablation |

---

## 6. Performance Testing

### 6.1 Latency Benchmarks

**Tool**: `criterion` (Rust), `hyperfine` (CLI)

**Benchmarks**:

```rust
// benches/search_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_search(c: &mut Criterion) {
    let index = load_test_index();

    c.bench_function("search_query", |b| {
        b.iter(|| {
            index.search(black_box("sanitize input"))
        });
    });
}

criterion_group!(benches, bench_search);
criterion_main!(benches);
```

**CLI Benchmarks**:

```shell
# Measure search latency
hyperfine --warmup 3 'cds search "sanitize" --format json'

# Expected: Mean < 500ms (p95 < 1s)
```

### 6.2 Load Testing

**Concurrent Queries**:

```shell
# Apache Bench (ab) for load testing daemon
ab -n 1000 -c 10 -p search_request.json -T application/json http://localhost:9876/rpc

# Expected: 95th percentile < 1s, no errors
```

### 6.3 Memory Profiling

**Tool**: `heaptrack` (Linux), `valgrind --tool=massif`

```shell
# Profile memory usage during indexing
valgrind --tool=massif --massif-out-file=massif.out cds init /large/repo

# Analyze
ms_print massif.out

# Expected: Peak memory < 4GB for 50K files
```

---

## 7. Quality Metrics

### 7.1 Code Quality

| Metric | Target | Tool |
|--------|--------|------|
| Test coverage | >80% | `cargo tarpaulin` |
| Clippy warnings | 0 | `cargo clippy` |
| Rustfmt compliance | 100% | `cargo fmt --check` |
| TypeScript errors | 0 | `tsc --noEmit` |

### 7.2 Documentation

- [ ] All public APIs documented (rustdoc/TSDoc)
- [ ] README with usage examples
- [ ] Architecture diagrams (text-based)
- [ ] Troubleshooting guide

### 7.3 Regression Prevention

**CI/CD Checks** (GitHub Actions):

```yaml
name: CDSAgent CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run unit tests
        run: cargo test --all-features

      - name: Run integration tests
        run: ./tests/integration/run_all.sh

      - name: Check code coverage
        run: cargo tarpaulin --out Xml
        # Fail if coverage < 80%

      - name: Benchmark regression
        run: cargo bench --no-fail-fast
        # Compare with baseline (fail if >10% slower)
```

---

## 8. Acceptance Criteria

### 8.1 Must-Pass Criteria (v1.0)

- [ ] All unit tests pass (Rust + TypeScript)
- [ ] Integration tests pass (CLI, API, agent workflows)
- [ ] Benchmark validation: File Acc@5 ≥ 80%
- [ ] Performance: Search latency p95 < 1s
- [ ] Code coverage ≥ 80%
- [ ] Zero critical security vulnerabilities

### 8.2 Quality Gates

Before merging to `main`:

1. All tests pass
2. Code review approved
3. Benchmark regression < 5%
4. Documentation updated

Before release (v1.0):

1. SWE-bench Lite validation complete
2. Performance benchmarks meet targets
3. User acceptance testing (5 beta users)

---

## 9. Test Data

### 9.1 Fixtures

**Synthetic Test Repos**:

```tree
tests/fixtures/
  ├── simple_repo/        # 10 files, basic structure
  ├── nested_classes/     # Complex inheritance
  ├── large_repo/         # 1K files for performance tests
  └── swebench_sample/    # 5 instances from SWE-bench Lite
```

### 9.2 Golden Outputs

**Store expected outputs for regression tests**:

```tree
tests/golden/
  ├── search_sanitize.json         # Expected search results
  ├── traverse_func_abc.json       # Expected traversal subgraph
  └── retrieve_entity_123.json     # Expected entity details
```

**Update Golden Files**:

```shell
# Regenerate golden outputs (after verified changes)
./tests/update_golden.sh
```

---

## 10. Open Questions

1. **Flaky Tests**: Strategy for handling non-deterministic agent behavior? (Seed LLM, mock responses)
2. **Benchmark Drift**: LocAgent results evolve with dataset updates—how to track? (Version benchmarks)
3. **Performance Baselines**: Store historical benchmark results for regression detection? (Use criterion baselines)

---

**Status**: Ready for test implementation. Critical for validating CDSAgent against LocAgent research.
