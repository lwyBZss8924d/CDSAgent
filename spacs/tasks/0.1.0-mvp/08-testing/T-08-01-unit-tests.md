# Task T-08-01: Unit Test Coverage (>80%)

**Issue**: [Sub-Issue 08.01 – Unit Tests](../../issues/04-0.1.0-mvp/08-testing/01-unit-tests.md)

**PRD References**: [PRD-08 §2.1, §4.1](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

**Owners**: Rust Dev 1 & Rust Dev 2

**Status**: ☐ Not Started | **Week**: 6-8

---

## Objective

Achieve ≥80 % line coverage across Rust crates (`cds-index`, `cds-tools`) with focused unit tests and property-based checks for graph invariants.

## Deliverables

- Unit test modules for graph, index, service, CLI
- Property tests using `proptest`
- Coverage report generation script (`just coverage`)
- Documentation `docs/testing/unit-tests.md`

## Implementation Steps

1. Write module-level tests verifying parser edge cases, graph relationships, BM25 scoring.
2. Add CLI command tests using `assert_cmd` & `predicates`.
3. Create property tests (e.g., traversal invariants, serialization round-trip).
4. Configure coverage tooling (e.g., `cargo llvm-cov`) and integrate into CI.

## Acceptance Criteria

- [ ] `cargo llvm-cov --workspace` shows ≥80 % line coverage.
- [ ] All unit tests pass under `just test-unit`.
- [ ] Property tests catch invalid graph states without flakiness.
- [ ] Coverage report stored/uploaded for future reference.

## Dependencies

- **Prerequisite**: Implementation tasks in T-02 & T-03 nearing completion.
- **Blocks**: [T-08-03 Parity Validation](T-08-03-parity-validation.md).

## Notes

- Exclude third-party generated code (e.g., tree-sitter bindings) from coverage metrics.
