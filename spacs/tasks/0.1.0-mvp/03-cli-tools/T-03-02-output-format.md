# Task T-03-02: Output Formatting (JSON, Text, Tree)

**Issue**: [Sub-Issue 03.02 – Output Format](../../issues/04-0.1.0-mvp/03-cli-tools/02-output-format.md)

**PRD References**: [PRD-03 §2.2](../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md), [PRD-05 §4](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)

**Owners**: Rust Dev 2

**Status**: ☐ Not Started | **Week**: 4-5

---

## Objective

Provide structured output modes that align with agent and human needs: machine-friendly JSON, concise text summaries, and LocAgent-style tree visualizations for traversals.

## Deliverables

- `crates/cds-tools/src/formatters/{json,text,tree}.rs`
- Formatter unit tests (`crates/cds-tools/tests/formatter_tests.rs`)
- Documentation updates in `docs/CLI.md`

## Implementation Steps

1. JSON formatter: shape responses to match PRD-05 schemas; support streaming/pagination.
2. Text formatter: fold/preview/full display with syntax highlighting hints.
3. Tree formatter: reproduce LocAgent’s expanded tree format with relation annotations.
4. Integrate format selection in CLI (`--format json|text|tree`).

## Acceptance Criteria

- [ ] JSON output validated against schema (CLI integration test).
- [ ] Text output includes entity metadata and truncated previews.
- [ ] Tree output mirrors LocAgent sample (indentation, relation arrows).
- [ ] Formatters handle >100 results with paging notice.

## Dependencies

- **Prerequisite**: [T-03-01 Core Commands](T-03-01-core-commands.md).
- **Blocks**: [T-03-03 Integration Tests](T-03-03-integration-tests.md), [T-04-02 Prompt Design](../04-agent-integration/T-04-02-prompt-design.md).

## Notes

- Ensure consistent coloring/spacing for readability; consider environment variable to disable ANSI colors for CI.
