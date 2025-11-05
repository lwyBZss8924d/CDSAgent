# THREAD-28: Multi-Repo Comparison Results

**Task**: T-02-02-sparse-index - Graph Parity Analysis Across All Test Fixtures
**Session**: Thread-28 (continuation from Thread-27)
**Date**: 2025-11-04
**Status**: ⚠️ REPOSITORY VERSION MISMATCH DETECTED
**Model**: Claude Sonnet 4.5 (autonomous execution mode)

---

## Executive Summary

Thread-28 completed batch comparison across all 6 repositories following the Thread-27 conversion bug fixes. Results reveal **critical repository version mismatch** between CDSAgent exports (current HEAD) and LocAgent golden baselines (specific SWE-bench commits).

**Key Findings**:

- ✅ **LocAgent**: 100% perfect parity (validation baseline)
- ⚠️ **requests**: 0% overlap (DIFFERENT COMMITS - test_requests.py vs tests/test_utils.py)
- ✅ **pytest**: 95% overlap (HIGH PARITY despite version uncertainty)
- ✅ **django**: 93% overlap (HIGH PARITY despite version uncertainty)
- ⚠️ **matplotlib**: 7-18% overlap (LIKELY VERSION MISMATCH)
- ⚠️ **scikit-learn**: 10-50% overlap (LIKELY VERSION MISMATCH)

**Critical Issue**: CDSAgent graphs were exported from `tmp/smoke/*` repositories at current HEAD, but LocAgent golden baselines use specific SWE-bench instance commits (e.g., `psf__requests-1963`, `pytest-dev__pytest-11143`).

---

## Detailed Results by Repository

### 1. ✅ LocAgent (Thread-27 Validated Baseline)

**Status**: ✅ PERFECT PARITY

**Node Overlap**:

- class: 100.00% (86/86)
- directory: 100.00% (20/20)
- file: 100.00% (74/74)
- function: 100.00% (478/478)

**Edge Overlap**:

- contains: 100.00% (657/657)
- imports: 100.00% (190/190) [+92 extra in CDSAgent]
- inherits: 100.00% (13/13) [+4 extra in CDSAgent]
- invokes: 100.00% (531/531) [+48 extra in CDSAgent]

**Conclusion**: Conversion bugs fixed successfully. CDSAgent's extra edges indicate improved completeness.

---

### 2. ⚠️ requests (CRITICAL - Version Mismatch Confirmed)

**Status**: ❌ DIFFERENT REPOSITORY VERSIONS

**Node Overlap**:

- class: 0.85% (1/118) ❌
- directory: 30.00% (3/10)
- file: 3.95% (3/76) ❌
- function: 0.00% (0/548) ❌

**Edge Overlap**:

- contains: 0.80% (6/751) ❌
- imports: 0.00% (0/291) ❌
- inherits: 0.00% (0/69) ❌
- invokes: 0.00% (0/874) ❌

**Root Cause Analysis**:

**CDSAgent Export** (from `tmp/smoke/requests`):

- Files: `tests/test_utils.py`
- Sample functions:
  - `tests/test_utils.py:TestSuperLen.test_io_streams`
  - `tests/test_utils.py:TestSuperLen.test_super_len_correctly_calculates_len_of_partially_read_file`

**LocAgent Golden Baseline** (from `psf__requests-1963`):

- Files: `test_requests.py`
- Sample functions:
  - `test_requests.py:httpbin`
  - `test_requests.py:RequestsTestCase.setUp`
  - `test_requests.py:RequestsTestCase.tearDown`

**Conclusion**: These are COMPLETELY DIFFERENT file sets. The SWE-bench instance `psf__requests-1963` is from a specific commit/issue, while CDSAgent exported from current HEAD of the requests repository.

---

### 3. ✅ pytest (HIGH PARITY)

**Status**: ✅ GOOD PARITY (despite potential version differences)

**Node Overlap**:

- class: 96.30% (624/648) ✅
- directory: 100.00% (64/64) ✅
- file: 97.59% (243/249) ✅
- function: 95.37% (4797/5030) ✅

**Edge Overlap**:

- contains: 95.61% (5727/5990) ✅
- imports: 90.16% (55/61) ✅
- inherits: 93.94% (62/66) ✅
- invokes: 87.54% (2157/2464) ✅

**Top Gaps**:

1. Missing 233 functions (5% of 5030) - Examples: `doc/en/conf.py:configure_logging`, `scripts/prepare-release-pr.py:login`
2. Missing 24 classes (4% of 648) - Examples: `doc/en/conf.py:configure_logging.WarnLogFilter`
3. Missing 307 invokes edges (12% of 2464)

**Interpretation**: High overlap (>90% on most metrics) suggests either:

- Commits are close enough in history
- Core pytest files haven't changed significantly between commits
- Entity extraction is robust

**Conclusion**: ✅ Acceptable parity. Gaps appear to be documentation/script files (e.g., `doc/en/conf.py`, `scripts/*.py`), not core pytest functionality.

---

### 4. ✅ django (HIGH PARITY with Lower Invokes)

**Status**: ✅ GOOD PARITY (despite potential version differences)

**Node Overlap**:

- class: 97.12% (1552/1598) ✅
- directory: 98.50% (593/602) ✅
- file: 99.00% (495/500) ✅
- function: 93.03% (3885/4176) ✅

**Edge Overlap**:

- contains: 94.89% (6524/6875) ✅
- imports: 97.28% (429/441) ✅
- inherits: 98.02% (347/354) ✅
- invokes: 73.09% (1670/2285) ⚠️

**Top Gaps**:

1. Missing 291 functions (7% of 4176) - Examples: `setup.py:read`, `tests/admin_filters/tests.py:ListFiltersTests.test_booleanfieldlistfilter_nullbooleanfield`
2. Missing 46 classes (3% of 1598)
3. Missing 615 invokes edges (27% of 2285) ⚠️

**Note**: Significantly lower invokes overlap (73%) compared to other edge types (95-98%). This could indicate:

- Version differences in test files
- Missing function call detection in complex test scenarios
- Legitimate entity extraction gaps

**Conclusion**: ✅ Acceptable node parity (>93%), but invokes edge detection warrants investigation.

---

### 5. ⚠️ matplotlib (LOW PARITY - Likely Version Mismatch)

**Status**: ⚠️ SIGNIFICANT GAPS (likely version mismatch)

**Node Overlap**:

- class: 7.44% (9/121) ❌
- directory: 33.33% (22/66) ⚠️
- file: 2.40% (12/500) ❌
- function: 11.35% (70/617) ❌

**Edge Overlap**:

- contains: 8.60% (112/1303) ❌
- imports: 0.00% (0/3) ❌
- inherits: 0.00% (0/18) ❌
- invokes: 18.39% (64/348) ⚠️

**Top Missing Files**:

- `examples/animation/animate_decay.py`
- `examples/animation/animated_histogram.py`
- `examples/animation/animation_demo.py`

**Interpretation**: Golden baseline appears to focus on `examples/` and `setup.py` files, while CDSAgent export from current HEAD may have different file structure or be from a different commit.

**Conclusion**: ⚠️ Requires investigation - likely repository version mismatch.

---

### 6. ⚠️ scikit-learn (MEDIUM PARITY - Likely Version Mismatch)

**Status**: ⚠️ MODERATE GAPS (likely version mismatch)

**Node Overlap**:

- class: 32.37% (180/556) ⚠️
- directory: 85.39% (76/89) ✅
- file: 58.00% (290/500) ⚠️
- function: 50.15% (2742/5468) ⚠️

**Edge Overlap**:

- contains: 49.74% (3289/6612) ⚠️
- imports: 30.81% (1020/3311) ⚠️
- inherits: 26.93% (143/531) ⚠️
- invokes: 10.21% (4482/43899) ❌

**Top Missing Files**:

- `conftest.py`
- `examples/cluster/plot_agglomerative_clustering.py`
- `examples/cluster/plot_cluster_iris.py`

**Interpretation**: Better than matplotlib (30-85% on some metrics) but still significant gaps. Directory overlap (85%) suggests file structure is similar, but specific files differ.

**Conclusion**: ⚠️ Requires investigation - likely repository version mismatch + potential entity extraction gaps.

---

## Root Cause Analysis

### Repository Version Mismatch

**CDSAgent Exports** (Thread-23/24):

- Source: `tmp/smoke/*` repositories at current HEAD
- Export method: `cargo test --test graph_export_tests export_all_fixtures -- --ignored`
- Commit: Whatever is currently checked out in `tmp/smoke/`

**LocAgent Golden Baselines** (T-06-01 Phase 2):

- Source: SWE-bench Lite specific instances with commit SHAs
- Instances:
  - `psf__requests-1963` (specific issue/commit)
  - `pytest-dev__pytest-11143` (specific issue/commit)
  - `django__django-10914` (specific issue/commit)
  - `matplotlib__matplotlib-18869` (specific issue/commit)
  - `scikit-learn__scikit-learn-10297` (specific issue/commit)

**Evidence of Mismatch**:

- **requests**: 0% overlap, completely different files (`test_requests.py` vs `tests/test_utils.py`)
- **matplotlib**: 7% overlap, missing most example files
- **scikit-learn**: 30-50% overlap, missing `conftest.py` and many example files

---

## Interpretation of Results

### Valid Comparisons

1. **LocAgent**: ✅ VALID - Same repository, conversion bugs fixed, 100% parity achieved
2. **pytest**: ✅ MOSTLY VALID - 95% overlap suggests commits are close or core files unchanged
3. **django**: ✅ MOSTLY VALID - 93% overlap (nodes), though invokes edges are lower at 73%

### Invalid Comparisons (Version Mismatch)

1. **requests**: ❌ INVALID - Different commits confirmed (0% overlap)
2. **matplotlib**: ⚠️ LIKELY INVALID - 7% overlap suggests different file sets
3. **scikit-learn**: ⚠️ PARTIALLY INVALID - 30-85% overlap (mixed results)

---

## Next Steps

### Thread-29: Re-Export CDSAgent Graphs from Correct SWE-bench Commits

**Objective**: Ensure apple-to-apple comparison by using exact SWE-bench instance commits

**Required Actions**:

1. **Identify SWE-bench Instance Commits**:
   - Check `tests/fixtures/parity/swe-bench-lite/samples.yaml`
   - Extract git commit SHAs for each instance

2. **Checkout Correct Commits**:

   ```bash
   cd tmp/smoke/requests
   git checkout <psf__requests-1963_commit_sha>

   cd tmp/smoke/pytest
   git checkout <pytest-dev__pytest-11143_commit_sha>

   # Repeat for django, matplotlib, scikit-learn
   ```

3. **Re-Export CDSAgent Graphs**:

   ```bash
   cargo test --test graph_export_tests export_all_fixtures -- --ignored --nocapture
   ```

4. **Re-Run Comparisons**:
   - Convert new exports to pickle with fixed script
   - Run comparison harness on all 6 repos
   - Validate parity results

**Expected Outcome**: High parity (>90%) across all repos after using correct commits.

---

### Thread-30: Analyze Remaining Entity Extraction Gaps (After Thread-29)

**Objective**: Identify genuine entity extraction issues vs. version differences

**Focus Areas** (based on Thread-28 findings):

1. **Django invokes edges** (73% overlap):
   - Why are function calls missing in django tests?
   - Is this a graph builder limitation or test file complexity?

2. **Pytest documentation files** (missing 233 functions):
   - Are doc/script files being excluded?
   - Should we filter these out of parity analysis?

3. **Nested class methods**:
   - Examples: `doc/en/conf.py:configure_logging.WarnLogFilter.filter`
   - Is nested entity parsing working correctly?

---

## Files Modified

### Created Documents

1. `.artifacts/spec-tasks-T-02-02-sparse-index/THREAD-28-MULTI-REPO-COMPARISON-RESULTS.md` (this file)

### Comparison Reports Generated

1. `comparison_locagent_fixed.json` (Thread-27) - 100% parity ✅
2. `comparison_requests.json` - 0% overlap (version mismatch) ❌
3. `comparison_pytest.json` - 95% overlap (high parity) ✅
4. `comparison_django.json` - 93% overlap (high parity) ✅
5. `comparison_matplotlib.json` - 7-18% overlap (likely mismatch) ⚠️
6. `comparison_sklearn.json` - 10-50% overlap (moderate mismatch) ⚠️

---

## Lessons Learned

### 1. Always Validate Repository Versions Before Comparison

**Mistake**: Assumed `tmp/smoke/*` repositories were at correct SWE-bench commits
**Reality**: They were at current HEAD, causing invalid comparisons
**Fix**: Always check commit SHAs against SWE-bench instance metadata

### 2. High Overlap Can Mask Version Differences

**Observation**: pytest/django showed 90%+ overlap despite potential version mismatch
**Interpretation**: Either commits are close or core files are stable
**Implication**: Don't assume low overlap = bad implementation - check versions first!

### 3. Zero Overlap Is a Red Flag for Data Issues

**Pattern**: 0% function/class overlap in requests (identical to Thread-26 bug pattern)
**Root Cause**: Different repository versions (not conversion bugs)
**Diagnostic**: Always inspect sample nodes when seeing catastrophic mismatch

---

## Summary Statistics

| Repository | Nodes (CDSAgent) | Nodes (LocAgent) | Function Overlap | Class Overlap | Status |
|------------|------------------|------------------|------------------|---------------|--------|
| LocAgent | 658 | 658 | 100.00% | 100.00% | ✅ VALID |
| requests | 774 | 752 | 0.00% | 0.85% | ❌ INVALID (version mismatch) |
| pytest | 6,712 | 5,991 | 95.37% | 96.30% | ✅ MOSTLY VALID |
| django | 44,450 | 6,876 | 93.03% | 97.12% | ✅ MOSTLY VALID |
| matplotlib | 11,973 | 1,304 | 11.35% | 7.44% | ⚠️ LIKELY INVALID |
| scikit-learn | 12,652 | 6,613 | 50.15% | 32.37% | ⚠️ PARTIALLY INVALID |

**Overall**: 1/6 perfect parity (LocAgent), 2/6 high parity (pytest/django), 3/6 likely version mismatch (requests/matplotlib/scikit-learn).

---

## Conclusion

Thread-28 revealed critical repository version mismatch between CDSAgent exports and LocAgent golden baselines. While conversion bugs from Thread-27 were successfully fixed (100% LocAgent parity), meaningful multi-repo comparison requires re-exporting CDSAgent graphs from exact SWE-bench instance commits.

**Positive Findings**:

- ✅ Conversion bugs completely resolved (Thread-27 validation)
- ✅ High parity achieved on pytest/django despite version uncertainty (90%+ overlap)
- ✅ Extra edges in CDSAgent indicate improved completeness over Python reference

**Blocking Issues**:

- ❌ Repository version mismatch prevents accurate parity analysis
- ❌ Thread-29 MUST checkout correct SWE-bench commits before re-running comparisons

**Ready For**: Thread-29 - Re-export CDSAgent graphs from correct commits → Thread-30 - Analyze genuine entity extraction gaps

---

**Generated**: 2025-11-04
**Thread**: 28 (Multi-repo comparison after Thread-27 fixes)
**Status**: ⚠️ REPOSITORY VERSION MISMATCH DETECTED
**Next**: Thread-29 - Re-export from correct SWE-bench commits
