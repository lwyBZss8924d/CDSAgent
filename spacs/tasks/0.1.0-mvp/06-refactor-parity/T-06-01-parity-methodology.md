# Task T-06-01: LocAgent Parity Validation Methodology

**Issue**: [Refactor Parity](../../issues/04-0.1.0-mvp/06-refactor-parity.md)

**PRD References**: [PRD-06 §2-5](../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md), [PRD-01 §3.2](../../prd/0.1.0-MVP-PRDs-v0/01-system-architecture-scope.md)

**Owner**: Rust Lead (with All Rust Developers)

**Status**: ☐ Not Started | **Week**: 1 (Target: 2025-10-21)

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
   ```
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

- [ ] `docs/parity-validation-methodology.md` published with comprehensive SOP
- [ ] `scripts/parity-check.sh` automation script functional and tested
- [ ] `tests/fixtures/parity/` directory created with golden outputs
- [ ] LocAgent baseline data extracted for 5 sample repos
- [ ] 50 search queries documented with expected top-10 results
- [ ] 10 traversal scenarios documented with expected outputs
- [ ] LocAgent version tracked in `locagent_version.txt`
- [ ] All LocAgent Python modules mapped to Rust equivalents (from PRD-06 §2)
- [ ] Performance targets documented (index, search, traverse, memory)
- [ ] Parity gates defined for Phases 1-4
- [ ] README.md includes instructions for regenerating baselines

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
