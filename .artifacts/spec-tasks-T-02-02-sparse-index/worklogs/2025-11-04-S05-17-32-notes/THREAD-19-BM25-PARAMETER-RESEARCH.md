# THREAD-19 BM25 PARAMETER RESEARCH

**Task**: T-02-02-sparse-index - BM25 Parameter Tuning Investigation
**Session**: 05, Thread 19
**Date**: 2025-11-04
**Duration**: 1.5h (06:30-08:00 UTC)
**Status**: ✅ COMPLETE - Research findings documented, decision pending

---

## Executive Summary

Thread-19 conducted comprehensive research into configuring Tantivy's BM25 k1 parameter to match LocAgent's k1=1.5 (currently Tantivy uses k1=1.2). **CRITICAL FINDING**: Tantivy hardcodes k1 and b as constants with no public API for configuration in any version (0.25.0 or main branch).

**Key Finding**: **k1 parameter tuning is blocked by Tantivy's architecture**. We must either (1) fork Tantivy, (2) focus on alternative optimizations (field boosts, chunking), or (3) contribute upstream to add configuration support.

---

## Research Methodology

### Phase 1: Source Code Analysis ✅

**Target**: Tantivy 0.25.0 (current version in use)

**File**: `~/.cargo/registry/src/.../tantivy-0.25.0/src/query/bm25.rs`

**Lines 8-9**:

```rust
const K1: Score = 1.2;
const B: Score = 0.75;
```

**Confirmed**:

- k1 and b are module-level constants
- No configuration API exposed
- Values used throughout `Bm25Weight` implementation
- No constructor or builder pattern for custom parameters

### Phase 2: Community Research ✅

**GitHub Issue #401** ("Custom weight/score calculation", Aug 2018):

- User requested custom BM25 formulas with adjustable coefficients
- Maintainer suggested "tunable parameters like in lucene"
- PR #411 ("Add constructors for setting bm25 coefficients") was created

**GitHub PR #411** (Closed Jan 2021, NOT MERGED):

- Proposed: Direct k1/b configuration in query objects
- Rejected: Maintainer preferred Lucene-style `searcher::search_with_similarity()`
- Alternative approach was never implemented

### Phase 3: Latest Version Check ✅

**Tantivy main branch** (2025-11-04):

- k1 and b remain hardcoded as 1.2 and 0.75
- No configuration API added since PR #411
- No alternative implementation visible

**Conclusion**: Configurable BM25 parameters are NOT available in Tantivy.

---

## Root Cause Recap (from Thread-18)

| Parameter | LocAgent | CDSAgent/Tantivy | Impact |
|-----------|----------|------------------|--------|
| **k1** | 1.5 | 1.2 | ~20% term saturation difference |
| **b** | 0.75 | 0.75 | ✅ Matching |
| **Field count** | 1 (content) | 5 (name/path/content/kind/boost) | Multi-field dilution |
| **Field boosts** | None | name=3.5x, path=3.0x | May over-rank short names |

**Hypothesis** (from Thread-18):
k1=1.2 vs k1=1.5 mismatch causes repeated technical terms to be under-valued, explaining why:

- "linear_model ridge.py parameters" performs poorly (repeated terms)
- "setuptools_scm integration" ranks build files low (compound keywords)
- Simple queries (requests repo: 98.33%) work perfectly

**Expected Impact** of k1=1.5: +5-10% global overlap (Thread-18 estimate)

---

## Solution Options Analysis

### Option 1: Fork Tantivy (Local Patch)

**Approach**:

1. Create local fork of Tantivy 0.25.0
2. Modify `src/query/bm25.rs` lines 8-9 to `K1 = 1.5`
3. Use `[patch.crates-io]` in Cargo.toml to override dependency

**Pros**:

- ✅ Quick implementation (~1 hour)
- ✅ Maintains all Tantivy optimizations
- ✅ Precise k1 matching with LocAgent
- ✅ No API changes required

**Cons**:

- ❌ Maintenance burden (track upstream updates)
- ❌ Version lock-in (can't upgrade Tantivy easily)
- ❌ Deployment complexity (must document patch)
- ❌ Team coordination overhead

**Estimated Effort**: 1-2 hours (fork + testing)

### Option 2: Contribute Upstream (Long-term Solution)

**Approach**:

1. Design Lucene-style similarity configuration API
2. Submit PR to Tantivy with configurable k1/b
3. Wait for merge and release
4. Upgrade CDSAgent to use new API

**Pros**:

- ✅ Benefits entire Tantivy community
- ✅ No maintenance burden long-term
- ✅ Clean upgrade path
- ✅ Aligns with maintainer's vision (Issue #401)

**Cons**:

- ❌ Long timeline (weeks to months)
- ❌ Uncertain acceptance (PR may be rejected)
- ❌ Blocks immediate optimization work
- ❌ Requires Tantivy expertise for quality PR

**Estimated Effort**: 8-16 hours (design + implementation + review cycles)

### Option 3: Alternative Optimizations (Accept k1=1.2)

**Approach**:

1. Accept k1=1.2 as fixed constraint
2. Focus on tuning other parameters:
   - Reduce field boosts (3.5x/3.0x → 2.0x/1.5x)
   - Optimize chunking strategy (align with LocAgent's EpicSplitter)
   - Tune path match bonus (currently 1.15)
   - Improve content synthesis (richer metadata)

**Pros**:

- ✅ No fork required
- ✅ No Tantivy expertise needed
- ✅ Immediate implementation possible
- ✅ Multiple tuning levers available

**Cons**:

- ❌ May not reach 75% target (k1 mismatch remains)
- ❌ Ignores root cause (Thread-18 identified k1 as primary blocker)
- ❌ More trial-and-error iterations
- ❌ Harder to predict impact

**Estimated Effort**: 4-8 hours (tuning experiments + validation)

### Option 4: Custom BM25 Implementation

**Approach**:

1. Implement pure-Rust BM25 scorer
2. Access Tantivy's term frequencies and field norms directly
3. Bypass Tantivy's BM25Weight entirely

**Pros**:

- ✅ Full control over k1/b parameters
- ✅ No fork required
- ✅ Can add other customizations

**Cons**:

- ❌ Complex implementation (4-8 hours)
- ❌ May bypass Tantivy optimizations
- ❌ Requires deep understanding of Tantivy internals
- ❌ Higher maintenance burden

**Estimated Effort**: 8-12 hours (implementation + testing + optimization)

---

## Current Baseline Performance (Thread-18 Results)

**Global Average**: 63.16% (target: 75%, **gap: 11.84%**)

**Per-Repo Breakdown**:

| Repo | Avg Overlap | Status | Primary Issue |
|------|-------------|--------|---------------|
| **requests** | 98.33% | ✅ EXCELLENT | None (proves BM25 works!) |
| **LocAgent** | 70.38% | ⚠️ GOOD | 26% ranking issues |
| **pytest** | 62.33% | ⚠️ OK | 50% ranking issues |
| **django** | 56.57% | ❌ NEEDS WORK | 40% ranking issues |
| **matplotlib** | 56.82% | ❌ NEEDS WORK | **60% ranking issues** |
| **scikit-learn** | 34.51% | ❌ WORST | 60% ranking + 30% retrieval |

**Failure Mode Distribution** (100 queries total):

- **PERFORMING_WELL**: 42% (target met)
- **RANKING_ISSUE**: 34% (**primary blocker**)
- **RETRIEVAL_GAP**: 8% (graph parity issues)
- **MODERATE**: 16% (edge cases)

**Key Insight**: 34% ranking issues >> 8% retrieval gaps → Parameter tuning (not algorithm changes) is the path forward.

---

## Decision Criteria

### Prioritization Framework

**Criteria Weighting**:

1. **Time-to-impact** (40%): How quickly can we improve overlap?
2. **Maintenance burden** (25%): Long-term cost and complexity
3. **Success probability** (20%): Likelihood of reaching 75% target
4. **Team skillset** (15%): Required expertise level

### Scoring Each Option

| Option | Time-to-Impact | Maintenance | Success Prob | Skillset | **Total Score** |
|--------|---------------|-------------|--------------|----------|----------------|
| **Fork Tantivy** | 9/10 (1-2h) | 3/10 (high burden) | 8/10 (k1 match) | 7/10 (Rust + Cargo) | **6.7/10** |
| **Upstream PR** | 1/10 (weeks) | 10/10 (none) | 7/10 (uncertain) | 4/10 (Tantivy expertise) | **4.9/10** |
| **Alt. Optimizations** | 7/10 (4-8h) | 9/10 (low) | 5/10 (k1 remains) | 9/10 (our code) | **7.25/10** |
| **Custom BM25** | 4/10 (8-12h) | 6/10 (medium) | 8/10 (full control) | 5/10 (Tantivy internals) | **5.7/10** |

**Winner: Option 3 (Alternative Optimizations)** - Best balance of speed, maintainability, and skillset fit.

---

## Recommendations

### Immediate Action (Thread-20): Focus on Alternative Optimizations

**Why**:

1. **Fastest path to improvement** (4-8 hours vs 1-2 hours for fork, but no maintenance burden)
2. **Multiple tuning levers** available (field boosts, chunking, path bonus)
3. **No dependency risks** (no fork, no waiting for upstream)
4. **requests repo proves concept**: 98.33% with k1=1.2 shows other factors can compensate

**Tuning Roadmap**:

**Phase 1: Field Boost Reduction** (2-3h)

- Current: name=3.5x, path=3.0x
- Target: name=2.0x, path=1.5x
- Rationale: May over-rank short names vs content relevance
- Expected impact: +3-5% overlap (reduce CDS_ONLY false positives)

**Phase 2: Path Match Bonus Tuning** (1-2h)

- Current: PATH_MATCH_BONUS=1.15
- Experiment: 1.0, 1.25, 1.5
- Rationale: Path tokens heavily weighted in queries
- Expected impact: +2-3% overlap

**Phase 3: Content Synthesis Enhancement** (2-3h)

- Add missing attribute types (decorators, base classes, imports)
- Improve docstring extraction (currently included)
- Align with LocAgent's node representation
- Expected impact: +2-4% overlap (reduce LOC_ONLY gaps)

**Total Estimated Impact**: +7-12% overlap (reach 70-75% target)

### Medium-Term (Post-0.1.0-MVP): Upstream Contribution

**Approach**:

1. Design clean API for configurable BM25 parameters
2. Implement following maintainer's Lucene-style vision (Issue #401)
3. Submit high-quality PR with tests and documentation
4. Document in CDSAgent for future upgrade

**Timeline**: After M5 (Production Release Candidate)

### Long-Term (If Alternative Optimizations Fail): Consider Fork

**Trigger**: If Phase 1-3 optimizations yield <70% global overlap

**Approach**:

1. Create minimal Tantivy patch (only `K1 = 1.5`)
2. Document in deployment guide
3. Track upstream updates monthly
4. Plan migration once upstream support lands

---

## Open Questions

1. **Can we reach 75% with k1=1.2?**
   → **Answer pending**: Phase 1-3 tuning experiments will reveal ceiling

2. **What's the true impact of k1=1.2 vs 1.5?**
   → **Estimate**: 5-10% based on Thread-18 analysis, but unvalidated

3. **Should we parallelize upstream contribution?**
   → **Recommendation**: No - focus on immediate optimization first, contribute after MVP

4. **Are there other BM25 implementations in Rust?**
   → **Not researched**: Could explore as fallback if Tantivy proves insufficient

---

## Artifacts Generated

### Research Documentation

- `.artifacts/.../THREAD-19-BM25-PARAMETER-RESEARCH.md` (this document)

### Source Analysis

- Tantivy 0.25.0 `src/query/bm25.rs` reviewed (244 lines)
- Constants confirmed: K1=1.2, B=0.75

### Community Research

- Issue #401 analyzed (custom scoring discussion)
- PR #411 analyzed (rejected coefficient approach)
- Latest main branch checked (no config API)

### Decision Framework

- 4 solution options documented with pros/cons
- Scoring matrix with weighted criteria
- Recommendation: Alternative Optimizations (Option 3)

---

## Next Steps (Thread-20 Roadmap)

### Phase 1: Field Boost Tuning (Est: 2-3h)

1. **Modify bm25.rs line 360-362**:

   ```rust
   // OLD:
   parser.set_field_boost(self.fields.name, 3.5);
   parser.set_field_boost(self.fields.path, 3.0);

   // NEW:
   parser.set_field_boost(self.fields.name, 2.0);
   parser.set_field_boost(self.fields.path, 1.5);
   ```

2. **Re-run smoke tests**:

   ```bash
   SMOKE_REPO_PATHS="$LOCAGENT,$REQUESTS,$DJANGO,$MATPLOTLIB,$PYTEST,$SKLEARN" \
   SMOKE_OVERLAP_DIAG=1 \
   cargo test -p cds-index smoke_sparse_index_overlap_report -- --ignored --nocapture
   ```

3. **Compare results**: Baseline (63.16%) vs new configuration

4. **Document findings**: Create THREAD-20-FIELD-BOOST-TUNING.md

### Phase 2: Path Bonus Tuning (Est: 1-2h)

1. Experiment with PATH_MATCH_BONUS values (1.0, 1.25, 1.5)
2. Run smoke tests for each configuration
3. Identify optimal value

### Phase 3: Content Synthesis (Est: 2-3h)

1. Analyze LOC_ONLY gaps (files LocAgent finds but CDSAgent misses)
2. Enhance synthesize_content() with missing attributes
3. Validate improvement

**Total Estimated Time**: 5-8 hours to reach 70-75% target

---

## Metrics Summary

| Metric | Value | Status |
|--------|-------|--------|
| **Tantivy versions checked** | 2 (0.25.0, main) | ✅ COMPLETE |
| **k1 parameter confirmed** | 1.2 (hardcoded) | ✅ VERIFIED |
| **Configuration API found** | None | ❌ NOT AVAILABLE |
| **Community research** | Issue #401, PR #411 | ✅ COMPLETE |
| **Solution options analyzed** | 4 options | ✅ COMPLETE |
| **Recommendation** | Alternative Optimizations | ✅ DEFINED |
| **Estimated impact (Option 3)** | +7-12% overlap | ⚠️ TO BE VALIDATED |

---

## Conclusion

Thread-19 confirmed that Tantivy's k1 parameter is hardcoded at 1.2 with no configuration API in any version. While the k1 mismatch (1.2 vs LocAgent's 1.5) is a contributing factor to the 11.84% overlap gap, **alternative optimization strategies offer a pragmatic path forward** without the complexity of forking Tantivy.

The **requests repo's 98.33% performance with k1=1.2** proves that excellent overlap is achievable within Tantivy's constraints. By focusing on field boost tuning, path match bonuses, and content synthesis enhancement, we can likely bridge the majority of the gap while maintaining a clean, maintainable codebase.

**Thread-20 Objective**: Implement Phase 1-3 optimizations and validate if 75% global overlap is achievable with k1=1.2.

---

**Generated**: 2025-11-04T08:00:00Z
**Thread**: 19 (Session 05)
**Commits**: Pending (documentation only)
**Status**: ✅ RESEARCH COMPLETE, READY FOR OPTIMIZATION
**Next**: Thread-20 - Field Boost Tuning Experiments
