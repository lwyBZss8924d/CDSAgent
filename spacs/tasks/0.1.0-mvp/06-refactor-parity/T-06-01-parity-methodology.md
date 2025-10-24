# Task T-06-01: LocAgent Parity Validation Methodology

**Issue**: [Refactor Parity](../../issues/04-0.1.0-mvp/06-refactor-parity.md)

**PRD References**: [PRD-06 §2-5](../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md), [PRD-01 §3.2](../../prd/0.1.0-MVP-PRDs-v0/01-system-architecture-scope.md)

**Owner**: Rust Lead (with All Rust Developers)

**Status**: ✅ Completed | **Week**: 1 (Target: 2025-10-23, Started: 2025-10-20)

**PR Link**: [PR #4](https://github.com/lwyBZss8924d/CDSAgent/pull/4)

---

## Objective

Establish a comprehensive parity validation methodology to ensure CDSAgent's Rust implementation maintains algorithmic fidelity with LocAgent's Python codebase while achieving 2-5x performance improvements. This methodology will serve as the foundation for continuous validation across all Rust crates during the 0.1.0-MVP development.

## Deliverables

- `docs/parity-validation-methodology.md` - Comprehensive SOP for parity validation
- `scripts/parity-check.sh` - Automation script for running parity checks
- `tests/fixtures/parity/` - Test fixture directory structure with golden outputs
  - `locagent_repo/` - LocAgent's own codebase (ground truth)
  - `sample_repos/` - 5 repos from SWE-bench Lite
  - `golden_outputs/` - Graph, search, traverse baselines from LocAgent
  - `locagent_version.txt` - Version tracking for baseline
  - `README.md` - Instructions for regenerating golden outputs

## Implementation Steps

### Phase 1: LocAgent Baseline Extraction (Day 1, 4 hours)

1. **Setup LocAgent Environment**
   - Verify Python 3.12 environment
   - Install LocAgent dependencies from `tmp/LocAgent/requirements.txt`
   - Set required environment variables (`PYTHONPATH`, `GRAPH_INDEX_DIR`, `BM25_INDEX_DIR`)

2. **Select Sample Repositories**
   - Choose 5 small-to-medium Python repos from SWE-bench Lite
   - Clone to `tests/fixtures/parity/sample_repos/`
   - Document repo metadata (name, commit hash, file count, LOC)

3. **Extract Golden Outputs**
   - Run LocAgent graph builder on each sample repo
   - Capture graph structure (node/edge counts by type, qualified names)
   - Run 50 benchmark search queries, record top-10 results per query
   - Run 10 traversal scenarios, record complete outputs
   - Document LocAgent commit hash and Python version used

### Phase 2: Methodology Documentation (Day 1-2, 8 hours)

1. **Write Comprehensive Methodology Guide** (`docs/parity-validation-methodology.md`)
   - **§1**: Module-by-module mapping (LocAgent → CDSAgent, from PRD-06 §2)
   - **§2**: Algorithm preservation checklist
     - Graph construction (directory traversal, AST parsing, entity extraction, edge creation)
     - BM25 indexing (tokenization, stop words, parameters, ranking)
     - Graph traversal (BFS algorithm, filters, output format)
   - **§3**: Output format preservation
     - Fold snippet format (`"{type} {name} - {file}:{line}"`)
     - Preview snippet (signature + first 5 lines)
     - Tree format (`├─[relation]→ Entity`)
   - **§4**: Performance validation targets (from PRD-06 §5.3)
     - Index 1K files: <3s (LocAgent baseline: ~5s)
     - Search query: <100ms (LocAgent baseline: ~200ms)
     - Traverse 2-hop: <200ms (LocAgent baseline: ~500ms)
     - Memory (10K files): <2GB (LocAgent baseline: ~3GB)
   - **§5**: Unit test coverage targets (>95% for core crates)
   - **§6**: Continuous validation strategy (phase-gated checks)
   - **§7**: Automated regression tests (run on every PR)

2. **Document Validation Gates**
   - Phase 1 Checkpoint (Week 2): Graph construction parity
   - Phase 2 Checkpoint (Week 5): BM25 search + traversal parity
   - Phase 3 Checkpoint (Week 7): Performance targets achieved
   - Phase 4 Checkpoint (Week 10): Full SWE-bench Lite parity (Acc@5 ≥80%)

3. **Define Parity Metrics**
   - Graph variance threshold: ≤2% from LocAgent
   - Search overlap@10: ≥90% on 50 queries
   - Traversal exact match: 10/10 samples
   - Performance speedup: 2-5x over Python baseline

### Phase 3: Automation Script (Day 2, 4 hours)

1. **Create `scripts/parity-check.sh`**
   - Input: Test type (graph, search, traverse, performance, all)
   - Load golden outputs from `tests/fixtures/parity/golden_outputs/`
   - Run corresponding CDSAgent Rust tests
   - Compare outputs with tolerance thresholds
   - Output: Pass/fail with detailed diff report

2. **Script Features**
   - Colored CLI output (✅ pass, ❌ fail, ⚠️ warning)
   - JSON diff for structured comparison
   - Performance metrics reporting (with comparison to baseline)
   - Exit code 0 for pass, 1 for fail (CI/CD integration)

3. **Integration with CI**
   - Document how to integrate into GitHub Actions
   - Fail PR if parity drops below thresholds
   - Weekly cron job to validate against latest LocAgent

### Phase 4: Test Fixtures Setup (Day 2, 2 hours)

1. **Create Directory Structure**

   ```tree
   tests/fixtures/parity/
   ├── locagent_repo/           # LocAgent's own codebase
   ├── sample_repos/            # 5 repos from SWE-bench Lite
   ├── golden_outputs/
   │   ├── graph_locagent.json  # Expected graph structure
   │   ├── search_queries.jsonl # 50 queries + expected top-10
   │   └── traverse_samples.jsonl # 10 traversal cases + outputs
   ├── locagent_version.txt     # LocAgent commit hash + Python version
   └── README.md                # Regeneration instructions
   ```

2. **Write Fixture README**
   - How to regenerate golden outputs
   - Required LocAgent environment setup
   - Commands to run for each fixture type
   - Versioning guidelines (when to update baselines)

### Phase 5: Documentation Integration (Day 2, 1 hour)

1. **Link to Related Documentation**
   - Update Issue-06 with methodology link
   - Reference from PRD-06
   - Add to DEVELOPMENT_STATUS.md under "Validation Gates"

2. **Create PR with Complete Methodology**
   - Branch: `feat/task/T-06-01-parity-methodology`
   - PR title: "feat(validation): T-06-01 - LocAgent Parity Validation Methodology"
   - PR body: Link to deliverables, acceptance criteria checklist

## Acceptance Criteria

### Phase 1: Documentation & Infrastructure (Completed 2025-10-20)

- [x] `docs/parity-validation-methodology.md` published with comprehensive SOP
- [x] `scripts/parity-check.sh` automation script functional and tested
- [x] `tests/fixtures/parity/` directory structure created
- [x] LocAgent version tracked in `locagent_version.txt`
- [x] All LocAgent Python modules mapped to Rust equivalents (from PRD-06 §2)
- [x] Performance targets documented (index, search, traverse, memory)
- [x] Parity gates defined for Phases 1-4
- [x] README.md includes instructions for regenerating baselines

### Phase 2: Baseline Extraction (In Progress - Target: 2025-10-23)

- [x] LocAgent baseline data extracted for 5 sample repos
- [x] 50 search queries documented with expected top-10 results
- [x] 10 traversal scenarios documented with expected outputs
- [x] Golden outputs populated in `tests/fixtures/parity/golden_outputs/`

Current status & known defects:

- [P1] Guard llama-index retrieval build — scripts/benchmark-performance.py:82-83
When benchmark_search runs against several SWE-bench Lite repos (e.g. django__django-10914 or scikit-learn__scikit-learn-10297), the call to build_code_retriever_from_repo raises ValueError: No files found with required_exts=['.py'] because llama-index's SimpleDirectoryReader insists on seeing a .py directly under the repo root. You already added a dummy-file workaround for this limitation in extract-search-baseline.py, but this script reuses the same retriever without that guard, so swe-lite baseline perf currently dies on those repos. Please apply the same workaround (create and clean up the dummy file before building the retriever) so the performance baseline flow succeeds.

---

## Phase 2 Rework Plan (2025-10-24)

**Goal**: Fully satisfy the outstanding Phase 2 acceptance criteria using *SWE-bench Lite* repositories, align artifacts with LocAgent methodology, and resolve review findings (missing baselines, worklogs, tracked scripts).

### 🎯 Scope & Success Criteria

- ✅ Five *distinct* repositories drawn from the official *SWE-bench Lite* split ([FAQ](https://www.swebench.com/SWE-bench/faq/?utm_source=openai)), each referenced by dataset instance ID, repository URL, and commit hash.
- ✅ Golden outputs captured for:
  - Graph structure (node + edge listings suitable for diffing).
  - Search queries (50 diversified queries × 6 repos [LocAgent + 5 samples]).
  - Graph traversals (10 scenarios × 6 repos).
  - Performance metrics (build/search/traverse timings, memory).
- ✅ No vendored source trees in the repository: sample repos fetched on demand via scripts; only derived fixtures (JSON/JSONL) committed.
- ✅ Scripts relocated to `scripts/` and versioned; worklogs and metadata updated to document 24 Oct activities.

### 📚 Data Source & Selection Procedure

1. Load the *SWE-bench* dataset via Hugging Face (`datasets.load_dataset("swe-bench", "lite")`) or the SWE-bench harness.
2. Filter for Python-focused tasks (language metadata) and diversity (framework, project size, dependency graph). Record the chosen instance IDs.
3. For each selected instance, extract:
   - `repo` URL (e.g., `https://github.com/<owner>/<project>`).
   - `base_commit` or `patch_commit` hash for reproducibility.
   - Optional metadata (issue title, label) for documentation.

Document the selection table inside `locagent_version.txt` (or a companion `SWE-bench-lite-samples.md`) with instance ID, repo, commit, file count, and rationale.

### 📂 Target Directory Layout

```tree
tests/fixtures/parity/
├── README.md                        # Updated regeneration guide (Hugging Face workflow)
├── locagent_version.txt             # Extended with SWE-bench Lite provenance
├── swe-bench-lite/                  # NEW: metadata only, no source
│   ├── samples.yaml                 # Instance IDs + repo info
│   └── download.sh                  # Helper script (optional)
└── golden_outputs/
    ├── graph_locagent.json          # Re-extracted with nodes & edges
    ├── graph_<instance>.json        # 5× SWE-bench Lite graphs
    ├── search_queries.jsonl         # 50 queries × 6 repos
    ├── traverse_samples.jsonl       # 10 scenarios × 6 repos
    └── performance_baselines.json   # Timing + memory metrics
```

> **Note**: raw repository checkouts live in ephemeral scratch space (e.g., `.artifacts/tmp/`) and are excluded from git; regeneration instructions describe how to fetch them when needed.

### 🔧 Implementation Steps

Step 1 – Script Toolkit (1.5 h)

- Move `tmp/extract-locagent-baseline.py` → `scripts/extract-parity-baseline.py`, extend to emit full node/edge arrays and support file-count limiting.
- Add:
  - `scripts/extract-search-baseline.py` – builds BM25 index & runs 50 queries.
  - `scripts/extract-traverse-baseline.py` – executes 10 traversal scenarios.
  - `scripts/benchmark-performance.py` – measures build/search/traverse timings + memory (psutil).
- Provide README usage snippets and ensure scripts guard against missing Hugging Face credentials.

Step 2 – Acquire SWE-bench Lite Samples (1.5 h)

- Write `scripts/fetch-swe-bench-lite.py` (or bash equivalent) that:
  - accepts dataset instance IDs,
  - clones the referenced repo at the specified commit into a temp directory,
  - prunes to Python files if needed.
- Log selected IDs + metadata to `tests/fixtures/parity/swe-bench-lite/samples.yaml`.
- Confirm each local clone stays outside git tracking (e.g., `.artifacts/tmp/swe-bench-lite/<id>/`).

Step 3 – Generate Baselines (4–5 h)

- Run `extract-parity-baseline.py` for:
  - `tmp/LocAgent`
  - each SWE-bench Lite sample (optionally limit to first 500 Python files for very large repos; record limit in output metadata).
- Execute search/traverse scripts once per repo; append results to shared JSONL files.
- Collect performance metrics sequentially to manage memory.
- Validate outputs (`jq`, schema checks, consistency between counts and lists).

Step 4 – Documentation & Logs (1 h)

- Update:
  - `tests/fixtures/parity/README.md` with Hugging Face workflow, regeneration commands, data provenance.
  - `locagent_version.txt` with SWE-bench Lite selections, commit hashes, extraction timestamps, script versions.
  - `.artifacts/spec-tasks-T-06-01-parity-methodology/metadata.yaml` acceptance notes and metrics.
- Create worklog entries: `2025-10-24-{work-summary,commit-log,notes}.md` (and 2025-10-25 if spillover).

Step 5 – Commits & Tracking (0.5 h)

- Commit scripts, golden outputs, documentation, worklogs, and TODO metadata updates.
- Ensure `.gitignore` still excludes raw repos while allowing generated JSON/JSONL.

### 📊 Timeline & Effort

| Phase | Task | Est. Duration |
|-------|------|---------------|
| 1     | Script toolkit | 1.5 h |
| 2     | SWE-bench Lite selection & download | 1.5 h |
| 3     | Baseline generation | 4–5 h |
| 4     | Docs & worklogs | 1 h |
| 5     | Commits & verification | 0.5 h |
| **Buffer** | Troubleshooting / reruns | 1.5 h |
| **Total** | | **9–11 h** |

### ✅ Deliverables Checklist

- `scripts/extract-parity-baseline.py`
- `scripts/extract-search-baseline.py`
- `scripts/extract-traverse-baseline.py`
- `scripts/benchmark-performance.py`
- `tests/fixtures/parity/swe-bench-lite/samples.yaml`
- `tests/fixtures/parity/golden_outputs/graph_locagent.json` (full listings)
- `tests/fixtures/parity/golden_outputs/graph_<instance>.json` ×5
- `tests/fixtures/parity/golden_outputs/search_queries.jsonl`
- `tests/fixtures/parity/golden_outputs/traverse_samples.jsonl`
- `tests/fixtures/parity/golden_outputs/performance_baselines.json`
- Updated `tests/fixtures/parity/README.md`
- Updated `tests/fixtures/parity/locagent_version.txt`
- Worklogs for 2025-10-24 (and 25 if used)
- Refreshed `.artifacts/.../metadata.yaml`, `spacs/tasks/0.1.0-mvp/TODO.yaml`

### ⚠️ Risks & Mitigations

- **Large repo size / runtime** – use `--max-files` filter and document subset.
- **Dataset access** – cache Hugging Face authentication and note environment requirements.
- **Memory pressure** – process repos sequentially; release graph objects (`del graph`, `gc.collect()`).
- **License compliance** – avoid committing third-party source; keep only metadata and derived artifacts.

Once these deliverables are merged, all review findings from 2025-10-24 are resolved, and Phase 2 acceptance criteria can be marked complete.

## Dependencies

- **Requires**: LocAgent repository access (`tmp/LocAgent/`)
- **Blocks**:
  - T-02-01 (Graph Builder) - Cannot start without parity methodology
  - T-02-02 (Sparse Index) - Needs search validation approach
  - T-08-03 (Parity Validation) - Needs methodology and baselines

## Notes

- This is a **cross-cutting concern** that validates work across all Rust crates
- Parity methodology is **living documentation** - update as we discover new validation needs
- Use LocAgent's `.scm` tree-sitter query files verbatim (copy from `tmp/LocAgent/repo_index/codeblocks/parser/queries/`)
- Accept BM25 scores within 0.01 tolerance; verify rank order is identical
- Document any intentional deviations from LocAgent with rationale

---

## Commit Message Template

```text
feat(validation): T-06-01 - Establish LocAgent parity validation methodology

Implement comprehensive parity validation methodology and automation to ensure
CDSAgent's Rust refactoring maintains algorithmic fidelity with LocAgent while
achieving 2-5x performance improvements.

## Methodology Documentation

- docs/parity-validation-methodology.md: Complete SOP for parity validation
  - Module-by-module mapping (LocAgent → CDSAgent)
  - Algorithm preservation checklist (graph, BM25, traversal)
  - Output format preservation rules (fold, preview, tree)
  - Performance validation targets (<3s index, <100ms search, <200ms traverse)
  - Unit test coverage targets (>95% for core crates)
  - Phase-gated checkpoints (Week 2, 5, 7, 10)
  - Automated regression tests (run on every PR)

## Automation Script

- scripts/parity-check.sh: Automation for running parity checks
  - Compare graph structure, search results, traversal outputs
  - Performance metrics reporting with baseline comparison
  - Colored CLI output (✅/❌/⚠️)
  - CI/CD integration (exit code 0/1)

## Test Fixtures & Baselines

- tests/fixtures/parity/: Golden outputs from LocAgent
  - 5 sample repos from SWE-bench Lite
  - Graph baselines (node/edge counts, qualified names)
  - Search baselines (50 queries with top-10 results)
  - Traversal baselines (10 scenarios with outputs)
  - LocAgent version tracking (commit hash + Python version)

## Validation Targets

- Graph variance: ≤2% from LocAgent
- Search overlap@10: ≥90% on 50 queries
- Traversal exact match: 10/10 samples
- Performance speedup: 2-5x over Python baseline

## Acceptance Criteria Met

- [x] Methodology documentation published
- [x] Automation script functional
- [x] Golden outputs extracted
- [x] Parity gates defined for all phases
- [x] All LocAgent modules mapped to Rust

Blocks: T-02-01 (Graph Builder), T-02-02 (Sparse Index), T-08-03 (Parity Validation)
Related: PRD-06 §2-5, Issue-06
```
