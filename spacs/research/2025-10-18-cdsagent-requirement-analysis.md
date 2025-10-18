# CDSAgent Requirement Analysis (2025-10-18)

## Source Material

- Issue tracker entry `spacs/issues/01-CDSAgent-MVP-definition.md` (original requirement text, Mandarin).
- LocAgent paper (arXiv:2503.09089v2) and reference repository `tmp/LocAgent/`.
- Claude Agent SDK TypeScript docs under `tmp/claude-agent-sdk/`.
- Claude Code CLI documentation under `tmp/claude-code-cli-docs/`.

## Problem Statement

Design the CDSAgent (Codebase Fast DeepSearch Agent) by refactoring the LocAgent framework so that:

- **CDS-Index** migrates LocAgent graph construction and sparse retrieval into a Rust-based service, while exposing LLM-facing APIs through a TypeScript layer using `@anthropic-ai/claude-agent-sdk` (with optional future swap to other agent SDKs such as the Codex SDK).
- **CDS-Tools** encapsulates the LocAgent runtime tools (SearchEntity, TraverseGraph, RetrieveEntity) as CLI commands (`cds -s/-t/-r/-c`) backed by the new Rust core.
- **CDS-Agent** packages these tools for shell-oriented agents (Claude Code CLI subagents by default) so that Claude Code can orchestrate retrieval workflows via `bash` piped command plans, hooks, and Claude Agent SDK subagent APIs.

## Architecture Overview

The CDSAgent stack decomposes into three primary tiers that mirror the original LocAgent pipeline while satisfying the new polyglot requirements.

1. **Data & Indexing Layer (CDS-Index)**
   - **Core services in Rust**
     - Re-implement LocAgent `dependency_graph` and `repo_index` packages as Rust crates:
       - `cds_graph`: parses repositories (AST-driven for Python first) to build the heterogeneous graph described in LocAgent §3.1 (nodes: directory, file, class, function; relations: contain/import/invoke/inherit).
       - `cds_sparse_index`: maintains hierarchical entity dictionaries and BM25 inverted indexes mirroring LocAgent's Sparse Hierarchical Entity Indexing (§3.1).
       - `cds_traversal`: exposes BFS/meta-path traversal primitives with type and relation filters, preserving tree-structured output to keep Claude context small.
     - Provide pluggable storage backends: start with on-disk JSON/SQLite for graph + BM25 artifacts, plan to upgrade to RocksDB or Postgres when multi-repo persistence becomes necessary.
     - Build incremental updaters so CLI can refresh affected nodes without full rebuild.
   - **Service interface**
     - Expose Rust functionality via a thin gRPC or JSON-RPC server (`cds-indexd`). Chosen interface should stream responses to support large traversal outputs.
     - Generate TypeScript bindings using `prost` + `ts-proto` (for gRPC) or `specta` + `tauri-specta` (for JSON-RPC / tauri-style) to minimize custom glue.
   - **TypeScript integration**
     - Create a `@cds/index-client` package that wraps the Rust server endpoints and conforms to Claude Agent SDK tool interfaces (`tool()` builders) so Claude, via TypeScript, can call graph operations directly.
     - Include fallback path to alternative SDKs by abstracting the transport (Agent SDK queries vs future OpenAI Codex SDK) behind a `GraphIndexProvider` interface.

2. **Tooling Layer (CDS-Tools CLI)**
   - **CLI wrapper**
     - Build a Rust-based binary `cds` that shells out to CDS-Index service or links to crates directly when running in local mode.
     - Commands:
       - `cds search --query <text> [--entity-type function|class|file|directory] [--limit N]` → mirrors LocAgent `SearchEntity`, emits fold/preview/full sections.
       - `cds traverse --start <entity_id> [--rel contain,invoke,...] [--type class,function] [--depth N]` → LocAgent `TraverseGraph` BFS with tree output (JSON + pretty text).
       - `cds retrieve --id <entity_id> [--format json|text]` → LocAgent `RetrieveEntity` returning file path, span, content.
       - `cds combo --plan <yaml|json>` → allow scripted hybrid pipelines (e.g., run search then traverse then pipe into semantic filters using `rg`, `ast-grep`).
     - Output modes: structured JSON (default for Claude) and human-readable text for developers. JSON should remain under agent context limits by chunking via pagination.
   - **Tool composition**
     - Provide `--pipe` flag to emit output to stdout for chaining with `rg`, `ast-grep`, `jq`. Document canonical bash recipes referenced by requirement (Search → Traverse → Retrieve with `sem`, `lex`, `rg`, `ast`).
     - Add exit codes and machine-parsable diagnostics so Claude hooks can infer retry/backoff logic.
   - **Configuration**
     - Support env vars for index directory (mirrors LocAgent `GRAPH_INDEX_DIR`, `BM25_INDEX_DIR`).
     - Provide `cds init` to scaffold config and ensure path compatibility with Claude Code CLI sandbox.

3. **Agent Orchestration Layer (CDS-Agent)**
   - **Claude Code subagent**
     - Define `code-retrievaler` subagent configuration (`CLAUDE.md` / subagent YAML) that limits available tools to `bash`, `cds`, `rg`, `jq`, `ast-grep`, etc., with prompts emphasizing hybrid retrieval strategies.
     - Use Claude Agent SDK `agents` option to register programmatic subagent definitions when running outside Claude Code CLI.
     - Implement hooks:
       - `PreToolUse`: inject current repository context, default index locations, and guardrails (e.g., prefer `cds search` before raw `rg`).
       - `PostToolUse`: summarize results and compress before feeding back to the LLM; log operations for audit.
       - `SubagentStop`: ensure session writes relevant metadata (which CDS tool calls happened) into research logs.
   - **Session management**
     - Use Agent SDK `query()` streaming mode to surface intermediate outputs from `cds` commands; map CLI STDOUT/STDERR to Claude tool outputs.
     - Implement permission middleware via SDK `canUseTool` to enforce safe command subsets.
   - **Multi-SDK compatibility**
     - Abstract agent runtime so switching to OpenAI Codex SDK or other backends only swaps the orchestration layer while reusing Rust CLI.

## Data & Control Flow

1. Repository registration triggers `cds-indexd` to parse code with Rust AST pipelines and update graph/indices.
2. Claude Code CLI (or other shell agent) issues `cds` commands via the `code-retrievaler` subagent. These commands call the CDS-Index service.
3. Results stream back as JSON; hooks summarize and maintain context discipline before returning to the LLM for follow-up reasoning or answer synthesis.
4. Optional: TypeScript layer can call the service directly (without shell) when running in API mode, using Claude Agent SDK custom tools.

## Implementation Roadmap (proposed)

1. **Foundational Spike (Hours: 1-2)**
   - Define proto / RPC schema for graph operations.
   - Port LocAgent graph builders to Rust (target Python language support first).
   - Build minimal CLI (`cds search`, `cds retrieve`) calling the Rust library directly.
2. **Service + SDK Integration (Hours: 3-5)**
   - Stand up `cds-indexd` with streaming responses; write TypeScript client wrappers.
   - Add Claude Agent SDK custom tool definitions for search/traverse/retrieve.
   - Implement CLI pagination, JSON output, and error handling.
3. **Agentization & Hooks (Hours: 6-7)**
   - Configure Claude Code subagent prompts, permissions, and hooks.
   - Implement `cds combo` for hybrid retrieval workflows.
   - Capture usage telemetry for research logs (`spacs/research` paths).
4. **Polyglot Hardening (Hours: 8+)**
   - Extend Rust parsers beyond Python (TypeScript, Java) or integrate tree-sitter.
   - Explore direct Agent SDK invocation bypassing shell when running as hosted service.
   - Evaluate optional Codex SDK swap to validate pluggable design.

## Key Risks & Open Questions

- **Rust ↔ TypeScript bridging:** Selecting a transport that balances low latency (for CLI use) and ease of SDK binding. Need to prototype gRPC vs JSON-RPC.
- **Index size & streaming:** Large repositories may exceed Claude context even after summarization; must chunk responses and let hooks down-sample.
- **Hybrid command safety:** Combining `cds` outputs with shell utilities increases complexity; permission hooks must prevent destructive commands.
- **Language coverage:** Initial focus on Python inherits LocAgent limitation. Plan for incremental addition via tree-sitter grammar modules.
- **Operational deployment:** Define how `cds-indexd` runs (local daemon vs containerized service) and authentication when exposed beyond localhost.

## Deliverables for This Analysis

- This document (placed under `spacs/research/`) summarizing design interpretation of the original requirement.
- Recommendation to update `spacs/issues/01-CDSAgent-MVP-definition.md` to link here and track follow-up implementation tasks (CDS-Index, CDS-Tools, CDS-Agent, SDK integration).
- Subsequent PRD alignment work is managed via `spacs/issues/03-CDSAgent-PRDs-alignment.md`.
