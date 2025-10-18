# Issue-04: CDS-Agent Integration - LLM Orchestration Layer

**Priority**: P1 (Critical Path - Agent Layer)
**Status**: ☐ Not Started
**Owner**: TypeScript Dev 1
**PRD Reference**: [PRD-04: CDS-Agent Integration](../../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

---

## Overview

CDS-Agent Integration provides the orchestration layer that enables LLMs (primarily Claude via Anthropic's Agent SDK) to autonomously navigate codebases using CDS-Tools CLI. This component implements the "brain" of CDSAgent, translating natural language issue descriptions into multi-step code retrieval workflows.

## Objective

Deliver a TypeScript-based agent layer that:

- Integrates Claude Agent SDK with streaming mode and bash tool access
- Implements `code-retrievaler` subagent with CDS-Tools CLI invocation
- Provides 3 hooks (PreToolUse, PostToolUse, SubagentStop) for context management
- Uses chain-of-thought prompting based on LocAgent research
- Validates end-to-end agent workflows on sample code localization tasks

## Dependencies

- **Requires**: CDS-Tools CLI ([03-cli-tools/](../03-cli-tools/)) must be functional
- **Coordinates With**: CDS-Index Service ([02-index-core/](../02-index-core/))
- **Timing**: Phase 2-3 (Weeks 5-7)

---

## Sub-Issues Breakdown

### 1. [SDK Bootstrap](01-sdk-bootstrap.md) - **P1, Week 5**

**Owner**: TypeScript Dev 1
**Scope**: Claude SDK setup, agent session management

- Claude Agent SDK integration with streaming mode
- Agent configuration and initialization
- API key management and error handling
- Multi-SDK abstraction layer (Claude, future: OpenAI Codex)

**Acceptance**:

- [ ] Agent initializes with API key and config
- [ ] Streaming mode returns incremental tool calls
- [ ] Session maintains conversation history

---

### 2. [Prompt Design](02-prompt-design.md) - **P1, Week 5**

**Owner**: TypeScript Dev 1
**Scope**: System prompt with chain-of-thought based on LocAgent

- 4-step CoT strategy (Extract Keywords → Search → Traverse → Synthesize)
- Tool descriptions (cds search, cds traverse, cds retrieve)
- Output format specification (JSON with locations + reasoning)
- `code-retrievaler` subagent configuration

**Acceptance**:

- [ ] System prompt includes all 3 tools
- [ ] Chain-of-thought steps clearly defined
- [ ] Output format matches LocAgent structure
- [ ] Subagent can be invoked from main agent

---

### 3. [Hooks Implementation](03-hooks.md) - **P1, Week 6**

**Owner**: TypeScript Dev 1
**Scope**: PreToolUse, PostToolUse, SubagentStop hooks

- **PreToolUse**: Inject index paths, validate tool calls, block unsafe commands
- **PostToolUse**: Compress large outputs, highlight errors, reduce context usage
- **SubagentStop**: Log sessions, audit tool calls, cleanup

**Acceptance**:

- [ ] PreToolUse injects required environment variables
- [ ] PostToolUse compresses outputs >10K tokens
- [ ] SubagentStop logs all tool calls for analysis
- [ ] Hooks reduce context usage by >30%

---

### 4. [Sample Transcripts](04-sample-transcripts.md) - **P1, Week 7**

**Owner**: TypeScript Dev 1 + QA Lead
**Scope**: E2E test scenarios and validation

- Sample code localization tasks (5-10 scenarios)
- Expected agent reasoning traces
- Validation scripts for output correctness
- Benchmark against LocAgent performance

**Acceptance**:

- [ ] Agent completes 5 sample tasks successfully
- [ ] Reasoning traces show CoT steps
- [ ] Accuracy: File Acc@5 ≥75% on test set
- [ ] Tool call efficiency comparable to LocAgent

---

## TypeScript Project Structure

```tree
cds-agent/
├── src/
│   ├── agent-config.ts         # Claude SDK configuration
│   ├── system-prompt.ts        # CoT prompt template
│   ├── subagents/
│   │   └── code-retrievaler/   # Subagent config
│   ├── hooks/
│   │   ├── pre-tool-use.ts     # Context injection
│   │   ├── post-tool-use.ts    # Output compression
│   │   └── subagent-stop.ts    # Session logging
│   ├── tools/
│   │   └── bash-tool.ts        # v0.1.0: CLI wrapper
│   │   └── mcp-tools.ts        # v0.2.0: Native tools (scaffolded)
│   └── main.ts                 # Entry point
├── tests/
│   ├── integration/
│   │   └── agent-workflows.test.ts
│   └── fixtures/
│       └── sample-issues.jsonl
├── package.json
├── tsconfig.json
└── README.md
```

**Key Dependencies**:

- `@anthropic-ai/claude-agent-sdk` - Claude SDK
- `bun` - Runtime (TypeScript execution)
- `zod` - Schema validation

---

## Acceptance Criteria Summary (from PRD-04 §8)

### Must-Have (v0.1.0 MVP)

- [ ] Claude SDK integrated with streaming mode
- [ ] `code-retrievaler` subagent functional with bash tool
- [ ] All 3 hooks implemented and reducing context usage
- [ ] Chain-of-thought prompting follows LocAgent 4-step strategy
- [ ] Agent completes 5 sample code localization tasks
- [ ] Tool call logging and session auditing

### Should-Have (v0.2.0)

- [ ] Native MCP tool wrappers (`cds_search`, `cds_traverse`, `cds_retrieve`)
- [ ] Multi-SDK support (OpenAI Codex adapter)
- [ ] Fine-tuning pipeline for agent optimization

---

## Performance Targets (from PRD-04 §3.1)

| Metric | Target | Validation Method |
|--------|--------|-------------------|
| Agent initialization | <2s | Integration test |
| Tool call overhead | <500ms | Beyond CDS-Tools execution time |
| Context compression (hooks) | <100ms | Unit test benchmark |
| Subagent spawn time | <1s | Integration test |

---

## Dependencies & Coordination

### Internal Dependencies

- SDK Bootstrap (01) must complete before Hooks (03)
- Prompt Design (02) runs in parallel with SDK Bootstrap
- Sample Transcripts (04) requires all components functional

### External Coordination

- **PRD-03 (CLI Tools)**: Agent invokes CLI via bash tool
- **PRD-02 (Index Service)**: CLI queries index service
- **PRD-08 (Testing)**: Integration tests validate agent workflows

---

## Implementation Phases

### Phase 2, Week 5: Agent Foundation

- [ ] Sub-issue 01: SDK Bootstrap
- [ ] Sub-issue 02: Prompt Design
- [ ] Milestone: Agent can invoke `cds search` via bash tool

### Phase 3, Week 6-7: Hooks & Validation

- [ ] Sub-issue 03: Hooks implementation
- [ ] Sub-issue 04: Sample transcripts and validation
- [ ] Milestone: Agent completes multi-step code localization task

---

## Testing Strategy

### Unit Tests

- [ ] Agent configuration and initialization
- [ ] Hook logic (context injection, compression, logging)
- [ ] Tool parameter validation

### Integration Tests (see [../08-testing/02-integration.md](../08-testing/02-integration.md))

- [ ] End-to-end: Issue → Agent → CLI → Index Service → Result
- [ ] Subagent invocation and delegation
- [ ] Hook effectiveness (context reduction)

### Validation Tests

- [ ] Sample code localization tasks (5-10 scenarios)
- [ ] Compare with LocAgent reasoning traces
- [ ] Measure File Acc@5 on test set

---

## Open Questions & Risks

### 1. MCP Tool Timing

**Decision**: Bash tool only in v0.1.0, MCP tools in v0.2.0 (per PRD-04 FR-MCP-1)
**Rationale**: JSON-RPC service needs battle-testing first, bash tool sufficient for parity
**Risk**: Tool call overhead (~50ms), acceptable for v0.1.0

### 2. Context Management

**Question**: How to prevent context overflow with large codebases?
**Mitigation**: PostToolUse hook compresses outputs, limits results to top 10
**Escalation**: If context still exceeds 1M tokens, implement sliding window in v0.2.0

### 3. Tool Call Loops

**Risk**: Agent may retry failed searches indefinitely
**Mitigation**: Implement max iteration limit (10 tool calls per task)
**Tracking**: Monitor via SubagentStop hook logs

---

## Related Issues

- **Sub-Issues**: [01-sdk-bootstrap.md](01-sdk-bootstrap.md), [02-prompt-design.md](02-prompt-design.md), [03-hooks.md](03-hooks.md), [04-sample-transcripts.md](04-sample-transcripts.md)
- **Depends On**: [03-cli-tools/](../03-cli-tools/) - CLI must be functional
- **Coordinates With**: [02-index-core/](../02-index-core/) - Index service
- **Tests**: [08-testing/02-integration.md](../08-testing/02-integration.md)

---

## Next Steps

1. [ ] Set up cds-agent TypeScript project with Bun (Week 5, Day 1)
2. [ ] Install Claude Agent SDK and dependencies (Week 5, Day 1)
3. [ ] Begin Sub-issue 01: SDK Bootstrap (Week 5, Day 2)
4. [ ] Begin Sub-issue 02: Prompt Design (Week 5, parallel)
5. [ ] Validate agent can invoke CLI successfully

---

**Status Updates**:

- *2025-10-19*: Issue created, awaiting CLI tools completion
