# PRD-02: CDS-Index Service - Graph Indexer & Sparse Search

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Component Overview

### 1.1 Purpose

CDS-Index Service is the foundational data layer that parses codebases into directed heterogeneous graphs and builds hierarchical sparse indices for efficient code retrieval. This component refactors LocAgent's Python-based indexing into high-performance Rust crates.

### 1.2 Scope

- AST-based code parsing (Python, TypeScript, Java)
- Heterogeneous graph construction (nodes: directory/file/class/function; edges: contain/import/invoke/inherit)
- Two-tier hierarchical indexing (name/ID upper index + BM25 content lower index)
- Index persistence and incremental updates
- Service interface for CDS-Tools CLI and TypeScript agent layer

### 1.3 LocAgent References

- **Paper §3.1**: "Sparse Hierarchical Entity Indexing" - <https://arxiv.org/html/2503.09089v2#S3.SS1>
- **Code**: `tmp/LocAgent/dependency_graph/`, `tmp/LocAgent/repo_index/`
- **Figures**: LocAgent Figure 2 (Overview), Figure 6 (Entity Index Structure)

---

## 2. Functional Requirements

### 2.1 Code Graph Construction

#### FR-CG-1: Multi-Language AST Parsing

**Requirement**: Parse source code into Abstract Syntax Trees (AST) to extract entities and relationships.

**Supported Languages (Priority Order)**:

1. Python (v1.0 - LocAgent baseline)
2. TypeScript/JavaScript (v1.1)
3. Java (v1.2)

**Implementation**:

- Use `tree-sitter` Rust bindings for parsing
- Create language-specific extractors implementing `ASTParser` trait
- Extract entities: directories, files, classes, functions (including nested)

**LocAgent Mapping**:

- Refactor: `repo_index/codeblocks/parser/python.py`, `java.py`
- Preserve: Entity extraction logic, tree-sitter queries (`.scm` files)

**Acceptance Criteria**:

- [ ] Can parse Python files and extract all classes/functions
- [ ] Identifies nested functions/classes (multiple levels)
- [ ] Captures entity metadata: name, file path, line range, type

#### FR-CG-2: Heterogeneous Graph Building

**Requirement**: Construct directed graph with 4 node types and 4 edge types per LocAgent specification.

**Node Types**:

- `Directory`: Folder in repository
- `File`: Source code file
- `Class`: Class/interface/struct definition
- `Function`: Function/method definition

**Edge Types** (Directed):

- `Contain`: Directory→File, File→Class, File→Function, Class→Function
- `Import`: File→File (import statements), File→Module
- `Invoke`: Function→Function (function calls)
- `Inherit`: Class→Class (inheritance, interface implementation)

**Graph Properties**:

- Nodes: Unique ID (hash or qualified name), attributes (name, type, path, line range)
- Edges: Source ID, target ID, relation type, metadata (e.g., import alias)

**LocAgent Mapping**:

- Refactor: `dependency_graph/build_graph.py` (graph construction logic)
- Preserve: Graph structure from LocAgent §3.1, tested on SWE-bench

**Acceptance Criteria**:

- [ ] Graph includes all 4 node types and 4 edge types
- [ ] Contain edges form valid tree (directory→file→class/func hierarchy)
- [ ] Import edges capture inter-file dependencies
- [ ] Invoke edges extracted via AST call expression analysis
- [ ] Inherit edges capture class inheritance chains

#### FR-CG-3: Incremental Graph Updates

**Requirement**: Update graph when code changes without full rebuild.

**Scenarios**:

- File added: Insert new File node + entities + edges
- File modified: Reparse file, update/delete affected nodes/edges
- File deleted: Remove File node + contained entities + connected edges

**Constraints**:

- Must preserve referential integrity (no dangling edges)
- Invalidate affected BM25 index entries

**Acceptance Criteria**:

- [ ] Can add/update/delete files without full reindex
- [ ] Graph remains consistent after incremental updates
- [ ] Update latency <500ms for single file change

### 2.2 Hierarchical Entity Indexing

#### FR-HI-1: Upper Index (Name/ID Lookup)

**Requirement**: Fast exact and prefix matching on entity identifiers.

**Index Structure**:

```rust
// Conceptual schema
type NameIndex = HashMap<String, Vec<EntityID>>;

struct EntityEntry {
    entity_id: EntityID,
    name: String,
    qualified_name: String, // e.g., "module.Class.method"
    entity_type: EntityType, // Directory/File/Class/Function
}
```

**Search Operations**:

- Exact match: `get("AuthService")` → [entity_id_1, entity_id_2, ...]
- Prefix match: `get_prefix("Auth")` → all entities starting with "Auth"
- Type filter: `get("auth", entity_type=Function)` → only functions

**LocAgent Mapping**:

- Refactor: LocAgent's name dictionary in `build_bm25_index.py`
- Preserve: Upper index checked first per LocAgent SearchEntity tool

**Acceptance Criteria**:

- [ ] Exact match O(1) average case
- [ ] Prefix search returns all matches
- [ ] Can filter by entity type

#### FR-HI-2: Lower Index (BM25 Content Search)

**Requirement**: Full-text search on code content using BM25 algorithm (sparse retrieval).

**Index Structure**:

- Inverted index: term → list of (entity_id, tf, positions)
- BM25 parameters: k1=1.5, b=0.75 (typical values)
- Tokenization: camelCase/snake_case splitting, stop words removal

**Indexed Content**:

- Function/method bodies
- Class definitions (excluding nested already indexed separately)
- Docstrings/comments (optional, configurable)

**Search Operations**:

```rust
fn bm25_search(query: &str, limit: usize, filters: Filters) -> Vec<ScoredEntity>;

struct ScoredEntity {
    entity_id: EntityID,
    score: f32,
    matched_terms: Vec<String>,
}
```

**LocAgent Mapping**:

- Refactor: `build_bm25_index.py`, `plugins/location_tools/retriever/bm25_retriever.py`
- Use: `tantivy` Rust crate (BM25 implementation) or custom BM25

**Acceptance Criteria**:

- [ ] Returns entities ranked by BM25 score
- [ ] Query latency <500ms for typical codebases (<10K files)
- [ ] Supports term highlighting (matched terms identified)

#### FR-HI-3: Hierarchical Search Strategy

**Requirement**: Combine upper and lower indices per LocAgent SearchEntity logic.

**Search Flow**:

```text
1. Query arrives: "sanitize user input"
2. Check upper index (name/ID): Exact match "sanitize*"
   → Found: [sanitize_html, sanitize_input]
3. If upper results < threshold (e.g., <5):
   → Fall back to lower index (BM25): "sanitize user input"
   → Found: [validate_user_data, clean_input, ...]
4. Merge and deduplicate results
5. Return top K entities (K=10 default)
```

**LocAgent Mapping**:

- Implements LocAgent SearchEntity tool logic (Figure 6, Appendix A.1.1)
- Upper index tried first (faster), fallback to BM25 (broader)

**Acceptance Criteria**:

- [ ] Upper index tried first for keyword queries
- [ ] BM25 fallback when upper results insufficient
- [ ] Combined results ranked appropriately

### 2.3 Graph Storage and Persistence

#### FR-GS-1: Index File Format

**Requirement**: Persist graph and indices to disk in efficient, versionable format.

**File Structure** (similar to LocAgent `graph_index_v2.3/`):

```tree
<REPO_INDEX_DIR>/
  ├── graph/
  │   ├── nodes.db           # Node metadata (JSON or binary)
  │   ├── edges.db           # Edge list with types
  │   └── metadata.json      # Graph version, stats
  ├── indices/
  │   ├── name_index.json    # Upper index (name/ID map)
  │   ├── bm25/              # BM25 inverted index files (tantivy format)
  │   │   ├── segments/
  │   │   └── meta.json
  │   └── entity_dict.json   # Full entity metadata
  └── config.toml            # Index configuration
```

**Serialization**:

- JSON for human-readable metadata
- Binary (bincode or custom) for large node/edge lists
- tantivy native format for BM25 index

**LocAgent Mapping**:

- Mirrors LocAgent's directory structure for compatibility
- Environment variable: `GRAPH_INDEX_DIR` (same as LocAgent)

**Acceptance Criteria**:

- [ ] Can serialize and deserialize graph without data loss
- [ ] Index loads in <2s for 10K file repository
- [ ] Supports versioning (detect incompatible index formats)

#### FR-GS-2: Concurrent Access

**Requirement**: Support concurrent reads during agent sessions.

**Constraints**:

- Multiple CLI tools may query index simultaneously
- Reads do not block each other
- Writes (incremental updates) lock affected portions

**Implementation**:

- Use Read-Write locks (Rust `RwLock`) or lock-free structures
- Memory-mapped files for read performance

**Acceptance Criteria**:

- [ ] Multiple concurrent `cds search` commands succeed
- [ ] Read latency unaffected by concurrent readers

---

## 3. Non-Functional Requirements

### 3.1 Performance

| Metric | Target | Rationale |
|--------|--------|-----------|
| Index build (1K files) | <5s | LocAgent baseline ~5s (Python), Rust should match/exceed |
| Search latency (BM25) | <500ms p95 | Interactive CLI use |
| Traverse latency (2-hop BFS) | <1s p95 | Agent multi-hop queries |
| Incremental update | <500ms | Real-time code editing |
| Memory usage | <2GB (10K files) | Laptop/desktop deployment |

### 3.2 Scalability

- Support repositories up to 50K files (stretch goal: 100K)
- Index size: <100MB per 1K files (compressed)

### 3.3 Accuracy

- **Critical**: Must reproduce LocAgent's graph structure exactly
- Validation: Compare node/edge counts on LocAgent benchmark repos
- Benchmark: SWE-bench Lite dataset (300 instances)

---

## 4. Architecture

### 4.1 Rust Crate Structure

```tree
cds-index/
  ├── cds_graph/           # Graph construction
  │   ├── ast_parser.rs    # tree-sitter integration
  │   ├── python.rs        # Python-specific parser
  │   ├── typescript.rs    # TypeScript parser
  │   ├── graph.rs         # Graph data structures
  │   └── builder.rs       # Graph builder logic
  ├── cds_sparse_index/    # Hierarchical indexing
  │   ├── name_index.rs    # Upper index
  │   ├── bm25.rs          # Lower index (tantivy wrapper)
  │   └── search.rs        # Hierarchical search
  ├── cds_traversal/       # Graph traversal
  │   ├── bfs.rs           # BFS algorithms
  │   ├── filters.rs       # Type/relation filters
  │   └── formatter.rs     # Tree output formatting
  ├── cds_storage/         # Persistence layer
  │   ├── serializer.rs    # Graph serialization
  │   └── loader.rs        # Index loading
  └── cds_service/         # Service interface
      ├── jsonrpc.rs       # JSON-RPC HTTP server (v0.1.0 deliverable)
      └── grpc_server.rs   # gRPC prototype (v0.2.0+ roadmap)
```

#### Service Interface Strategy

- **v0.1.0 (MVP)**: Ship a lightweight JSON-RPC server (`cds-indexd`) that mirrors LocAgent’s local Python invocation model while allowing CLI and TypeScript clients to share an in-memory index.
- **v0.2.0+**: Promote the gRPC transport for deployment scenarios that need streaming responses or cross-language clients; keep the module skeleton in-place but mark it experimental until validated.
- **Rationale**: LocAgent ran tools in-process with Python, so JSON-RPC is sufficient for parity; gRPC becomes valuable once CDS-Index is hosted remotely or needs bidirectional streaming.

### 4.2 Key Interfaces

#### Graph Data Structures

```rust
// Simplified schema (actual implementation in cds_graph/graph.rs)

#[derive(Serialize, Deserialize)]
pub struct CodeGraph {
    pub nodes: HashMap<NodeID, Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: NodeID,
    pub node_type: NodeType,
    pub name: String,
    pub file_path: Option<PathBuf>,
    pub line_range: Option<(usize, usize)>,
    pub attributes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub enum NodeType {
    Directory,
    File,
    Class,
    Function,
}

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub source: NodeID,
    pub target: NodeID,
    pub edge_type: EdgeType,
}

#[derive(Serialize, Deserialize)]
pub enum EdgeType {
    Contain,
    Import,
    Invoke,
    Inherit,
}
```

#### Search Interface

```rust
// Simplified API (actual implementation in cds_sparse_index/search.rs)

pub trait EntitySearcher {
    fn search(&self, query: &str, options: SearchOptions) -> Result<Vec<EntityMatch>>;
}

pub struct SearchOptions {
    pub limit: usize,
    pub entity_types: Option<Vec<NodeType>>,
    pub use_bm25_fallback: bool,
}

pub struct EntityMatch {
    pub entity_id: NodeID,
    pub name: String,
    pub snippet: CodeSnippet, // fold/preview/full
    pub score: f32,
}

pub struct CodeSnippet {
    pub fold: String,      // One-line signature
    pub preview: String,   // ~5 lines
    pub full: String,      // Complete code
}
```

---

## 5. LocAgent Refactoring Plan

### 5.1 Module Mapping

| LocAgent Module | CDS-Index Crate | Changes |
|----------------|----------------|---------|
| `dependency_graph/build_graph.py` | `cds_graph::builder` | Python → Rust, tree-sitter AST |
| `dependency_graph/traverse_graph.py` | `cds_traversal::bfs` | Python → Rust, preserve BFS logic |
| `repo_index/codeblocks/parser/` | `cds_graph::ast_parser` | Use tree-sitter Rust bindings |
| `build_bm25_index.py` | `cds_sparse_index::bm25` | Use tantivy or custom BM25 |
| `plugins/location_tools/retriever/` | `cds_sparse_index::search` | Hierarchical search logic |

### 5.2 Algorithm Preservation

**Critical Algorithms to Preserve**:

1. **Graph Construction** (LocAgent `build_graph.py`):
   - Walk directory tree in depth-first order
   - Parse each file with tree-sitter
   - Extract classes (with nested classes), functions (with nested functions)
   - Build Contain edges: directory→file, file→class, class→function
   - Extract imports via AST import nodes → Import edges
   - Extract function calls via AST call_expression → Invoke edges
   - Extract inheritance via AST class definition bases → Inherit edges

2. **BM25 Indexing** (LocAgent `build_bm25_index.py`):
   - Tokenize code: split camelCase, snake_case, remove stop words
   - Build inverted index: term → [(entity_id, tf, positions), ...]
   - Store document lengths for BM25 normalization
   - Use k1=1.5, b=0.75 parameters (LocAgent defaults)

3. **Hierarchical Search** (LocAgent SearchEntity tool):
   - Check upper index (name/ID) first with exact/prefix match
   - If results < threshold (5), query BM25 lower index
   - Rank by: upper index (exact match=1.0) > BM25 score
   - Return fold/preview/full for each entity

**Validation**:

- Compare graph node/edge counts on sample repos
- Verify BM25 search returns same top-K entities as LocAgent
- Benchmark traversal results (same subgraphs)

### 5.3 Tree-sitter Integration

**LocAgent Uses**: tree-sitter with Python and Java grammars (queries in `.scm` files)

**CDSAgent Approach**:

```rust
// Use tree-sitter Rust bindings
use tree_sitter::{Parser, Language};

extern "C" {
    fn tree_sitter_python() -> Language;
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_java() -> Language;
}

// Reuse LocAgent .scm query files
const PYTHON_QUERY: &str = include_str!("queries/python.scm");
```

**Query Example** (from LocAgent `python.scm`):

```scheme
(function_definition
  name: (identifier) @function.name
  parameters: (parameters) @function.params
  body: (block) @function.body)

(class_definition
  name: (identifier) @class.name
  body: (block) @class.body)
```

---

## 6. Testing and Validation

### 6.1 Unit Tests

- [ ] AST parser extracts all functions/classes from test files
- [ ] Graph builder creates correct node/edge types
- [ ] Name index returns exact and prefix matches
- [ ] BM25 index ranks entities correctly (relevance)

### 6.2 Integration Tests

- [ ] Index full repository (e.g., LocAgent itself)
- [ ] Verify node/edge counts match expected values
- [ ] Search for known entities returns expected results

### 6.3 Benchmark Validation

- [ ] Run on LocAgent's SWE-bench Lite repos
- [ ] Compare graph structure: node counts, edge type distribution
- [ ] Compare search results: top-10 entities for 50 sample queries

### 6.4 Performance Tests

- [ ] Measure index build time (use `criterion` benchmarks)
- [ ] Measure search latency under load (100 concurrent queries)
- [ ] Profile memory usage (valgrind, heaptrack)

---

## 7. Dependencies

### 7.1 Rust Crates

| Crate | Purpose | Version |
|-------|---------|---------|
| `tree-sitter` | AST parsing | 0.20+ |
| `tree-sitter-python` | Python grammar | Latest |
| `tree-sitter-typescript` | TS/JS grammar | Latest |
| `tantivy` | BM25 full-text search | 0.21+ |
| `serde` | Serialization | 1.0+ |
| `serde_json` | JSON format | 1.0+ |
| `tokio` | Async runtime (for service) | 1.28+ |
| `tonic` | gRPC (if using gRPC) | 0.10+ |

### 7.2 External Tools

- tree-sitter CLI (for grammar testing)
- LocAgent repository (validation reference)

---

## 8. Acceptance Criteria Summary

### Must-Have (v0.1.0 MVP)

- [ ] Index Python repositories with full graph + BM25
- [ ] Hierarchical search matches LocAgent behavior
- [ ] Meets performance targets (§3.1)
- [ ] Passes benchmark validation (§6.3)
- [ ] Expose JSON-RPC service (`cds-indexd`) reachable from CLI and TypeScript agent

### Should-Have (v0.2.0)

- [ ] Support TypeScript/JavaScript and Rust parsing
- [ ] Incremental index updates
- [ ] gRPC service interface (for remote deployments)

### Nice-to-Have (Future)

- [ ] Support Go and Java
- [ ] Parallel index building
- [ ] Distributed index storage

---

## 9. Open Questions

1. **gRPC promotion timing**: JSON-RPC is locked for v0.1.0 (see PRD-05); determine readiness criteria for enabling gRPC in v0.2.0.
2. **BM25 Implementation**: Use tantivy or custom? (Prototype both)
3. **Memory-Mapping**: Use `memmap2` for large index files? (Performance test)
4. **Entity ID Format**: Hash (SHA-256) vs qualified name string? (Evaluate collision risk)

---

## 10. Next Steps

1. Review by Rust team and LocAgent experts
2. Prototype graph builder on small repo (validate tree-sitter)
3. Implement BM25 index (tantivy vs custom decision)
4. Integration with CDS-Tools CLI (PRD-03)

---

**Status**: Ready for technical review and prototyping phase.
