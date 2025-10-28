# Work Summary - 2025-10-28

**Task**: T-02-01-graph-builder - Graph Builder - AST Parsing & Construction
**Date**: 2025-10-28
**Author**: Rust Dev 1

---

## Today's Objectives

- [x] Implement multi-target alias resolution (Vec `<GraphNodeIndex>`)
- [x] Add resolve_targets() method for multi-alias scenarios
- [x] Implement PendingWildcardExport deferred handling
- [x] Add unit test for multi-target invoke edge attachment
- [ ] Audit extra invoke edges (565 vs 531)
- [ ] Align call resolution with LocAgent heuristics
- [ ] Achieve ≤2% variance on invoke edges

## Work Completed

### Session 1: Multi-Target Alias Resolution (03:24:29Z)

**Overview**: Rewrote alias map infrastructure to support multiple targets per alias, enabling invoke edges to attach to all reachable definitions. Added deferred wildcard export handling to queue re-export expansion until explicit `__all__` data is available.

**Multi-Target Alias Map** (crates/cds-index/src/graph/builder.rs, +150 lines, -25 lines):

- **Alias Map Refactoring**: Changed from `HashMap<String, GraphNodeIndex>` to `HashMap<String, Vec<GraphNodeIndex>>`
  - Supports multiple definitions for same name (e.g., `merge` in multiple modules)
  - `insert_alias()` now deduplicates entries within each Vec
  - `build_alias_map()` collects all candidates from imports and wildcard exposures

- **resolve_targets() Method**: New unified target resolution for invoke/inherit edges
  - Checks alias_map first (imported/wildcard names)
  - Falls back to file_symbols (local definitions)
  - Returns `Vec<GraphNodeIndex>` of all candidates
  - Used by `connect_behavior_edges()` to wire all possible targets

- **PendingWildcardExport Struct**: Queue wildcard re-export expansion
  - Fields: `source_idx: GraphNodeIndex`, `module_path: PathBuf`
  - Enqueued when wildcard import found but target module's `__all__` not yet parsed
  - `resolve_pending_wildcard_exports()` processes after all files indexed
  - Retry logic with max 4 attempts to handle cyclic dependencies

- **Wildcard Export Edge Addition**: `add_wildcard_export_edges()` improvement
  - Checks for explicit `__all__` or wildcard sources in target module
  - Resolves effective exports via `resolve_exports()`
  - Creates import edges for each exported name
  - Returns boolean indicating success (true if any edges added)

**Behavior Edge Wiring** (crates/cds-index/src/graph/builder.rs:1076-1097):

- **connect_behavior_edges() Update**: Iterate over all targets from resolve_targets()
  - Uses `seen_targets: HashSet<GraphNodeIndex>` to prevent duplicate edges
  - Checks `behavior_edge_cache` before adding (global dedup across all calls)
  - Ensures each (caller, callee, kind) triple added exactly once

**Unit Test** (crates/cds-index/tests/graph_builder_tests.rs:420-480, +61 lines):

- **invoke_edges_include_all_alias_candidates** (61 lines)
  - Validates multi-alias imports attach invoke edges to all definitions
  - Scenario: two modules both export `merge()` function, client imports both
  - Client calls `merge()` → edges to **both** function definitions
  - Test setup: pkg/a.py::merge, pkg/b.py::merge, client imports both
  - Assertion: client::main has invoke edges to both a::merge and b::merge

**All Tests Passing**: ✅ cargo test --test graph_builder_tests (8 tests green)

## Code Changes

### Files Modified

1. **crates/cds-index/src/graph/builder.rs** (+150 lines, -25 lines)
   - Refactored alias map to Vec `<GraphNodeIndex>`
   - Added resolve_targets() method (lines 1099-1119)
   - Added PendingWildcardExport struct and processing (lines 102-106, 755-779)
   - Updated connect_behavior_edges() to iterate over all targets (lines 1076-1097)
   - Added add_wildcard_export_edges() enqueuing logic (lines 531-533)
   - Modified add_attribute_import_edge() return type to bool (line 607)

2. **crates/cds-index/tests/graph_builder_tests.rs** (+61 lines)
   - Added invoke_edges_include_all_alias_candidates test (lines 420-480)

### Statistics

- **Lines Added**: 186 (builder: +150, tests: +61, action log: ~7)
- **Lines Deleted**: 25 (builder refactoring)
- **Net Change**: +161 lines
- **Tests Added**: 1 (invoke_edges_include_all_alias_candidates)
- **Total Tests**: 8 (all passing)
- **Test Coverage**: ~30% (estimated, up from ~25%)

## Parity Results (Day 4)

**Command**: `cargo test --test graph_parity_tests -- graph_parity_baselines --nocapture`

**LocAgent Fixture** (658 nodes, 1,419 edges):

| Edge Type | Expected (LocAgent) | Actual (CDSAgent) | Variance | Status |
|-----------|---------------------|-------------------|----------|--------|
| Contains  | 657                 | 657               | 0%       | ✅ Exact |
| Imports   | 218                 | 218               | 0%       | ✅ Exact |
| Inherits  | 13                  | 13                | 0%       | ✅ Exact |
| Invokes   | 531                 | 565               | **+6.4%** | ⚠️ Over |

**Analysis**:

- **Imports Maintained**: 0% variance (exact match 218/218) - Day 3 fix still holding ✅
- **Invokes Increased**: +6.4% variance (up from +1.9% yesterday)
  - Extra 34 invoke edges discovered by multi-target resolution
  - Examples from PARITY_DEBUG output:
    - `auto_search_main.py::main → auto_search_main.py::merge` (self-module call)
    - `dependency_graph/batch_build_graph.py::run → util/benchmark/setup_repo.py::setup_repo`
  - Root cause: CDSAgent now discovers **all** reachable targets per alias, while LocAgent's `find_all_possible_callee` may have more conservative heuristics

**Key Decisions**:

1. **Trade-off Accepted**: Temporarily increased invoke variance to ensure completeness
   - Multi-target resolution prevents missing edges (false negatives)
   - Next step: audit extra edges to identify filtering criteria

2. **Filtering Strategy**: Align with LocAgent's call resolution heuristics
   - Review `find_all_possible_callee()` in LocAgent (tmp/LocAgent/dependency_graph/build_graph.py)
   - May need to filter self-recursive calls (same file)
   - May need distance/scope constraints (e.g., only same package imports)

## Challenges & Solutions

### Challenge 1: Alias Map Type Change

**Problem**: Original `HashMap<String, GraphNodeIndex>` only stored single target per alias, causing missing edges when multiple modules export same name.

**Solution**:

- Changed to `HashMap<String, Vec<GraphNodeIndex>>`
- Added `insert_alias()` helper with deduplication
- Updated all alias map consumers to iterate over Vec

**Trade-off**: Slightly more complex code, but ensures no false negatives (missing edges)

### Challenge 2: Invoke Variance Increased

**Problem**: Multi-target resolution discovers 34 more invoke edges than LocAgent, increasing variance from +1.9% to +6.4%.

**Solution** (next steps):

- Audit extra edges via PARITY_DEBUG output
- Compare with LocAgent's `find_all_possible_callee` implementation
- Add filtering heuristics (e.g., self-recursion, cross-package constraints)

## Next Steps

### Immediate (Day 5 - Tomorrow)

1. **Audit Extra Invoke Edges** (~2 hours)
   - Enable PARITY_DEBUG and capture full edge diff
   - Categorize extra edges: self-recursive, cross-package, legitimate
   - Identify common patterns for filtering

2. **Implement Call Resolution Heuristics** (~4 hours)
   - Mirror LocAgent's `find_all_possible_callee` logic
   - Add filters: self-recursion, package boundaries, import distance
   - Re-run parity tests after each filter

3. **Achieve ≤2% Variance** (~2 hours)
   - Iterate filters until variance ≤2%
   - Update unit tests if behavior changes
   - Document filtering rationale in notes

4. **Expand Unit Test Coverage** (ongoing)
   - Target >80% coverage (currently ~30%)
   - Add tests for filtering heuristics
   - Property-based tests for graph invariants

5. **Run Parity on SWE-bench Fixtures** (1-2 days)
   - Validate on 5 larger repos (django, scikit-learn, etc.)
   - Ensure variance ≤2% across all fixtures
   - Document any repo-specific edge cases

## Acceptance Criteria Progress

- [x] **4 node + 4 edge types**: All implemented and tested
- [x] **FQN format**: Matches LocAgent (validated via parity)
- [⏳] **Unit tests**: 8 tests passing, ~30% coverage (needs >80%)
- [⏳] **Parity variance**: Imports 0% ✅, Invokes +6.4% ⚠️ (target ≤2%)

## Time Tracking

- **Session 1 (03:24:29Z)**: ~4 hours (multi-target alias implementation + testing)
- **Total Day 4**: 4 hours
- **Cumulative**: 19.5 hours (Day 1: 2h, Day 2: 11h, Day 3: 2.5h, Day 4: 4h)
- **Remaining**: 20.5 hours (estimate: 2-3 days to complete parity + expand tests)

---

**Status**: In Progress - Day 4 Complete
**Next Session**: Day 5 - Audit extra invoke edges and implement filtering heuristics
