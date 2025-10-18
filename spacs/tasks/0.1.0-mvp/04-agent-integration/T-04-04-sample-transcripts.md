# Task T-04-04: Sample Transcripts & E2E Validation

**Issue**: [Sub-Issue 04.04 – Sample Transcripts](../../issues/04-0.1.0-mvp/04-agent-integration/04-sample-transcripts.md)

**PRD References**: [PRD-04 §7](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md), [PRD-08 §2.2](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

**Owners**: TypeScript Dev 1 & QA Lead

**Status**: ☐ Not Started | **Week**: 7

---

## Objective

Produce reference interaction transcripts demonstrating successful multi-step code localization, and automate regression tests against those transcripts.

## Deliverables

- `tests/fixtures/sample-issues.jsonl` (SWE-bench Lite subset)
- `tests/integration/agent-workflows.test.ts`
- Transcript artifacts `tests/fixtures/transcripts/*.md`
- Documentation `docs/agent/sample-workflows.md`

## Implementation Steps

1. Select 5–10 representative issues (bug fix, feature, refactor) from SWE-bench Lite.
2. Run agent end-to-end, capture prompts, tool calls, observations, and final answer.
3. Convert transcripts into fixtures for deterministic replay tests.
4. Automate regression check: agent must reproduce final output within tolerance.

## Acceptance Criteria

- [ ] At least 5 transcripts stored with metadata (issue ID, repo, outcome).
- [ ] Replay tests pass on CI (ensuring prompts + hooks deterministic enough).
- [ ] Documentation guides contributors on recording new transcripts.
- [ ] Failure diffs highlight mismatched steps for debugging.

## Dependencies

- **Prerequisite**: [T-04-02 Prompt Design](T-04-02-prompt-design.md), [T-04-03 Hooks](T-04-03-hooks.md).
- **Blocks**: [T-08-02 Integration Tests](../08-testing/T-08-02-integration-tests.md), release notes.

## Notes

- Store raw transcripts in `.jsonl` for programmatic diffing; human-friendly Markdown optional.
