# Issue-02: CDS-Index Core - Graph Indexer & Sparse Search

**Priority**: P0 (Critical Path - Foundation)
**Status**: ☐ Not Started
**Owner**: Rust Dev 1 + Rust Dev 2 + Rust Dev 3
**PRD Reference**: [PRD-02: CDS-Index Service](../../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md)

---

## Overview

CDS-Index Core is the foundational data layer that parses codebases into directed heterogeneous graphs and builds hierarchical sparse indices for efficient code retrieval. This is the most critical component of CDSAgent, as all other layers depend on it.

## Objective

Deliver a high-performance Rust-based indexing system that:

- Parses Python codebases into graph structures (nodes + edges)
- Builds two-tier hierarchical indices (name/ID + BM25 content)
- Provides JSON-RPC service interface for CLI and Agent layers
- Achieves 2-5x speedup over LocAgent's Python implementation
- Maintains exact algorithmic fidelity with LocAgent

## Dependencies

- **Requires**: Tree-sitter bindings, LocAgent reference implementation
- **Blocks**: 03-cli-tools (CLI depends on index), 04-agent-integration (agent queries index)
- **Timing**: Phase 1-2 (Weeks 1-5)

---

## Sub-Issues Breakdown

### 1. [Graph Build](01-graph-build.md) - **P0, Week 1-2**

**Owner**: Rust Dev 1
**Scope**: AST parsing, entity extraction, graph construction

- Tree-sitter Python parser integration
- Extract 4 node types (directory, file, class, function)
- Create 4 edge types (contain, import, invoke, inherit)
- Validate node/edge counts vs LocAgent

**Acceptance**:

- [ ] Can index LocAgent repo (~150 files)
- [ ] Node/edge counts match LocAgent exactly
- [ ] Unit tests >95% coverage

---

### 2. [Sparse Index](02-sparse-index.md) - **P0, Week 3-4**

**Owner**: Rust Dev 2
**Scope**: Name/ID upper index + BM25 lower index

- Implement name/ID HashMap index
- Integrate tantivy for BM25 or build custom
- Hierarchical search logic (upper → BM25 fallback)
- Validate search results vs LocAgent (50 queries)

**Acceptance**:

- [ ] Hierarchical search matches LocAgent top-10 (≥90% overlap)
- [ ] Search latency <500ms p95
- [ ] Unit tests >95% coverage

---

### 3. [Service Layer](03-service-layer.md) - **P1, Week 4-5**

**Owner**: Rust Dev 3
**Scope**: JSON-RPC HTTP server exposing index APIs

- Implement JSON-RPC server (axum or jsonrpc crate)
- Expose `search_entities`, `traverse_graph`, `retrieve_entity` methods
- Health and metrics endpoints
- Coordinate with [05-api-contracts.md](../05-api-contracts.md) for schemas

**Acceptance**:

- [ ] JSON-RPC service passes all contract tests
- [ ] CLI can call service via HTTP
- [ ] TypeScript agent can call service
- [ ] Service startup time <2s

---

### 4. [Serialization & Fixtures](04-serialization-fixtures.md) - **P1, Week 2-4**

**Owner**: Rust Dev 2 (parallel with sparse index work)
**Scope**: Persistent storage, test fixtures, validation

- Define index file format (JSON + binary)
- Implement serialization/deserialization
- Create test fixtures (sample repos, golden outputs)
- Setup parity tests with LocAgent

**Acceptance**:

- [ ] Can serialize and deserialize graph without data loss
- [ ] Index loads in <2s for 10K file repository
- [ ] Parity tests pass (see [06-refactor-parity.md](../06-refactor-parity.md))

---

## Rust Crate Structure

```tree
cds-index/
├── cds_graph/           # Graph construction (Sub-issue 01)
│   ├── src/
│   │   ├── ast_parser.rs    # tree-sitter integration
│   │   ├── python.rs        # Python-specific parser
│   │   ├── graph.rs         # Graph data structures
│   │   └── builder.rs       # Graph builder logic
│   └── Cargo.toml
├── cds_sparse_index/    # Hierarchical indexing (Sub-issue 02)
│   ├── src/
│   │   ├── name_index.rs    # Upper index
│   │   ├── bm25.rs          # Lower index (tantivy wrapper)
│   │   └── search.rs        # Hierarchical search
│   └── Cargo.toml
├── cds_traversal/       # Graph traversal (used by Sub-issue 02)
│   ├── src/
│   │   ├── bfs.rs           # BFS algorithms
│   │   ├── filters.rs       # Type/relation filters
│   │   └── formatter.rs     # Tree output formatting
│   └── Cargo.toml
├── cds_storage/         # Persistence layer (Sub-issue 04)
│   ├── src/
│   │   ├── serializer.rs    # Graph serialization
│   │   └── loader.rs        # Index loading
│   └── Cargo.toml
├── cds_service/         # Service interface (Sub-issue 03)
│   ├── src/
│   │   ├── jsonrpc.rs       # JSON-RPC HTTP server (v0.1.0)
│   │   └── grpc_server.rs   # gRPC prototype (v0.2.0+, experimental)
│   └── Cargo.toml
└── Cargo.toml           # Workspace config
```

---

## Acceptance Criteria Summary (from PRD-02 §8)

### Must-Have (v0.1.0 MVP)

- [ ] Index Python repositories with full graph + BM25
- [ ] Hierarchical search matches LocAgent behavior
- [ ] Meets performance targets (<5s/1K files indexing, <500ms search)
- [ ] Passes benchmark validation (node/edge counts, search overlap)
- [ ] Expose JSON-RPC service (`cds-indexd`) reachable from CLI and TypeScript agent

### Should-Have (v0.2.0)

- [ ] Support TypeScript/JavaScript and Rust parsing (deferred)
- [ ] Incremental index updates (Issue-02 TODO in main issue tracker)
- [ ] gRPC service interface for remote deployments (PRD-02 §4.1 notes v0.2.0)

---

## Performance Targets (from PRD-02 §3.1)

| Metric | Target | Validation Method |
|--------|--------|-------------------|
| Index build (1K files) | <5s | `criterion` benchmark |
| Search latency (BM25) | <500ms p95 | `hyperfine` CLI timing |
| Traverse latency (2-hop BFS) | <1s p95 | Integration test |
| Incremental update | <500ms | Unit test (deferred to v0.2.0) |
| Memory usage | <2GB (10K files) | `valgrind --tool=massif` |

---

## Dependencies & Coordination

### Internal Dependencies

- Graph Build (01) must complete before Sparse Index (02) can test
- Service Layer (03) requires both Graph (01) and Sparse Index (02)
- Serialization (04) runs in parallel, validates all modules

### External Coordination

- **PRD-05 (API Contracts)**: Service Layer must implement agreed schemas
- **PRD-06 (Refactor Parity)**: Continuous validation against LocAgent
- **08-testing/01-unit.md**: Unit test harness for all crates
- **08-testing/04-benchmark.md**: SWE-bench Lite validation

---

## Implementation Phases

### Phase 1 (Week 1-2): Graph Foundation

- [ ] Sub-issue 01: Graph Build completed
- [ ] Sub-issue 04: Serialization scaffolded
- [ ] Milestone: Can index LocAgent repo

### Phase 2 (Week 3-5): Search & Service

- [ ] Sub-issue 02: Sparse Index completed
- [ ] Sub-issue 03: Service Layer completed
- [ ] Sub-issue 04: Serialization finalized
- [ ] Milestone: CLI can query index via service

---

## Testing Strategy

### Unit Tests (>95% coverage per crate)

- See [08-testing/01-unit.md](../08-testing/01-unit.md)
- Each crate has comprehensive unit tests
- Run on every PR via CI

### Integration Tests

- See [08-testing/02-integration.md](../08-testing/02-integration.md)
- End-to-end: Index repo → search → traverse → retrieve
- Validate on LocAgent benchmarks

### Parity Tests

- See [06-refactor-parity.md](../06-refactor-parity.md)
- Compare graph structure with LocAgent
- Validate search results (50 queries, ≥90% overlap)

---

## Open Questions & Risks

### 1. BM25 Implementation Choice

**Question**: Use tantivy or custom BM25?
**Status**: Prototype both in Week 3, decide based on accuracy
**Owner**: Rust Dev 2
**Resolution Criteria**: Choose tantivy if accuracy within 5% of LocAgent; otherwise custom

### 2. Service Interface (JSON-RPC vs gRPC)

**Decision**: JSON-RPC for v0.1.0 (per PRD-02 §4.1 "Service Interface Strategy")
**Rationale**: LocAgent ran tools in-process; JSON-RPC sufficient for parity
**gRPC**: Scaffolded but marked experimental (v0.2.0+)

### 3. Tree-sitter Compatibility

**Risk**: Python parser may not extract all entities correctly
**Mitigation**: Test early (Week 1) on LocAgent repo, validate counts
**Escalation**: If mismatch >5%, investigate LocAgent's queries and replicate exactly

---

## Related Issues

- **Sub-Issues**: [01-graph-build.md](01-graph-build.md), [02-sparse-index.md](02-sparse-index.md), [03-service-layer.md](03-service-layer.md), [04-serialization-fixtures.md](04-serialization-fixtures.md)
- **Depends On**: [05-api-contracts.md](../05-api-contracts.md) - API schemas
- **Blocks**: [03-cli-tools/](../03-cli-tools/) - CLI implementation
- **Blocks**: [04-agent-integration/](../04-agent-integration/) - Agent queries
- **Validates**: [06-refactor-parity.md](../06-refactor-parity.md) - LocAgent parity
- **Tests**: [08-testing/](../08-testing/) - All test suites

---

## Next Steps

1. [ ] Assign owners to sub-issues (Rust Dev 1, 2, 3)
2. [ ] Set up Rust workspace with crate structure (Week 1, Day 1-2)
3. [ ] Copy LocAgent tree-sitter queries to project (Week 1, Day 1)
4. [ ] Begin Sub-issue 01: Graph Build (Week 1, Day 3)
5. [ ] Establish weekly sync for progress tracking

---

**Status Updates**:

- *2025-10-19*: Issue created, sub-issues defined, awaiting team assignment
