# Task T-04-02: Prompt Design (Chain-of-Thought System Prompt)

**Issue**: [Sub-Issue 04.02 – Prompt Design](../../issues/04-0.1.0-mvp/04-agent-integration/02-prompt-design.md)

**PRD References**: [PRD-04 §2.2, §3](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

**Owners**: TypeScript Dev 1 (primary), PM/Writer (review)

**Status**: ☐ Not Started | **Week**: 5

---

## Objective

Craft the agent system prompt and subagent instructions that encode LocAgent’s 4-step reasoning process, tool descriptions, and output format requirements.

## Deliverables

- `cds-agent/src/system-prompt.ts`
- `cds-agent/.claude/agents/code-retrievaler.yaml`
- Prompt documentation in `docs/agent/system-prompt.md`

## Implementation Steps

1. Translate PRD-04 CoT steps (keyword extraction → search → traverse → synthesize) into prompt template.
2. Document tool signatures (`cds search`, `cds traverse`, `cds retrieve`) with usage examples.
3. Define final answer JSON schema and ensure instructions enforce it.
4. Review prompt against LocAgent examples; iterate with sample issues.

## Acceptance Criteria

- [ ] Prompt instructs agent to follow 4-step workflow and limit context size.
- [ ] Tool descriptions match CLI behavior and supported flags.
- [ ] Output schema validated by unit test (JSON parse + schema check).
- [ ] Subagent config restricts tools and includes summary of allowed commands.

## Dependencies

- **Prerequisite**: [T-04-01 SDK Bootstrap](T-04-01-sdk-bootstrap.md), [T-03-02 Output Format](../03-cli-tools/T-03-02-output-format.md).
- **Blocks**: [T-04-04 Sample Transcripts](T-04-04-sample-transcripts.md).

## Notes

- Include dual-language guidance if working with bilingual issue descriptions (per Issue-01).
