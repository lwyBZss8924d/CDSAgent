# Work Summary - Session 05

**Task**: T-02-02-sparse-index - Sparse Index - Name/ID + BM25 Search
**Session**: 05
**Date**: 2025-11-02 (Day 3)
**Phase**: Phase 3 - BM25 Integration & Parity Validation
**Author**: Claude Code Agent
**Status**: ⏳ INITIALIZED - Ready to begin

---

## Session Objectives

This session completes **Phase 3** of T-02-02-sparse-index: integrating the BM25 lower index with the graph builder and validating search parity against LocAgent baselines.

### Primary Objectives

- [ ] **BM25 Graph Integration**: Implement `BM25Index::from_graph()` builder
- [ ] **Hierarchical Search**: Create unified `SparseIndex` wrapper combining NameIndex + BM25
- [ ] **Parity Validation**: Build test harness for 50 LocAgent search queries
- [ ] **Acceptance Criteria**: Achieve search overlap@10 ≥90% on parity baselines
- [ ] **Performance Validation**: Verify search latency <500ms p95
- [ ] **Documentation**: Update module docs with usage examples

### Success Criteria

| Metric | Target | Current Status |
|--------|--------|----------------|
| Search overlap@10 | ≥90% | ❌ Not tested (Phase 3 goal) |
| Search latency p95 | <500ms | ✅ <1μs (upper index only) |
| Index build time | <5s for 1K | ✅ 2.287 ms (Phase 1) |
| Test coverage | >95% | ✅ 97.20% (Phase 1-2) |
| All tests passing | 100% | ✅ 78/78 (Phase 1-2) |

---

## Current State Analysis

### Completed Work (Sessions 01-04, 8.3h)

**Phase 0** - Planning & Analysis (1.75h):

- ✅ Comprehensive planning and spec alignment
- ✅ Parity baselines review (50 search queries)

**Phase 1** - Upper Index (3.3h):

- ✅ `NameIndex` with exact/prefix matching (68 ns / 699 ns)
- ✅ Coverage: 97.20% lines, 95.35% functions
- ✅ 8 tests passing

**Phase 2** - Custom Tokenizer + BM25 Scaffold (3.2h):

- ✅ `Tokenizer` with offset preservation (387 lines)
- ✅ `Bm25Index` scaffold with Tantivy backend (+442 lines)
- ✅ Stop-word automation via `export_stop_words.py`
- ✅ 78/78 tests passing, ~95% coverage

**Total Progress**: 8.3h / 32h estimated (26%), Phases 0-2 complete

### Implementation Status

#### Index Module (`crates/cds-index/src/index/`)

| Component | Status | API | Tests | Notes |
|-----------|--------|-----|-------|-------|
| `name_index.rs` | ✅ COMPLETE | `exact_match()`, `prefix_match()`, `from_graph()` | 8 passing | Phase 1 delivered |
| `tokenizer.rs` | ✅ COMPLETE | `tokenize()`, `tokenize_with_offsets()` | 7 passing | Phase 2 delivered |
| `bm25.rs` | 🚧 SCAFFOLD | `create_in_dir()`, `open()`, `search()` | 2 passing | **Missing `from_graph()`** |
| `stop_words.rs` | ✅ COMPLETE | `STOP_WORDS` constant | - | Phase 2 delivered |
| `sparse_index.rs` | ❌ NOT STARTED | - | - | **Phase 3 deliverable** |

### Parity Resources Available

**Location**: `tests/fixtures/parity/golden_outputs/`

**Files:**

- ✅ `search_queries.jsonl` - 50 queries with LocAgent top_10 results
- ✅ `graph_locagent.json` - LocAgent graph (658 nodes)
- ✅ `graph_*.json` - 5 SWE-bench repos (658-6,876 nodes each)

**Example Query Format:**

```json
{
  "repo": "LocAgent",
  "query": "graph builder",
  "top_10": [
    {
      "file": "dependency_graph/build_graph.py",
      "name": "",
      "type": "codeblock",
      "score": 2.29,
      "line": 453,
      "text": "def get_inner_nodes(query_node, src_node, graph)..."
    },
    ...
  ],
  "total_results": 10
}
```

---

## Phase 3 Implementation Plan

### Thread 01: Planning & Integration Strategy (30 min)

**Objectives:**

- Review Phase 2 scaffold and identify integration points
- Design `BM25Index::from_graph()` signature and behavior
- Plan hierarchical search architecture (`SparseIndex` wrapper)
- Define parity test structure

**Key Questions:**

1. How should `from_graph()` map `GraphNode` → `Bm25Document`?
   - **Answer**: Extract `display_name`, `file_path`, `kind` from node
   - **Content source**: Synthesize from name + attributes (no source text in graph)

2. Should hierarchical search be in `SparseIndex` or caller code?
   - **Answer**: `SparseIndex` encapsulates strategy, cleaner API

3. What's the parity test harness structure?
   - **Answer**: Load 50 queries, run search, calculate overlap@10, assert ≥0.90

### Thread 02: Implement `BM25Index::from_graph()` (1-1.5h)

**File**: `crates/cds-index/src/index/bm25.rs`

**Signature:**

```rust
impl Bm25Index {
    /// Build a BM25 index from graph entities
    pub fn from_graph(
        graph: &Graph,
        path: impl AsRef<Path>,
        config: AnalyzerConfig,
    ) -> Result<Self> {
        // 1. Create new index in path
        let mut idx = Self::create_in_dir(path, config)?;

        // 2. Extract documents from graph nodes
        let docs: Vec<Bm25Document> = graph.nodes()
            .filter(|n| matches!(n.kind, NodeKind::Class | NodeKind::Function | ...))
            .map(|n| Bm25Document {
                entity_id: &n.id,
                name: Some(&n.display_name),
                path: n.file_path.as_ref().unwrap().to_str().unwrap(),
                kind: n.kind,
                content: &synthesize_content(n),
            })
            .collect();

        // 3. Bulk index
        idx.replace_documents(docs.iter())?;

        Ok(idx)
    }
}

fn synthesize_content(node: &GraphNode) -> String {
    // Combine display_name + attributes for indexing
    format!("{} {}", node.display_name, node.attributes.values().join(" "))
}
```

**Expected Lines**: ~80-120 lines (including tests)

### Thread 03: Create `SparseIndex` Wrapper (1-1.5h)

**File**: `crates/cds-index/src/index/sparse_index.rs` (NEW)

**API Design:**

```rust
pub struct SparseIndex {
    upper: NameIndex,
    lower: Bm25Index,
}

impl SparseIndex {
    /// Build from graph with both upper and lower indices
    pub fn from_graph(
        graph: &Graph,
        path: impl AsRef<Path>,
        config: AnalyzerConfig,
    ) -> Result<Self> {
        let upper = NameIndex::from_graph(graph);
        let lower = Bm25Index::from_graph(graph, path, config)?;
        Ok(Self { upper, lower })
    }

    /// Hierarchical search: exact → prefix → BM25 fallback
    pub fn search(
        &self,
        query: &str,
        limit: usize,
        kind_filter: Option<NodeKind>,
    ) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        // 1. Try exact match
        let exact = self.upper.exact_match(query, limit, kind_filter);
        results.extend(exact.into_iter().map(SearchResult::from));
        if results.len() >= limit {
            return Ok(results.truncate(limit));
        }

        // 2. Try prefix match
        let prefix = self.upper.prefix_match(query, limit - results.len(), kind_filter);
        results.extend(prefix.into_iter().map(SearchResult::from));
        if results.len() >= limit {
            return Ok(results);
        }

        // 3. Fallback to BM25
        let bm25 = self.lower.search(query, limit - results.len(), kind_filter)?;
        results.extend(bm25);

        Ok(results)
    }
}
```

**Expected Lines**: ~150-200 lines (including tests)

### Thread 04: Build Parity Test Harness (1h)

**File**: `crates/cds-index/tests/search_parity_tests.rs` (NEW)

**Structure:**

```rust
use cds_index::index::SparseIndex;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ParityQuery {
    repo: String,
    query: String,
    top_10: Vec<ParityResult>,
}

#[derive(Deserialize)]
struct ParityResult {
    file: String,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    score: f32,
    line: usize,
}

#[test]
fn search_overlap_at_10_locagent_baseline() {
    // Load queries
    let queries = load_queries("tests/fixtures/parity/golden_outputs/search_queries.jsonl");

    // Build index from LocAgent graph
    let graph = load_graph("tests/fixtures/parity/golden_outputs/graph_locagent.json");
    let index = SparseIndex::from_graph(&graph, "/tmp/test_index", Default::default()).unwrap();

    // Calculate overlap@10 for each query
    let mut overlaps = Vec::new();
    for query in &queries {
        let results = index.search(&query.query, 10, None).unwrap();
        let cds_ids: HashSet<_> = results.iter().map(|r| &r.entity_id).collect();
        let loc_ids: HashSet<_> = query.top_10.iter()
            .map(|r| format!("{}:{}", r.file, r.line))
            .collect();

        let overlap = cds_ids.intersection(&loc_ids).count() as f32 / 10.0;
        overlaps.push(overlap);

        println!("Query '{}': overlap@10 = {:.2}", query.query, overlap);
    }

    let mean_overlap = overlaps.iter().sum::<f32>() / overlaps.len() as f32;
    println!("Mean overlap@10: {:.2}", mean_overlap);

    assert!(mean_overlap >= 0.90, "Expected ≥0.90, got {:.2}", mean_overlap);
}
```

**Expected Lines**: ~200-250 lines

### Thread 05: Run Parity Validation & Tuning (1-2h)

**Process:**

1. Run `cargo test search_parity_tests -- --nocapture`
2. Analyze overlap@10 results for each query
3. Identify failure patterns:
   - Tokenization mismatch (CamelCase, stop words)?
   - Entity ID mapping issues?
   - BM25 parameter tuning needed?
4. Iterate:
   - Adjust tokenizer rules if needed
   - Tune BM25 k1 (1.2-2.0), b (0.5-1.0)
   - Re-run until overlap@10 ≥0.90

**Success Criteria:**

- Mean overlap@10 ≥0.90 across 50 queries
- No regressions in upper index performance (still <1μs)
- All 90+ tests passing

### Thread 06: Performance Benchmarking (30 min)

**File**: `crates/cds-index/benches/search_bench.rs`

**Add Benchmarks:**

```rust
#[bench]
fn bench_sparse_search_exact(b: &mut Bencher) {
    // Measure exact match performance
}

#[bench]
fn bench_sparse_search_prefix(b: &mut Bencher) {
    // Measure prefix match performance
}

#[bench]
fn bench_sparse_search_bm25(b: &mut Bencher) {
    // Measure BM25 search performance
}

#[bench]
fn bench_sparse_search_hierarchical(b: &mut Bencher) {
    // Measure full hierarchical search
    // Target: p95 <500ms
}
```

**Expected Results:**

- Exact: <100 ns (already 68 ns)
- Prefix: <1 μs (already 699 ns)
- BM25: <10 ms (Tantivy baseline)
- Hierarchical: <500 ms p95 (acceptance target)

### Thread 07: Documentation & Code Review (30 min)

**Tasks:**

1. Update `crates/cds-index/src/index/mod.rs`:

   ```rust
   //! Index module - Name/ID HashMap + BM25 search
   //!
   //! ## Overview
   //!
   //! This module provides a hierarchical sparse index for code entity search:
   //! - **Upper Index** (`NameIndex`): Fast exact/prefix matching on entity names
   //! - **Lower Index** (`Bm25Index`): Full-text search with BM25 ranking
   //!
   //! ## Usage
   //!
   //! ```rust
   //! use cds_index::index::SparseIndex;
   //!
   //! let graph = Graph::from_repo("path/to/repo")?;
   //! let index = SparseIndex::from_graph(&graph, "/tmp/index", Default::default())?;
   //!
   //! // Search returns hierarchical results: exact → prefix → BM25
   //! let results = index.search("graphbuilder", 10, None)?;
   //! ```
   ```

2. Add rustdoc comments to all public APIs
3. Run `cargo doc --open` and verify
4. Self-review checklist:
   - ✓ All acceptance criteria met?
   - ✓ Code quality (clippy, fmt)?
   - ✓ Test coverage maintained (>95%)?

---

## Parity Validation Strategy

### Overlap@10 Calculation

For each of 50 queries:

```text
CDSAgent_ids = SparseIndex.search(query, limit=10).map(|r| r.entity_id)
LocAgent_ids = baseline.top_10.map(|r| format!("{}:{}", r.file, r.line))

overlap@10 = |CDSAgent_ids ∩ LocAgent_ids| / 10
```

**Target**: Mean overlap@10 ≥0.90 (90% agreement with LocAgent)

### Known Challenges & Mitigations

| Challenge | Impact | Mitigation Strategy |
|-----------|--------|---------------------|
| **Entity ID mismatch** | LocAgent uses `file:line:name`, CDSAgent uses `file:class.method` | Fuzzy matching on file path + name, normalize IDs |
| **Tokenization differences** | LocAgent uses BM25Okapi, CDSAgent uses custom tokenizer | Already aligned stop words, stem rules (Phase 2) |
| **Ranking differences** | BM25 parameter tuning may be needed | Start k1=1.5, b=0.75, tune if overlap <90% |
| **Content synthesis** | Graph nodes lack source text | Synthesize from `display_name` + `attributes` |

---

## Expected Session Outcomes

### Deliverables

1. ✅ `BM25Index::from_graph()` - Graph entity indexing (~100 lines)
2. ✅ `SparseIndex` - Hierarchical search wrapper (~180 lines)
3. ✅ Parity test harness - 50 query validation (~230 lines)
4. ✅ Benchmarks - Search performance measurement (~50 lines)
5. ✅ Documentation - Module docs + examples

**Total Expected Code**: ~560 lines (new) + ~100 lines (tests)

### Acceptance Criteria

| Criterion | Target | Current | Expected |
|-----------|--------|---------|----------|
| Upper index (name/ID) with prefix | ✓ | ✅ COMPLETE | ✅ Maintained |
| Lower index (BM25 k1=1.5, b=0.75) | ✓ | 🚧 Scaffold | ✅ **Phase 3 delivers** |
| Search latency <500ms p95 | ✓ | ✅ <1μs (upper) | ✅ Validated |
| Index build <5s for 1K files | ✓ | ✅ 2.287 ms | ✅ Maintained |
| **Search overlap@10 ≥90%** | ✓ | ❌ Not tested | ✅ **CRITICAL PATH** |
| Unit test coverage >95% | ✓ | ✅ 97.20% | ✅ Maintained |

### Success Metrics

- **Primary**: Mean overlap@10 ≥0.90 across 50 queries
- **Secondary**: All 90+ tests passing
- **Tertiary**: Zero clippy warnings, code formatted

---

## Risks & Contingencies

### High Risk

**Risk**: Overlap@10 <90% due to fundamental architectural mismatch

- **Probability**: Low (tokenizer already aligned with LocAgent)
- **Impact**: High (blocks acceptance criteria)
- **Contingency**:
  1. Analyze failure patterns in detail
  2. Implement fuzzy ID matching layer
  3. Add entity name boosting to BM25
  4. If still <80%, escalate to Phase 4 for advanced ranking

### Medium Risk

**Risk**: Entity ID mapping complexity (file:line vs file:class.method)

- **Probability**: Medium
- **Impact**: Medium (may reduce overlap@10 by 5-10%)
- **Mitigation**:
  - Normalize both formats to common representation
  - Implement fuzzy matching with Levenshtein distance

### Low Risk

**Risk**: BM25 parameter tuning iteration takes >1h

- **Probability**: Low (starting values well-researched)
- **Impact**: Low (delays timeline but not blocking)
- **Mitigation**: Automate parameter sweep with grid search

---

## Next Steps After Session 05

### If Phase 3 Completes Successfully

**Session 06**: Phase 4 - Hierarchical Search Strategy (2-3h)

- Advanced query understanding
- Multi-level search orchestration
- Context-aware ranking
- Boosting strategies

**Session 07**: Phase 5 - Comprehensive Benchmarking (2-3h)

- Full parity validation on 5 SWE-bench repos
- Performance profiling
- Optimization passes

### If Parity Validation Fails

**Session 06**: Phase 3 Continuation - Debug & Tune (2-3h)

- Deep dive into tokenization mismatches
- Parameter tuning automation
- Fuzzy ID matching implementation
- Re-validation on 50 queries

---

## References

### Specifications

- **PRD**: `spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md`
- **Issue**: `spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md`
- **Task**: `spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md`

### Parity Resources

- **Queries**: `tests/fixtures/parity/golden_outputs/search_queries.jsonl` (50 queries)
- **Methodology**: `docs/parity-validation-methodology.md`
- **Graphs**: `tests/fixtures/parity/golden_outputs/graph_*.json` (6 repos)

### Implementation

- **Index Module**: `crates/cds-index/src/index/`
- **Graph API**: `crates/cds-index/src/graph/` (T-02-01 complete)
- **Tests**: `crates/cds-index/tests/`

---

**Time Spent**: 0h (initialized)
**Status**: ⏳ INITIALIZED - Ready to begin Thread 01

**Next Action**: Begin Phase 3 Planning & Integration Strategy
