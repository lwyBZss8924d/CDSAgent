# Issue-06: Rust Refactoring Parity with LocAgent

**Priority**: P0 (Critical Path - Cross-Cutting Concern)
**Status**: ☐ Not Started
**Owner**: Rust Lead + All Rust Developers
**PRD Reference**: [PRD-06: Rust Refactoring Plan](../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md)

---

## Overview

Ensure CDSAgent's Rust refactoring maintains algorithmic fidelity with LocAgent's Python implementation while achieving 2-5x performance improvements. This is a cross-cutting concern that validates work across all Rust crates.

## Objective

Provide continuous validation that:

- Graph structure matches LocAgent exactly (4 node types, 4 edge types)
- Search results match LocAgent's top-10 on benchmark queries
- Traversal algorithms produce identical outputs
- Performance targets are met or exceeded

This issue tracks the **methodology** for parity checks; specific validations happen within [02-index-core/](02-index-core/), [03-cli-tools/](03-cli-tools/), and [08-testing/](08-testing/).

## Dependencies

- **Shared With**: Issues 02-index-core, 03-cli-tools, 08-testing
- **Requires**: LocAgent repository access (`tmp/LocAgent/`)
- **Blocks**: Phase 2-3 signoff (no signoff without parity proof)

---

## Validation Methodology

### 1. Module-by-Module Mapping (from PRD-06 §2)

| LocAgent Module | CDSAgent Equivalent | Validation Owner | Status |
|----------------|---------------------|------------------|--------|
| `dependency_graph/build_graph.py` | `cds_graph::builder` | Rust Dev 1 | ☐ |
| `dependency_graph/traverse_graph.py` | `cds_traversal::bfs` | Rust Dev 1 | ☐ |
| `repo_index/codeblocks/parser/` | `cds_graph::ast_parser` | Rust Dev 1 | ☐ |
| `build_bm25_index.py` | `cds_sparse_index::bm25` | Rust Dev 2 | ☐ |
| `repo_index/index/code_index.py` | `cds_sparse_index::search` | Rust Dev 2 | ☐ |
| `plugins/location_tools/` | `cds_cli::commands` | Rust Dev 2 | ☐ |

### 2. Algorithm Preservation Checklist

#### Graph Construction (PRD-06 §2.1)

- [ ] **Directory traversal**: Use same walk order as LocAgent
- [ ] **AST parsing**: Reuse LocAgent's `.scm` tree-sitter queries
- [ ] **Entity extraction**: Match nested class/function handling
- [ ] **Edge creation**: Validate contain/import/invoke/inherit logic

**Validation**: Parse LocAgent repo, compare `node_count` and `edge_count` by type.

#### BM25 Indexing (PRD-06 §2.2)

- [ ] **Tokenization**: Match LocAgent's camelCase/snake_case splitting
- [ ] **Stop words**: Use same stop word list
- [ ] **BM25 parameters**: k1=1.5, b=0.75 (LocAgent defaults)
- [ ] **Ranking**: Top-10 results match LocAgent on 50 queries

**Validation**: Run 50 benchmark queries, compute overlap@10 (target: ≥90%).

#### Graph Traversal (PRD-06 §2.1)

- [ ] **BFS algorithm**: Same queue-based traversal
- [ ] **Filters**: Relation type, entity type, depth limit
- [ ] **Output format**: Tree-structured with LocAgent's exact format

**Validation**: Traverse sample graphs, diff outputs character-by-character.

### 3. Output Format Preservation (PRD-06 §2.3)

| Output | LocAgent Format | CDSAgent Implementation | Validation |
|--------|----------------|------------------------|------------|
| Fold snippet | `"{type} {name} - {file}:{line}"` | `format_snippet()` | String comparison |
| Preview snippet | Signature + first 5 lines | `format_snippet()` | Line count + content |
| Tree format | `├─[relation]→ Entity` | `format_tree()` | Exact character match |

**Validation**: Generate outputs for 10 entities, compare byte-for-byte.

---

## Performance Validation (PRD-06 §5.3)

| Metric | LocAgent (Python) Baseline | CDSAgent (Rust) Target | Validation Method |
|--------|----------------------------|----------------------|-------------------|
| Index 1K files | ~5s | <3s (1.6x faster) | `criterion` benchmark |
| Search query | ~200ms | <100ms (2x faster) | `hyperfine` CLI timing |
| Traverse 2-hop | ~500ms | <200ms (2.5x faster) | Integration test |
| Memory (10K files) | ~3GB | <2GB | `valgrind --tool=massif` |

**Acceptance**: All metrics meet or exceed targets.

---

## Unit Test Coverage Targets (PRD-06 §5.1)

| Crate | Target Coverage | Validation Owner | Status |
|-------|----------------|------------------|--------|
| `cds_graph` | >95% | Rust Dev 1 | ☐ |
| `cds_sparse_index` | >95% | Rust Dev 2 | ☐ |
| `cds_traversal` | >95% | Rust Dev 1 | ☐ |
| `cds_storage` | >90% | Rust Dev 3 | ☐ |
| `cds_cli` | >85% | Rust Dev 2 | ☐ |

**Validation**: Run `cargo tarpaulin` per crate, fail CI if below target.

---

## Continuous Validation Strategy

### Phase-Gated Checks

**Phase 1 Checkpoint** (Week 2):

- [ ] Graph construction parity validated on LocAgent repo
- [ ] Entity/edge counts match LocAgent

**Phase 2 Checkpoint** (Week 5):

- [ ] BM25 search results validated (50 queries, ≥90% overlap)
- [ ] Traversal outputs match LocAgent exactly (10 samples)

**Phase 3 Checkpoint** (Week 7):

- [ ] Performance targets achieved (index, search, traverse)
- [ ] Memory usage within bounds

**Phase 4 Checkpoint** (Week 10):

- [ ] Full SWE-bench Lite parity (Acc@5 ≥80%, matching LocAgent)

### Automated Regression Tests

```bash
# tests/parity/run_parity_checks.sh

# 1. Graph parity
cargo test --test graph_parity -- --nocapture
# Expected: "✅ Graph structure matches LocAgent (nodes: 1234, edges: 5678)"

# 2. Search parity
cargo test --test search_parity -- --nocapture
# Expected: "✅ 48/50 queries match LocAgent top-10 (96% overlap)"

# 3. Traverse parity
cargo test --test traverse_parity -- --nocapture
# Expected: "✅ All 10 traversal outputs match LocAgent exactly"
```

Run on every PR, fail if parity drops.

---

## Parity Test Data

### Fixtures (shared with [08-testing/04-benchmark.md](08-testing/04-benchmark.md))

```tree
tests/fixtures/parity/
├── locagent_repo/           # LocAgent's own codebase (ground truth)
├── sample_repos/            # 5 repos from SWE-bench Lite
├── golden_outputs/
│   ├── graph_locagent.json  # Expected graph structure
│   ├── search_queries.jsonl # 50 queries + expected top-10
│   └── traverse_samples.jsonl # 10 traversal cases + outputs
└── README.md                # How to regenerate golden outputs
```

### Regenerating Golden Outputs

```bash
# Run LocAgent to produce baseline
cd tmp/LocAgent
python auto_search_main.py --dataset custom --output ../../tests/fixtures/parity/golden_outputs/

# Document LocAgent version
echo "LocAgent commit: $(git rev-parse HEAD)" > ../../tests/fixtures/parity/locagent_version.txt
```

---

## Acceptance Criteria (from PRD-06 §9)

- [ ] All LocAgent Python modules have Rust equivalents (§2 mapping complete)
- [ ] Unit tests pass with >95% coverage (per-crate targets met)
- [ ] Search results match LocAgent's top-10 on 50 sample queries (≥90% overlap)
- [ ] Traversal outputs match LocAgent exactly (10/10 samples)
- [ ] Performance meets or exceeds targets (§5.3 benchmarks pass)
- [ ] Code is idiomatic Rust (passes `cargo clippy` with zero warnings)

---

## Open Questions & Risks

### 1. BM25 Implementation Choice

**Question**: Use `tantivy` crate or custom BM25?
**Decision**: Prototype both (Week 3), compare accuracy. Choose custom if tantivy differs >5% from LocAgent.
**Tracking**: [02-index-core/02-sparse-index.md](02-index-core/02-sparse-index.md)

### 2. Tree-sitter Query Reuse

**Question**: Can we use LocAgent's `.scm` files verbatim?
**Decision**: Yes, copy from `tmp/LocAgent/repo_index/codeblocks/parser/queries/` and embed via `include_str!()`.
**Tracking**: [02-index-core/01-graph-build.md](02-index-core/01-graph-build.md)

### 3. Floating-Point Precision

**Risk**: BM25 scores may differ slightly due to Rust vs Python floating-point handling.
**Mitigation**: Accept scores within 0.01 tolerance; verify rank order is identical.

---

## Related Issues

- [02-index-core/01-graph-build.md](02-index-core/01-graph-build.md) - Graph construction validation
- [02-index-core/02-sparse-index.md](02-index-core/02-sparse-index.md) - BM25 accuracy validation
- [03-cli-tools/02-output-format.md](03-cli-tools/02-output-format.md) - Output format parity
- [08-testing/04-benchmark.md](08-testing/04-benchmark.md) - SWE-bench Lite parity

---

## Ongoing Responsibilities

### Rust Lead

- [ ] Review parity test results weekly
- [ ] Escalate any >5% deviation from LocAgent
- [ ] Sign off on Phase 2/3/4 parity gates

### All Rust Developers

- [ ] Run parity checks before PRs
- [ ] Document any intentional deviations from LocAgent
- [ ] Update golden outputs if LocAgent baseline changes

---

**Status Updates**:

- *2025-10-19*: Parity validation plan established, fixtures setup pending
