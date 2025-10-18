# PRD-06: Rust Refactoring Plan - LocAgent Python → CDSAgent Rust

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Document Overview

### 1.1 Purpose

This document provides a detailed refactoring plan to migrate LocAgent's Python codebase (graph construction, indexing, retrieval) to Rust while preserving algorithmic fidelity and improving performance. This is critical for CDSAgent to match LocAgent's published benchmark results.

### 1.2 Goals

- **Preserve Research Fidelity**: Reproduce LocAgent's graph structure, search behavior, and tool outputs exactly
- **Performance**: Achieve 2-5x speedup over Python baseline through Rust optimization
- **Maintainability**: Clear module structure, idiomatic Rust code
- **Extensibility**: Support for multiple languages (Python in v0.1.0; add TypeScript/JavaScript + Rust in v0.2.0, Go/Java in v0.3.0)

### 1.3 LocAgent References

- **Repository**: `tmp/LocAgent/` (reference implementation)
- **Key Modules**: `dependency_graph/`, `repo_index/`, `plugins/location_tools/`
- **Paper**: <https://arxiv.org/html/2503.09089v2> (algorithms and benchmarks)

---

## 2. Module-by-Module Refactoring Map

### 2.1 dependency_graph/ → cds_graph

#### LocAgent Module: `dependency_graph/build_graph.py`

**Functionality**:

- Walk repository directory tree
- Parse Python files with tree-sitter
- Extract entities (directories, files, classes, functions)
- Build graph edges (contain, import, invoke, inherit)

**Rust Equivalent**: `cds_graph/src/builder.rs`

**Key Algorithms to Preserve**:

(1) **Directory Traversal** (LocAgent `build_graph.py::build_repo_graph()`):

```python
# LocAgent (simplified)
def build_repo_graph(repo_path):
    graph = Graph()
    for root, dirs, files in os.walk(repo_path):
        for file in files:
            if file.endswith('.py'):
                parse_file(file, graph)
    return graph
```

**Rust Refactoring**:

```rust
// cds_graph/src/builder.rs
use walkdir::WalkDir;

pub fn build_repo_graph(repo_path: &Path) -> Result<CodeGraph> {
    let mut graph = CodeGraph::new();

    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("py"))
    {
        parse_file(entry.path(), &mut graph)?;
    }

    Ok(graph)
}
```

(2) **AST Parsing** (LocAgent `repo_index/codeblocks/parser/python.py`):

```python
# LocAgent uses tree-sitter queries (.scm files)
import tree_sitter
parser = tree_sitter.Parser()
parser.set_language(Language('build/python.so', 'python'))
tree = parser.parse(source_code.encode())
```

**Rust Refactoring**:

```rust
// cds_graph/src/parsers/python.rs
use tree_sitter::{Parser, Language};

extern "C" {
    fn tree_sitter_python() -> Language;
}

pub fn parse_python_file(source: &str) -> Result<Vec<Entity>> {
    let mut parser = Parser::new();
    parser.set_language(unsafe { tree_sitter_python() })?;

    let tree = parser.parse(source, None).ok_or(ParseError)?;
    extract_entities(&tree, source)
}
```

(3) **Entity Extraction** (LocAgent query in `repo_index/codeblocks/parser/queries/python.scm`):

```scheme
; LocAgent tree-sitter query for Python
(class_definition
  name: (identifier) @class.name
  body: (block) @class.body) @class.def

(function_definition
  name: (identifier) @function.name
  parameters: (parameters) @function.params
  body: (block) @function.body) @function.def
```

**Rust Implementation**:

```rust
// Reuse LocAgent .scm files directly
const PYTHON_QUERY: &str = include_str!("queries/python.scm");

use tree_sitter::Query;

pub fn extract_entities(tree: &Tree, source: &str) -> Vec<Entity> {
    let query = Query::new(tree.language(), PYTHON_QUERY).unwrap();
    let mut cursor = QueryCursor::new();

    let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    // Extract entities from matches...
}
```

**Validation**:

- Parse LocAgent's own codebase, compare entity counts
- Verify nested class/function extraction matches LocAgent

#### LocAgent Module: `dependency_graph/traverse_graph.py`

**Functionality**:

- BFS traversal with filters (relation type, entity type, depth)
- Tree-formatted output

**Rust Equivalent**: `cds_traversal/src/bfs.rs`

**Key Algorithm**:

```python
# LocAgent BFS (simplified)
def traverse_graph(graph, start, depth, relations, entity_types):
    queue = [(start, 0)]
    visited = set()
    result = []

    while queue:
        node, d = queue.pop(0)
        if d > depth or node in visited:
            continue

        visited.add(node)
        result.append(node)

        for edge in graph.edges_from(node):
            if edge.type in relations:
                queue.append((edge.target, d + 1))

    return result
```

**Rust Refactoring**:

```rust
// cds_traversal/src/bfs.rs
use std::collections::{VecDeque, HashSet};

pub fn traverse_graph(
    graph: &CodeGraph,
    start: &NodeID,
    depth: usize,
    relations: &[RelationType],
    entity_types: &[EntityType],
) -> Vec<NodeID> {
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), 0));

    let mut visited = HashSet::new();
    let mut result = Vec::new();

    while let Some((node, d)) = queue.pop_front() {
        if d > depth || visited.contains(&node) {
            continue;
        }

        visited.insert(node.clone());
        result.push(node.clone());

        for edge in graph.edges_from(&node) {
            if relations.contains(&edge.relation) {
                queue.push_back((edge.target.clone(), d + 1));
            }
        }
    }

    result
}
```

**Validation**:

- Run on sample graphs, verify traversal order matches LocAgent
- Test filters (relation, entity type, depth)

### 2.2 repo_index/ → cds_sparse_index

#### LocAgent Module: `build_bm25_index.py`

**Functionality**:

- Tokenize code (camelCase/snake_case splitting)
- Build BM25 inverted index
- Store index to disk

**Rust Equivalent**: `cds_sparse_index/src/bm25.rs`

**Key Algorithms**:

(1) **Tokenization** (LocAgent custom tokenizer):

```python
# LocAgent tokenizer
def tokenize_code(code):
    # Split camelCase: "getUserName" → ["get", "User", "Name"]
    tokens = re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?=[A-Z]|$)|\d+', code)

    # Split snake_case: "get_user_name" → ["get", "user", "name"]
    tokens = [t.split('_') for t in tokens]

    # Remove stop words
    tokens = [t for t in tokens if t not in STOP_WORDS]

    return tokens
```

**Rust Refactoring**:

```rust
// cds_sparse_index/src/tokenizer.rs
use regex::Regex;

lazy_static! {
    static ref CAMEL_CASE_RE: Regex = Regex::new(r"[A-Z]?[a-z]+|[A-Z]+(?=[A-Z]|$)|\d+").unwrap();
    static ref STOP_WORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("the"); s.insert("a"); s.insert("is"); // ...
        s
    };
}

pub fn tokenize_code(code: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for token in CAMEL_CASE_RE.find_iter(code) {
        for sub in token.as_str().split('_') {
            if !STOP_WORDS.contains(sub.to_lowercase().as_str()) {
                tokens.push(sub.to_lowercase());
            }
        }
    }

    tokens
}
```

(2) **BM25 Index Building**:

**Option A**: Use `tantivy` crate (BM25 built-in):

```rust
// cds_sparse_index/src/bm25_tantivy.rs
use tantivy::*;

pub fn build_index(entities: &[Entity]) -> Result<Index> {
    let schema = build_schema();
    let index = Index::create_in_dir("index/", schema)?;
    let mut writer = index.writer(50_000_000)?;

    for entity in entities {
        let mut doc = Document::new();
        doc.add_text(field_id, &entity.id);
        doc.add_text(field_code, &entity.code);
        writer.add_document(doc)?;
    }

    writer.commit()?;
    Ok(index)
}
```

**Option B**: Custom BM25 (closer to LocAgent):

```rust
// Replicate LocAgent's BM25 exactly
pub struct BM25Index {
    inverted_index: HashMap<String, Vec<(String, f32)>>,  // term → [(doc_id, tf), ...]
    doc_lengths: HashMap<String, usize>,
    avg_doc_len: f32,
    k1: f32,
    b: f32,
}

impl BM25Index {
    pub fn search(&self, query: &str) -> Vec<(String, f32)> {
        // BM25 scoring as in LocAgent
    }
}
```

**Decision**: Start with `tantivy` (faster), validate against LocAgent. If results differ, implement custom BM25.

**Validation**:

- Index LocAgent's test repos
- Compare top-10 search results for 50 sample queries
- Verify BM25 scores within 5% of LocAgent's

#### LocAgent Module: `repo_index/index/code_index.py`

**Functionality**:

- Load/save index to disk
- Combine name/ID index with BM25

**Rust Equivalent**: `cds_sparse_index/src/search.rs`

**Hierarchical Search Logic**:

```python
# LocAgent SearchEntity (simplified)
def search_entity(query, limit):
    # Step 1: Upper index (name/ID)
    results = name_index.search(query)

    # Step 2: If insufficient, use BM25
    if len(results) < 5:
        bm25_results = bm25_index.search(query)
        results.extend(bm25_results)

    # Deduplicate and rank
    results = deduplicate(results)
    results.sort(key=lambda r: r.score, reverse=True)

    return results[:limit]
```

**Rust Refactoring**:

```rust
// cds_sparse_index/src/search.rs
pub fn search_entity(query: &str, limit: usize) -> Vec<SearchResult> {
    let mut results = name_index.search(query);

    if results.len() < 5 {
        let bm25_results = bm25_index.search(query);
        results.extend(bm25_results);
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.dedup_by_key(|r| r.entity_id.clone());
    results.truncate(limit);

    results
}
```

### 2.3 plugins/location_tools/ → cds CLI integration

#### LocAgent Module: `plugins/location_tools/locationtools.py`

**Functionality**:

- Expose SearchEntity, TraverseGraph, RetrieveEntity as Python functions
- Format outputs (fold/preview/full)

**Rust Equivalent**: `cds_cli/src/commands/` (PRD-03)

**Output Formatting** (LocAgent):

```python
# LocAgent fold/preview/full
def format_snippet(entity):
    fold = f"{entity.type} {entity.name} - {entity.file}:{entity.line_start}"
    preview = entity.code.split('\n')[:5]
    full = entity.code
    return {"fold": fold, "preview": "\n".join(preview), "full": full}
```

**Rust Refactoring**:

```rust
// cds_cli/src/output/formatter.rs
pub fn format_snippet(entity: &Entity) -> CodeSnippet {
    let fold = format!("{} {} - {}:{}",
        entity.entity_type, entity.name, entity.file_path, entity.line_range.0);

    let lines: Vec<&str> = entity.code.lines().collect();
    let preview = lines.iter().take(5).cloned().collect::<Vec<_>>().join("\n");

    CodeSnippet {
        fold,
        preview,
        full: entity.code.clone(),
    }
}
```

---

## 3. Refactoring Principles

### 3.1 Preserve Algorithms, Optimize Data Structures

| LocAgent (Python) | CDSAgent (Rust) | Benefit |
|------------------|-----------------|---------|
| `dict` for graphs | `HashMap` with `ahash` | Faster hashing |
| `list` for BFS queue | `VecDeque` | O(1) pop_front |
| `set` for visited | `HashSet` | Same complexity, faster |
| `json.dumps()` | `serde_json` | Faster serialization |

### 3.2 Idiomatic Rust

- Use `Result<T, E>` for error handling (not panics)
- Use `Option<T>` for nullable values
- Borrow when possible (`&str`, `&[T]`)
- Parallelize where safe (e.g., batch file parsing with `rayon`)

### 3.3 Tree-sitter Integration

**Reuse LocAgent's `.scm` query files**:

- Copy `tmp/LocAgent/repo_index/codeblocks/parser/queries/` to Rust project
- Embed via `include_str!()` macro

**Example**:

```rust
// cds_graph/src/parsers/queries/python.scm (copied from LocAgent)
const PYTHON_CLASS_QUERY: &str = include_str!("queries/python.scm");
```

---

## 4. Performance Optimization Strategies

### 4.1 Parallelization

| Task | Strategy | Speedup |
|------|----------|---------|
| File parsing | `rayon::par_iter()` over files | 3-5x (multi-core) |
| BM25 indexing | Parallel document processing | 2-3x |
| Graph traversal | Not parallelizable (sequential BFS) | N/A |

**Example**:

```rust
use rayon::prelude::*;

let entities: Vec<Entity> = files
    .par_iter()
    .filter_map(|file| parse_file(file).ok())
    .flatten()
    .collect();
```

### 4.2 Memory Efficiency

- **mmap for large indices**: Use `memmap2` crate to memory-map BM25 index
- **String interning**: Deduplicate file paths, entity names with `string-interner`
- **Compact graph representation**: Store edges as `Vec<(u32, u32, u8)>` (node IDs as u32, relation as u8)

### 4.3 Benchmarking

Use `criterion` crate for micro-benchmarks:

```rust
#[bench]
fn bench_parse_python_file(b: &mut Bencher) {
    let source = include_str!("test_data/large_file.py");
    b.iter(|| parse_python_file(source));
}
```

---

## 5. Validation Plan

### 5.1 Unit Test Coverage

| Module | Test | Validation |
|--------|------|------------|
| `cds_graph::builder` | Parse LocAgent repo | Match entity/edge counts |
| `cds_sparse_index::bm25` | Search 50 queries | Top-10 results match LocAgent |
| `cds_traversal::bfs` | Traverse sample graph | Same nodes, same order |
| `cds_cli::formatter` | Format snippet | Fold/preview/full identical |

### 5.2 Integration Tests

- **End-to-end**: Index repo → search → traverse → retrieve
- **Benchmark repos**: Run on LocAgent's SWE-bench Lite repos
- **Output comparison**: Use `diff` to compare JSON outputs with LocAgent

### 5.3 Performance Benchmarks

| Metric | LocAgent (Python) | CDSAgent (Rust) Target |
|--------|------------------|----------------------|
| Index 1K files | ~5s | <3s (1.6x faster) |
| Search query | ~200ms | <100ms (2x faster) |
| Traverse 2-hop | ~500ms | <200ms (2.5x faster) |

---

## 6. Rust Crate Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tree-sitter` | 0.20+ | AST parsing |
| `tree-sitter-python` | Latest | Python grammar |
| `serde` | 1.0+ | Serialization |
| `serde_json` | 1.0+ | JSON I/O |
| `tantivy` | 0.21+ | BM25 (Option A) |
| `walkdir` | 2.0+ | Directory traversal |
| `rayon` | 1.7+ | Parallelization |
| `regex` | 1.9+ | Tokenization |
| `ahash` | 0.8+ | Fast hashing |
| `memmap2` | 0.7+ | Memory mapping |
| `criterion` | 0.5+ | Benchmarking |
| `clap` | 4.0+ | CLI (in cds_cli) |

---

## 7. Migration Phases

### Phase 1: Core Graph (Week 1-2)

- [ ] Implement `cds_graph` crate
- [ ] Parse Python files, extract entities
- [ ] Build graph (nodes + edges)
- [ ] Unit tests: validate against LocAgent

### Phase 2: Sparse Indexing (Week 3-4)

- [ ] Implement tokenizer
- [ ] Build BM25 index (tantivy or custom)
- [ ] Implement hierarchical search
- [ ] Integration test: search accuracy

### Phase 3: CLI & Formatting (Week 5)

- [ ] Implement `cds` CLI commands
- [ ] Format outputs (fold/preview/full, tree)
- [ ] End-to-end test: full workflow

### Phase 4: Optimization (Week 6)

- [ ] Parallelize file parsing
- [ ] Memory-map large indices
- [ ] Benchmark against Python baseline

---

## 8. Open Questions

1. **BM25 Implementation**: Use tantivy or custom? (Prototype both, compare accuracy)
2. **Parallelism Safety**: Which operations can be safely parallelized? (File parsing yes, graph building needs synchronization)
3. **Tree-sitter Grammars**: Bundle .so files or compile at build time? (Compile at build for portability)

---

## 9. Acceptance Criteria

- [ ] All LocAgent Python modules have Rust equivalents
- [ ] Unit tests pass with >95% coverage
- [ ] Search results match LocAgent's top-10 on 50 sample queries
- [ ] Performance meets or exceeds targets (§5.3)
- [ ] Code is idiomatic Rust (passes `clippy`)

---

## 10. Next Steps

1. Set up Rust workspace with crate structure
2. Copy LocAgent's tree-sitter queries to Rust project
3. Implement Phase 1 (graph building)
4. Continuous comparison with LocAgent outputs

---

**Status**: Ready for implementation. Critical for CDSAgent's success and benchmark validation.
