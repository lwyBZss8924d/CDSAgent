# Task T-02-01: Graph Builder (AST Parsing & Construction)

**Issue**: [Sub-Issue 02.01 – Graph Build](../../issues/04-0.1.0-mvp/02-index-core/01-graph-build.md)

**PRD References**: [PRD-02 §2.1](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-06 §2.1](../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md)

**Owners**: Rust Dev 1, aided by Refactor Parity lead

**Status**: ☐ Not Started | **Week**: 2

---

## Objective

Implement the Rust-based dependency graph builder that mirrors LocAgent’s `build_graph.py`, parsing Python repositories into heterogeneous graphs with nodes (directory/file/class/function) and edges (contain/import/invoke/inherit).

## Deliverables

- `crates/cds-index/src/graph/parser.rs` (tree-sitter Python integration)
- `crates/cds-index/src/graph/builder.rs` (graph construction pipeline)
- `crates/cds-index/src/graph/traversal.rs` (basic adjacency helpers reused by TraverseGraph)
- Unit tests under `crates/cds-index/tests/graph_builder_tests.rs`
- Fixture repository snapshots in `tests/fixtures/repos/`

## Implementation Steps

1. **Tree-sitter setup**
   - Add Python grammar to build script; expose AST node helpers (classes, functions, imports).
2. **Entity extraction**
   - Implement walkers that emit `GraphNode` structs with qualified names, file paths, and line ranges.
3. **Edge construction**
   - Derive contain relationships from nesting; resolve import/invoke/inherit via AST analysis.
4. **Graph assembly**
   - Populate `petgraph::Graph<Node, Edge>` (or bespoke struct) with stable IDs and metadata.
5. **Testing & validation**
   - Compare node/edge counts against LocAgent output for 3 sample repositories.

## Acceptance Criteria

- [ ] Parses Python repositories and produces a graph with all 4 node types and 4 edge types.
- [ ] Fully-qualified names match LocAgent format (e.g., `path/to/file.py::Class::method`).
- [ ] Unit tests cover typical and edge cases (decorators, nested classes, async functions).
- [ ] Graph parity script reports ≤2% variance in node/edge counts vs. LocAgent baseline.

## Dependencies

- **Prerequisite**: [T-06-01 Refactor Parity Methodology](../06-refactor-parity/locagent-comparison.md) for baseline data.
- **Blocks**: [T-02-02 Sparse Index](T-02-02-sparse-index.md), [T-03-01 Core Commands](../03-cli-tools/T-03-01-core-commands.md).

## Notes

- Follow the same skip-directory logic as LocAgent (`.git`, `.github`, vendor folders).
- Store raw source snippets alongside nodes to avoid rereading files during retrieval.
