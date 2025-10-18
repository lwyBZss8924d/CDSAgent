# Task T-08-04: Benchmark & SWE-bench Evaluation

**Issue**: [Sub-Issue 08.04 – Benchmark Testing](../../issues/04-0.1.0-mvp/08-testing/04-benchmark.md)

**PRD References**: [PRD-08 §2.4, §5](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

**Owners**: QA Lead, Rust Dev 2

**Status**: ☐ Not Started | **Week**: 10

---

## Objective

Quantify performance and quality metrics (latency, accuracy) by running automated benchmarks including SWE-bench Lite evaluation.

## Deliverables

- SWE-bench scripts (`tests/benchmarks/swe-bench/run-eval.sh`)
- Criterion benches (`crates/cds-index/benches/{graph_bench.rs,search_bench.rs}`)
- Memory profiling report (`docs/testing/performance.md`)
- Dashboard data for latency/accuracy

## Implementation Steps

1. Prepare dataset download + preprocessing (SWE-bench Lite, LocBench).
2. Run evaluation pipeline comparing predicted locations vs. ground truth; collect precision@5 metrics.
3. Execute Criterion benchmarks for search/traverse operations under different repo sizes.
4. Profile memory usage (heaptrack / valgrind massif) and document results.

## Acceptance Criteria

- [ ] File Acc@5 ≥75 %, Func Acc@10 ≥55 % (targets from PRD-08).
- [ ] Search latency p95 <500 ms; traverse latency p95 <1 s.
- [ ] Benchmark reports stored in repo (CSV/Markdown) and linked from docs.
- [ ] Performance regression alerts integrated into CI (optional but recommended).

## Dependencies

- **Prerequisite**: Completion of parity validation and integration tests.
- **Blocks**: Release notes / final sign-off.

## Notes

- For reproducibility, pin dataset versions and record commit hashes of benchmarks.
