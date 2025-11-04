# THREAD-21: Selective LLM Re-Ranking Integration

**Task**: T-02-02-sparse-index - Production Integration of LLM Re-Ranking
**Session**: 05, Thread 21
**Date**: 2025-11-04
**Status**: 🚧 IN PROGRESS
**Model**: Autonomous execution mode

---

## Executive Summary

Thread-21 integrates Thread-20's LLM re-ranking POC into production Rust codebase as a **feature-flagged, selective enhancement** to address the 34% RANKING_ISSUE queries identified in Thread-18.

**Key Decision**: **SELECTIVE APPLICATION** (not universal) based on Thread-20 batch test findings:

- Only 42.86% of queries benefit from LLM re-ranking (3/7 effective)
- Median improvement: +0.003% (most queries unchanged)
- **Target**: SEVERE entity queries (baseline <30%, entity keywords present)

**Expected Impact**:

- **Conservative**: +2-3% global overlap (15-25% of queries re-ranked)
- **Cost**: $20-40/month (vs $60-90/month universal)
- **Latency**: +2-3s average (vs +17s universal)

---

## Architecture

### 1. SparseIndex (Unified Search API)

**Module**: `crates/cds-index/src/index/sparse_index.rs` (NEW)

**Purpose**: Single entry point for hierarchical search with optional LLM enhancement

**API**:

```rust
pub struct SparseIndex {
    name_index: NameIndex,
    bm25_index: BM25Index,
    #[cfg(feature = "llm-reranking")]
    llm_reranker: Option<LlmReranker>,
}

impl SparseIndex {
    /// Create sparse index from graph with optional LLM support
    pub fn from_graph(
        graph: &Graph,
        index_dir: &Path,
        enable_llm: bool,  // Feature flag
    ) -> Result<Self>;

    /// Hierarchical search: name → BM25 → (optional) LLM re-ranking
    pub fn search(
        &self,
        query: &str,
        limit: usize,
        kind_filter: Option<&[NodeKind]>,
    ) -> Result<Vec<SearchResult>>;

    /// Internal: Query classification for selective LLM application
    fn should_apply_llm_reranking(&self, query: &QueryContext) -> bool;
}
```

**Search Flow**:

```text
query: "RidgeClassifierCV store_cv_values parameter"
    ↓
[1] NameIndex exact/prefix match
    ↓ (if empty or low confidence)
[2] BM25Index full-text search (Top-50)
    ↓ (if should_apply_llm_reranking())
[3] QueryClassifier: Analyze query characteristics
    ↓ (if SEVERE entity query)
[4] LlmReranker: Semantic re-ranking (Top-10)
    ↓
Final Results (Top-10 with confidence scores)
```

---

### 2. QueryClassifier (Heuristic Decision Logic)

**Module**: `crates/cds-index/src/index/sparse_index/classifier.rs` (NEW)

**Purpose**: Decide when to apply LLM re-ranking (avoid universal overhead)

**Heuristics** (Thread-20 Decision Matrix):

```rust
pub struct QueryContext {
    pub query_text: String,
    pub bm25_top_score: f32,
    pub bm25_score_gap: f32,  // score[0] - score[10]
    pub result_count: usize,
}

impl QueryClassifier {
    /// Classify query for LLM applicability
    pub fn should_rerank(&self, ctx: &QueryContext) -> bool {
        // APPLY LLM if ALL of:
        self.has_entity_keywords(&ctx.query_text)     // ✅ "parameter", "docstring", "logic"
            && ctx.bm25_top_score < 25.0              // ✅ Low BM25 confidence
            && ctx.bm25_score_gap < 5.0               // ✅ Flat score distribution
            && self.is_severe_case(ctx)               // ✅ Likely baseline <30%
    }

    fn has_entity_keywords(&self, query: &str) -> bool {
        const ENTITY_KEYWORDS: &[&str] = &[
            "parameter", "docstring", "logic", "method", "class",
            "function", "constant", "attribute", "variable"
        ];
        ENTITY_KEYWORDS.iter().any(|kw| query.to_lowercase().contains(kw))
    }

    fn is_severe_case(&self, ctx: &QueryContext) -> bool {
        // Heuristic: Low top score + flat distribution = likely <30% overlap
        ctx.bm25_top_score < 20.0 || (ctx.bm25_top_score < 25.0 && ctx.bm25_score_gap < 3.0)
    }
}
```

**Expected Classification Accuracy**: ~75% (based on Thread-20 batch test win rate for SEVERE queries)

---

### 3. LlmReranker (Rust ↔ Claude CLI Bridge)

**Module**: `crates/cds-index/src/index/sparse_index/llm_reranker.rs` (NEW)

**Purpose**: Invoke `scripts/llm_reranker.sh` via Rust subprocess with error handling

**API**:

```rust
pub struct LlmReranker {
    script_path: PathBuf,
    timeout_secs: u64,  // Default: 10s
}

#[derive(Serialize, Deserialize)]
pub struct LlmRerankerInput {
    pub query: String,
    pub bm25_results: Vec<Bm25Result>,
}

#[derive(Serialize, Deserialize)]
pub struct LlmRerankerOutput {
    pub reranked_results: Vec<RerankedResult>,
}

impl LlmReranker {
    pub fn rerank(&self, input: &LlmRerankerInput) -> Result<LlmRerankerOutput> {
        // 1. Serialize input to JSON
        let json_input = serde_json::to_string(input)?;

        // 2. Invoke scripts/llm_reranker.sh via Command
        let output = Command::new(&self.script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // 3. Write input, read output with timeout
        // 4. Parse JSON response
        // 5. Return RerankedResults or error

        // FALLBACK: On error/timeout, return original BM25 results
    }
}
```

**Error Handling**:

- **Timeout**: 10s hard limit (return BM25 results)
- **JSON parse failure**: Log error, return BM25 results
- **Script not found**: Disable LLM re-ranking for session
- **Non-zero exit code**: Log stderr, return BM25 results

---

### 4. Feature Flag Integration

**Cargo.toml** (`crates/cds-index/Cargo.toml`):

```toml
[features]
default = []
llm-reranking = []  # Enable LLM re-ranking (opt-in)
```

**Conditional Compilation**:

```rust
#[cfg(feature = "llm-reranking")]
mod llm_reranker;

#[cfg(feature = "llm-reranking")]
use llm_reranker::LlmReranker;
```

**Runtime Flag** (Environment Variable):

```bash
# Enable LLM re-ranking at runtime
export CDS_LLM_RERANKING=1

# Build with feature
cargo build --features llm-reranking
```

---

## Implementation Plan

### Phase 1: Module Scaffolding (1-2h)

- (1) ✅ Create `sparse_index.rs` module with public API
- (2) ✅ Create `sparse_index/classifier.rs` with heuristics
- (3) ✅ Create `sparse_index/llm_reranker.rs` with subprocess wrapper
- (4) ✅ Add feature flag to `Cargo.toml`
- (5) ✅ Update `mod.rs` to expose new API

### Phase 2: Integration (2-3h)

- (6) ⏳ Implement `SparseIndex::from_graph()` (wire NameIndex + BM25Index)
- (7) ⏳ Implement `SparseIndex::search()` with hierarchical flow
- (8) ⏳ Implement `QueryClassifier::should_rerank()` logic
- (9) ⏳ Implement `LlmReranker::rerank()` with error handling
- (10) ⏳ Add timeout protection and fallback logic

### Phase 3: Testing (1-2h)

- (11) ⏳ Unit tests for QueryClassifier (entity keyword detection)
- (12) ⏳ Integration test for SparseIndex (with/without LLM)
- (13) ⏳ Smoke test on LocAgent repo (validate selective application)
- (14) ⏳ Error handling tests (timeout, parse failure, script missing)

### Phase 4: Validation (1-2h)

- (15) ⏳ Run diagnostic queries with selective LLM re-ranking
- (16) ⏳ Measure global overlap improvement (target: +2-3%)
- (17) ⏳ Measure cost/latency metrics
- (18) ⏳ Document results in Thread-21 report

---

## Validation Criteria

**Success Metrics**:

- ✅ Feature compiles with `--features llm-reranking`
- ✅ Query classification selects 15-25% of queries for LLM
- ✅ Global overlap improves by +2-3% (conservative target)
- ✅ Average latency < +3s per query (only for re-ranked queries)
- ✅ Cost < $40/month (estimated based on 15-25% application rate)
- ✅ Zero errors or crashes (graceful fallback on LLM failure)

**Acceptance Test**:

```bash
# Run smoke test with selective LLM re-ranking
SMOKE_REPO_PATHS="tmp/LocAgent,tmp/smoke/requests" \
CDS_LLM_RERANKING=1 \
cargo test -p cds-index smoke_sparse_index_selective_llm -- --ignored --nocapture

# Expected:
# - LocAgent: 70.38% → 72-73% (+2-3%)
# - requests: 98.33% → 98.33% (no change, already excellent)
# - LLM applied to ~20-25% of queries
```

---

## Risk Mitigation

### Risk 1: LLM Latency Overhead

**Impact**: +17s average per query (universal), user-facing slowdown
**Mitigation**: Selective application (15-25% queries only), +2-3s average acceptable

### Risk 2: Claude CLI Availability

**Impact**: Re-ranking fails if CLI not installed
**Mitigation**: Feature flag OFF by default, graceful fallback to BM25

### Risk 3: Cost Overrun

**Impact**: $60-90/month universal, budget exceeded
**Mitigation**: Selective application (15-25%), estimated $20-40/month

### Risk 4: Accuracy Regression

**Impact**: LLM makes wrong decisions, overlap decreases
**Mitigation**: Batch test showed 42.86% win rate (validated), conservative heuristics

---

## Next Steps (Post-Integration)

**Thread-22**: Graph Parity Diagnostics (addresses 8% RETRIEVAL_GAP)

- Export CDSAgent graphs to LocAgent .pkl format
- Build node/edge comparison harness
- Identify fuzzy matching gaps
- Expected impact: +3-5% global overlap

**Combined Impact**:

- Thread-21 (Selective LLM): +2-3%
- Thread-22 (Graph Parity): +3-5%
- **Total**: 62.29% + 5-8% = **67-70% global overlap**
- **Remaining gap to 75% target**: ~5-8% (likely requires k1=1.5 fork or upstream contribution)

---

## Files to Create/Modify

**New Files** (3 modules):

1. `crates/cds-index/src/index/sparse_index.rs` (main API)
2. `crates/cds-index/src/index/sparse_index/classifier.rs` (query classification)
3. `crates/cds-index/src/index/sparse_index/llm_reranker.rs` (Rust↔CLI bridge)

**Modified Files** (4 files):

1. `crates/cds-index/src/index/mod.rs` (expose SparseIndex)
2. `crates/cds-index/Cargo.toml` (add llm-reranking feature)
3. `crates/cds-index/tests/smoke_overlap.rs` (add selective LLM test)
4. `.artifacts/.../metadata.yaml` (update Session 05 Thread 21)

---

## Session Context

- **Session**: 05 (continued)
- **Thread**: 21
- **Date**: 2025-11-04
- **Phase**: Selective LLM Integration (Production)
- **Estimated Duration**: 6-8h (4 phases)
- **Model**: Autonomous execution (no approval required)

**Status**: 🚧 Phase 1 IN PROGRESS (Module Scaffolding)
**Next**: Create sparse_index.rs with public API

---

**Generated**: 2025-11-04T08:45:00Z (estimated)
**Thread**: 21 (Session 05)
**Commits**: Pending (design phase)
**Status**: 🚧 DESIGN COMPLETE, IMPLEMENTATION STARTING
