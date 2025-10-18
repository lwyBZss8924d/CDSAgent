# Sub-Issue 04.02: Prompt Design - Chain-of-Thought System Prompt

**Priority**: P1
**Owner**: TypeScript Dev 1
**Timing**: Phase 2, Week 5
**PRD Reference**: [PRD-04 §2.1.2, §2.2](../../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

## Objective

Design system prompt with LocAgent's 4-step CoT strategy and configure `code-retrievaler` subagent.

## Key Components

### System Prompt (LocAgent §3.3)

```markdown
# CDSAgent: Autonomous Code Localization Assistant

## Reasoning Strategy (Chain-of-Thought)
1. **Extract Keywords**: Analyze issue, identify technical terms
2. **Search Entry Points**: Use `cds search` with keywords
3. **Traverse Dependencies**: Use `cds traverse` to explore relationships
4. **Synthesize Answer**: Use `cds retrieve` for final verification

## Output Format
{
  "locations": [{"file": "...", "line_range": [...], "reason": "..."}],
  "reasoning": "..."
}
```

### Subagent Config (`.claude/agents/code-retrievaler.yaml`)

```yaml
name: code-retrievaler
tools:
  - name: bash
    allowed_commands: [cds, rg, jq, ast-grep]
    denied_patterns: [rm, mv, curl, "> /dev"]
hooks:
  preToolUse: hooks/pre-tool-use.ts
  postToolUse: hooks/post-tool-use.ts
```

## Acceptance Criteria

- [ ] System prompt includes 4-step CoT
- [ ] Tool descriptions match LocAgent Table 1
- [ ] Subagent restricts to safe commands
- [ ] Output format specified

**Related**: [00-overview.md](00-overview.md), [../06-refactor-parity.md](../06-refactor-parity.md)
