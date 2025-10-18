# PRD-04: CDS-Agent Integration - LLM Orchestration with Claude SDK

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Component Overview

### 1.1 Purpose

CDS-Agent Integration provides the orchestration layer that enables LLMs (primarily Claude via Anthropic's Agent SDK) to autonomously navigate codebases using CDS-Tools CLI. This component implements the "brain" of CDSAgent, translating natural language issue descriptions into multi-step code retrieval workflows.

### 1.2 Scope

- Claude Agent SDK integration with streaming mode
- `code-retrievaler` subagent configuration with bash tool access
- Hooks (PreToolUse, PostToolUse, SubagentStop) for context management
- MCP (Model Context Protocol) tools registration
- Chain-of-thought prompting based on LocAgent research
- Multi-SDK abstraction layer (Claude, OpenAI Codex, future)

### 1.3 LocAgent References

- **Paper §3.3**: "Autonomous Agent Workflow" - <https://arxiv.org/html/2503.09089v2#S3.SS3>
- **Paper §4**: "Experimental Setup" (chain-of-thought prompts)
- **Code**: `tmp/LocAgent/auto_search_main.py`, `util/prompts/`
- **Claude SDK Docs**: `tmp/claude-agent-sdk/`

---

## 2. Functional Requirements

### 2.1 Claude Agent SDK Integration

#### FR-SDK-1: Agent Session Management

**Purpose**: Initialize and manage Claude agent sessions with proper configuration.

**Configuration Interface**:

```typescript
// cds-agent/src/agent-config.ts

import { ClaudeAgentClient } from '@anthropic-ai/claude-agent-sdk';

export const createCDSAgent = async (config: CDSAgentConfig): Promise<ClaudeAgentClient> => {
  return new ClaudeAgentClient({
    apiKey: process.env.ANTHROPIC_API_KEY,
    model: 'claude-sonnet-4-5',  // 1M token context
    mode: 'streaming',            // Interactive multi-step
    systemPrompt: await loadSystemPrompt(),
    allowedTools: ['bash'],
    permissionMode: 'restricted',
    maxTokens: 8192,              // Per response
    temperature: 0.0,             // Deterministic for code tasks
  });
};

interface CDSAgentConfig {
  indexPath: string;
  allowedCommands: string[];     // ['cds', 'rg', 'jq', 'ast-grep']
  maxSearchResults: number;
  maxTraverseDepth: number;
}

> **MVP Note**: The Claude session exposes only the built-in `bash` tool in v0.1.0. Dedicated MCP wrappers (`cds_search`, `cds_traverse`, `cds_retrieve`) graduate in v0.2.0 once the JSON-RPC service is battle-tested.
```

**LocAgent Mapping**:

- Streaming mode enables LocAgent's iterative reasoning (§3.3)
- Temperature 0.0 matches LocAgent's deterministic setup

**Acceptance Criteria**:

- [ ] Can initialize agent with API key and config
- [ ] Streaming mode returns incremental tool calls
- [ ] Session maintains conversation history

#### FR-SDK-2: System Prompt with Chain-of-Thought

**Purpose**: Provide agent with LocAgent-style reasoning framework.

**System Prompt Template** (based on LocAgent prompts):

```markdown
# CDSAgent: Autonomous Code Localization Assistant

You are an expert code localization agent. Your task is to find the specific code locations (files, classes, functions) that need modification to address a given software issue.

## Available Tools

You have access to the following code retrieval tools via bash commands:

1. **cds search "<query>"**
   - Search for code entities matching keywords
   - Returns: entity IDs, names, file paths, code snippets (fold/preview/full)
   - Use for: Initial exploration, finding entry points

2. **cds traverse <entity_id> [options]**
   - Navigate code relationships (function calls, inheritance, imports)
   - Options: --depth N, --relations invoke|contain|inherit, --direction forward|backward
   - Use for: Tracing dependencies, finding callers/callees

3. **cds retrieve <entity_id>**
   - Fetch complete code for specific entities
   - Use for: Final verification, detailed analysis

## Reasoning Strategy (Chain-of-Thought)

Follow this 4-step approach (based on LocAgent research):

### Step 1: Extract Keywords
- Analyze the issue description
- Identify technical terms, error messages, feature names
- Extract entity types mentioned (files, classes, functions)

### Step 2: Search Entry Points
- Use `cds search` with extracted keywords
- Filter by entity type if known (e.g., --type function)
- Review fold/preview snippets to identify relevant entities

### Step 3: Traverse Dependencies
- Starting from high-relevance entities, use `cds traverse`
- Follow function calls (invoke), inheritance (inherit), or containment
- Explore 1-2 hops to find related code

### Step 4: Synthesize Answer
- Use `cds retrieve` for final code inspection
- Return file paths, line ranges, and brief explanations
- Ensure all locations are directly relevant to the issue

## Important Guidelines

- Always think step-by-step before taking action
- If search returns no results, try broader keywords or different entity types
- Use `--format json` for machine-readable output when piping commands
- Combine tools: search → traverse → retrieve
- Limit results to avoid overwhelming context (use --limit flag)
- Skip `cds combo` in v0.1.0; it is reserved for manual operator workflows until v0.2.0

## Output Format

Provide your final answer as:

```json
{
  "locations": [
    {
      "file": "path/to/file.py",
      "line_range": [15, 32],
      "entity": "function_name",
      "reason": "Why this location is relevant"
    }
  ],
  "reasoning": "Summary of your search process"
}
```

**LocAgent Mapping**:

- 4-step CoT from LocAgent §3.3 (Extract Keywords → Search → Traverse → Synthesize)
- Tool descriptions match LocAgent Table 1

**Acceptance Criteria**:

- [ ] System prompt includes all 3 tools
- [ ] Chain-of-thought steps clearly defined
- [ ] Output format specified

### 2.2 Subagent Configuration

#### FR-SUB-1: `code-retrievaler` Subagent

**Purpose**: Specialized subagent for code retrieval with restricted tool access.

**Subagent Definition** (`.claude/agents/code-retrievaler.yaml`):

```yaml
name: code-retrievaler
description: "Specialized agent for codebase retrieval using CDS-Tools CLI"

systemPrompt: |
  You are the Code Retrieval sub-agent. Your role is to execute codebase queries
  using the cds CLI and return structured results to the main agent.

  You can only use bash commands with cds, rg, jq, and ast-grep.
  Focus on efficient, targeted searches.

tools:
  - name: bash
    allowed_commands:
      - cds
      - cds search
      - cds traverse
      - cds retrieve
      - rg
      - jq
      - ast-grep
      - echo
      - cat
    denied_patterns:
      - "rm"
      - "mv"
      - "curl"
      - "wget"
      - "> /dev"  # No file writes

model: claude-sonnet-4-5
maxTokens: 4096
temperature: 0.0

hooks:
  preToolUse: hooks/pre-tool-use.ts
  postToolUse: hooks/post-tool-use.ts
  subagentStop: hooks/subagent-stop.ts
```

**LocAgent Mapping**:

- Mirrors LocAgent's tool-based architecture
- Restricted bash ensures safety (no file modifications)

**Acceptance Criteria**:

- [ ] Subagent can be invoked from main agent
- [ ] Only allowed commands executable
- [ ] Denied patterns blocked with error message

#### FR-SUB-2: Subagent Invocation

**Purpose**: Enable main agent to delegate retrieval tasks to subagent.

**Invocation Example**:

```typescript
// Main agent delegates to code-retrievaler
const result = await mainAgent.invokeSubagent('code-retrievaler', {
  task: 'Find all functions related to XSS sanitization',
  context: {
    indexPath: '/path/to/graph_index',
    maxResults: 10
  }
});
```

**Acceptance Criteria**:

- [ ] Main agent can spawn subagent
- [ ] Subagent has access to context (index paths)
- [ ] Results return to main agent

### 2.3 Hooks for Context Management

#### FR-HOOK-1: PreToolUse Hook

**Purpose**: Inject context and validate tool calls before execution.

**Hook Implementation** (`hooks/pre-tool-use.ts`):

```typescript
import { PreToolUseHook } from '@anthropic-ai/claude-agent-sdk';

const DEFAULT_INDEX_URL = process.env.CDS_INDEX_URL ?? 'http://localhost:9876';

export const preToolUse: PreToolUseHook = async (context) => {
  const { toolName, toolInput, agentState } = context;

  // Inject index paths if using cds commands
  if (toolName === 'bash' && toolInput.command.startsWith('cds')) {
    const indexPath = process.env.GRAPH_INDEX_DIR;
    if (!indexPath) {
      return {
        allow: false,
        error: 'GRAPH_INDEX_DIR not set. Run: export GRAPH_INDEX_DIR=/path/to/index'
      };
    }

    // Inject index path as env var
    toolInput.env = {
      ...toolInput.env,
      GRAPH_INDEX_DIR: indexPath,
      CDS_INDEX_URL: DEFAULT_INDEX_URL
    };
  }

  // Validate search query not too broad
  if (toolInput.command.includes('cds search "*"')) {
    return {
      allow: false,
      suggestion: 'Search query too broad. Use specific keywords.'
    };
  }

return { allow: true };
};
```

> Default environment values (`GRAPH_INDEX_DIR`, `CDS_INDEX_URL=http://localhost:9876`) mirror the deployment defaults in PRD-07.

**LocAgent Mapping**:

- Ensures agent has necessary context (LocAgent assumes preloaded indices)

**Acceptance Criteria**:

- [ ] Hook can block invalid tool calls
- [ ] Injects required environment variables
- [ ] Provides actionable error messages

#### FR-HOOK-2: PostToolUse Hook

**Purpose**: Summarize and compress tool outputs before feeding to LLM.

**Hook Implementation** (`hooks/post-tool-use.ts`):

```typescript
import { PostToolUseHook } from '@anthropic-ai/claude-agent-sdk';

export const postToolUse: PostToolUseHook = async (context) => {
  const { toolName, toolOutput, agentState } = context;

  // Compress large JSON outputs
  if (toolOutput.stdout && toolOutput.stdout.length > 10000) {
    const parsed = JSON.parse(toolOutput.stdout);

    if (parsed.results && Array.isArray(parsed.results)) {
      // Keep top 10 results, summarize rest
      const topResults = parsed.results.slice(0, 10);
      const summary = `Showing top 10 of ${parsed.results.length} results. Use --limit to adjust.`;

      return {
        modifiedOutput: {
          stdout: JSON.stringify({ ...parsed, results: topResults, summary }),
          stderr: toolOutput.stderr
        }
      };
    }
  }

  // Highlight errors in stderr
  if (toolOutput.stderr) {
    console.error(`[CDS Tool Error] ${toolOutput.stderr}`);
  }

  return { modifiedOutput: toolOutput };
};
```

**LocAgent Mapping**:

- LocAgent limits context by using fold/preview (§3.2)
- Hook achieves same effect via post-processing

**Acceptance Criteria**:

- [ ] Large outputs compressed to fit context
- [ ] Errors logged for debugging
- [ ] Modified output passed to agent

#### FR-HOOK-3: SubagentStop Hook

**Purpose**: Log subagent session results and cleanup.

**Hook Implementation** (`hooks/subagent-stop.ts`):

```typescript
import { SubagentStopHook } from '@anthropic-ai/claude-agent-sdk';
import fs from 'fs/promises';

export const subagentStop: SubagentStopHook = async (context) => {
  const { subagentName, finalOutput, toolCallHistory } = context;

  // Log session for research/debugging
  const log = {
    subagent: subagentName,
    timestamp: new Date().toISOString(),
    toolCalls: toolCallHistory.length,
    commands: toolCallHistory.map(t => t.toolInput.command),
    finalOutput
  };

  await fs.appendFile('spacs/research/agent-sessions.jsonl', JSON.stringify(log) + '\n');

  return { continueToMain: true };
};
```

**Acceptance Criteria**:

- [ ] Logs all tool calls for audit
- [ ] Saves session data for analysis
- [ ] Returns control to main agent

### 2.4 MCP Tools Registration

> **Roadmap (v0.2.0)**: Native MCP tool wrappers graduate after the JSON-RPC service stabilises. They are documented here for future planning but are not part of the v0.1.0 acceptance criteria.

#### FR-MCP-1: Custom Tools as MCP (v0.2.0)

**Purpose**: Register CDS-Tools as native Claude tools (alternative to bash).

**MCP Tool Definition** (TypeScript):

```typescript
import { tool } from '@anthropic-ai/claude-agent-sdk';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export const cdsSearchTool = tool({
  name: 'cds_search',
  description: 'Search for code entities using keywords',
  parameters: {
    query: { type: 'string', description: 'Search keywords' },
    entity_type: { type: 'string', enum: ['file', 'class', 'function'], optional: true },
    limit: { type: 'number', default: 10, optional: true }
  },
  execute: async ({ query, entity_type, limit }) => {
    const cmd = `cds search "${query}" ${entity_type ? `--type ${entity_type}` : ''} --limit ${limit} --format json`;
    const { stdout, stderr } = await execAsync(cmd, { env: process.env });

    if (stderr) {
      throw new Error(`CDS Search Error: ${stderr}`);
    }

    return JSON.parse(stdout);
  }
});

export const cdsTraverseTool = tool({
  name: 'cds_traverse',
  description: 'Navigate code graph from entity',
  parameters: {
    entity_id: { type: 'string' },
    depth: { type: 'number', default: 1, optional: true },
    relations: { type: 'array', items: { type: 'string' }, optional: true },
    direction: { type: 'string', enum: ['forward', 'backward', 'bidirectional'], default: 'forward', optional: true }
  },
  execute: async ({ entity_id, depth, relations, direction }) => {
    const relArg = relations ? `--relations ${relations.join(',')}` : '';
    const cmd = `cds traverse ${entity_id} --depth ${depth} ${relArg} --direction ${direction} --format json`;
    const { stdout } = await execAsync(cmd, { env: process.env });
    return JSON.parse(stdout);
  }
});

// Register tools with agent
agent.registerTools([cdsSearchTool, cdsTraverseTool, /* ... */]);
```

**Advantages over Bash**:

- Type-safe parameters (validated by SDK)
- Direct JSON return (no parsing needed)
- Better error handling

**Acceptance Criteria**:

- [ ] Tools callable as native agent functions
- [ ] Parameters validated before execution
- [ ] Errors propagated to agent gracefully

---

## 3. Non-Functional Requirements

### 3.1 Performance

| Metric | Target | Rationale |
|--------|--------|-----------|
| Agent initialization | <2s | Session startup |
| Tool call latency | <500ms overhead | Beyond CDS-Tools execution time |
| Context compression (hooks) | <100ms | Per tool output |
| Subagent spawn time | <1s | Delegation overhead |

### 3.2 Reliability

- Handle CDS-Tools errors gracefully (retry, fallback)
- Prevent infinite tool call loops (max iterations)
- Validate all tool outputs before returning to agent

### 3.3 Security

- Restrict bash commands via allow-list
- No file writes through bash tool
- API key management (env vars, not hardcoded)
- Audit logging for all tool executions

---

## 4. Architecture

### 4.1 Directory Structure

```tree
cds-agent/
  ├── src/
  │   ├── index.ts              # Main agent entry point
  │   ├── agent-config.ts       # Agent initialization
  │   ├── tools/
  │   │   ├── cds-search.ts     # MCP tool: search
  │   │   ├── cds-traverse.ts   # MCP tool: traverse
  │   │   └── cds-retrieve.ts   # MCP tool: retrieve
  │   ├── hooks/
  │   │   ├── pre-tool-use.ts
  │   │   ├── post-tool-use.ts
  │   │   └── subagent-stop.ts
  │   ├── prompts/
  │   │   ├── system-prompt.md  # Main agent system prompt
  │   │   └── subagent-prompts/ # Subagent prompts
  │   └── utils/
  │       ├── output-parser.ts  # Parse CDS-Tools JSON
  │       └── context-manager.ts # Manage agent state
  ├── .claude/
  │   └── agents/
  │       └── code-retrievaler.yaml  # Subagent definition
  ├── package.json
  ├── tsconfig.json
  └── README.md
```

### 4.2 Agent Workflow (Sequence Diagram)

```text
User → MainAgent: "Find XSS vulnerability in user profile"
MainAgent → MainAgent: [CoT] Extract keywords: "XSS", "user profile", "sanitize"
MainAgent → CodeRetrievaler: Delegate search task
CodeRetrievaler → PreToolUseHook: Validate cds command
PreToolUseHook → CodeRetrievaler: Inject GRAPH_INDEX_DIR
CodeRetrievaler → CDS-Tools: bash: cds search "XSS sanitize user profile"
CDS-Tools → CDS-Index: Query hierarchical index
CDS-Index → CDS-Tools: Return 5 results (JSON)
CDS-Tools → CodeRetrievaler: stdout: {...}
CodeRetrievaler → PostToolUseHook: Compress output
PostToolUseHook → CodeRetrievaler: Modified output (top 5)
CodeRetrievaler → MainAgent: Return search results
MainAgent → MainAgent: [CoT] Analyze, decide to traverse
MainAgent → CodeRetrievaler: Traverse from entity_abc (invoke, depth 2)
... (repeat) ...
MainAgent → User: Final locations with reasoning
```

---

## 5. LocAgent Refactoring Plan

### 5.1 Prompt Migration

| LocAgent Prompt | CDSAgent Equivalent | Changes |
|----------------|---------------------|---------|
| `util/prompts/system_prompt.j2` | `src/prompts/system-prompt.md` | Adapt for Claude SDK, add tool descriptions |
| `util/prompts/pipelines/auto_search_prompt.py` | Hook logic in `pre-tool-use.ts` | Python → TypeScript |
| Few-shot examples (if any) | Embedded in system prompt | Convert to Markdown |

### 5.2 Agent Loop Refactoring

**LocAgent Agent Loop** (`auto_search_main.py`):

```python
while not done:
    response = llm.generate(history)
    if response.has_tool_call():
        tool_result = execute_tool(response.tool_call)
        history.append(tool_result)
    else:
        done = True
```

**CDSAgent (Claude SDK handles loop internally)**:

```typescript
// Claude SDK manages loop; we just register tools and hooks
const response = await agent.query({
  userMessage: issueDescription,
  maxIterations: 10  // Prevent infinite loops
});
```

### 5.3 Chain-of-Thought Alignment

**LocAgent CoT Steps** (from paper §3.3):

1. Extract keywords from issue
2. Search entities (SearchEntity tool)
3. Traverse dependencies (TraverseGraph tool)
4. Synthesize final locations

**CDSAgent Implementation**:

- Encode steps in system prompt (§2.1 FR-SDK-2)
- Agent follows steps due to prompting (no hardcoded logic)
- Hooks ensure context doesn't explode

---

## 6. Testing and Validation

### 6.1 Unit Tests

- [ ] Agent initialization with valid/invalid API keys
- [ ] Tool registration (MCP tools callable)
- [ ] Hook execution (pre/post/subagent stop)

### 6.2 Integration Tests

- [ ] End-to-end: Issue description → tool calls → final answer
- [ ] Subagent delegation works correctly
- [ ] Hooks modify outputs as expected

### 6.3 Benchmark Validation

- [ ] Run on LocAgent's SWE-bench Lite instances
- [ ] Compare tool call sequences (should be similar to LocAgent traces)
- [ ] Measure accuracy (% correct file/function localizations)

---

## 7. Multi-SDK Extensibility

### 7.1 Abstraction Layer

**Purpose**: Support swapping Claude SDK for OpenAI Codex, Gemini, etc.

**Interface** (`src/sdk-adapter.ts`):

```typescript
export interface AgentSDKAdapter {
  initialize(config: SDKConfig): Promise<void>;
  query(userMessage: string, options?: QueryOptions): Promise<AgentResponse>;
  registerTool(tool: ToolDefinition): void;
  setHooks(hooks: AgentHooks): void;
}

// Claude implementation
export class ClaudeSDKAdapter implements AgentSDKAdapter { /* ... */ }

// Future: OpenAI Codex
export class CodexSDKAdapter implements AgentSDKAdapter { /* ... */ }

// Factory
export const createAgent = (sdkType: 'claude' | 'codex'): AgentSDKAdapter => {
  switch (sdkType) {
    case 'claude': return new ClaudeSDKAdapter();
    case 'codex': return new CodexSDKAdapter();
  }
};
```

**Acceptance Criteria**:

- [ ] Can switch SDK via config parameter
- [ ] Core logic (prompts, tools) reusable across SDKs

---

## 8. Acceptance Criteria Summary

### Must-Have (v0.1.0)

- [ ] Claude agent with streaming mode functional
- [ ] System prompt with chain-of-thought steps
- [ ] `code-retrievaler` subagent with bash tool
- [ ] All 3 hooks (Pre/Post/SubagentStop) implemented
- [ ] Can execute end-to-end code localization task

### Should-Have (v0.2.0)

- [ ] MCP tools (native cds_search, etc.)
- [ ] Multi-SDK adapter pattern
- [ ] Session logging and replay

### Nice-to-Have (Future)

- [ ] Fine-tuning on agent trajectories (LocAgent §5.2)
- [ ] Interactive debugging mode
- [ ] Cost tracking per session

---

## 9. Open Questions

1. **Bash vs MCP**: Bash-only in v0.1.0; evaluate MCP promotion after JSON-RPC hardening (v0.2.0).
2. **Subagent Necessity**: Do we need subagent or just main agent with tools? (Modularity vs complexity)
3. **Prompt Tuning**: How much prompt engineering vs few-shot examples? (LocAgent used fine-tuning)

---

## 10. Next Steps

1. Implement basic Claude agent with bash tool
2. Add system prompt with CoT steps
3. Test on sample issue (e.g., "Find XSS in user profile")
4. Integrate hooks and measure context usage
5. Benchmark against LocAgent trajectories

---

**Status**: Ready for prototyping with Claude SDK. Requires CDS-Tools CLI (PRD-03) for tool execution.
