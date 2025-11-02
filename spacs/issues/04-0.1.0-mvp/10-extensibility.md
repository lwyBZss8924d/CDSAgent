# Issue-10: Extensibility & Future Work Backlog

**Priority**: P2 (Integration & Polish - Future Planning)
**Status**: ☐ Not Started
**Owner**: Tech Lead + Product Manager
**PRD Reference**: [PRD-10: Extensibility & Future](../../prd/0.1.0-MVP-PRDs-v0/10-extensibility-future.md)

---

## Overview

Document extensibility mechanisms built into CDSAgent v0.1.0 and maintain backlog of future enhancements for v0.2.0, v0.3.0, and beyond. This ensures the MVP is architected for evolution without major refactoring.

## Objective

Establish a forward-looking roadmap that:

- Captures features deferred from v0.1.0 scope
- Tracks technical debt and improvement opportunities
- Guides v0.2.0+ planning conversations
- Maintains alignment with research advances (LocAgent, LLM capabilities)

## Dependencies

- **Feeds Into**: v0.2.0 planning (tracked in Issue-02 TODOs)
- **Requires**: Implementation learnings from v0.1.0
- **Timing**: Continuous (updated throughout v0.1.0 development)

---

## Extensibility Mechanisms in v0.1.0

### 1. Multi-SDK Abstraction Layer (PRD-10 §2)

**Built-in Extension Point**:

```typescript
// cds-agent/src/sdk-adapter.ts
export interface AgentSDKAdapter {
  initialize(config: SDKConfig): Promise<void>;
  query(userMessage: string, options?: QueryOptions): Promise<AgentResponse>;
  registerTool(tool: ToolDefinition): void;
  setHooks(hooks: AgentHooks): void;
}

// Claude implementation (v0.1.0)
export class ClaudeSDKAdapter implements AgentSDKAdapter { /* ... */ }

// Future: OpenAI Codex (v0.2.0)
export class CodexSDKAdapter implements AgentSDKAdapter { /* ... */ }

// Factory
export const createAgent = (sdkType: 'claude' | 'codex'): AgentSDKAdapter => {
  switch (sdkType) {
    case 'claude': return new ClaudeSDKAdapter();
    case 'codex': return new CodexSDKAdapter();
  }
};
```

**v0.1.0 Validation**:

- [ ] ClaudeSDKAdapter fully implements interface
- [ ] Mock SDK for testing demonstrates swappability
- [ ] Interface is SDK-agnostic (no Claude-specific leakage)

**v0.2.0 Task**: Implement `CodexSDKAdapter` for OpenAI models

---

### 2. Multi-Language Parser Plugin Architecture (PRD-10 §3)

**Built-in Extension Point**:

```rust
// cds_graph/src/parsers/mod.rs
pub trait LanguageParser {
    fn name(&self) -> &str;
    fn file_extensions(&self) -> &[&str];
    fn parse(&self, source: &str) -> Result<Vec<Entity>>;
}

// Python parser (v0.1.0)
pub struct PythonParser { /* tree-sitter Python */ }

impl LanguageParser for PythonParser {
    fn name(&self) -> &str { "python" }
    fn file_extensions(&self) -> &[&str] { &["py"] }
    fn parse(&self, source: &str) -> Result<Vec<Entity>> { /* ... */ }
}

// Future: TypeScript parser (v0.2.0)
pub struct TypeScriptParser { /* tree-sitter TypeScript */ }
```

**Parser Registry**:

```rust
// cds_graph/src/parsers/registry.rs
pub struct ParserRegistry {
    parsers: HashMap<&'static str, Box<dyn LanguageParser>>,
}

impl ParserRegistry {
    pub fn register(&mut self, parser: Box<dyn LanguageParser>) {
        for ext in parser.file_extensions() {
            self.parsers.insert(ext, parser.clone());
        }
    }

    pub fn parse_file(&self, path: &Path) -> Result<Vec<Entity>> {
        let ext = path.extension().and_then(|s| s.to_str()).ok_or(...)?;
        let parser = self.parsers.get(ext).ok_or(...)?;
        let source = fs::read_to_string(path)?;
        parser.parse(&source)
    }
}
```

**v0.1.0 Validation**:

- [ ] PythonParser registered and functional
- [ ] Registry allows runtime parser addition
- [ ] File extension → parser lookup works

**v0.2.0 Task**: Add TypeScriptParser, RustParser via same registry

---

### 3. Tool Extensibility (PRD-10 §4)

**Built-in Extension Point**:

```shell
# CLI composability (v0.1.0)
cds search "query" | jq '.results[].entity_id' | xargs cds retrieve

# Future: Additional tools (v0.2.0+)
cds semantic-search "embeddings query"  # Vector search
cds ast-query --pattern 'if $COND: raise $ERR'  # AST patterns
```

**MCP Tool Registration** (v0.2.0):

```typescript
// Future: Native MCP tools (not bash wrappers)
agent.registerTools([
  cdsSearchTool,     // Direct TypeScript → Rust calls
  cdsTraverseTool,
  cdsRetrieveTool,
  cdsSemanticSearchTool,  // New in v0.2.0
]);
```

**v0.1.0 Validation**:

- [ ] Bash tool allows CLI composition
- [ ] CLI commands are pipeable
- [ ] JSON output is jq-parseable

**v0.2.0 Task**: Implement MCP tool wrappers (tracked in Issue-02 TODO)

---

## Future Work Backlog (by Version)

### v0.2.0 Roadmap (3-4 months post-v0.1.0)

#### 1. Incremental Index Updates (from PRD-10 §6.1, Issue-02 TODO)

**Problem**: Full reindex on code changes is slow for large repos.
**Solution**: File-level incremental updates.

```rust
// cds_graph/src/incremental.rs
pub fn update_file(graph: &mut CodeGraph, file_path: &Path) -> Result<()> {
    // 1. Remove old entities/edges for this file
    // 2. Reparse file
    // 3. Add new entities/edges
    // 4. Update affected BM25 entries
}
```

**Acceptance**: Update <500ms for single file change.

#### 2. MCP Tool Wrappers (from PRD-04 FR-MCP-1, Issue-02 TODO)

**Problem**: Bash tool overhead (~50ms per call).
**Solution**: Native TypeScript tools calling Rust via RPC.

```typescript
export const cdsSearchTool = tool({
  name: 'cds_search',
  description: 'Search for code entities using keywords',
  parameters: { query: { type: 'string' }, /* ... */ },
  execute: async ({ query, entity_type, limit }) => {
    const result = await rpcClient.call('search_entities', { query, entity_type, limit });
    return result;
  }
});
```

**Acceptance**: Tool call latency <10ms (vs 50ms for bash).

#### 3. Multi-Language Support (from PRD-01 §4.2, Issue-02 TODO)

**Languages**: TypeScript/JavaScript (v0.2.0), Rust (v0.2.0), Go (v0.3.0).
**Implementation**:

- Add tree-sitter grammars
- Implement `LanguageParser` trait per language
- Validate parity with Python baseline (same accuracy on multilang repos)
**Acceptance**: Each language passes SWE-bench Lite equivalent (if available).

#### 4. LanceDB Evaluation (from Issue-02 TODO)

**Problem**: Explore unified storage for graph + vectors.
**Research**:

- Can LanceDB store heterogeneous graphs?
- How to combine BM25 + vector embeddings?
- Performance vs current file-based index?
**Outcome**: Decision doc (adopt, defer, or reject LanceDB).

---

### v0.3.0 Roadmap (6-9 months post-v0.1.0)

#### 1. Semantic/Vector Search (from PRD-10 §5)

**Problem**: BM25 misses semantically similar code without keyword match.
**Solution**: Hybrid retrieval (BM25 + vector embeddings).

```shell
# New CLI command
cds semantic-search "find code that handles authentication" --k 10
```

**Implementation**:

- Embed code using CodeBERT or similar
- Store embeddings in vector DB (FAISS, LanceDB, or Qdrant)
- Combine BM25 + semantic scores
**Acceptance**: Improves recall@10 by ≥15% on benchmark queries.

#### 2. Go Language Support (from PRD-01 §4.2)

**Implementation**: Add tree-sitter-go parser, test on Go repos.

#### 3. gRPC Service (from PRD-05 §9)

**Problem**: JSON-RPC may be bottleneck for high-throughput scenarios.
**Solution**: Promote gRPC module from experimental to stable.
**Acceptance**: gRPC service passes all API contract tests, latency <50ms (vs 100ms JSON-RPC).

---

### v1.0+ Roadmap (Future)

#### 1. Fine-Tuning Pipeline (from PRD-10 §6)

**Problem**: Claude/GPT-4 may not optimize for code localization.
**Solution**: Fine-tune on successful agent trajectories (LocAgent §5.2 approach).

```shell
# Collect trajectories
python collect_agent_traces.py --dataset swebench --output traces.jsonl

# Fine-tune model
python finetune.py --model claude-sonnet-4-5 --data traces.jsonl --output cdsagent-tuned
```

**Acceptance**: Fine-tuned model improves Acc@5 by ≥10% over base model.

#### 2. Multi-Repo Federated Search

**Problem**: Search across multiple codebases (e.g., microservices).
**Solution**: Federated query router + index sharding.

#### 3. Web UI for Index Exploration

**Problem**: CLI not accessible to all developers.
**Solution**: Web UI for browsing graph, running searches, visualizing dependencies.

#### 4. Real-Time Code Change Monitoring

**Problem**: Index becomes stale as code changes.
**Solution**: File watcher + auto-indexing daemon.

---

## Research Tracking

### LocAgent Evolution

- [ ] Monitor LocAgent paper updates (new versions on arXiv)
- [ ] Track LocAgent repo changes (new features, benchmarks)
- [ ] Incorporate improvements into CDSAgent roadmap

### LLM Advances

- [ ] Claude 4.5+ capabilities (larger context, better tool use)
- [ ] GPT-5 / Gemini 2.0 agent features
- [ ] Codex successor models (GitHub Copilot Workspace)

### Code Intelligence Research

- [ ] Graph neural networks for code (e.g., GraphCodeBERT)
- [ ] Program synthesis advances
- [ ] Static analysis integration (e.g., CodeQL)

---

## Deliverables

### Week 10 (Phase 4 End)

- [ ] Extensibility mechanisms documented (§1-3)
- [ ] v0.2.0 backlog prioritized (§4)
- [ ] v0.3.0 vision outlined (§5)

### Post-v0.1.0 Release

- [ ] v0.2.0 planning kickoff (with PM, stakeholders)
- [ ] Research roadmap updated quarterly
- [ ] Community feedback incorporated (GitHub issues, Discord)

---

## Acceptance Criteria (from PRD-10 §8)

### Extensibility Validated

- [ ] Multi-SDK interface allows Claude → Codex swap without CLI changes
- [ ] Multi-language registry allows adding languages without refactoring
- [ ] Tool composability demonstrated (cds + rg + ast-grep pipelines work)

### Backlog Maintained

- [ ] v0.2.0 features have clear requirements
- [ ] v0.3.0 vision aligns with LocAgent research trends
- [ ] Technical debt tracked (GitHub issues labeled "tech-debt")

---

## Related Issues

- [Issue-02: Architecture Plan](../02-CDSAgent-Tech-Architecture-Plan.md) - v0.2.0 TODOs
- [01-architecture-and-roadmap.md](01-architecture-and-roadmap.md) - Roadmap timeline
- [09-roadmap.md](09-roadmap.md) - v0.1.0 phases

---

## Ongoing Responsibilities

### Tech Lead

- [ ] Review extensibility PRs to ensure interfaces remain flexible
- [ ] Triage future feature requests (assign to v0.2.0 vs v0.3.0 vs later)
- [ ] Update backlog quarterly based on research/community input

### Product Manager

- [ ] Prioritize v0.2.0 features based on user feedback
- [ ] Coordinate with stakeholders on roadmap
- [ ] Communicate feature timelines

---

**Status Updates**:

- *2025-10-19*: Extensibility patterns established, v0.2.0 backlog seeded with Issue-02 TODOs
