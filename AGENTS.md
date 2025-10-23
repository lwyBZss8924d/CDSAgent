# AGENTS.md

This file provides guidance to Codex when working with code in this "CDSAgent" repository.

📌 @docs/WORKTREE_WORKFLOW.md ["CDSAgent" codebase Spec-Tasks DEV-COOKING Workflow SOP"](docs/WORKTREE_WORKFLOW.md)
📌 @docs/NEXT_TASK_CHECKLIST.md ["Next Task Selection & Environment Setup Checklist"](docs/NEXT_TASK_CHECKLIST.md)

---

## 🎯 Current Task: T-05-02 TypeScript Client Bindings & SDK Integration

**Branch**: `feat/task/T-05-02-typescript-bindings`
**Worktree**: `~/dev-space/CDSAgent-T-05-02-typescript-bindings`
**Status**: 🚧 In Progress | **Priority**: P0 (M1 Critical Path)
**Timeline**: Week 1 (Target: 2025-10-24) | **M1 Deadline**: 2025-10-26

### Objective

Generate strongly typed TypeScript client methods for the JSON-RPC API, enabling the agent to call CDS-Index Service methods with compile-time safety.

### Deliverables

1. **`cds-agent/src/types/api.ts`** - TypeScript type definitions from JSON Schema
2. **`cds-agent/src/client/jsonrpc.ts`** - JSON-RPC client with fetch + retry + error handling
3. **`cds-agent/tests/jsonrpc-client.test.ts`** - Unit tests (success + error scenarios)

### Dependencies

- ✅ **T-05-01** (JSON-RPC Schema) - COMPLETED & MERGED (PR #3)
  - Schema: `docs/api/jsonrpc-schema.json` (18.9KB)
  - Docs: `docs/api/README.md`, `error-codes.md`, `versioning.md`
- ⏳ **T-02-03** (Service Layer) - NOT STARTED (runtime dependency, not blocking client implementation)

### Acceptance Criteria

- [ ] Client methods compile and work: `searchEntities()`, `traverseGraph()`, `retrieveEntity()`, `getEntityDetails()`
- [ ] TypeScript types align with `docs/api/jsonrpc-schema.json` (no `any` usage)
- [ ] Unit tests cover success + error scenarios
- [ ] Client integrated with agent entrypoint (`main.ts`)

### Task References

- **Task Spec**: [spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-02-typescript-bindings.md](spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-02-typescript-bindings.md)
- **PRD**: [PRD-05 §3](spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md) (API Specifications)
- **PRD**: [PRD-04 §2.1](spacs/prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md) (Agent Integration)
- **Worklog**: `.artifacts/spec-tasks-T-05-02-typescript-bindings/`

---

## Repository Overview

CDSAgent is a graph-based code retrieval system built with Rust (core indexing) and TypeScript (LLM orchestration). The repository contains:

- **Rust**: `crates/cds-index/` — Core indexing service with graph builder and sparse search
- **Rust**: `crates/cds-tools/` — CLI tools for search, traverse, and retrieve operations
- **TypeScript/Bun**: `cds-agent/` — Claude Agent SDK integration with hooks and prompts
- **Documentation**: `spacs/` — PRDs, issues, tasks, and planning documents
- **Reference**: `tmp/LocAgent/` — Original Python implementation for parity validation

## CDSAgent Core Tech Methods References from Paper & Repo !!! IMPORTANT

- **CDSAgent follows LocAgent Paper**: `tmp/LocAgent/arXiv-2503.09089v2`
- **LocAgent paper source**: <https://arxiv.org/html/2503.09089v2>
- **CDSAgent refactors LocAgent Repo**: `tmp/LocAgent/`
- **LocAgent repo source**: <https://github.com/gersteinlab/LocAgent>

## Project Structure

```tree
CDSAgent/
├── crates/
│   ├── cds-index/          # Rust: Index Service
│   │   ├── src/
│   │   │   ├── graph/      # AST parsing & graph building
│   │   │   ├── index/      # Name index + BM25 search
│   │   │   ├── service/    # JSON-RPC server
│   │   │   └── bin/        # cds-index-service binary
│   │   └── tests/          # Integration tests
│   └── cds-tools/          # Rust: CLI Tools
│       ├── src/
│       │   ├── commands/   # search, traverse, retrieve
│       │   ├── client/     # JSON-RPC client
│       │   └── formatters/ # JSON, text, tree output
│       └── tests/          # CLI integration tests
├── cds-agent/              # TypeScript/Bun: Agent
│   ├── src/
│   │   ├── agent-config.ts
│   │   ├── system-prompt.ts
│   │   └── hooks/          # PreToolUse, PostToolUse, SubagentStop
│   └── .claude/agents/     # Subagent configurations
├── spacs/                  # Specifications
│   ├── prd/                # Product requirements
│   ├── issues/             # Technical specs
│   ├── tasks/              # Implementation tasks
│   └── plan/               # Backlog and roadmap
├── tmp/                    # Reference implementations
│   └── LocAgent/           # Original Python code (parity reference)
├── justfile                # Build automation
├── Cargo.toml              # Rust workspace
└── README.md               # Development guide
```

## T-05-02: TypeScript Client Development Commands

### Environment Setup

```bash
# Navigate to T-05-02 worktree
cd ~/dev-space/CDSAgent-T-05-02-typescript-bindings

# Install dependencies (Bun + Claude SDK + dev tools)
cd cds-agent
bun install

# Verify TypeScript configuration
bun run typecheck
```

### Type Generation from JSON Schema

#### Option 1: quicktype (automated)

```bash
# Generate TypeScript types from JSON Schema
bunx quicktype ../docs/api/jsonrpc-schema.json \
  -o src/types/api.ts \
  --lang typescript \
  --just-types \
  --prefer-unions

# Review generated types
cat src/types/api.ts
```

#### Option 2: Manual typing with Zod (recommended for runtime validation)

```typescript
// src/types/api.ts
import { z } from 'zod';

export const EntityTypeSchema = z.enum(['directory', 'file', 'class', 'function']);
export const RelationTypeSchema = z.enum(['contain', 'import', 'invoke', 'inherit']);

export const EntitySchema = z.object({
  id: z.string(),
  name: z.string(),
  entity_type: EntityTypeSchema,
  file_path: z.string(),
  line_range: z.tuple([z.number(), z.number()]),
  score: z.number().min(0).max(1),
  snippet: z.object({
    fold: z.string(),
    preview: z.string().optional(),
    full: z.string().optional(),
  }),
});

export type Entity = z.infer<typeof EntitySchema>;
```

### Development Workflow

```bash
# Start agent (development mode)
bun run dev

# Run unit tests
bun test

# Watch mode for tests
bun run test:watch

# Type checking (no emit)
bun run typecheck

# Linting
bun run lint

# Format code
bun run fmt
```

### Implementation Checklist

#### Step 1: Create directory structure

```bash
mkdir -p cds-agent/src/client
mkdir -p cds-agent/src/types
mkdir -p cds-agent/tests
```

#### Step 2: Implement type definitions (`src/types/api.ts`)

- [ ] Core types: `Entity`, `EntityType`, `RelationType`, `SnippetMode`
- [ ] Request types: `SearchRequest`, `TraverseRequest`, `RetrieveRequest`, `InitializeRequest`
- [ ] Response types: `SearchResult`, `TraversalResult`, `EntityDetails`, `InitializeResponse`
- [ ] Error types: `JSONRPCError`, error code enums
- [ ] NO `any` types allowed

#### Step 3: Implement JSON-RPC client (`src/client/jsonrpc.ts`)

- [ ] Base client class with configurable endpoint
- [ ] Generic `call<T>(method, params)` method
- [ ] Retry logic with exponential backoff (3 retries, 1s/2s/4s delays)
- [ ] Error mapping (JSON-RPC errors → typed exceptions)
- [ ] Typed wrapper methods:
  - [ ] `searchEntities(query, types?, limit?)`
  - [ ] `traverseGraph(startEntities, depth, relations?)`
  - [ ] `retrieveEntity(entityId, snippetMode?)`
  - [ ] `getEntityDetails(entityId)`
  - [ ] `initializeIndex(repoPath, config?)`

#### Step 4: Write unit tests (`tests/jsonrpc-client.test.ts`)

- [ ] Mock JSON-RPC server responses (using Bun test mocking)
- [ ] Test success scenarios (all 4 methods)
- [ ] Test error scenarios:
  - [ ] Network errors (timeout, connection refused)
  - [ ] JSON-RPC errors (-32001 to -32004)
  - [ ] Invalid response format
  - [ ] Retry logic verification
- [ ] Test type safety (no runtime `any` leakage)

#### Step 5: Integration with agent (`src/main.ts`)

- [ ] Import client in agent entrypoint
- [ ] Initialize with service URL from env var (`CDS_INDEX_SERVICE_URL`)
- [ ] Expose client methods to agent hooks

### API Reference Quick Access

#### Service Endpoint

```bash
# Default (configurable via env)
export CDS_INDEX_SERVICE_URL=http://localhost:9876/rpc
```

#### 4 JSON-RPC Methods

1. **`search_entities`** - Hierarchical search (name index + BM25)
   - Params: `query`, `entity_types?`, `limit?`, `snippet_mode?`
   - Returns: `entities[]`, `total_count`, `query_metadata`

2. **`traverse_graph`** - BFS graph traversal
   - Params: `start_entities`, `depth`, `relations?`, `snippet_mode?`
   - Returns: `nodes[]`, `edges[]`, `metadata`

3. **`retrieve_entity`** - Fetch single entity details
   - Params: `entity_id`, `snippet_mode?`
   - Returns: `entity` with full details

4. **`initialize_index`** - Index a repository
   - Params: `repo_path`, `language?`, `config?`
   - Returns: `status`, `stats` (node/edge counts)

#### Core Types (from `docs/api/jsonrpc-schema.json`)

- `EntityType`: `"directory" | "file" | "class" | "function"`
- `RelationType`: `"contain" | "import" | "invoke" | "inherit"`
- `SnippetMode`: `"fold" | "preview" | "full"`
- `Entity`: `{ id, name, entity_type, file_path, line_range, score, snippet }`

#### Error Codes (see `docs/api/error-codes.md`)

- `-32001`: Index not initialized
- `-32002`: Entity not found
- `-32003`: Invalid traversal depth
- `-32004`: Index operation failed

### Testing Strategy

#### Unit Tests (`bun test`)

```typescript
import { describe, test, expect, mock } from 'bun:test';
import { JSONRPCClient } from '../src/client/jsonrpc';

describe('JSONRPCClient', () => {
  test('searchEntities returns typed results', async () => {
    const client = new JSONRPCClient('http://mock:9876/rpc');

    // Mock fetch response
    global.fetch = mock(() => Promise.resolve({
      ok: true,
      json: async () => ({
        jsonrpc: '2.0',
        id: 1,
        result: {
          entities: [{ id: 'test', name: 'TestFunc', /* ... */ }],
          total_count: 1,
        },
      }),
    }));

    const result = await client.searchEntities('test query');
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].name).toBe('TestFunc');
  });

  test('handles JSON-RPC errors', async () => {
    // Test error code -32001 (index not initialized)
    global.fetch = mock(() => Promise.resolve({
      ok: true,
      json: async () => ({
        jsonrpc: '2.0',
        id: 1,
        error: { code: -32001, message: 'Index not initialized' },
      }),
    }));

    await expect(client.searchEntities('test')).rejects.toThrow('Index not initialized');
  });
});
```

#### Integration Tests (manual, requires running service)

```bash
# Start CDS-Index service (Rust)
cd ~/dev-space/CDSAgent
cargo run --bin cds-index-service

# Run TypeScript client against real service
cd cds-agent
bun run dev
```

### Task-Specific Notes

- **Strict typing**: NO `any` types allowed; use `unknown` and type guards if needed
- **Runtime validation**: Recommend `zod` for validating JSON-RPC responses
- **Retry logic**: Exponential backoff (1s, 2s, 4s) for network errors only (not JSON-RPC errors)
- **Error handling**: Map error codes to custom exception classes (`IndexNotInitializedError`, etc.)
- **Service discovery**: Support env var `CDS_INDEX_SERVICE_URL` (default: `http://localhost:9876/rpc`)
- **Timeout**: 30s default, configurable per request
- **Logging**: Integrate with agent's logging system (respect `LOG_LEVEL` env var)

---

## Commands and workflows

### Refactor Reference: LocAgent (Python) — tmp/LocAgent

Environment

- Python 3.12 (conda recommended)
- Install deps:

```shell
# from repo root
conda create -n locagent python=3.12 -y
conda activate locagent
pip install -r tmp/LocAgent/requirements.txt
```

Key environment variables (export before running):

```shell
# Add project root (or tmp/LocAgent) to PYTHONPATH for imports
export PYTHONPATH="$PYTHONPATH:$(pwd)/tmp/LocAgent"
# Prebuilt index locations (optional but recommended)
export GRAPH_INDEX_DIR="{INDEX_DIR}/{DATASET_NAME}/graph_index_v2.3"
export BM25_INDEX_DIR="{INDEX_DIR}/{DATASET_NAME}/BM25_index"
```

Build indexes (optional but recommended)

- Graph index (batch over datasets):

```shell
python tmp/LocAgent/dependency_graph/batch_build_graph.py \
  --dataset 'czlll/SWE-bench_Lite' \
  --split 'test' \
  --num_processes 50 \
  --download_repo
```

- BM25 sparse index:

```shell
python tmp/LocAgent/build_bm25_index.py \
  --dataset 'czlll/SWE-bench_Lite' \
  --split 'test' \
  --num_processes 100 \
  --download_repo
```

Run code localization

```shell
# Set your model endpoint first (example)
# export OPENAI_API_KEY="..."
# export OPENAI_API_BASE="https://api.openai.com/v1"

python tmp/LocAgent/auto_search_main.py \
  --dataset 'czlll/SWE-bench_Lite' \
  --split 'test' \
  --model 'azure/gpt-4o' \
  --localize \
  --merge \
  --output_folder results/location \
  --eval_n_limit 300 \
  --num_processes 50 \
  --use_function_calling \
  --simple_desc
```

Scripts (shortcuts)

```shell
# generate graph index for SWE-bench and Loc-Bench
bash tmp/LocAgent/scripts/gen_graph_index.sh
# generate BM25 index
bash tmp/LocAgent/scripts/gen_bm25_index.sh
# example run with env var template
bash tmp/LocAgent/scripts/run.sh
```

Format (no dedicated linter configured)

```shell
black tmp/LocAgent
```

Tests

- No test suite currently included for LocAgent.

### Email Agent demo (TypeScript + Bun) — tmp/claude-agent-sdk/claude-code-sdk-demos/email-agent

Important rules (from this subproject’s CLAUDE.md)

- Always use Bun for this project
- Subagents live under agent/.claude/agents

Setup and run (local-only demo)

```shell
# from repo root
cd tmp/claude-agent-sdk/claude-code-sdk-demos/email-agent
bun install
cp .env.example .env  # edit credentials for local IMAP testing
bun run dev           # starts server and client
# open http://localhost:3000
```

Build client bundle

```shell
bun run build
```

Tests (Jest)

```shell
# all tests
bun run test
# watch mode
bun run test:watch
# coverage
bun run test:coverage
# run a single test file or by name
bun run jest -- path/to/file.test.ts -t "test name pattern"
```

Linting

- No ESLint/Prettier config is wired in this demo; rely on TypeScript + Jest checks.

## High-level architecture and structure

### LocAgent (Python)

- dependency_graph/ — builds and traverses a directed heterogeneous code graph
  - build_graph.py (graph construction, graph version v2.3)
  - traverse_graph.py (RepoEntitySearcher, RepoDependencySearcher APIs)
  - batch_build_graph.py (dataset-scale index build)
- repo_index/ — semantic code search and indexing
  - index/ (FAISS-backed retrieval, settings/types)
  - codeblocks/ (code parsing to blocks, language-specific parsers)
  - utils/ (repo utilities, tokenization)
- plugins/location_tools/ — retrieval and repo operations
  - retriever/ (BM25 and fuzzy retrievers)
  - repo_ops/ (repo-level ops, issue handling)
  - utils/ (formatting, dependency helpers)
- util/ — runtime, prompts, and action framework
  - runtime/ (IPython execution, function-calling glue)
  - prompts/ (Jinja2 templates and pipelines)
  - actions/ (action spec and parser)
  - benchmark/ (git repo prep, patch parsing, oracle generation)
- evaluation/ — metrics and notebook for evaluation
- Entry points — auto_search_main.py (main agent), scripts/*.sh (common flows)

Data flow (big picture)

- Retrieve candidates via BM25/semantic index (repo_index, plugins/retriever)
- Traverse code graph for multi-hop reasoning (dependency_graph)
- Orchestrate agent actions and tool calls (util/runtime, actions, prompts)
- Produce localization outputs and optional evaluation artifacts (evaluation)

Key environment assumptions

- GRAPH_INDEX_DIR and BM25_INDEX_DIR point to prebuilt indices (optional; built on-demand otherwise)
- Model access via litellm-compatible endpoints (e.g., OpenAI/Azure/Claude/Qwen)

### Email Agent demo (TypeScript + Bun)

- ccsdk/ — Claude Agent SDK integration (client/session/tools)
- server/ — REST/WebSocket endpoints and server bootstrap
- client/ — React UI (bundled via Bun), real-time streaming
- database/ — SQLite-backed local cache and search
- agent/.claude/agents — subagents and prompts for Claude Code agent flows

Notes

- This is a local demo with plain-text env vars and no auth; do not deploy to production.

## Cross-references and source materials

- tmp/LocAgent/README.md and CLAUDE.md — commands, datasets, env vars, and model support
- tmp/LocAgent/AGENTS.md — architecture summary and common commands
- tmp/claude-agent-sdk/claude-code-sdk-demos/email-agent/README.md and CLAUDE.md — demo setup, Bun usage, test scripts
- tmp/claude-code-cli-docs — CLI documentation (no build/run)
