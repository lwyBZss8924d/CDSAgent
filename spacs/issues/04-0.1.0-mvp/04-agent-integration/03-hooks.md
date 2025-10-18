# Sub-Issue 04.03: Hooks - Context Management & Session Logging

**Priority**: P1
**Owner**: TypeScript Dev 1
**Timing**: Phase 3, Week 6
**PRD Reference**: [PRD-04 §2.3](../../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

## Objective

Implement 3 hooks to inject context, compress outputs, and log sessions for >30% context reduction.

## Key Implementations

### PreToolUse Hook (Context Injection)

```typescript
export const preToolUse: PreToolUseHook = async (context) => {
  if (context.toolInput.command.startsWith('cds')) {
    if (!process.env.GRAPH_INDEX_DIR) {
      return { allow: false, error: 'GRAPH_INDEX_DIR not set' };
    }
    context.toolInput.env = {
      ...context.toolInput.env,
      GRAPH_INDEX_DIR: process.env.GRAPH_INDEX_DIR
    };
  }
  return { allow: true };
};
```

### PostToolUse Hook (Output Compression)

```typescript
export const postToolUse: PostToolUseHook = async (context) => {
  if (context.toolOutput.stdout.length > 10000) {
    const parsed = JSON.parse(context.toolOutput.stdout);
    const compressed = { ...parsed, results: parsed.results.slice(0, 10) };
    return { modifiedOutput: { stdout: JSON.stringify(compressed) } };
  }
  return { modifiedOutput: context.toolOutput };
};
```

### SubagentStop Hook (Session Logging)

```typescript
export const subagentStop: SubagentStopHook = async (context) => {
  const log = {
    subagent: context.subagentName,
    toolCalls: context.toolCallHistory.length,
    timestamp: new Date().toISOString()
  };
  await fs.appendFile('spacs/research/agent-sessions.jsonl', JSON.stringify(log) + '\n');
  return { continueToMain: true };
};
```

## Acceptance Criteria

- [ ] PreToolUse injects required env vars
- [ ] PostToolUse compresses outputs >10K chars
- [ ] SubagentStop logs all sessions
- [ ] Hooks reduce context usage by >30%

**Related**: [00-overview.md](00-overview.md), [../08-testing/02-integration.md](../08-testing/02-integration.md)
