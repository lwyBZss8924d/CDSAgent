# FEASIBILITY ANALYSIS: LLM-Based Re-Ranking with Claude Code CLI

**Experiment Proposal**: Extend BM25.rs with LLM-based re-ranking using Claude Code CLI headless mode
**Sub-Agent**: `ast-graph-index-ranker` (model: haiku-4-5)
**Date**: 2025-11-04
**Status**: ✅ FEASIBLE with caveats

---

## Executive Summary

**Verdict**: **HIGHLY FEASIBLE** as an experimental optimization layer. This approach combines BM25's speed with LLM's semantic understanding to improve ranking quality without modifying Tantivy's core parameters.

**Quick Assessment**:

- ✅ **Technical Feasibility**: 9/10 (Claude Code CLI headless mode fully supports this)
- ✅ **Implementation Effort**: 6-10 hours (sub-agent + Rust wrapper + testing)
- ⚠️ **Production Readiness**: 6/10 (latency and cost concerns for high-volume)
- ✅ **Experimental Value**: 10/10 (could unlock significant improvements)

**Recommended Use Case**: **Offline index optimization** or **query-time re-ranking for complex queries** (not all queries).

---

## Proposed Architecture

### Two-Stage Ranking Pipeline

```text
┌─────────────────────────────────────────────────────────────┐
│                    STAGE 1: BM25 Retrieval                  │
│  Input: Query ("linear_model ridge.py parameters")          │
│  Process: Tantivy BM25 search (k1=1.2)                      │
│  Output: Top-50 candidates with scores                      │
│  Latency: ~50-100ms                                         │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│              STAGE 2: LLM-Based Re-Ranking                  │
│  Sub-Agent: ast-graph-index-ranker (haiku-4-5)              │
│  Input: Top-50 BM25 results + graph context                 │
│  Process: Semantic relevance scoring                        │
│  Output: Re-ranked top-10 with confidence scores            │
│  Latency: ~500-1500ms (Haiku)                               │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    Final Results (Top-10)                   │
│  Combines: BM25 lexical + LLM semantic signals              │
│  Expected: +5-15% overlap improvement                       │
└─────────────────────────────────────────────────────────────┘
```

---

## Technical Implementation

### 1. Sub-Agent Definition (`.claude/agents/ast-graph-index-ranker.md`)

```markdown
---
name: ast-graph-index-ranker
description: Re-ranks BM25 search results using semantic understanding and graph context. Use for improving code search relevance.
tools: Read, Grep, Glob
model: haiku
---

You are an expert code search ranking specialist. Your task is to re-rank BM25 search results based on semantic relevance to the query.

**Input Format** (JSON via stdin):
{
  "query": "search query text",
  "bm25_results": [
    {"path": "file.py", "score": 15.3, "entity_id": "...", "content": "..."},
    ...
  ],
  "graph_context": {
    "edges": [...],
    "attributes": {...}
  }
}

**Re-Ranking Criteria**:
1. **Semantic relevance**: Does the file/entity directly address the query intent?
2. **Technical accuracy**: Are the matched terms used in the correct context?
3. **Importance signals**: Is this a core implementation vs test/example file?
4. **Graph centrality**: Is this entity highly connected to other relevant entities?

**Output Format** (JSON to stdout):
{
  "reranked_results": [
    {"path": "file.py", "adjusted_score": 18.5, "confidence": 0.92, "reasoning": "..."},
    ...
  ]
}

⚠️ sub-agent MUST disables reasoning!

**Guidelines**:
- Preserve BM25 score as baseline, adjust by ±30% maximum
- Provide confidence scores (0.0-1.0) for each re-ranking decision
- Include brief reasoning for top-5 changes
- Complete re-ranking within 1 second (time budget)
```

### 2. Rust Wrapper Script (`scripts/llm_reranker.sh`)

```bash
#!/bin/bash
# LLM-based re-ranking wrapper using Claude Code CLI headless mode

set -euo pipefail

# Input: JSON with query + BM25 results (via stdin)
# Output: JSON with re-ranked results (to stdout)

QUERY_DATA="$(cat)"

# Construct prompt for sub-agent
PROMPT="Re-rank these BM25 search results based on semantic relevance:

$QUERY_DATA

Provide re-ranked results with adjusted scores and confidence levels."

# Invoke Claude Code CLI in headless mode
claude -p "$PROMPT" \
  --output-format json \
  --model haiku \
  --agents '{
    "ast-graph-index-ranker": {
      "description": "Re-ranks search results using semantic understanding",
      "prompt": "$(cat .claude/agents/ast-graph-index-ranker.md | sed -n '/^---$/,/^---$/!p' | tail -n +2)",
      "tools": ["Read", "Grep", "Glob"],
      "model": "haiku"
    }
  }' \
  --allowedTools "Read,Grep,Glob" \
  --append-system-prompt "You must respond with valid JSON only. No explanations outside the JSON structure." \
  2>/dev/null | jq -r '.result'
```

### 3. Rust Integration (`crates/cds-index/src/index/llm_reranker.rs`)

```rust
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::Write;

#[derive(Serialize)]
struct RerankRequest {
    query: String,
    bm25_results: Vec<SearchResult>,
    graph_context: Option<GraphContext>,
}

#[derive(Deserialize)]
struct RerankResponse {
    reranked_results: Vec<AdjustedResult>,
}

#[derive(Deserialize)]
struct AdjustedResult {
    path: String,
    adjusted_score: f32,
    confidence: f32,
    reasoning: Option<String>,
}

pub fn llm_rerank(
    query: &str,
    bm25_results: Vec<SearchResult>,
    graph: Option<&DependencyGraph>,
) -> Result<Vec<SearchResult>> {
    // Serialize request
    let graph_context = graph.map(|g| extract_graph_context(g, &bm25_results));
    let request = RerankRequest {
        query: query.to_string(),
        bm25_results: bm25_results.clone(),
        graph_context,
    };
    let request_json = serde_json::to_string(&request)?;

    // Invoke re-ranking script
    let mut child = Command::new("./scripts/llm_reranker.sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn llm_reranker.sh")?;

    // Write request to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(request_json.as_bytes())?;
    }

    // Read response from stdout
    let output = child.wait_with_output()?;
    if !output.status.success() {
        anyhow::bail!("llm_reranker.sh failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let response: RerankResponse = serde_json::from_slice(&output.stdout)
        .context("failed to parse llm_reranker response")?;

    // Apply adjusted scores to original results
    let mut reranked = bm25_results;
    for adjusted in response.reranked_results {
        if let Some(result) = reranked.iter_mut().find(|r| r.path == adjusted.path) {
            result.score = adjusted.score;
            // Optionally: store confidence and reasoning in result metadata
        }
    }

    // Re-sort by adjusted scores
    reranked.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    Ok(reranked)
}

fn extract_graph_context(graph: &DependencyGraph, results: &[SearchResult]) -> GraphContext {
    // Extract relevant graph edges and attributes for top results
    // This gives LLM additional context for re-ranking decisions
    GraphContext {
        edges: vec![],  // TODO: Implement edge extraction
        attributes: Default::default(),
    }
}
```

### 4. Integration Point in `bm25.rs`

```rust
// In Bm25Index::search() method, after initial BM25 ranking

pub fn search(
    &self,
    query: &str,
    limit: usize,
    kind_filter: Option<&[NodeKind]>,
    enable_llm_reranking: bool,  // NEW: Feature flag
) -> Result<Vec<SearchResult>> {
    // ... existing BM25 search logic ...

    let mut results = /* ... BM25 results ... */;

    // Optional LLM-based re-ranking
    if enable_llm_reranking && results.len() >= 10 {
        results = llm_reranker::llm_rerank(query, results, Some(&self.graph))?;
    }

    Ok(results)
}
```

---

## Feasibility Assessment

### ✅ **Strong Positives**

1. **Claude Code CLI Fully Supports This**
   - Headless mode (`-p --output-format json`) designed for automation
   - Sub-agents can be defined dynamically via `--agents` JSON
   - Haiku-4-5 model available and fast (~1-2s response time)
   - JSON input/output with structured parsing

2. **Separates Concerns Cleanly**
   - BM25 handles lexical matching (fast, deterministic)
   - LLM handles semantic re-ranking (slower, but smarter)
   - No need to modify Tantivy internals

3. **Graph Context Integration**
   - Can pass entity relationships to LLM for richer re-ranking
   - Leverages CDSAgent's unique advantage (graph structure)
   - LLM can reason about code dependencies

4. **Experimentation-Friendly**
   - Feature flag (`enable_llm_reranking`) for easy A/B testing
   - Can tune prompts without recompiling Rust
   - Can swap models (haiku → sonnet) for quality vs speed tradeoffs

5. **Incremental Deployment**
   - Start with offline evaluation only
   - Graduate to query-time for complex queries (threshold-based)
   - Eventually optimize for production if successful

### ⚠️ **Concerns & Mitigations**

#### 1. **Latency** (MAJOR)

**Issue**: Each search adds 500-1500ms for LLM re-ranking

- BM25: ~50-100ms
- LLM (Haiku): ~500-1500ms
- **Total**: ~600-1600ms per query

**Mitigations**:

- **Selective re-ranking**: Only for queries with low BM25 confidence (<70% overlap)
- **Async processing**: Run re-ranking in background, return BM25 results immediately
- **Caching**: Cache re-ranked results for common queries
- **Batch mode**: Process multiple queries in parallel (future optimization)

**Recommended Strategy**: Use for **offline index optimization** initially, not real-time search.

#### 2. **Cost** (MODERATE)

**Issue**: Haiku API calls add cost per query

- Haiku-4-5: ~$0.001-0.003 per query (estimated)
- 1,000 queries = ~$1-3
- 100,000 queries = ~$100-300

**Mitigations**:

- **Experimental phase only**: Limit to evaluation workload (100-1000 queries)
- **Cost-aware triggering**: Only re-rank when BM25 confidence is low
- **Caching**: Amortize cost across repeated queries
- **Local LLM fallback**: Consider smaller local models for production

**Recommended Strategy**: Budget for experimental phase ($10-50), evaluate ROI before scaling.

#### 3. **Determinism** (MODERATE)

**Issue**: LLM outputs may vary between runs (temperature > 0)

- BM25: Always returns same results for same query
- LLM: May produce slightly different rankings

**Mitigations**:

- **Low temperature**: Set temperature=0.0 for deterministic outputs
- **Confidence thresholding**: Only apply re-ranking if LLM confidence >0.8
- **Ensemble scoring**: Blend BM25 (70%) + LLM (30%) scores

**Recommended Strategy**: Use deterministic mode (temp=0) for reproducibility.

#### 4. **Error Handling** (MODERATE)

**Issue**: Claude API may fail (rate limits, timeouts, network errors)

**Mitigations**:

- **Graceful degradation**: Fall back to BM25-only results on error
- **Timeout protection**: Max 3-second timeout for re-ranking
- **Retry logic**: 1-2 retries with exponential backoff
- **Monitoring**: Log re-ranking success/failure rates

**Recommended Strategy**: Treat LLM re-ranking as **optional enhancement**, never block on it.

#### 5. **Complexity** (LOW-MODERATE)

**Issue**: Adds moving parts (sub-agent, shell script, JSON marshaling)

**Mitigations**:

- **Well-defined interface**: JSON input/output schema
- **Comprehensive tests**: Unit tests for Rust wrapper, integration tests for end-to-end
- **Documentation**: Clear setup guide and troubleshooting

**Recommended Strategy**: Start simple, iterate based on results.

---

## Expected Impact

### Performance Improvements (Estimated)

Based on Thread-18 failure mode analysis:

**Target Queries**: 34% with RANKING_ISSUE (files found but ranked poorly)

- **Current**: 34 queries out of 100 have ranking problems
- **With LLM re-ranking**: Estimated +50-70% of these queries improve
- **Net gain**: +5-10% global overlap (from 63.16% → 68-73%)

**Example Scenarios**:

1. **Query**: "linear_model ridge.py parameters"
   - BM25 ranks test files above implementation
   - LLM recognizes `sklearn/linear_model/_ridge.py` as core implementation
   - **Expected improvement**: +2-3 ranks for relevant file

2. **Query**: "setuptools_scm integration"
   - BM25 ranks utility files above build config
   - LLM recognizes `setup.py` and `pyproject.toml` as integration points
   - **Expected improvement**: +3-5 ranks for relevant files

3. **Query**: "traverse call graph"
   - BM25 ranks literal term matches
   - LLM understands semantic intent (graph traversal algorithms)
   - **Expected improvement**: +1-2 ranks for relevant files

### Comparison with Thread-19 Options

| Approach | Estimated Impact | Implementation Time | Maintenance Burden | Cost |
|----------|-----------------|---------------------|-------------------|------|
| **Field Boost Tuning** | +3-5% | 2-3h | Low | $0 |
| **Path Bonus Tuning** | +2-3% | 1-2h | Low | $0 |
| **Content Synthesis** | +2-4% | 2-3h | Low | $0 |
| **LLM Re-Ranking** | **+5-10%** | **6-10h** | Medium | **$10-50/experiment** |
| **Fork Tantivy (k1=1.5)** | +5-10% | 1-2h | High | $0 |

**Insight**: LLM re-ranking offers **similar impact to k1 tuning** without the maintenance burden of forking Tantivy, but adds cost and latency.

---

## Implementation Roadmap

### Phase 1: Proof of Concept (Est: 3-4h)

**Goal**: Validate that Claude Code CLI headless mode can re-rank BM25 results

**Tasks**:

1. Create `.claude/agents/ast-graph-index-ranker.md` sub-agent definition (30min)
2. Write `scripts/llm_reranker.sh` wrapper script (1h)
3. Test end-to-end with 5 sample queries (1h)
4. Measure latency and cost (30min)
5. Document findings in POC report (1h)

**Deliverables**:

- Working sub-agent that can re-rank 5 queries
- Latency measurements (p50, p95, p99)
- Cost estimates per query
- POC report with recommendations

**Success Criteria**:

- Latency <2s per query
- Cost <$0.01 per query
- Qualitative improvement visible in top-10 results

### Phase 2: Rust Integration (Est: 3-4h)

**Goal**: Integrate LLM re-ranker into Rust codebase

**Tasks**:

1. Implement `llm_reranker.rs` module (2h)
2. Add feature flag to `bm25.rs::search()` (30min)
3. Write unit tests for Rust wrapper (1h)
4. Add integration test with mock LLM responses (1h)

**Deliverables**:

- `crates/cds-index/src/index/llm_reranker.rs` (200-300 lines)
- 10+ unit tests
- 2+ integration tests

**Success Criteria**:

- All tests pass
- Graceful error handling for API failures
- Clean separation of concerns

### Phase 3: Evaluation (Est: 2-3h)

**Goal**: Quantify impact on overlap metrics

**Tasks**:

1. Re-run smoke tests with LLM re-ranking enabled (1h)
2. Compare results: baseline vs LLM re-ranking (30min)
3. Analyze per-query improvements (30min)
4. Cost analysis: total API spend for 100 queries (15min)
5. Write evaluation report (1h)

**Deliverables**:

- Updated smoke test results with re-ranking enabled
- Comparison report: overlap@10 improvements
- Cost breakdown and ROI analysis
- Recommendation: Continue vs pivot

**Success Criteria**:

- Global overlap improves by ≥5% (63.16% → 68%+)
- Cost remains <$1 per 100 queries
- No regressions on well-performing queries (requests: 98.33%)

### Phase 4: Optimization (Est: 2-3h, if Phase 3 successful)

**Goal**: Reduce latency and cost for production readiness

**Tasks**:

1. Implement selective re-ranking (confidence threshold) (1h)
2. Add result caching (30min)
3. Optimize prompt for token efficiency (30min)
4. Benchmark optimized version (1h)

**Deliverables**:

- Selective re-ranking with <80% confidence filter
- LRU cache for top 100 queries
- Optimized sub-agent prompt (reduce tokens by 30%)

**Success Criteria**:

- Average latency <800ms (50% of queries skip re-ranking)
- Cost <$0.50 per 100 queries (with caching)
- Maintain ≥5% overlap improvement

---

## Risk Analysis

### High-Risk Factors

1. **Latency kills user experience** (Probability: HIGH, Impact: HIGH)
   - **Mitigation**: Async re-ranking, selective triggering, caching
   - **Fallback**: Offline-only use case (index optimization)

2. **Cost escalates with scale** (Probability: MEDIUM, Impact: HIGH)
   - **Mitigation**: Budget cap, cost-aware triggering, caching
   - **Fallback**: Local LLM (e.g., Llama 3.2) for production

3. **API failures disrupt search** (Probability: MEDIUM, Impact: MEDIUM)
   - **Mitigation**: Graceful degradation to BM25-only
   - **Fallback**: Always return BM25 results as baseline

### Low-Risk Factors

1. **LLM doesn't improve ranking** (Probability: LOW, Impact: LOW)
   - **Mitigation**: Quick POC (Phase 1) validates before heavy investment
   - **Fallback**: Abort after Phase 1 if no improvement

2. **Implementation complexity** (Probability: LOW, Impact: LOW)
   - **Mitigation**: Claude Code CLI handles complexity, clean interface
   - **Fallback**: Manual re-ranking for evaluation only

---

## Alternative Architectures

### Option A: Offline Index Optimization (Recommended)

**Use Case**: Pre-compute optimized rankings during index build

**Workflow**:

1. Build BM25 index as usual
2. For each entity, run sample queries and collect rankings
3. Use LLM to identify mis-ranked entities
4. Adjust entity boost factors in index
5. Rebuild index with optimized boosts

**Pros**:

- ✅ Zero query-time latency
- ✅ One-time cost per index build
- ✅ Deterministic results

**Cons**:

- ❌ Doesn't adapt to query variations
- ❌ Requires re-indexing to update

**Recommended for**: Initial experimentation phase

### Option B: Query-Time Re-Ranking (Selective)

**Use Case**: Re-rank only when BM25 confidence is low

**Workflow**:

1. Run BM25 search
2. Compute confidence score (e.g., top-1 score / top-10 average)
3. If confidence <0.7, trigger LLM re-ranking
4. Otherwise, return BM25 results directly

**Pros**:

- ✅ Balances latency and quality
- ✅ Focuses LLM on hard queries
- ✅ Adapts to query patterns

**Cons**:

- ❌ Variable latency (user confusion)
- ❌ Still requires API calls

**Recommended for**: Production deployment (if Phase 3 successful)

### Option C: Hybrid Ensemble

**Use Case**: Blend BM25 + LLM scores with learned weights

**Workflow**:

1. Run BM25 search → get lexical scores
2. Run LLM re-ranking → get semantic scores
3. Ensemble: `final_score = 0.7 * bm25_score + 0.3 * llm_score`
4. Re-rank by final scores

**Pros**:

- ✅ Preserves BM25 strengths
- ✅ Adds semantic signals
- ✅ Weights are tunable

**Cons**:

- ❌ Requires offline training for optimal weights
- ❌ Adds complexity

**Recommended for**: Advanced optimization (Phase 4+)

---

## Decision Matrix

### Should We Implement This?

**If your goal is...**

1. **Reach 75% overlap ASAP** → **YES, but with Option 3 (Alternative Optimizations) first**
   - Rationale: Field boost tuning (4-8h) has better ROI for immediate impact
   - Then try LLM re-ranking as "Option 5" if still below target

2. **Explore cutting-edge techniques** → **YES, Phase 1 POC immediately**
   - Rationale: This is novel, leverages CDSAgent's graph context
   - High learning value even if not production-ready

3. **Minimize cost and complexity** → **NO, stick to Option 3 (Alternative Optimizations)**
   - Rationale: Field boosts, path bonus, content synthesis are free and simple
   - LLM re-ranking adds cost and moving parts

4. **Validate k1 parameter impact** → **MAYBE, as a comparison baseline**
   - Rationale: If LLM re-ranking yields +5-10%, it indirectly validates k1 hypothesis
   - Can inform decision to fork Tantivy later

### Recommended Strategy

**Short-term (Thread-20)**:

1. Implement Option 3 (Alternative Optimizations) first
2. Run experiments: field boosts, path bonus, content synthesis
3. Measure overlap improvement from these "free" optimizations

**Medium-term (Thread-21-22)**:

1. If still below 75% after Option 3 → Run Phase 1 POC for LLM re-ranking
2. Validate that 5-10% improvement is achievable
3. Assess cost and latency for experimental workload

**Long-term (Post-MVP)**:

1. If POC successful → Implement Phase 2-3 (Rust integration + evaluation)
2. If evaluation successful → Deploy Option B (selective re-ranking) to production
3. If successful at scale → Contribute upstream to Tantivy (configurable k1/b)

---

## Conclusion

Your proposed **LLM-based re-ranking with Claude Code CLI headless mode** is **highly feasible** and represents an innovative approach to improving search quality. The architecture is clean, the tooling supports it well, and the expected impact is significant (+5-10% overlap).

**However**, I recommend implementing **Thread-19 Option 3 (Alternative Optimizations)** first, as it offers:

- Faster implementation (4-8h vs 6-10h)
- Zero cost ($0 vs $10-50)
- Zero latency overhead (0ms vs 500-1500ms)
- Similar estimated impact (+7-12% vs +5-10%)

**If Option 3 yields <70% global overlap**, then LLM re-ranking becomes the next best option, combining the strengths of BM25 (speed) and LLM (semantic understanding) without the maintenance burden of forking Tantivy.

---

## Next Steps

**Immediate (Thread-20)**: Implement Option 3 (Alternative Optimizations)

1. Field boost tuning (2-3h)
2. Path match bonus tuning (1-2h)
3. Content synthesis enhancement (2-3h)

**If overlap <70% after Option 3**:

**Thread-21**: Run Phase 1 POC for LLM re-ranking

1. Create sub-agent definition (30min)
2. Write wrapper script (1h)
3. Test with 5 sample queries (1h)
4. Measure and document (1.5h)

**Decision Point**: Continue to Phase 2-3 only if POC shows ≥5% improvement

---

**Generated**: 2025-11-04T07:38:53Z  
**Thread**: 19 (Session 05)
**Status**: ✅ FEASIBILITY ANALYSIS COMPLETE
**Recommendation**: Pursue Option 3 first, then LLM re-ranking if needed
