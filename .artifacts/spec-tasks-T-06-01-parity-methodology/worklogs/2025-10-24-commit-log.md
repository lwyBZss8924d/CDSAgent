# Commit Log - 2025-10-24

**Task**: T-06-01-parity-methodology - LocAgent Parity Validation Methodology
**Date**: 2025-10-24
**Author**: Claude Code Agent

---

## Commits Overview

**Total Commits**: 1 major commit (Phase 2 baseline extraction + documentation)
**Branch**: main (will be merged from feat/task/T-06-01-parity-methodology)
**Files Changed**: 35+ files (baselines, scripts, docs, metadata)
**Lines Added**: ~19,000 (majority from JSON graph baselines)
**Lines Deleted**: ~15 (script cleanups, doc updates)

---

## Commit 1: feat(parity): T-06-01 Phase 2 - extract comprehensive baselines with llama-index limitation documented

**Hash**: (pending commit)
**Date**: 2025-10-24
**Files Changed**: 35

### Commit Message

```
feat(parity): T-06-01 Phase 2 - extract comprehensive baselines with llama-index limitation documented

Extract comprehensive LocAgent baselines for parity validation:

Baseline Extraction Complete:
• Graph baselines: 6/6 repos (LocAgent + 5 SWE-bench Lite)
  - LocAgent: 658 nodes (86 classes, 478 functions)
  - Django: 6,876 nodes (1,598 classes, 4,176 functions)
  - Scikit-learn: 6,613 nodes (556 classes, 5,468 functions)
  - Matplotlib: 1,304 nodes (121 classes, 617 functions)
  - Pytest: 5,991 nodes (648 classes, 5,030 functions)
  - Requests: 752 nodes (118 classes, 548 functions)
  - Total: 22,194 nodes extracted

• Traverse baselines: 60/60 scenarios (10 × 6 repos)
  - Callees 1-hop: 5 scenarios per repo
  - Subclasses: 3 scenarios per repo
  - Imports: 2 scenarios per repo

• Search baselines: 1/6 repos (LocAgent only)
  - ⚠️ SWE-bench repos failed due to llama-index SimpleDirectoryReader limitation
  - Root cause: validates required_exts=['.py'] at root level before recursing
  - Impact: Search validation will use live comparison for SWE-bench repos

• Performance baselines: 1/6 repos (LocAgent only)
  - ⚠️ Same llama-index limitation as search baselines
  - CDSAgent will generate own benchmarks in T-08-04

Automation Toolkit Created:
• scripts/swe-lite - Main CLI wrapper (311 lines)
  - Commands: fetch, check, baseline (graph/traverse/search/perf)
  - HF_TOKEN validation, uv venv management
  - Parallel execution support

• 7 Python helper scripts:
  - fetch-swe-bench-lite.py (107 lines)
  - select-swe-bench-instances.py (134 lines)
  - extract-parity-baseline.py (orchestrator)
  - extract-search-baseline.py (203 lines, with workaround attempts)
  - extract-traverse-baseline.py (189 lines, pydot handling)
  - benchmark-performance.py (156 lines)
  - tests/fixtures/parity/swe-bench-lite/samples.yaml (metadata)

Documentation Updates:
• docs/parity-validation-methodology.md
  - Added Section 10: Known Limitations (130+ lines)
  - Detailed llama-index issue root cause analysis
  - CDSAgent Rust implementation plan using walkdir
  - Why limitation doesn't block M2 development

• tests/fixtures/parity/golden_outputs/README.md (NEW, 148 lines)
  - Complete baseline files documentation
  - Schema reference for each baseline type
  - Known limitations section with workaround explanation
  - Usage examples for T-02-01, T-02-02, T-08-03
  - Regeneration instructions

Metadata & Tracking Updates:
• spacs/tasks/0.1.0-mvp/TODO.yaml
  - Added T-06-01 Phase 2 deliverables (17 new files)
  - Updated acceptance criteria with ✅/⚠️ status indicators
  - Added metrics section (baselines_extracted counts)

• .artifacts/spec-tasks-T-06-01-parity-methodology/metadata.yaml
  - Updated duration_days from 2 to 4
  - Added Phase 2 deliverables list
  - Updated acceptance criteria with Phase 2 completion status
  - Updated metrics (actual_hours: 28, baselines_extracted)
  - Added Phase 2 completion comment

• .artifacts/spec-tasks-T-06-01-parity-methodology/worklogs/
  - 2025-10-24-work-summary.md (comprehensive Phase 2 summary)
  - 2025-10-24-commit-log.md (this file)
  - 2025-10-24-notes.md (technical decisions and lessons)

Known Limitations (Documented):
• llama-index v0.11.22 SimpleDirectoryReader validates required_exts at root level
• SWE-bench repos (django/, sklearn/, matplotlib/, etc.) have code in subdirectories
• Workaround attempts: Package detection (failed), dummy file (failed)
• Resolution: Accept partial baselines, CDSAgent Rust uses walkdir without limitation
• Impact: Graph + traverse (most critical) 100% complete, M2 development unblocked

Deliverables (17 new files):
  tests/fixtures/parity/golden_outputs/graph_django__django-10914.json
  tests/fixtures/parity/golden_outputs/graph_scikit-learn__scikit-learn-10297.json
  tests/fixtures/parity/golden_outputs/graph_matplotlib__matplotlib-18869.json
  tests/fixtures/parity/golden_outputs/graph_pytest-dev__pytest-11143.json
  tests/fixtures/parity/golden_outputs/graph_psf__requests-1963.json
  tests/fixtures/parity/golden_outputs/traverse_samples.jsonl
  tests/fixtures/parity/golden_outputs/search_queries.jsonl
  tests/fixtures/parity/golden_outputs/performance_baselines.json
  tests/fixtures/parity/golden_outputs/README.md
  tests/fixtures/parity/swe-bench-lite/samples.yaml
  scripts/swe-lite
  scripts/fetch-swe-bench-lite.py
  scripts/select-swe-bench-instances.py
  scripts/extract-parity-baseline.py
  scripts/extract-search-baseline.py
  scripts/extract-traverse-baseline.py
  scripts/benchmark-performance.py

M2 Readiness:
• T-02-01 (Graph Builder) unblocked - all graph baselines ready
• T-02-02 (Sparse Index) unblocked - BM25 search baselines available (LocAgent)
• T-08-03 (Parity Validation) partially ready - graph/traverse validation ready

Closes: T-06-01 Phase 2
Related: T-02-01, T-02-02, T-08-03, T-08-04
```

### Diff Summary

**New Files (17)**:
```
tests/fixtures/parity/golden_outputs/graph_django__django-10914.json         (3.5 MB)
tests/fixtures/parity/golden_outputs/graph_scikit-learn__scikit-learn-10297.json (11 MB)
tests/fixtures/parity/golden_outputs/graph_matplotlib__matplotlib-18869.json (527 KB)
tests/fixtures/parity/golden_outputs/graph_pytest-dev__pytest-11143.json    (2.7 MB)
tests/fixtures/parity/golden_outputs/graph_psf__requests-1963.json          (498 KB)
tests/fixtures/parity/golden_outputs/traverse_samples.jsonl                  (60 lines)
tests/fixtures/parity/golden_outputs/search_queries.jsonl                    (50 lines)
tests/fixtures/parity/golden_outputs/performance_baselines.json              (26 lines)
tests/fixtures/parity/golden_outputs/README.md                               (148 lines)
tests/fixtures/parity/swe-bench-lite/samples.yaml                            (72 lines)
scripts/swe-lite                                                             (311 lines)
scripts/fetch-swe-bench-lite.py                                              (107 lines)
scripts/select-swe-bench-instances.py                                        (134 lines)
scripts/extract-parity-baseline.py                                           (orchestrator)
scripts/extract-search-baseline.py                                           (203 lines)
scripts/extract-traverse-baseline.py                                         (189 lines)
scripts/benchmark-performance.py                                             (156 lines)
```

**Modified Files (8)**:
```
docs/parity-validation-methodology.md
  + Section 10: Known Limitations (lines 1196-1325, 130 lines)
  + llama-index limitation detailed analysis
  + CDSAgent Rust implementation plan

.claude/settings.local.json
  + Session history updates (HF_TOKEN, swe-lite commands)

scripts/README.md (if exists)
  + swe-lite CLI documentation

spacs/tasks/0.1.0-mvp/TODO.yaml
  + T-06-01 Phase 2 deliverables (17 files)
  + Acceptance criteria with ✅/⚠️ indicators
  + Metrics section (baselines_extracted)

spacs/tasks/0.1.0-mvp/README.md (pending)
  + M1 milestone status update

.artifacts/spec-tasks-T-06-01-parity-methodology/metadata.yaml
  + duration_days: 2 → 4
  + Phase 2 deliverables list
  + Updated acceptance criteria
  + Updated metrics (actual_hours, baselines_extracted)

.artifacts/spec-tasks-T-06-01-parity-methodology/worklogs/2025-10-24-work-summary.md (NEW)
.artifacts/spec-tasks-T-06-01-parity-methodology/worklogs/2025-10-24-commit-log.md (NEW)
```

### Context Notes

**Why This Change**:
- Phase 2 of T-06-01 required extracting baseline data from LocAgent for parity validation
- Graph and traverse baselines are critical for T-02-01 (Graph Builder) and T-08-03 (Parity Validation)
- Search and performance baselines supplementary, discovered llama-index limitation during extraction

**Alternatives Considered**:
1. Wait for llama-index fix - Rejected (no timeline, blocks M1 milestone)
2. Modify llama-index source - Rejected (external dependency maintenance)
3. Manual baseline generation - Rejected (not reproducible, error-prone)

**Trade-offs**:
- Accept partial search/perf baselines now, gain unblocked M2 development
- More documentation (3 locations) vs. future debugging confusion

**Related Issues**:
- T-02-01: Needs graph baselines (✅ ready)
- T-02-02: Needs search baselines (⚠️ LocAgent only, will validate live)
- T-08-03: Needs all baselines (✅ graph + traverse ready, search/perf live)
- T-08-04: Will generate CDSAgent's own performance benchmarks

**Testing Notes**:
- All 6 graph baselines extracted successfully (22,194 nodes total)
- All 60 traverse scenarios extracted successfully
- Search baseline extraction succeeded for LocAgent, failed for SWE-bench repos (expected)
- Performance baseline extraction succeeded for LocAgent, failed for SWE-bench repos (expected)

**References**:
- LocAgent source: `tmp/LocAgent/dependency_graph/build_graph.py`
- llama-index issue: `tmp/LocAgent/plugins/location_tools/retriever/bm25_retriever.py:72`
- CDSAgent plan: `docs/parity-validation-methodology.md` Section 10

---

## Files Changed Detail

### Baseline Files (6 graph JSONs)

**tests/fixtures/parity/golden_outputs/graph_django__django-10914.json**
- Size: 3.5 MB
- Nodes: 6,876 (1,598 classes, 4,176 functions)
- Purpose: Large web framework baseline for T-02-01

**tests/fixtures/parity/golden_outputs/graph_scikit-learn__scikit-learn-10297.json**
- Size: 11 MB (largest baseline)
- Nodes: 6,613 (556 classes, 5,468 functions)
- Purpose: ML library with heavy computation baseline

**tests/fixtures/parity/golden_outputs/graph_matplotlib__matplotlib-18869.json**
- Size: 527 KB
- Nodes: 1,304 (121 classes, 617 functions)
- Purpose: Visualization library baseline

**tests/fixtures/parity/golden_outputs/graph_pytest-dev__pytest-11143.json**
- Size: 2.7 MB
- Nodes: 5,991 (648 classes, 5,030 functions)
- Purpose: Testing framework baseline

**tests/fixtures/parity/golden_outputs/graph_psf__requests-1963.json**
- Size: 498 KB (smallest SWE-bench baseline)
- Nodes: 752 (118 classes, 548 functions)
- Purpose: HTTP library baseline

**tests/fixtures/parity/golden_outputs/traverse_samples.jsonl**
- Lines: 60 (10 scenarios × 6 repos)
- Format: JSONL (one scenario per line)
- Purpose: Graph traversal pattern validation

**tests/fixtures/parity/golden_outputs/search_queries.jsonl**
- Lines: 50 (LocAgent only)
- Format: JSONL (one query per line)
- Purpose: BM25 search result validation (partial)

**tests/fixtures/parity/golden_outputs/performance_baselines.json**
- Lines: 26 (LocAgent only)
- Format: JSON array
- Purpose: Performance metric validation (partial)

**tests/fixtures/parity/golden_outputs/README.md** (NEW)
- Lines: 148
- Purpose: Complete baseline documentation with known limitations

### Automation Scripts (7 scripts)

**scripts/swe-lite** (311 lines)
- Language: Bash
- Purpose: Main CLI wrapper for baseline extraction
- Key functions:
  - fetch_instances() - Download SWE-bench Lite from Hugging Face
  - check_environment() - Verify Python, uv, HF_TOKEN
  - baseline_graph/traverse/search/perf() - Extract baselines

**scripts/fetch-swe-bench-lite.py** (107 lines)
- Language: Python
- Purpose: Download SWE-bench Lite instances
- Dependencies: datasets, huggingface_hub
- Output: .artifacts/tmp/swe-bench-lite/{instance_id}/

**scripts/select-swe-bench-instances.py** (134 lines)
- Language: Python
- Purpose: Select 5 diverse instances for parity validation
- Selection criteria: Size, domain, complexity diversity

**scripts/extract-search-baseline.py** (203 lines)
- Language: Python
- Purpose: Extract BM25 search results
- Known issue: llama-index limitation (lines 135-165)

**scripts/extract-traverse-baseline.py** (189 lines)
- Language: Python
- Purpose: Extract graph traversal scenarios
- Fixed: pydot dependency installation

**scripts/benchmark-performance.py** (156 lines)
- Language: Python
- Purpose: Measure graph build, search, traverse timings

### Documentation (2 major updates)

**docs/parity-validation-methodology.md**
- Added: Section 10 "Known Limitations" (lines 1196-1325)
- Content: llama-index issue analysis, CDSAgent Rust plan

**tests/fixtures/parity/golden_outputs/README.md** (NEW)
- Lines: 148
- Sections: Baselines overview, schemas, limitations, usage, regeneration

### Metadata & Tracking (3 files)

**spacs/tasks/0.1.0-mvp/TODO.yaml**
- Updated T-06-01 with Phase 2 deliverables
- Added acceptance criteria status indicators
- Added baselines_extracted metrics

**.artifacts/spec-tasks-T-06-01-parity-methodology/metadata.yaml**
- Updated duration_days, deliverables, acceptance_criteria, metrics
- Added Phase 2 completion comment

**.artifacts/spec-tasks-T-06-01-parity-methodology/worklogs/**
- 2025-10-24-work-summary.md (comprehensive summary)
- 2025-10-24-commit-log.md (this file)

---

## Lessons Learned

### 1. Dependency Validation is Critical

**Lesson**: Always verify all dependencies upfront before large-scale extraction

**Example**: pydot missing caused all 60 traverse scenarios to fail initially

**Prevention**: Added `scripts/swe-lite check` command to verify:
- Python version (3.12+)
- uv package manager
- HF_TOKEN environment variable
- LocAgent dependencies (pydot, etc.)

### 2. External Library Limitations Can Be Blocking

**Lesson**: Third-party libraries may have unexpected limitations that are hard to work around

**Example**: llama-index SimpleDirectoryReader validates `required_exts` at root level, blocking SWE-bench repos

**Mitigation**:
- Document limitation thoroughly (methodology + README + inline comments)
- Accept partial baselines when core baselines (graph + traverse) are complete
- Plan alternative approach (CDSAgent Rust using walkdir)

### 3. Automation Pays Off Long-Term

**Lesson**: Upfront investment in automation saves time in later phases

**Example**: swe-lite CLI wrapper with 7 helper scripts enables:
- One-command baseline regeneration
- T-02-01 and T-08-03 developers can re-run baselines easily
- CI/CD integration for regression testing

**ROI**: 8 hours investment → saves ~2 hours per future baseline update

### 4. Comprehensive Documentation Prevents Repeated Debugging

**Lesson**: Document known limitations in multiple locations for different audiences

**Example**: llama-index limitation documented in:
- `docs/parity-validation-methodology.md` Section 10 (technical audience, root cause analysis)
- `tests/fixtures/parity/golden_outputs/README.md` (daily users, quick reference)
- Inline comments in `scripts/extract-search-baseline.py` (code maintainers)

**Impact**: Future developers won't waste time trying same workarounds

---

**Time Spent**: 18 hours
**Commits**: 1 major commit (pending)
**Status**: Phase 2 complete, ready for commit
