# Task T-03-03: CLI Integration Tests

**Issue**: [Sub-Issue 03.03 – Integration Tests](../../issues/04-0.1.0-mvp/03-cli-tools/03-integration-tests.md)

**PRD References**: [PRD-03 §5](../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md), [PRD-08 §2.2](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

**Owners**: QA Lead (primary), Rust Dev 2 (support)

**Status**: ☐ Not Started | **Week**: 5

---

## Objective

Validate end-to-end CLI workflows against running index service, covering typical queries, traversal scenarios, and error handling.

## Deliverables

- Shell smoke test `tests/cli_integration_test.sh`
- Rust integration tests `crates/cds-tools/tests/cli_integration.rs`
- Test data fixtures under `tests/fixtures/cli/`

## Implementation Steps

1. Spin up index service with fixture graph using `cargo run` or `just run-service-fixture`.
2. Execute CLI commands via shell script, asserting exit codes and output snippets.
3. Implement Rust integration tests using `assert_cmd`/`predicates` for structured verification.
4. Integrate tests into CI (`just test-cli` and `cargo test --test cli_integration`).

## Acceptance Criteria

- [ ] Smoke script covers search/traverse/retrieve happy paths.
- [ ] Integration tests verify JSON schema adherence and tree format output.
- [ ] Negative cases (service offline, invalid entity ID) handled with meaningful errors.
- [ ] Tests run in <2 minutes and documented in README.

## Dependencies

- **Prerequisite**: [T-03-01 Core Commands](T-03-01-core-commands.md), [T-03-02 Output Format](T-03-02-output-format.md), [T-02-04 Serialization](../02-index-core/T-02-04-serialization.md).
- **Blocks**: [T-08-02 Integration Tests](../08-testing/T-08-02-integration-tests.md).

## Notes

- Use minimal fixture repo to keep test runtime small; rely on parity tasks for larger datasets.
