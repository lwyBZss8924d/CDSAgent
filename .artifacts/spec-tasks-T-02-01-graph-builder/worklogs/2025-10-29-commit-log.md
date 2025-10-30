# Commit Log - 2025-10-29

**Task**: T-02-01-graph-builder  
**Author**: Rust Dev 1

---

## Commits Made Today

```table
| # | Commit | Title | git notes Title | Timestamp (UTC) | Status |
|---|--------|-------|-----------------|-----------------|--------|
| – | – (pending) | fix(parity): align SWE fixtures with TYPE_CHECKING import semantics | Day 5 Sessions 3-03 to 3-07 | – | 🚧 |
```

> **Status**: Code changes completed but not yet committed. Changes remain uncommitted locally pending final invoke variance audit (+1.29% remaining).

## Planned Commit Details

### Commit Message

```text
fix(parity): align SWE fixtures with TYPE_CHECKING import semantics

Day 5 (Sessions 3-03 to 3-07): Scoped TYPE_CHECKING & SWE parity improvements

- Extended find_in_block() to descend into if TYPE_CHECKING blocks
  - Restores pytest mark decorator inherits (0% variance)
  - Enables discovery of type-only class definitions
- Updated process_from_import() fallback for grouped imports
  - Emits alias-labelled file edges for each entity in from ... import ...
  - Restores Django SWE fixture import parity (0% variance)
  - Propagates aliases to scoped entities (class/function bodies)
- Removed ad-hoc PARITY_DEBUG traces
- Updated parity harness diagnostics

Parity Results:
- All fixtures ≤2% variance (within tolerance)
- Imports: 0% variance (exact match)
- Inherits: 0% variance (exact match)
- Invokes: pytest +1.29% (2442/2474), requests +0.46%, scikit-learn +0.09%

Files changed: 6 code files, +1,072/-140 lines
```

### Files Changed

**Code Files** (7 total):

1. `crates/cds-index/src/graph/builder/behaviors.rs` - Fallback textual scanner cleanup, scoped import handling
2. `crates/cds-index/src/graph/builder/imports.rs` - Alias-labelled file edges, scoped entity propagation
3. `crates/cds-index/src/graph/builder/python/ast_utils.rs` - TYPE_CHECKING block traversal in find_in_block()
4. `crates/cds-index/src/graph/builder/state.rs` - Scope stack threading for ImportDirective
5. `crates/cds-index/src/graph/mod.rs` - Export declarations for new graph builder submodules
6. `crates/cds-index/src/graph/parser.rs` - ImportDirective scope updates
7. `crates/cds-index/tests/graph_parity_tests.rs` - Enhanced diagnostics, removed debug prints

**Statistics**:

- **Lines added**: +1,072
- **Lines deleted**: -140
- **Net change**: +932 lines

### Validation

```shell
# All tests passing with parity within tolerance
PARITY_FIXTURE=pytest-dev__pytest-11143 PARITY_DEBUG=1 \
  cargo test -p cds-index --test graph_parity_tests -- graph_parity_baselines --nocapture

PARITY_FIXTURE=django__django-10914 PARITY_DEBUG=1 \
  cargo test -p cds-index --test graph_parity_tests -- graph_parity_baselines --nocapture

cargo test -p cds-index --test graph_parity_tests -- graph_parity_baselines --nocapture
```

**Result**: ✅ All fixtures pass with ≤2% variance

## Notes

- **Commit Pending**: Awaiting final audit of remaining pytest invoke variance (+1.29%, 32 edges)
- **Rationale**: Changes are functionally complete and all parity tests pass, but documentation review pending
- **Next Step**: Audit remaining invoke gaps in `_pytest/config` and `_pytest/monkeypatch` helpers before finalizing commit
- **PR Status**: Remains in draft pending code commit
