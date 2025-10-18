# Task T-04-01: SDK Bootstrap (Claude Agent Integration)

**Issue**: [Sub-Issue 04.01 – SDK Bootstrap](../../issues/04-0.1.0-mvp/04-agent-integration/01-sdk-bootstrap.md)

**PRD References**: [PRD-04 §2.1](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

**Owners**: TypeScript Dev 1

**Status**: ☐ Not Started | **Week**: 5

---

## Objective

Set up the Claude Agent SDK with bash tool access so CDSAgent can orchestrate CLI commands via streaming sessions.

## Deliverables

- `cds-agent/src/agent-config.ts` (session bootstrap)
- `cds-agent/src/main.ts` (entrypoint)
- `.env.example` / `.env` documentation
- Unit tests under `cds-agent/tests/agent-config.test.ts`

## Implementation Steps

1. Install and configure `@anthropic-ai/claude-agent-sdk` with Bun runtime.
2. Implement session factory that enforces streaming mode, tool whitelist (`bash`), permission prompts.
3. Handle environment variable loading (API keys, index URLs) with validation.
4. Write quick smoke test to launch session and run noop command.

## Acceptance Criteria

- [ ] `bun run dev` starts session, prints greeting, and awaits user tool calls.
- [ ] SDK config uses streaming + `permissionMode=restricted` as per PRD.
- [ ] Environment variables validated with helpful error messages.
- [ ] Automated test verifies configuration defaults.

## Dependencies

- **Prerequisite**: [T-03-01 Core Commands](../03-cli-tools/T-03-01-core-commands.md).
- **Blocks**: [T-04-02 Prompt Design](T-04-02-prompt-design.md), [T-04-03 Hooks](T-04-03-hooks.md).

## Notes

- Document CLI exec path fallback for Windows/macOS/Linux.
- Track API usage costs; add TODO for rate limiting in v0.2.0.
