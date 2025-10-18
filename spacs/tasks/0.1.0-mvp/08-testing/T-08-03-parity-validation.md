# Task T-08-03: Parity Validation (LocAgent Alignment)

**Issue**: [Sub-Issue 08.03 – Parity Validation](../../issues/04-0.1.0-mvp/08-testing/03-parity.md)

**PRD References**: [PRD-08 §2.3](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md), [PRD-06 §5](../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md)

**Owners**: Rust Dev 1 + QA Lead

**Status**: ☐ Not Started | **Week**: 9

---

## Objective

Demonstrate that CDSAgent reproduces LocAgent behavior within acceptable tolerances for graph structure, retrieval results, and output formatting.

## Deliverables

- Parity scripts (`tests/parity/*.rs`, `.ts`, `.py`)
- Baseline data extracted from LocAgent runs
- Report `docs/testing/parity-report.md`

## Implementation Steps

1. Capture LocAgent graph/index outputs for selected repos; store under `tests/fixtures/locagent/`.
2. Implement comparison tests for node/edge counts, relation types, BM25 scoring, CLI/agent outputs.
3. Run on 50 SWE-bench Lite instances; summarize discrepancies.
4. File follow-up issues for any significant deviations (>5 %).

## Acceptance Criteria

- [ ] Graph parity difference ≤2 % nodes/edges vs. LocAgent.
- [ ] BM25 top-10 results overlap ≥80 %.
- [ ] CLI JSON output matches schema + sample tree format identical.
- [ ] Parity report reviewed and signed off by PM/Tech Lead.

## Dependencies

- **Prerequisite**: [T-08-01](T-08-01-unit-tests.md), LocAgent baseline scripts.
- **Blocks**: Release go/no-go decision.

## Notes

- Automate LocAgent run via container to keep reproducible; store version hash in report.
