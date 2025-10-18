# Issue-08: Testing & Quality Assurance - Comprehensive Test Coverage

**Priority**: P1 (Critical Path - Quality Gates)
**Status**: ☐ Not Started
**Owner**: QA Lead + All Devs
**PRD Reference**: [PRD-08: Testing & Quality Assurance](../../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

---

## Overview

Testing & Quality Assurance establishes a comprehensive test suite across unit, integration, parity validation, and benchmark testing to ensure CDSAgent meets LocAgent's accuracy and performance standards. This component provides the quality gates required for production readiness.

## Objective

Deliver a multi-layered test strategy that:

- Provides unit test coverage (>80%) for all core modules in Rust
- Implements integration tests for end-to-end workflows (Index → CLI → Agent)
- Validates parity with LocAgent on SWE-bench Lite subset (50 instances)
- Benchmarks performance targets (File Acc@5 ≥75%, search latency <500ms)
- Establishes continuous testing infrastructure (CI/CD pipeline)

## Dependencies

- **Requires**: All implementation issues ([02-index-core/](../02-index-core/), [03-cli-tools/](../03-cli-tools/), [04-agent-integration/](../04-agent-integration/))
- **Coordinates With**: Deployment ([07-deployment/](../07-deployment/))
- **Timing**: Phase 3-4 (Weeks 6-10, continuous)

---

## Sub-Issues Breakdown

### 1. [Unit Tests](01-unit-tests.md) - **P1, Weeks 6-8**

**Owner**: Rust Dev 1 + Rust Dev 2
**Scope**: Component-level testing for Rust modules

- Graph builder (tree-sitter parsing, graph construction)
- Sparse index (name search, BM25 retrieval)
- Service layer (JSON-RPC handlers, error handling)
- CLI commands (argument parsing, output formatting)

**Acceptance**:

- [ ] >80% code coverage for cds-index crate
- [ ] >80% code coverage for cds-tools crate
- [ ] All unit tests pass in CI/CD pipeline
- [ ] Property-based tests for graph operations (proptest)

---

### 2. [Integration Tests](02-integration.md) - **P1, Week 7**

**Owner**: QA Lead + TypeScript Dev 1
**Scope**: End-to-end workflow validation

- Index Service → CLI → Agent pipeline
- Multi-step agent workflows (search → traverse → retrieve)
- Error handling and recovery
- Docker deployment validation

**Acceptance**:

- [ ] Agent completes 5 sample code localization tasks
- [ ] Integration tests run in Docker Compose environment
- [ ] Error scenarios tested (service unavailable, invalid query)
- [ ] Performance under load (10 concurrent requests)

---

### 3. [Parity Validation](03-parity.md) - **P1, Week 9**

**Owner**: Rust Dev 1 + QA Lead
**Scope**: LocAgent behavior alignment

- Algorithm validation (graph traversal, BM25 scoring)
- Output format parity (JSON structure, tree format)
- Accuracy comparison on SWE-bench Lite subset (50 instances)
- Regression detection (golden outputs)

**Acceptance**:

- [ ] Graph structure matches LocAgent on 10 sample repos
- [ ] BM25 scores within 5% of LocAgent
- [ ] File Acc@5 ≥75% on SWE-bench Lite subset
- [ ] Zero regressions vs. LocAgent golden outputs

---

### 4. [Benchmark Testing](04-benchmark.md) - **P1, Week 10**

**Owner**: QA Lead + Rust Dev 2
**Scope**: Performance validation and SWE-bench evaluation

- SWE-bench Lite evaluation (300 instances, if time permits)
- Search latency benchmarking (p50, p95, p99)
- Memory profiling (heap usage, index size)
- Throughput testing (queries per second)

**Acceptance**:

- [ ] File Acc@5 ≥75% on SWE-bench Lite subset
- [ ] Search latency p95 <500ms
- [ ] Index build <5 minutes for medium repo (10K files)
- [ ] Memory usage <4GB for typical codebase

---

## Testing Project Structure

```tree
tests/
├── unit/                       # Unit tests (Rust)
│   ├── graph/
│   │   ├── parser_tests.rs     # Tree-sitter parsing
│   │   ├── builder_tests.rs    # Graph construction
│   │   └── traversal_tests.rs  # Dependency traversal
│   ├── index/
│   │   ├── name_index_tests.rs # Name/ID search
│   │   └── bm25_tests.rs       # BM25 retrieval
│   ├── service/
│   │   ├── handler_tests.rs    # JSON-RPC handlers
│   │   └── error_tests.rs      # Error handling
│   └── cli/
│       ├── search_tests.rs     # cds search
│       ├── traverse_tests.rs   # cds traverse
│       └── retrieve_tests.rs   # cds retrieve
├── integration/                # Integration tests
│   ├── agent-workflows.test.ts # E2E agent scenarios
│   ├── docker-deployment.test.ts
│   └── error-recovery.test.ts
├── parity/                     # LocAgent parity validation
│   ├── graph-parity.rs         # Graph structure comparison
│   ├── bm25-parity.rs          # BM25 scoring comparison
│   ├── output-parity.test.ts   # Output format validation
│   └── golden/                 # Golden outputs from LocAgent
│       ├── sample_repo_1.json
│       └── sample_repo_2.json
├── benchmarks/                 # Performance benchmarks
│   ├── swe-bench/
│   │   ├── run-eval.sh         # SWE-bench runner
│   │   └── results/            # Evaluation results
│   ├── search_bench.rs         # Search latency benchmark
│   ├── memory_bench.rs         # Memory profiling
│   └── fixtures/
│       └── test-repos/         # Sample codebases
└── fixtures/                   # Shared test data
    ├── sample_issues.jsonl     # Agent test issues
    ├── sample_repos/           # Minimal test repos
    └── expected_outputs/       # Expected results
```

**Key Dependencies**:

- Rust testing: `cargo test`, `proptest`, `criterion`
- TypeScript testing: `bun test` (Jest-compatible)
- CI/CD: GitHub Actions or GitLab CI

---

## Acceptance Criteria Summary (from PRD-08 §8)

### Must-Have (v0.1.0 MVP)

- [ ] Unit test coverage >80% for Rust code
- [ ] Integration tests cover agent workflows
- [ ] Parity validated on SWE-bench Lite subset (50 instances)
- [ ] File Acc@5 ≥75% on benchmark
- [ ] Search latency p95 <500ms
- [ ] All tests pass in CI/CD pipeline

### Should-Have (v0.2.0)

- [ ] Property-based testing for graph invariants
- [ ] Fuzzing for parser robustness
- [ ] Continuous parity monitoring (daily runs)
- [ ] SWE-bench full dataset evaluation (300 instances)

---

## Performance Targets (from PRD-08 §3.1)

| Metric | Target | Validation Method |
|--------|--------|-------------------|
| Unit test execution | <5 min | `cargo test --all` timing |
| Integration test execution | <10 min | E2E test suite timing |
| Parity validation | <30 min | 50 instance comparison |
| SWE-bench subset evaluation | <2 hours | 50 instance full run |

---

## Dependencies & Coordination

### Internal Dependencies

- Unit Tests (01) start in Week 6, run continuously
- Integration Tests (02) require all services functional (Week 7)
- Parity Validation (03) requires LocAgent reference outputs (Week 9)
- Benchmark Testing (04) requires full pipeline stable (Week 10)

### External Coordination

- **PRD-02 (Index Core)**: Graph and index modules need test fixtures
- **PRD-03 (CLI Tools)**: CLI commands need integration tests
- **PRD-04 (Agent)**: Agent workflows need E2E tests
- **PRD-07 (Deployment)**: Docker tests validate deployment

---

## Implementation Phases

### Phase 3, Week 6-7: Foundation Testing

- [ ] Sub-issue 01: Unit tests for graph and index
- [ ] Sub-issue 02: Integration tests for agent workflows
- [ ] Milestone: >80% unit coverage, agent E2E tests pass

### Phase 4, Week 9-10: Parity & Benchmarks

- [ ] Sub-issue 03: Parity validation with LocAgent
- [ ] Sub-issue 04: SWE-bench benchmark evaluation
- [ ] Milestone: Parity validated, File Acc@5 ≥75%

---

## Testing Strategy

### Test Pyramid

```text
         /\
        /E2E\        ← Integration tests (agent workflows)
       /------\
      /Parity  \     ← LocAgent comparison
     /----------\
    /   Unit     \   ← Component tests (graph, index, CLI)
   /--------------\
```

### Continuous Testing

- **Pre-commit**: Unit tests, linting
- **CI/CD**: Full test suite on every PR
- **Nightly**: Parity validation, SWE-bench subset
- **Release**: Full SWE-bench evaluation (v0.2.0)

---

## Open Questions & Risks

### 1. SWE-bench Data Availability

**Question**: Can we access SWE-bench Lite dataset without restrictions?
**Mitigation**: Dataset is public on HuggingFace (czlll/SWE-bench_Lite)
**Tracking**: Download in CI/CD pipeline, cache for fast runs

### 2. Parity Validation Criteria

**Decision**: Use LocAgent's golden outputs from tmp/LocAgent repo
**Approach**: Run LocAgent on 50 instances, save outputs as fixtures
**Risk**: LocAgent output may vary with different API responses

### 3. Performance Regression Detection

**Question**: How to prevent performance regressions in CI/CD?
**Mitigation**: Benchmark tests with threshold assertions (e.g., p95 <600ms)
**Escalation**: If regression detected, block merge until fixed

---

## Related Issues

- **Sub-Issues**: [01-unit-tests.md](01-unit-tests.md), [02-integration.md](02-integration.md), [03-parity.md](03-parity.md), [04-benchmark.md](04-benchmark.md)
- **Depends On**: [02-index-core/](../02-index-core/), [03-cli-tools/](../03-cli-tools/), [04-agent-integration/](../04-agent-integration/)
- **Coordinates With**: [07-deployment/](../07-deployment/)

---

## Next Steps

1. [ ] Set up test directories and fixtures (Week 6, Day 1)
2. [ ] Install testing dependencies (cargo test, proptest, criterion)
3. [ ] Begin Sub-issue 01: Unit tests for graph builder
4. [ ] Set up CI/CD pipeline (GitHub Actions)
5. [ ] Download SWE-bench Lite dataset for benchmarks

---

**Status Updates**:

- *2025-10-19*: Issue created, test infrastructure pending
