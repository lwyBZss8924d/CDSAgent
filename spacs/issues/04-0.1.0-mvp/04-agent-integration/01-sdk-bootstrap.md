# Sub-Issue 04.01: SDK Bootstrap - Claude Agent SDK Setup

**Priority**: P1
**Owner**: TypeScript Dev 1
**Timing**: Phase 2, Week 5
**PRD Reference**: [PRD-04 §2.1](../../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

## Objective

Set up Claude Agent SDK with streaming mode, bash tool access, and multi-SDK abstraction layer for future extensibility.

## Key Implementation

```typescript
// cds-agent/src/agent-config.ts
import { ClaudeAgentClient } from '@anthropic-ai/claude-agent-sdk';

export const createCDSAgent = async (config: CDSAgentConfig) => {
  return new ClaudeAgentClient({
    apiKey: process.env.ANTHROPIC_API_KEY!,
    model: 'claude-sonnet-4-5',
    mode: 'streaming',
    systemPrompt: await loadSystemPrompt(),
    allowedTools: ['bash'],
    maxTokens: 8192,
    temperature: 0.0
  });
};

// Multi-SDK abstraction (PRD-10 §2)
export interface AgentSDKAdapter {
  initialize(config: SDKConfig): Promise<void>;
  query(message: string): Promise<AgentResponse>;
  registerTool(tool: ToolDefinition): void;
}
```

## Acceptance Criteria

- [ ] Agent initializes with API key
- [ ] Streaming mode functional
- [ ] Session history maintained
- [ ] Multi-SDK interface defined (Claude implementation in v0.1.0)

**Related**: [00-overview.md](00-overview.md), [../10-extensibility.md](../10-extensibility.md)
