# Tasks: CDS-Index Core - Graph & Sparse Index

**Work Stream**: Issue-02: CDS-Index Core
**Issue Reference**: [../../issues/04-0.1.0-mvp/02-index-core/](../../issues/04-0.1.0-mvp/02-index-core/)
**PRD Reference**: [PRD-02: CDS-Index Core](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-core.md)

## Task Overview

| Task ID | Title | Owner | Status | Week |
|---------|-------|-------|--------|------|
| T-02-01 | Graph Builder - AST Parsing & Construction | Rust Dev 1 | ☐ Not Started | W2 |
| T-02-02 | Sparse Index - Name/ID + BM25 Search | Rust Dev 2 | ☐ Not Started | W2-3 |
| T-02-03 | Service Layer - JSON-RPC Endpoints | Rust Dev 1 | ☐ Not Started | W3-4 |
| T-02-04 | Serialization & Test Fixtures | Rust Dev 2 | ☐ Not Started | W4 |

## Dependencies

- **Prerequisite**: T-06-01 (Parity Methodology) must complete before T-02-01
- **Prerequisite**: T-05-01 (API Contracts) must complete before T-02-03
- **Enables**: T-03-01 (CLI Commands) - requires service layer functional

## Task Details

### T-02-01: Graph Builder

**File**: `T-02-01-graph-builder.md`
**Issue**: [Sub-Issue 02.01](../../issues/04-0.1.0-mvp/02-index-core/01-graph-build.md)
**PRD**: PRD-02 §2.1, §4.1

**Scope**:

- Tree-sitter Python integration
- AST entity extraction (class, function, directory, file)
- Graph construction (4 node types, 4 edge types)
- Parity with LocAgent dependency_graph/build_graph.py

**Deliverables**:

- `cds-index/src/graph/parser.rs`
- `cds-index/src/graph/builder.rs`
- Unit tests with >80% coverage

---

### T-02-02: Sparse Index

**File**: `T-02-02-sparse-index.md`
**Issue**: [Sub-Issue 02.02](../../issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md)
**PRD**: PRD-02 §2.1, §4.2

**Scope**:

- Upper index: Name/ID HashMap with prefix matching
- Lower index: BM25 content search (k1=1.5, b=0.75)
- Parity with LocAgent repo_index/index/

**Deliverables**:

- `cds-index/src/index/name_index.rs`
- `cds-index/src/index/bm25.rs`
- Performance benchmarks (search latency <500ms p95)

---

### T-02-03: Service Layer

**File**: `T-02-03-service-layer.md`
**Issue**: [Sub-Issue 02.03](../../issues/04-0.1.0-mvp/02-index-core/03-service-layer.md)
**PRD**: PRD-02 §2.2, §4.3

**Scope**:

- JSON-RPC service (Axum framework)
- 4 endpoints: search, traverse, retrieve, health
- Error handling and logging

**Deliverables**:

- `cds-index/src/service/handlers.rs`
- `cds-index/src/service/server.rs`
- Integration tests

---

### T-02-04: Serialization & Fixtures

**File**: `T-02-04-serialization.md`
**Issue**: [Sub-Issue 02.04](../../issues/04-0.1.0-mvp/02-index-core/04-serialization-fixtures.md)
**PRD**: PRD-02 §2.3, §4.4

**Scope**:

- Graph serialization (JSON or bincode)
- BM25 index persistence
- Test fixtures for unit/integration tests

**Deliverables**:

- `cds-index/src/persistence/`
- Test fixtures under `tests/fixtures/`

---

## Phase Milestones

### Week 2: Prototype Complete

- [ ] T-02-01: Graph builder parses Python and constructs graph
- [ ] T-02-02: BM25 index performs basic search

**Validation**: Graph structure matches LocAgent on 3 sample repos

### Week 4: Service Functional

- [ ] T-02-03: JSON-RPC service exposes all 4 endpoints
- [ ] T-02-04: Indexes persist and load successfully

**Validation**: CLI can invoke service endpoints via JSON-RPC

---

## Quick Links

- [Issue-02 Overview](../../issues/04-0.1.0-mvp/02-index-core/00-overview.md)
- [PRD-02: CDS-Index Core](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-core.md)
- [Parity Validation](../../issues/04-0.1.0-mvp/06-refactor-parity.md)
