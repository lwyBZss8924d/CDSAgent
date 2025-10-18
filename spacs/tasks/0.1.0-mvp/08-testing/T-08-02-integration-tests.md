# Task T-08-02: Integration Tests (E2E Workflows)

**Issue**: [Sub-Issue 08.02 – Integration Tests](../../issues/04-0.1.0-mvp/08-testing/02-integration.md)

**PRD References**: [PRD-08 §2.2](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md), [PRD-04 §7](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

**Owners**: QA Lead (primary), TypeScript Dev 1

**Status**: ☐ Not Started | **Week**: 7

---

## Objective

Build automated tests that exercise the full pipeline: index service → CLI → agent → final answer, including dockerized runs.

## Deliverables

- `tests/integration/agent-workflows.test.ts`
- `tests/integration/docker-deployment.test.ts`
- Combined test script `just test-e2e`
- Documentation `docs/testing/integration-tests.md`

## Implementation Steps

1. Compose fixture environment (graph snapshot + docker-compose override).
2. Implement TypeScript Jest test to run agent scenario end-to-end and verify output JSON.
3. Add docker-based integration test to ensure containers communicate correctly.
4. Hook into CI (GitHub Actions) with scheduled nightly run.

## Acceptance Criteria

- [ ] E2E test covers at least one SWE-bench Lite issue to completion.
- [ ] Docker integration test verifies services healthy and CLI responds.
- [ ] Tests run deterministically (≤5 % variance) and produce logs/artifacts for debugging.
- [ ] Failing tests provide actionable error messages.

## Dependencies

- **Prerequisite**: T-02, T-03, T-04, T-07 deliverables available.
- **Blocks**: Release readiness, parity validation.

## Notes

- Consider parallelizing tests with independent fixture directories to keep runtime manageable.
