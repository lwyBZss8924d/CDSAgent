# PRD-01: CDSAgent System Architecture

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Author:** CDSAgent Architecture Team

---

## 1. Document Overview

### 1.1 Purpose

This document defines the high-level system architecture for CDSAgent (Codebase Fast DeepSearch Agent), a code localization assistant that refactors the LocAgent framework using Rust for performance-critical components and TypeScript for LLM agent orchestration.

### 1.2 Scope

- Overall system architecture and component relationships
- Technology stack and architectural decisions
- System boundaries and integration points
- Design principles derived from LocAgent research

### 1.3 References

- **LocAgent Paper**: <https://arxiv.org/html/2503.09089v2> (§2, §3 - Overview and Method)
- **LocAgent Repository**: tmp/LocAgent/ (architecture reference)
- **Issue-02**: spacs/issues/02-CDSAgent-Tech-Architecture-Plan.md
- **Requirement Analysis**: spacs/research/2025-10-18-cdsagent-requirement-analysis.md

---

## 2. System Vision

### 2.1 Problem Statement

Developers need to quickly locate relevant code snippets in large codebases when addressing issues or bugs. Existing approaches either:

- Provide raw text search (grep, ripgrep) without understanding code structure
- Require manual navigation of complex dependency graphs
- Fail to leverage LLM reasoning for multi-hop code exploration

### 2.2 Solution Overview

CDSAgent combines three key innovations from LocAgent research:

1. **Graph-Based Code Indexing**: Directed heterogeneous graphs capturing code relationships (contain, import, invoke, inherit)
2. **Hierarchical Sparse Retrieval**: Two-tier indexing (name/ID + BM25 content) for efficient keyword matching
3. **LLM-Guided Navigation**: Claude Agent SDK orchestration for autonomous multi-hop code traversal

### 2.3 Key Objectives

- **Performance**: Rust-based core for sub-second query responses on large codebases
- **Accuracy**: Maintain LocAgent's benchmark results (SWE-bench Lite accuracy)
- **Extensibility**: Pluggable LLM SDK architecture (Claude, OpenAI Codex, etc.)
- **Developer UX**: Intuitive CLI tools that compose via Unix pipelines

---

## 3. High-Level Architecture

### 3.1 Three-Tier Architecture

```text
┌─────────────────────────────────────────────────────────────────┐
│                   CDS-Agent Layer (TypeScript)                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ Claude Agent SDK / code-retrievaler subagent             │   │
│  │ - Hooks (PreToolUse, PostToolUse, SubagentStop)          │   │
│  │ - MCP Tools Registration                                 │   │
│  │ - Session Management & Streaming                         │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              ↓ bash / tool calls
┌─────────────────────────────────────────────────────────────────┐
│                    CDS-Tools Layer (Rust CLI)                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ cds search   │ cds traverse  │ cds retrieve │ cds combo  │   │
│  │ - Keyword search │ BFS graph │ Entity fetch │ Hybrid     │   │
│  │ - JSON/text out  │ navigation│ Full content │ pipelines  │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              ↓ RPC / direct calls
┌─────────────────────────────────────────────────────────────────┐
│                  CDS-Index Layer (Rust Core)                    │
│  ┌─────────────────┬──────────────────┬─────────────────────┐   │
│  │ cds_graph       │ cds_sparse_index │ cds_traversal       │   │
│  │ - AST parsing   │ - Name/ID index  │ - BFS algorithms    │   │
│  │ - Graph builder │ - BM25 inverted  │ - Meta-path filters │   │
│  │ - Node/edge DB  │   index          │ - Tree formatting   │   │
│  └─────────────────┴──────────────────┴─────────────────────┘   │
│                          ↓ persistent storage                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ Graph Index Files: nodes.db, edges.db, entities.json     │   │
│  │ Sparse Indices: bm25_index/, name_dict.json              │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 Component Descriptions

#### **Layer 1: CDS-Index Service** (Rust Core)

- **Purpose**: Build and maintain code graph and search indices
- **Technology**: Rust (performance-critical parsing, indexing, graph algorithms)
- **Refactors**: LocAgent `dependency_graph/`, `repo_index/`
- **Key Operations**:
  - AST-based code parsing (Python in v0.1.0; roadmap adds TypeScript/JavaScript + Rust in v0.2.0, Go in v0.3.0 via tree-sitter)
  - Heterogeneous graph construction (4 node types, 4 edge types)
  - Hierarchical entity indexing (name/ID + BM25 content)
  - Incremental index updates

#### **Layer 2: CDS-Tools CLI** (Rust Binary)

- **Purpose**: Unified command-line interface for code retrieval
- **Technology**: Rust with `clap` for CLI, JSON/text output
- **Refactors**: LocAgent `plugins/location_tools/`, runtime APIs
- **Core Commands**:
  - `cds search`: Keyword → entities (fold/preview/full)
  - `cds traverse`: Entity → subgraph (BFS with filters)
  - `cds retrieve`: Entity ID → full code
  - `cds combo`: Hybrid retrieval pipelines

#### **Layer 3: CDS-Agent Integration** (TypeScript + Agent Config)

- **Purpose**: LLM orchestration for autonomous code search
- **Technology**: TypeScript with `@anthropic-ai/claude-agent-sdk`
- **Refactors**: LocAgent agent logic, prompts, tool calling
- **Key Features**:
  - `code-retrievaler` subagent with restricted bash tools
  - PreToolUse/PostToolUse hooks for context injection
  - MCP tools registration for direct TS → Rust calls
  - Multi-SDK abstraction layer (Claude, Codex, Gemini)

### 3.3 Data Flow

**Typical Code Search Session:**

```text
1. User Query → Claude Agent SDK
   ↓
2. Agent (Chain-of-Thought) → "Search for sanitize functions"
   ↓
3. Tool Call: bash → cds search "sanitize input"
   ↓
4. CDS-Tools CLI → CDS-Index Service (RPC)
   ↓
5. Index Service → BM25 search + Name/ID lookup
   ↓
6. Results (JSON) → CLI → stdout
   ↓
7. PostToolUse Hook → Summarize, compress context
   ↓
8. Agent observes results → "Found 3 functions, traverse calls"
   ↓
9. Tool Call: bash → cds traverse --start func_id_123 --depth 1
   ↓
10. (Repeat) → Final answer with code locations
```

---

## 4. Technology Stack

### 4.1 Core Technologies

| Component | Language | Key Libraries/Frameworks | Rationale |
|-----------|----------|-------------------------|-----------|
| **CDS-Index** | Rust | tree-sitter, tantivy, serde | Performance (AST parsing, BM25), memory safety |
| **CDS-Tools** | Rust | clap, tokio, serde_json | Fast CLI, async I/O, structured output |
| **CDS-Agent** | TypeScript | @anthropic-ai/claude-agent-sdk | Native Claude integration, hooks, MCP |
| **Bridge Layer** | gRPC/JSON-RPC | tonic (Rust), @grpc/grpc-js (TS) | Low-latency Rust ↔ TS communication |
| **Storage** | File-based + SQLite | sled (Rust), better-sqlite3 (TS) | Lightweight, embedded persistence |

### 4.2 Language Support Roadmap

- **v0.1.0 (MVP)**: Python repositories only (consistent with LocAgent’s evaluated scope). citetmp/LocAgent/arXiv-2503.09089v2/3_method.tex:54-58
- **v0.2.0**: Enable TypeScript/JavaScript and Rust parsing via tree-sitter modules; treat as optional plugins until parity validation completes.
- **v0.3.0**: Extend ingestion to Go; reassess demand for additional languages (e.g., Java, C++).

Each stage requires regression checks that mirror the LocAgent Python baselines before marking the language as production-ready.

### 4.3 Development Tools

- **Build System**: Cargo (Rust), Bun (TypeScript)
- **Testing**: cargo test, jest, criterion (benchmarks)
- **Linting**: clippy, rustfmt, prettier
- **Documentation**: rustdoc, typedoc

---

## 5. Architectural Principles

### 5.1 Principle 1: Preserve LocAgent Research Fidelity

**Rationale**: LocAgent achieved state-of-the-art results on SWE-bench Lite by:

- Using heterogeneous graphs (not just file trees)
- Combining name/ID + content indexing (ablation study showed both needed)
- Tree-formatted graph output (improves LLM reasoning per Fatemi et al. 2023)

**Application**: CDSAgent Rust refactoring must:

- Reproduce identical graph structure (4 node types, 4 edge types)
- Maintain two-tier hierarchical indexing
- Use same BFS traversal with type/relation filters
- Format outputs as expanded trees

**Validation**: Benchmark against LocAgent's published metrics

### 5.2 Principle 2: CLI-First Design for Agent Composition

**Rationale**: Unix philosophy enables hybrid retrieval:

```shell
cds search "XSS vulnerability" | jq '.ids[]' | cds traverse --depth 2 | rg "sanitize"
```

**Application**:

- All tools output structured JSON (machine-readable)
- Support stdin/stdout piping
- Composable flags and filters
- Human-readable text mode for debugging

### 5.3 Principle 3: Pluggable LLM SDK Architecture

**Rationale**: Avoid lock-in to single agent framework

**Application**:

- Abstract agent operations behind `AgentProvider` interface
- CDS-Tools CLI is SDK-agnostic (works via bash or API)
- TypeScript layer provides adapters for Claude, Codex, etc.
- Core Rust components never depend on specific LLM

### 5.4 Principle 4: Performance by Default

**Rationale**: Large codebases (10K+ files) require fast indexing/querying

**Application**:

- Rust for all hot paths (parsing, BM25, graph traversal)
- Lazy loading of index data (mmap, streaming)
- Incremental updates (no full reindex on code changes)
- Target: <1s for search, <2s for 2-hop traversal

### 5.5 Principle 5: Developer Experience Priority

**Rationale**: Adoption depends on intuitive UX

**Application**:

- Clear, consistent CLI command structure
- Rich error messages with suggestions
- Progress indicators for long operations
- Extensive examples in documentation

---

## 6. System Boundaries

### 6.1 In Scope

- Code graph indexing for Python, TypeScript, Java (via tree-sitter)
- Keyword and BM25-based code search
- Graph traversal with meta-path filtering
- Claude Agent SDK integration with hooks
- CLI tools for manual and agent-driven use
- Local file system storage

### 6.2 Out of Scope (Future Work)

- Vector/semantic search (no embeddings in v1)
- Real-time code change monitoring (manual re-index)
- Multi-repository federated search
- Web UI (CLI only in v1)
- Code editing/patching (retrieval only)
- Authentication/authorization (local use)

### 6.3 External Dependencies

- **LocAgent Research**: Algorithm reference, benchmark datasets
- **tree-sitter**: Multi-language AST parsing
- **Claude API**: LLM inference (requires API key)
- **SWE-bench Lite**: Validation dataset

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| Index build time | <5s per 1K files | `cds index --benchmark` |
| Search latency | <500ms (p95) | `cds search` with timing |
| Traverse latency | <1s for 2 hops | `cds traverse --depth 2` |
| Memory usage | <2GB for 10K files | Process monitoring |

### 7.2 Reliability

- Index corruption resilience (checkpointing)
- Graceful degradation (partial index available)
- Error recovery (retry logic in agent hooks)

### 7.3 Maintainability

- <15% code duplication
- >80% test coverage (Rust crates)
- Documented APIs (rustdoc, README per crate)

### 7.4 Portability

- Linux, macOS, Windows support
- Minimal system dependencies (statically linked where possible)

---

## 8. Success Criteria

### 8.1 Functional Success

- [ ] Can index Python repository and query via CLI
- [ ] Search returns results matching LocAgent's format
- [ ] Traverse produces tree-formatted subgraphs
- [ ] Claude agent can execute multi-step code search
- [ ] Hooks inject/compress context successfully

### 8.2 Performance Success

- [ ] Achieves >80% of LocAgent's accuracy on SWE-bench Lite
- [ ] Meets latency targets (§7.1)
- [ ] Rust indexing 2x faster than Python baseline

### 8.3 Extensibility Success

- [ ] Can swap Claude SDK for mock SDK without changing CLI
- [ ] Adding new language requires only tree-sitter grammar
- [ ] Hooks are configurable via YAML/JSON

---

## 9. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Rust ↔ TS bridging complexity | High | Prototype gRPC vs JSON-RPC early (Phase 1) |
| Index size exceeds memory | Medium | Implement mmap + lazy loading, pagination |
| Claude API rate limits | Medium | Local caching, exponential backoff in hooks |
| Tree-sitter grammar gaps | Low | Fallback to simple regex parsing for unsupported languages |
| LocAgent accuracy regression | High | Continuous benchmarking against published metrics |

---

## 10. Next Steps

1. **Detailed Component PRDs**: Refer to PRD-02 (CDS-Index), PRD-03 (CDS-Tools), PRD-04 (CDS-Agent)
2. **API Specifications**: PRD-05 defines Rust ↔ TS interfaces
3. **Refactoring Plan**: PRD-06 maps LocAgent code to Rust modules
4. **Implementation Roadmap**: PRD-09 defines 4-phase development plan

---

## Appendix A: Architectural Diagrams (Text)

### A.1 Component Dependency Graph

```text
┌────────────────┐
│  Claude Agent  │
│  (TypeScript)  │
└───────┬────────┘
        │ uses
        ↓
┌────────────────┐       ┌──────────────┐
│   CDS-Tools    │──────→│  CDS-Index   │
│   (Rust CLI)   │ calls │ (Rust Crates)│
└────────────────┘       └──────────────┘
        │                        │
        ↓                        ↓
   [stdout/JSON]          [Index Files]
```

### A.2 LocAgent → CDSAgent Mapping

| LocAgent Component | CDSAgent Equivalent | Language |
|-------------------|---------------------|----------|
| dependency_graph/ | cds_graph crate | Python → Rust |
| repo_index/ | cds_sparse_index crate | Python → Rust |
| plugins/location_tools/ | cds CLI commands | Python → Rust |
| auto_search_main.py | code-retrievaler subagent | Python → TypeScript |
| util/prompts/ | Claude agent system prompts | Python → TypeScript |

---

**Document Status**: Ready for review by architecture team and stakeholders.

**Next Review**: After PRD-02 to PRD-10 completion (Round 1).
