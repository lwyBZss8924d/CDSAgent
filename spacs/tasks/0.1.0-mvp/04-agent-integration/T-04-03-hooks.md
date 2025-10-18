# Task T-04-03: Agent Hooks (PreToolUse, PostToolUse, SubagentStop)

**Issue**: [Sub-Issue 04.03 – Hooks](../../issues/04-0.1.0-mvp/04-agent-integration/03-hooks.md)

**PRD References**: [PRD-04 §2.3](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

**Owners**: TypeScript Dev 1

**Status**: ☐ Not Started | **Week**: 6

---

## Objective

Implement Claude Agent SDK hooks to inject environment context, trim tool output, and log sessions for analytics.

## Deliverables

- `cds-agent/src/hooks/pre-tool-use.ts`
- `cds-agent/src/hooks/post-tool-use.ts`
- `cds-agent/src/hooks/subagent-stop.ts`
- Hook unit tests `cds-agent/tests/hooks.test.ts`

## Implementation Steps

1. **PreToolUse**: enforce allowed commands, inject `GRAPH_INDEX_DIR`, `CDS_INDEX_URL`, reject risky shell usage.
2. **PostToolUse**: summarize large outputs, limit JSON arrays, highlight errors.
3. **SubagentStop**: emit structured logs to `logs/session-*.jsonl`, include tool history.
4. Add configuration to enable/disable hooks for debugging; document in README.

## Acceptance Criteria

- [ ] Hooks execute on every tool call without significant latency (<10 ms overhead).
- [ ] Environment variables injected correctly (verified by integration test).
- [ ] Outputs >10 KB truncated with summary message.
- [ ] Session log includes task metadata and ToolCall timeline.

## Dependencies

- **Prerequisite**: [T-04-01](T-04-01-sdk-bootstrap.md), [T-03-01](../03-cli-tools/T-03-01-core-commands.md).
- **Blocks**: [T-04-04 Sample Transcripts](T-04-04-sample-transcripts.md), [T-08-02 Integration Tests](../08-testing/T-08-02-integration-tests.md).

## Notes

- Design hooks to be reusable in future MCP tool implementation (v0.2.0).
