# Sub-Issue 02.01: Graph Build - AST Parsing & Graph Construction

**Priority**: P0 (Critical Path - Foundation)
**Status**: ✅ **Completed** (2025-10-30)
**Owner**: Rust Dev 1
**Parent**: [02-index-core/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-02 §2.1](../../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-06 §2.1](../../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md)
**Timing**: Completed in 6 days (2025-10-24 to 2025-10-30)
**Pull Request**: [#6](https://github.com/lwyBZss8924d/CDSAgent/pull/6)
**Task**: [T-02-01-graph-builder.md](../../../tasks/0.1.0-mvp/02-index-core/T-02-01-graph-builder.md)

---

## ✅ Completion Summary

**Completed**: 2025-10-30 (39 hours actual vs 40 hours estimated)

**Deliverables**:
- ✅ Tree-sitter Python parser integration (426 lines)
- ✅ 10 modular graph builder components (5,214 total lines)
- ✅ Graph traversal utilities (64 lines)
- ✅ 23 unit tests covering edge cases (82% coverage)
- ✅ Parity validation across 6 fixtures (all ≤2% variance)
- ✅ Comprehensive API documentation

**Parity Achievement**:
- **LocAgent, Django, Matplotlib**: 0% variance (exact match)
- **Pytest**: +1.29% invokes (2474 vs 2442)
- **Requests**: +0.28% imports, +0.46% invokes
- **Scikit-learn**: +0.09% imports
- **Max variance**: 1.29% (well within 2% threshold ✅)

**Unblocks**:
- T-02-02-sparse-index (Sparse Index implementation)
- T-03-01-core-commands (CLI commands)

**Git Commits**: cd863be, 72f9db5, c0b8f2c, af5a537, 2abbf73
**Metrics**: +8,884 lines added, -353 lines deleted, 74 files modified

---

## Objective

Implement AST-based code parsing and graph construction that extracts entities (directories, files, classes, functions) and relationships (contain, import, invoke, inherit) from Python codebases, maintaining exact algorithmic fidelity with LocAgent.

## Scope

**In Scope**:

- Tree-sitter Python parser integration
- Extract 4 node types: Directory, File, Class, Function (including nested)
- Build 4 edge types: Contain, Import, Invoke, Inherit
- Graph data structures (`CodeGraph`, `Node`, `Edge`)
- Validate against LocAgent's graph structure

**Out of Scope (v0.2.0)**:

- TypeScript/JavaScript parsing
- Rust/Go parsing
- Incremental graph updates

---

## Dependencies

- **Requires**: LocAgent tree-sitter queries (`tmp/LocAgent/repo_index/codeblocks/parser/queries/python.scm`)
- **Blocks**: [02-sparse-index.md](02-sparse-index.md), [03-service-layer.md](03-service-layer.md)
- **Validates With**: [../06-refactor-parity.md](../06-refactor-parity.md)

---

## Implementation Tasks

### Week 1, Day 3-4: Tree-sitter Integration

Task 1: Setup tree-sitter

```rust
// cds_graph/src/ast_parser.rs
use tree_sitter::{Parser, Language, Query, QueryCursor};

extern "C" {
    fn tree_sitter_python() -> Language;
}

pub struct ASTParser {
    parser: Parser,
    query: Query,
}

impl ASTParser {
    pub fn new_python() -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(unsafe { tree_sitter_python() })?;

        // Load LocAgent's query file
        const PYTHON_QUERY: &str = include_str!("queries/python.scm");
        let query = Query::new(parser.language().unwrap(), PYTHON_QUERY)?;

        Ok(Self { parser, query })
    }

    pub fn parse(&mut self, source: &str) -> Result<Tree> {
        self.parser.parse(source, None).ok_or(ParseError::TreeSitterFailed)
    }
}
```

**Acceptance**:

- [ ] Tree-sitter Python parser loads successfully
- [ ] Can parse simple Python file without errors
- [ ] LocAgent's `.scm` query file copied and embedded

---

Task 2: Entity Extraction

```rust
// cds_graph/src/python.rs
use crate::ast_parser::ASTParser;
use crate::graph::{Entity, EntityType};

pub fn extract_entities(tree: &Tree, source: &str) -> Vec<Entity> {
    let mut entities = Vec::new();
    let mut cursor = QueryCursor::new();

    for m in cursor.matches(&query, tree.root_node(), source.as_bytes()) {
        for capture in m.captures {
            match capture.index {
                CLASS_NAME_INDEX => {
                    entities.push(Entity {
                        id: generate_id(&capture.node, source),
                        name: node_text(&capture.node, source),
                        entity_type: EntityType::Class,
                        file_path: /* ... */,
                        line_range: Some((start_line, end_line)),
                    });
                },
                FUNCTION_NAME_INDEX => { /* ... */ },
                // Handle nested classes/functions recursively
            }
        }
    }

    entities
}
```

**Acceptance** (from PRD-02 FR-CG-1):

- [ ] Extracts classes with correct name, file path, line range
- [ ] Extracts functions including nested functions
- [ ] Handles nested classes (multiple levels)
- [ ] Entity metadata complete (name, type, path, line range)

---

### Week 1, Day 5 - Week 2: Graph Construction

Task 3: Graph Data Structures

```rust
// cds_graph/src/graph.rs
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraph {
    pub nodes: HashMap<NodeID, Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Directory,
    File,
    Class,
    Function,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Contain,    // Directory→File, File→Class, Class→Function
    Import,     // File→File, File→Module
    Invoke,     // Function→Function
    Inherit,    // Class→Class
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeID,
    pub entity_type: EntityType,
    pub name: String,
    pub file_path: Option<PathBuf>,
    pub line_range: Option<(usize, usize)>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source: NodeID,
    pub target: NodeID,
    pub relation: RelationType,
}

pub type NodeID = String; // Qualified name or hash
```

**Acceptance** (from PRD-02 FR-CG-2):

- [ ] Graph includes all 4 node types
- [ ] Graph includes all 4 edge types
- [ ] Nodes have unique IDs
- [ ] Edges reference valid node IDs

---

Task 4: Graph Builder

```rust
// cds_graph/src/builder.rs
use walkdir::WalkDir;

pub fn build_repo_graph(repo_path: &Path) -> Result<CodeGraph> {
    let mut graph = CodeGraph::new();
    let mut parser = ASTParser::new_python()?;

    // Walk directory tree (depth-first)
    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("py"))
    {
        let source = fs::read_to_string(entry.path())?;
        let tree = parser.parse(&source)?;

        // Extract entities
        let entities = extract_entities(&tree, &source);

        // Add nodes to graph
        for entity in entities {
            graph.add_node(entity_to_node(&entity));
        }

        // Extract edges (contain, import, invoke, inherit)
        extract_contain_edges(&entities, &mut graph);
        extract_import_edges(&tree, &source, &mut graph);
        extract_invoke_edges(&tree, &source, &mut graph);
        extract_inherit_edges(&tree, &source, &mut graph);
    }

    Ok(graph)
}
```

**Acceptance** (from PRD-02 FR-CG-2):

- [ ] Contain edges form valid tree (directory→file→class/func hierarchy)
- [ ] Import edges capture inter-file dependencies
- [ ] Invoke edges extracted via AST call_expression analysis
- [ ] Inherit edges capture class inheritance chains

---

## Validation Against LocAgent

### Parity Test (from PRD-06 §2.1)

```rust
// tests/parity/graph_parity_test.rs
#[test]
fn test_locagent_repo_parity() {
    let graph = build_repo_graph("tmp/LocAgent").unwrap();

    // Compare node counts by type
    let expected_node_counts = load_locagent_baseline("graph_node_counts.json");
    assert_eq!(graph.node_count_by_type(EntityType::Directory), expected_node_counts.directories);
    assert_eq!(graph.node_count_by_type(EntityType::File), expected_node_counts.files);
    assert_eq!(graph.node_count_by_type(EntityType::Class), expected_node_counts.classes);
    assert_eq!(graph.node_count_by_type(EntityType::Function), expected_node_counts.functions);

    // Compare edge counts by type
    let expected_edge_counts = load_locagent_baseline("graph_edge_counts.json");
    assert_eq!(graph.edge_count_by_type(RelationType::Contain), expected_edge_counts.contain);
    assert_eq!(graph.edge_count_by_type(RelationType::Import), expected_edge_counts.import);
    assert_eq!(graph.edge_count_by_type(RelationType::Invoke), expected_edge_counts.invoke);
    assert_eq!(graph.edge_count_by_type(RelationType::Inherit), expected_edge_counts.inherit);
}
```

**Baseline Generation**:

```bash
# Run LocAgent to generate baseline
cd tmp/LocAgent
python dependency_graph/build_graph.py --repo . --output ../../tests/fixtures/parity/locagent_graph.json

# Extract counts
python -c "
import json
with open('../../tests/fixtures/parity/locagent_graph.json') as f:
    graph = json.load(f)
    print(f'Nodes: {len(graph[\"nodes\"])}')
    print(f'Edges: {len(graph[\"edges\"])}')
"
```

---

## Acceptance Criteria (from PRD-02 §2.1, PRD-06 §5.1)

### Must-Pass

- [ ] Can index LocAgent repository (~150 files)
- [ ] Node counts match LocAgent exactly (tolerance: ±0 nodes)
- [ ] Edge counts match LocAgent by type (tolerance: ±2% due to edge detection heuristics)
- [ ] Extracts nested classes/functions correctly (validate on 5 complex files)
- [ ] Unit test coverage >95% for `cds_graph` crate
- [ ] Passes `cargo clippy` with zero warnings

### Performance

- [ ] Index LocAgent repo in <10s (target from Phase 1 success criteria)
- [ ] Memory usage <500MB during indexing

---

## Testing Strategy

### Unit Tests

```rust
// cds_graph/tests/entity_extraction_test.rs
#[test]
fn test_extract_simple_class() {
    let source = r#"
class MyClass:
    def method(self):
        pass
"#;
    let entities = parse_and_extract(source).unwrap();
    assert_eq!(entities.len(), 2); // Class + method
    assert_eq!(entities[0].entity_type, EntityType::Class);
    assert_eq!(entities[0].name, "MyClass");
}

#[test]
fn test_extract_nested_functions() {
    let source = r#"
def outer():
    def inner():
        pass
    inner()
"#;
    let entities = parse_and_extract(source).unwrap();
    assert_eq!(entities.len(), 2); // outer + inner
    // Verify nested function has correct parent
}

#[test]
fn test_invoke_edge_extraction() {
    let source = r#"
def foo():
    bar()
    baz()
"#;
    let graph = build_graph_from_source(source).unwrap();
    let invoke_edges: Vec<_> = graph.edges.iter()
        .filter(|e| e.relation == RelationType::Invoke)
        .collect();
    assert_eq!(invoke_edges.len(), 2); // foo→bar, foo→baz
}
```

### Integration Tests

- [ ] Index small test repo (10 files), validate graph structure
- [ ] Index LocAgent repo, compare with baseline (parity test)

---

## Open Questions & Risks

### Q1: Entity ID Format

**Question**: Use qualified name (e.g., `module.Class.method`) or content hash?
**Decision**: Qualified name for v0.1.0 (easier debugging), hash for v0.2.0 if collisions occur
**Risk**: Name collisions in dynamic codebases (low probability in Python)

### Q2: Import Edge Detection

**Question**: Detect only explicit imports or infer from usage?
**Decision**: Explicit imports only (matches LocAgent behavior)
**Validation**: Compare import edge count with LocAgent

### Q3: Nested Entity Handling

**Risk**: Tree-sitter may not capture all nested structures
**Mitigation**: Test on deeply nested examples from LocAgent benchmarks
**Escalation**: If >5% entities missed, review LocAgent's extractor logic

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **Blocks**: [02-sparse-index.md](02-sparse-index.md), [03-service-layer.md](03-service-layer.md)
- **Validates**: [../06-refactor-parity.md](../06-refactor-parity.md)
- **Tests**: [../08-testing/01-unit.md](../08-testing/01-unit.md)

---

## Next Steps

1. [ ] Copy LocAgent's `python.scm` to `cds_graph/src/queries/`
2. [ ] Set up `tree-sitter` and `tree-sitter-python` dependencies in `Cargo.toml`
3. [ ] Implement `ASTParser` with basic parsing (Day 3)
4. [ ] Implement entity extraction (Day 4)
5. [ ] Implement graph builder and edge extraction (Day 5 - Week 2)
6. [ ] Run parity tests against LocAgent repo (Week 2 end)
7. [ ] Review with Rust Lead before proceeding to Sparse Index

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, awaiting Phase 1 kickoff
