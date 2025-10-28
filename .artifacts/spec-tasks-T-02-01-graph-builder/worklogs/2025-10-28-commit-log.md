# Commit Log - 2025-10-28

**Task**: T-02-01-graph-builder
**Author**: Rust Dev 1

---

## Commit 1: feat(graph): T-02-01 Day 4 - multi-target alias resolution and wildcard export handling

**Hash**: `147b4c2`
**Date**: 2025-10-28
**Branch**: feat/task/T-02-01-graph-builder

### Summary

Implemented multi-target alias resolution to ensure invoke edges attach to all reachable definitions when multiple modules export the same name. Added deferred wildcard export handling via PendingWildcardExport queue.

### Files Changed (19 files)

**Code**:

- `crates/cds-index/src/graph/builder.rs` (+150, -25)
- `crates/cds-index/tests/graph_builder_tests.rs` (+61, -0)

**Artifacts**:

- `.artifacts/spec-tasks-T-02-01-graph-builder/metadata.yaml` (updated metrics, acceptance criteria)
- `.artifacts/spec-tasks-T-02-01-graph-builder/worklogs/2025-10-28-work-summary.md` (new)
- `.artifacts/spec-tasks-T-02-01-graph-builder/worklogs/2025-10-28-notes.md` (new)
- `.artifacts/spec-tasks-T-02-01-graph-builder/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-2025-10-28-01.txt` (new)
- `.artifacts/spec-tasks-T-02-01-graph-builder/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-2025-10-27-01.txt` (updated)

**Documentation**:

- `docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md` (restructured into modular chapters)
- `docs/checkpoint/01-overview.md` (new)
- `docs/checkpoint/02-phases-overview.md` (new)
- `docs/checkpoint/03-phase1-review.md` (new)
- `docs/checkpoint/04-phase2-verification.md` (new)
- `docs/checkpoint/05-phase3-update.md` (new)
- `docs/checkpoint/06-phase4-git.md` (new)
- `docs/checkpoint/07-phase5-final.md` (new)
- `docs/checkpoint/08-common-issues.md` (new)
- `docs/checkpoint/09-template.md` (new)
- `docs/checkpoint/10-example.md` (new)
- `docs/checkpoint/11-commands.md` (new)

### Statistics

- **Total Changes**: +3,349 insertions, -2,385 deletions
- **Net Change**: +964 lines
- **Code Changes**: +186 insertions, -25 deletions (builder + tests)
- **Documentation**: +2,419 insertions, -2,377 deletions (checkpoint workflow split)
- **Artifacts**: +744 insertions, +17 deletions (worklogs + metadata)

### Diff Summary (Code)

#### builder.rs (+150, -25)

**New Structures**:

```rust
// Line 102-106
struct PendingWildcardExport {
    source_idx: GraphNodeIndex,
    module_path: PathBuf,
}
```

**State Fields**:

```rust
// Line 201
pending_wildcard_exports: Vec<PendingWildcardExport>,
```

**Multi-Target Alias Resolution**:

```rust
// Lines 961-997: insert_alias() now handles Vec<GraphNodeIndex>
fn insert_alias(
    aliases: &mut HashMap<String, Vec<GraphNodeIndex>>,
    name: String,
    target: GraphNodeIndex,
) {
    if name.is_empty() {
        return;
    }
    let entry = aliases.entry(name).or_default();
    if !entry.contains(&target) {
        entry.push(target);
    }
}

// Lines 1099-1119: New resolve_targets() method
fn resolve_targets(
    &self,
    rel_path: &Path,
    alias_map: &HashMap<String, Vec<GraphNodeIndex>>,
    name: &str,
) -> Vec<GraphNodeIndex> {
    let mut result = Vec::new();
    if let Some(entries) = alias_map.get(name) {
        result.extend(entries.iter().copied());
    }
    if let Some(symbols) = self.file_symbols.get(rel_path) {
        if let Some(&idx) = symbols.get(name) {
            if !result.contains(&idx) {
                result.push(idx);
            }
        }
    }
    result
}
```

**Wildcard Export Handling**:

```rust
// Lines 566-588: add_wildcard_export_edges()
fn add_wildcard_export_edges(
    &mut self,
    source_idx: GraphNodeIndex,
    module_path: &Path,
) -> bool {
    let has_explicit_exports = self
        .module_exports
        .get(module_path)
        .map(|info| !info.names.is_empty() || !info.sources.is_empty())
        .unwrap_or(false);
    if !has_explicit_exports {
        return false;
    }

    let exports = self.resolve_exports(module_path);
    let mut added = false;
    for name in exports {
        if self.add_attribute_import_edge(source_idx, module_path, &name, None) {
            added = true;
        }
    }
    added
}

// Lines 755-779: resolve_pending_wildcard_exports()
fn resolve_pending_wildcard_exports(&mut self) {
    let mut remaining = Vec::new();
    let mut progress = true;
    let mut attempts = 0;

    while progress && attempts < 4 {
        progress = false;
        attempts += 1;
        let pending = std::mem::take(&mut self.pending_wildcard_exports);
        for entry in pending {
            if self.add_wildcard_export_edges(entry.source_idx, &entry.module_path) {
                progress = true;
            } else {
                remaining.push(entry);
            }
        }
        self.pending_wildcard_exports = remaining;
        remaining = Vec::new();
    }
}
```

**Behavior Edge Updates**:

```rust
// Lines 1076-1097: connect_behavior_edges() iterates over all targets
fn connect_behavior_edges(
    &mut self,
    caller_idx: GraphNodeIndex,
    rel_path: &Path,
    alias_map: &HashMap<String, Vec<GraphNodeIndex>>,
    names: &[String],
    kind: EdgeKind,
) {
    let mut seen_targets = HashSet::new();
    for name in names {
        let targets = self.resolve_targets(rel_path, alias_map, name);
        for target_idx in targets {
            if seen_targets.insert(target_idx)
                && self.behavior_edge_cache.insert((caller_idx, target_idx, kind))
            {
                self.graph.add_edge(caller_idx, target_idx, kind);
            }
        }
    }
}
```

#### graph_builder_tests.rs (+61, -0)

**New Test**:

```rust
// Lines 420-480
#[test]
fn invoke_edges_include_all_alias_candidates() {
    let dir = create_temp_test_dir("multi_alias");

    // Setup: pkg/a.py exports merge(), pkg/b.py exports merge()
    create_test_file(&dir, "pkg/a.py", "def merge(x, y):\n    return x + y\n");
    create_test_file(&dir, "pkg/b.py", "def merge(x, y):\n    return x * y\n");
    create_test_file(&dir, "client.py",
        "from pkg.a import merge as merge_a\n\
         from pkg.b import merge as merge_b\n\
         def main():\n    merge_a(1, 2)\n    merge_b(3, 4)\n"
    );

    let builder = GraphBuilder::new(&dir);
    let result = builder.build().unwrap();
    let graph = result.graph;

    let main_idx = graph.get_index("client.py::main").unwrap();
    let invokes: Vec<_> = graph
        .graph()
        .edges(main_idx)
        .filter(|e| e.weight().kind == EdgeKind::Invoke)
        .map(|e| e.target())
        .collect();

    assert_eq!(invokes.len(), 2);
    let has_merge_a = invokes.iter().any(|&idx| {
        graph.node(idx).unwrap().id.contains("pkg/a.py::merge")
    });
    let has_merge_b = invokes.iter().any(|&idx| {
        graph.node(idx).unwrap().id.contains("pkg/b.py::merge")
    });
    assert!(has_merge_a);
    assert!(has_merge_b);
}
```

### Context Notes

#### Why Multi-Target Alias Map?

**Problem**: Single-target alias map (`HashMap<String, GraphNodeIndex>`) could only store one definition per name, causing missing invoke edges when multiple modules export the same function.

**Solution**: Changed to `HashMap<String, Vec<GraphNodeIndex>>` to support multiple targets per alias.

**Trade-off**:

- ✅ Ensures completeness (no false negatives)
- ⚠️ May increase false positives (extra edges)
- ⚠️ Invoke variance increased from +1.9% to +6.4% (34 extra edges)

**Next Steps**: Add filtering heuristics to reduce false positives while maintaining completeness.

#### Why Deferred Wildcard Exports?

**Problem**: Wildcard imports (`from module import *`) need target module's `__all__` list, but that module may not be parsed yet.

**Solution**: Queue wildcard export expansion in `PendingWildcardExport`, process after all files indexed with retry logic (max 4 attempts).

**Impact**: Correctly respects `__all__` constraints, no false imports of private symbols.

### Parity Impact

**LocAgent Fixture** (658 nodes, 1,419 edges):

- Contains: 657/657 (0%) ✅ Exact
- Imports: 218/218 (0%) ✅ Exact (maintained from Day 3)
- Inherits: 13/13 (0%) ✅ Exact
- Invokes: 565/531 (+6.4%) ⚠️ Over (up from +1.9%)

**Analysis**: Extra 34 edges due to multi-target resolution discovering more call paths than LocAgent. Examples:

- `auto_search_main.py::main → auto_search_main.py::merge` (self-module)
- `batch_build_graph.py::run → setup_repo.py::setup_repo` (cross-module)

**Root Cause**: CDSAgent discovers ALL reachable targets, LocAgent's `find_all_possible_callee` may filter more aggressively.

### Related Commits

- **3083e00** (Day 3): "feat(graph): implement export tracking system and resolve import parity"
- **00da9c2** (Day 2): "feat(graph): add parity harness and refine import resolution"
- **82936fa** (Day 2): "feat(graph): implement core graph builder with AST parsing and edge resolution"

### Follow-up Actions

- [ ] Audit extra 34 invoke edges via PARITY_DEBUG
- [ ] Implement filtering heuristics (self-recursion, package boundaries)
- [ ] Re-run parity to achieve ≤2% variance
- [ ] Expand unit test coverage to >80%

---

End of Commit Log - Day 4 Commit 1
