# Sub-Issue 03.02: Output Format - JSON/Text/Tree with LocAgent Parity

**Priority**: P1 (Critical Path - Output Validation)
**Status**: ☐ Not Started
**Owner**: Rust Dev 2 (parallel with commands)
**Parent**: [03-cli-tools/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-03 §5.2-5.3](../../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md), [PRD-06 §2.3](../../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md)
**Timing**: Phase 2, Week 4 (parallel with commands)

---

## Objective

Implement JSON, text, and tree output formatting that maintains exact parity with LocAgent's SearchEntity and TraverseGraph outputs, ensuring agent compatibility and human readability.

## Scope

**In Scope**:

- JSON output schema (agent-compatible, matches LocAgent structure)
- Text output (human-readable summaries)
- Tree output (traverse command, exact LocAgent format)
- Snippet formatting (fold/preview/full)
- Validate output parity with LocAgent on 10 sample queries

**Out of Scope (v0.2.0)**:

- DOT format (GraphViz) for graph visualization
- Colored output (terminal colors)
- Markdown output format

---

## Dependencies

- **Requires**: [01-command-impl.md](01-command-impl.md) (commands must call formatters)
- **Validates With**: [../06-refactor-parity.md](../06-refactor-parity.md)
- **Tests**: [03-integration-tests.md](03-integration-tests.md)

---

## Implementation Tasks

### Week 4, Day 1-2: JSON Output

Task 1: Search JSON Output (LocAgent Parity)

```rust
// cds-cli/src/output/formatter.rs
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct SearchOutput {
    pub query: String,
    pub total_results: usize,
    pub results: Vec<EntityResult>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityResult {
    pub entity_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub file_path: String,
    pub line_range: (usize, usize),
    pub score: f32,
    pub snippet: SnippetResult,
}

#[derive(Serialize, Deserialize)]
pub struct SnippetResult {
    pub fold: String,
    pub preview: String,
    pub full: String,
}

pub fn format_search_json(
    result: &SearchResult,
    pretty: bool,
) -> Result<String, serde_json::Error> {
    let output = SearchOutput {
        query: result.query.clone(),
        total_results: result.results.len(),
        results: result.results.iter().map(|r| EntityResult {
            entity_id: r.entity_id.clone(),
            name: r.name.clone(),
            entity_type: format!("{:?}", r.entity_type).to_lowercase(),
            file_path: r.file_path.clone(),
            line_range: r.line_range,
            score: r.score,
            snippet: format_snippet(&r),
        }).collect(),
    };

    if pretty {
        serde_json::to_string_pretty(&output)
    } else {
        serde_json::to_string(&output)
    }
}

fn format_snippet(entity: &EntityResult) -> SnippetResult {
    // Fold: one-line summary
    let fold = format!(
        "{} {} - {}:{}",
        entity.entity_type,
        entity.name,
        entity.file_path,
        entity.line_range.0
    );

    // Preview: signature + first few lines
    let code_lines: Vec<&str> = entity.code.lines().collect();
    let preview = if code_lines.len() > 5 {
        code_lines[..5].join("\n") + "\n    ..."
    } else {
        entity.code.clone()
    };

    // Full: complete code
    let full = entity.code.clone();

    SnippetResult { fold, preview, full }
}
```

**LocAgent Output Reference** (from PRD-03 §5.2):

```python
# LocAgent SearchEntity output (from locationtools.py)
fold = f"{entity.type} {entity.name} - {entity.file}:{entity.line}"
preview = entity.signature + "\n" + "\n".join(entity.body[:5])
full = entity.code
```

**Acceptance**:

- [ ] JSON schema matches LocAgent SearchEntity output
- [ ] Fold format: `"{type} {name} - {file}:{line}"`
- [ ] Preview includes signature + first 5 lines
- [ ] Full includes complete entity code
- [ ] Validate with JSON schema tool

---

Task 2: Traverse JSON Output

```rust
// cds-cli/src/output/formatter.rs (continued)
#[derive(Serialize, Deserialize)]
pub struct TraverseOutput {
    pub start_entities: Vec<String>,
    pub depth: usize,
    pub filters: TraverseFilters,
    pub subgraph: SubgraphResult,
}

#[derive(Serialize, Deserialize)]
pub struct TraverseFilters {
    pub relations: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct SubgraphResult {
    pub nodes: Vec<NodeResult>,
    pub edges: Vec<EdgeResult>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeResult {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub file: String,
}

#[derive(Serialize, Deserialize)]
pub struct EdgeResult {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub relation: String,
}

pub fn format_traverse_json(
    results: &[TraverseResult],
    pretty: bool,
) -> Result<String, serde_json::Error> {
    // Aggregate all results
    let mut all_nodes = Vec::new();
    let mut all_edges = Vec::new();

    for result in results {
        for node in &result.nodes {
            all_nodes.push(NodeResult {
                id: node.id.clone(),
                name: node.name.clone(),
                entity_type: format!("{:?}", node.entity_type).to_lowercase(),
                file: node.file_path.clone(),
            });
        }
        for edge in &result.edges {
            all_edges.push(EdgeResult {
                source: edge.source.clone(),
                target: edge.target.clone(),
                relation: format!("{:?}", edge.relation).to_lowercase(),
            });
        }
    }

    let output = TraverseOutput {
        start_entities: results.iter().map(|r| r.start_id.clone()).collect(),
        depth: results[0].max_depth,
        filters: TraverseFilters {
            relations: None, // TODO: extract from params
            types: None,
        },
        subgraph: SubgraphResult {
            nodes: all_nodes,
            edges: all_edges,
        },
    };

    if pretty {
        serde_json::to_string_pretty(&output)
    } else {
        serde_json::to_string(&output)
    }
}
```

**Acceptance**:

- [ ] JSON schema matches LocAgent TraverseGraph output
- [ ] Includes start entities, depth, filters, subgraph (nodes + edges)
- [ ] Node and edge types in lowercase (LocAgent convention)

---

### Week 4, Day 3-4: Text & Tree Output

Task 3: Search Text Output

```rust
// cds-cli/src/output/formatter.rs (continued)
pub fn format_search_text(result: &SearchResult) -> String {
    let mut output = String::new();
    output.push_str(&format!("Found {} results for \"{}\"\n\n", result.results.len(), result.query));

    for (i, entity) in result.results.iter().enumerate() {
        output.push_str(&format!(
            "[{}] {} ({}) - {}:{}-{}\n",
            i + 1,
            entity.name,
            entity.entity_type,
            entity.file_path,
            entity.line_range.0,
            entity.line_range.1
        ));

        // Show preview (first 3 lines)
        let code_lines: Vec<&str> = entity.code.lines().collect();
        for line in code_lines.iter().take(3) {
            output.push_str(&format!("    {}\n", line));
        }
        if code_lines.len() > 3 {
            output.push_str("    ...\n");
        }
        output.push('\n');
    }

    output
}

pub fn format_search_fold(result: &SearchResult) -> String {
    result.results.iter().map(|entity| {
        format!(
            "{} {} - {}:{}",
            entity.entity_type,
            entity.name,
            entity.file_path,
            entity.line_range.0
        )
    }).collect::<Vec<_>>().join("\n")
}

pub fn format_search_preview(result: &SearchResult) -> String {
    result.results.iter().map(|entity| {
        let code_lines: Vec<&str> = entity.code.lines().collect();
        let preview = code_lines.iter().take(5).cloned().collect::<Vec<_>>().join("\n");
        format!("{}\n{}\n", entity.name, preview)
    }).collect::<Vec<_>>().join("\n---\n\n")
}

pub fn format_search_full(result: &SearchResult) -> String {
    result.results.iter().map(|entity| {
        format!("# {} ({})\n# File: {}\n\n{}\n", entity.name, entity.entity_type, entity.file_path, entity.code)
    }).collect::<Vec<_>>().join("\n\n")
}
```

**Acceptance**:

- [ ] Text output is human-readable
- [ ] Fold shows one-line summary per entity
- [ ] Preview shows signature + 5 lines
- [ ] Full shows complete code with headers

---

Task 4: Tree Output (LocAgent Format Parity)

```rust
// cds-cli/src/output/tree.rs
use std::collections::HashMap;

pub fn format_traverse_tree(results: &[TraverseResult]) -> String {
    let mut output = String::new();

    for result in results {
        let tree = build_tree_structure(&result);
        output.push_str(&render_tree(&tree, &result.start_id, "", true));
    }

    output
}

struct TreeNode {
    entity: NodeResult,
    children: Vec<(String, String)>, // (relation, child_id)
}

fn build_tree_structure(result: &TraverseResult) -> HashMap<String, TreeNode> {
    let mut tree: HashMap<String, TreeNode> = HashMap::new();

    // Build node map
    for node in &result.nodes {
        tree.insert(node.id.clone(), TreeNode {
            entity: node.clone(),
            children: Vec::new(),
        });
    }

    // Add edges as children
    for edge in &result.edges {
        if let Some(parent) = tree.get_mut(&edge.source) {
            parent.children.push((
                format!("{:?}", edge.relation),
                edge.target.clone(),
            ));
        }
    }

    tree
}

fn render_tree(
    tree: &HashMap<String, TreeNode>,
    node_id: &str,
    prefix: &str,
    is_last: bool,
) -> String {
    let mut output = String::new();

    let node = match tree.get(node_id) {
        Some(n) => n,
        None => return output,
    };

    // Render current node (LocAgent format)
    // Format: "├─[relation]→ Entity (type) [id] - file:line"
    output.push_str(&format!(
        "{}{} ({}) [{}] - {}:{}\n",
        prefix,
        node.entity.name,
        node.entity.entity_type,
        node.entity.id,
        node.entity.file,
        node.entity.line_range.0
    ));

    // Render children
    let child_count = node.children.len();
    for (i, (relation, child_id)) in node.children.iter().enumerate() {
        let is_last_child = i == child_count - 1;
        let connector = if is_last_child { "└─" } else { "├─" };
        let child_prefix = if is_last_child { "   " } else { "│  " };

        output.push_str(&format!(
            "{}{}[{}]→ ",
            prefix,
            connector,
            relation
        ));

        let new_prefix = format!("{}{}", prefix, child_prefix);
        output.push_str(&render_tree(tree, child_id, &new_prefix, is_last_child));
    }

    output
}
```

**LocAgent Tree Format Reference** (from PRD-03 §5.3):

```text
RootEntity (type) [id] - file:line
├─[relation]→ ChildEntity1 (type) [id] - file:line
│  └─[relation]→ GrandchildEntity (type) [id] - file:line
└─[relation]→ ChildEntity2 (type) [id] - file:line
```

**Acceptance**:

- [ ] Tree format uses LocAgent characters: `├─`, `└─`, `│`, `→`
- [ ] Each line: `{name} ({type}) [{id}] - {file}:{line}`
- [ ] Relation shown in brackets: `[invoke]→`
- [ ] Indentation preserved for nested levels
- [ ] Character-for-character match with LocAgent on sample traversals

---

### Week 4, Day 5: Validation

Task 5: Output Parity Tests

```rust
// cds-cli/tests/output_parity_test.rs
use serde_json;

#[test]
fn test_search_json_schema() {
    let search_result = SearchResult {
        query: "sanitize".to_string(),
        results: vec![EntityResult {
            entity_id: "abc123".to_string(),
            name: "sanitize_html".to_string(),
            entity_type: EntityType::Function,
            file_path: "utils/sanitize.py".to_string(),
            line_range: (15, 32),
            score: 1.0,
            code: "def sanitize_html(input: str) -> str:\n    return bleach.clean(input)".to_string(),
        }],
    };

    let json_output = format_search_json(&search_result, true).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_output).unwrap();

    // Validate schema
    assert_eq!(parsed["query"], "sanitize");
    assert_eq!(parsed["total_results"], 1);
    assert!(parsed["results"].is_array());
    assert_eq!(parsed["results"][0]["entity_id"], "abc123");
    assert_eq!(parsed["results"][0]["name"], "sanitize_html");
    assert_eq!(parsed["results"][0]["type"], "function");
}

#[test]
fn test_tree_format_locagent_parity() {
    let traverse_result = TraverseResult {
        start_id: "entity_root".to_string(),
        nodes: vec![
            NodeResult {
                id: "entity_root".to_string(),
                name: "process_request".to_string(),
                entity_type: EntityType::Function,
                file: "server.py".to_string(),
                line_range: (12, 25),
            },
            NodeResult {
                id: "entity_child".to_string(),
                name: "validate_input".to_string(),
                entity_type: EntityType::Function,
                file: "validators.py".to_string(),
                line_range: (45, 60),
            },
        ],
        edges: vec![EdgeResult {
            source: "entity_root".to_string(),
            target: "entity_child".to_string(),
            relation: RelationType::Invoke,
        }],
        max_depth: 2,
    };

    let tree_output = format_traverse_tree(&[traverse_result]);

    // Expected LocAgent format
    let expected = "process_request (function) [entity_root] - server.py:12\n\
                    └─[invoke]→ validate_input (function) [entity_child] - validators.py:45\n";

    assert_eq!(tree_output, expected);
}

#[test]
fn test_fold_format() {
    let entity = EntityResult {
        entity_id: "abc".to_string(),
        name: "sanitize_html".to_string(),
        entity_type: EntityType::Function,
        file_path: "utils/sanitize.py".to_string(),
        line_range: (15, 32),
        score: 1.0,
        code: "def sanitize_html(...):\n    pass".to_string(),
    };

    let snippet = format_snippet(&entity);

    // LocAgent fold format: "{type} {name} - {file}:{line}"
    assert_eq!(snippet.fold, "function sanitize_html - utils/sanitize.py:15");
}
```

**Golden Output Comparison**:

```bash
# tests/fixtures/parity/compare_outputs.sh

#!/bin/bash
set -e

echo "Comparing CDSAgent outputs with LocAgent baselines..."

# 1. Search output comparison
cds search "sanitize" --format json > cds_search.json
# Compare with LocAgent baseline
diff -u tests/fixtures/parity/golden_outputs/search_sanitize.json cds_search.json

# 2. Traverse tree comparison
cds traverse entity_abc --depth 2 --format tree > cds_tree.txt
diff -u tests/fixtures/parity/golden_outputs/traverse_entity_abc_tree.txt cds_tree.txt

echo "Output parity validation passed!"
```

**Acceptance**:

- [ ] All parity tests pass (JSON, tree, fold formats)
- [ ] 10 sample queries match LocAgent outputs character-for-character
- [ ] JSON schema validated with JSON Schema validator

---

## Acceptance Criteria (from PRD-03 §8, PRD-06 §2.3)

### Must-Pass

- [ ] JSON output matches LocAgent SearchEntity structure
- [ ] Tree format character-for-character identical to LocAgent
- [ ] Fold format: `"{type} {name} - {file}:{line}"`
- [ ] Preview format: signature + first 5 lines + "..."
- [ ] Full format: complete entity code
- [ ] Output parity validated on 10 sample queries
- [ ] JSON schema validation passes

### Formatting Quality

- [ ] Pretty-printed JSON is readable
- [ ] Text output is well-formatted
- [ ] Tree indentation is consistent
- [ ] No extra whitespace or formatting issues

---

## Testing Strategy

### Unit Tests

- [ ] JSON serialization/deserialization
- [ ] Tree rendering logic
- [ ] Snippet formatting (fold/preview/full)
- [ ] Edge cases (empty results, deep nesting)

### Parity Tests

- [ ] Compare with LocAgent outputs on 10 queries
- [ ] Validate JSON schema compliance
- [ ] Character-for-character tree format comparison

---

## Open Questions & Risks

### Q1: Tree Format Ambiguity

**Question**: How to handle multi-parent nodes in tree output?
**Decision**: Show first occurrence fully, subsequent as references (Week 4)
**LocAgent Behavior**: Investigate how LocAgent handles cycles/DAGs

### Q2: Unicode Characters

**Risk**: Tree characters (├─, └─) may not render on all terminals
**Mitigation**: Use UTF-8 encoding, document terminal requirements
**Fallback**: ASCII mode (use +-, |, \->) if needed in v0.2.0

### Q3: Large Output Handling

**Risk**: Full code output may be too large for terminal
**Mitigation**: Warn if output >10K lines, suggest --output flag
**Escalation**: Implement pagination in v0.2.0 if needed

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **Depends On**: [01-command-impl.md](01-command-impl.md)
- **Validates**: [../06-refactor-parity.md](../06-refactor-parity.md)
- **Tests**: [03-integration-tests.md](03-integration-tests.md)

---

## Next Steps

1. [ ] Implement JSON output formatters (Week 4, Day 1)
2. [ ] Implement text output formatters (Week 4, Day 2)
3. [ ] Implement tree output formatter (Week 4, Day 3)
4. [ ] Write output parity tests (Week 4, Day 4)
5. [ ] Run comparison with LocAgent baselines (Week 4, Day 5)
6. [ ] Fix any format discrepancies

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, awaiting command implementation
