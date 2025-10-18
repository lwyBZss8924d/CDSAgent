# Tasks: CDS-Agent Integration - LLM Orchestration Layer

**Work Stream**: Issue-04: CDS-Agent Integration
**Issue Reference**: [../../issues/04-0.1.0-mvp/04-agent-integration/](../../issues/04-0.1.0-mvp/04-agent-integration/)
**PRD Reference**: [PRD-04: CDS-Agent Integration](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

## Task Overview

| Task ID | Title | Owner | Status | Week |
|---------|-------|-------|--------|------|
| T-04-01 | SDK Bootstrap - Claude Agent SDK setup | TypeScript Dev 1 | ☐ Not Started | W5 |
| T-04-02 | Prompt Design - Chain-of-thought system prompt | TypeScript Dev 1 | ☐ Not Started | W5 |
| T-04-03 | Hooks - PreToolUse, PostToolUse, SubagentStop | TypeScript Dev 1 | ☐ Not Started | W6 |
| T-04-04 | Sample Transcripts - E2E test scenarios | TypeScript Dev 1 + QA Lead | ☐ Not Started | W7 |

## Dependencies

- **Prerequisite**: T-03-01 (CLI Commands) must be functional
- **Enables**: T-08-02 (Integration Tests) - requires agent workflows

## Task Details

### T-04-01: SDK Bootstrap

**File**: `T-04-01-sdk-bootstrap.md`
**Issue**: [Sub-Issue 04.01](../../issues/04-0.1.0-mvp/04-agent-integration/01-sdk-bootstrap.md)
**PRD**: PRD-04 §2.1, §4.1

**Scope**:

- Claude Agent SDK integration (@anthropic-ai/claude-agent-sdk)
- Agent session management
- Streaming mode configuration
- API key management

**Deliverables**:

- `cds-agent/src/agent-config.ts`
- `cds-agent/src/main.ts`
- TypeScript project setup (Bun runtime)

---

### T-04-02: Prompt Design

**File**: `T-04-02-prompt-design.md`
**Issue**: [Sub-Issue 04.02](../../issues/04-0.1.0-mvp/04-agent-integration/02-prompt-design.md)
**PRD**: PRD-04 §2.2, §4.2

**Scope**:

- System prompt with LocAgent's 4-step CoT strategy
- Tool descriptions (cds search, cds traverse, cds retrieve)
- Output format specification (JSON with locations + reasoning)
- `code-retrievaler` subagent configuration

**Deliverables**:

- `cds-agent/src/system-prompt.ts`
- `cds-agent/.claude/agents/code-retrievaler.yaml`

---

### T-04-03: Hooks Implementation

**File**: `T-04-03-hooks.md`
**Issue**: [Sub-Issue 04.03](../../issues/04-0.1.0-mvp/04-agent-integration/03-hooks.md)
**PRD**: PRD-04 §2.3, §4.3

**Scope**:

- PreToolUse: Inject index paths, validate tool calls
- PostToolUse: Compress outputs >10K tokens
- SubagentStop: Log sessions, audit tool calls

**Deliverables**:

- `cds-agent/src/hooks/pre-tool-use.ts`
- `cds-agent/src/hooks/post-tool-use.ts`
- `cds-agent/src/hooks/subagent-stop.ts`

---

### T-04-04: Sample Transcripts

**File**: `T-04-04-sample-transcripts.md`
**Issue**: [Sub-Issue 04.04](../../issues/04-0.1.0-mvp/04-agent-integration/04-sample-transcripts.md)
**PRD**: PRD-04 §7, PRD-08 §5

**Scope**:

- 5-10 sample code localization tasks (from SWE-bench Lite)
- Expected agent reasoning traces
- Validation scripts for output correctness

**Deliverables**:

- `tests/fixtures/sample-issues.jsonl`
- `tests/integration/agent-workflows.test.ts`

---

## Phase Milestones

### Week 5: Agent Foundation

- [ ] T-04-01: Agent initializes and streams responses
- [ ] T-04-02: Prompt includes CoT strategy and tool descriptions

**Validation**: Agent can invoke `cds search` via bash tool

### Week 6-7: Hooks & Validation

- [ ] T-04-03: All 3 hooks implemented and tested
- [ ] T-04-04: Agent completes 5 sample tasks successfully

**Validation**: Agent completes multi-step code localization task

---

## Quick Links

- [Issue-04 Overview](../../issues/04-0.1.0-mvp/04-agent-integration/00-overview.md)
- [PRD-04: CDS-Agent Integration](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)
- [CLI Tools Tasks](../03-cli-tools/README.md)
