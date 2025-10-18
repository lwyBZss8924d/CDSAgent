# Tasks: Testing & Quality Assurance - Comprehensive Test Coverage

**Work Stream**: Issue-08: Testing & Quality Assurance
**Issue Reference**: [../../issues/04-0.1.0-mvp/08-testing/](../../issues/04-0.1.0-mvp/08-testing/)
**PRD Reference**: [PRD-08: Testing & Quality Assurance](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

## Task Overview

| Task ID | Title | Owner | Status | Week |
|---------|-------|-------|--------|------|
| T-08-01 | Unit Tests - Component-level coverage | Rust Dev 1 + Rust Dev 2 | ☐ Not Started | W6-8 |
| T-08-02 | Integration Tests - E2E workflow validation | QA Lead + TypeScript Dev 1 | ☐ Not Started | W7 |
| T-08-03 | Parity Validation - LocAgent behavior alignment | Rust Dev 1 + QA Lead | ☐ Not Started | W9 |
| T-08-04 | Benchmark Testing - Performance & SWE-bench | QA Lead + Rust Dev 2 | ☐ Not Started | W10 |

## Dependencies

- **Prerequisite**: All implementation tasks (T-02, T-03, T-04) must be in progress
- **Continuous**: Testing runs throughout development

## Task Details

### T-08-01: Unit Tests

**File**: `T-08-01-unit-tests.md`
**Issue**: [Sub-Issue 08.01](../../issues/04-0.1.0-mvp/08-testing/01-unit-tests.md)
**PRD**: PRD-08 §2.1, §4.1

**Scope**:

- >80% code coverage for cds-index and cds-tools crates
- Property-based tests for graph invariants (proptest)
- Unit tests for all modules (parser, builder, index, service, CLI)

**Deliverables**:

- `tests/unit/graph/` - Graph builder tests
- `tests/unit/index/` - BM25 and name index tests
- `tests/unit/service/` - Service handler tests
- `tests/unit/cli/` - CLI command tests
- CI/CD workflow for coverage tracking

---

### T-08-02: Integration Tests

**File**: `T-08-02-integration-tests.md`
**Issue**: [Sub-Issue 08.02](../../issues/04-0.1.0-mvp/08-testing/02-integration.md)
**PRD**: PRD-08 §2.2, §4.2

**Scope**:

- End-to-end agent workflows (Index → CLI → Agent)
- Multi-step code localization scenarios
- Error handling and recovery
- Docker deployment validation

**Deliverables**:

- `tests/integration/agent-workflows.test.ts`
- `tests/integration/docker-deployment.test.ts`
- `tests/integration/cli_integration_test.sh`

---

### T-08-03: Parity Validation

**File**: `T-08-03-parity-validation.md`
**Issue**: [Sub-Issue 08.03](../../issues/04-0.1.0-mvp/08-testing/03-parity.md)
**PRD**: PRD-08 §2.3, §4.3

**Scope**:

- Graph structure comparison with LocAgent
- BM25 scoring parity (within 5%)
- Output format validation (JSON, tree)
- SWE-bench Lite subset evaluation (50 instances)

**Deliverables**:

- `tests/parity/graph_parity_test.rs`
- `tests/parity/bm25_parity_test.rs`
- `tests/parity/output-parity.test.ts`
- `tests/parity/swe_bench_parity.py`
- Golden outputs from LocAgent

---

### T-08-04: Benchmark Testing

**File**: `T-08-04-benchmark-testing.md`
**Issue**: [Sub-Issue 08.04](../../issues/04-0.1.0-mvp/08-testing/04-benchmark.md)
**PRD**: PRD-08 §2.4, §4.4

**Scope**:

- SWE-bench Lite evaluation (300 instances if time permits)
- Search latency benchmarking (p95 <500ms)
- Memory profiling (heap usage, index size)
- Throughput testing (>100 QPS)

**Deliverables**:

- `tests/benchmarks/swe-bench/run-eval.sh`
- `cds-agent/src/eval/swe_bench_runner.ts`
- `tests/benchmarks/search_bench.rs` (criterion)
- `tests/benchmarks/memory_bench.rs`
- Benchmark results and metrics

---

## Phase Milestones

### Week 6-7: Foundation Testing

- [ ] T-08-01: >80% unit coverage achieved
- [ ] T-08-02: Agent E2E tests pass (5 sample tasks)

**Validation**: All unit and integration tests pass in CI/CD

### Week 9-10: Parity & Benchmarks

- [ ] T-08-03: Parity validated on 50 SWE-bench instances
- [ ] T-08-04: File Acc@5 ≥75%, search latency p95 <500ms

**Validation**: Ready for v0.1.0 release

---

## Test Pyramid

```text
         /\
        /E2E\        ← T-08-02: Integration tests
       /------\
      /Parity  \     ← T-08-03: LocAgent comparison
     /----------\
    /   Unit     \   ← T-08-01: Component tests
   /--------------\
```

---

## Quick Links

- [Issue-08 Overview](../../issues/04-0.1.0-mvp/08-testing/00-overview.md)
- [PRD-08: Testing & Quality Assurance](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)
- [Parity Methodology](../../issues/04-0.1.0-mvp/06-refactor-parity.md)
