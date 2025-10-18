# Sub-Issue 08.01: Unit Tests - Component-Level Test Coverage

**Priority**: P1
**Owner**: Rust Dev 1 + Rust Dev 2
**Timing**: Phase 3, Weeks 6-8 (continuous)
**PRD Reference**: [PRD-08 §2.1, §4.1](../../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

## Objective

Achieve >80% unit test coverage for all Rust modules (graph, index, service, CLI) with property-based tests for graph operations.

## Key Implementations

### Graph Builder Tests

```rust
// tests/unit/graph/parser_tests.rs
#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_python_function() {
        let source = r#"
def add(a, b):
    return a + b
"#;
        let parser = ASTParser::new_python().unwrap();
        let entities = parser.parse_file("test.py", source).unwrap();

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, EntityType::Function);
        assert_eq!(entities[0].name, "add");
        assert_eq!(entities[0].line_range, (1, 2));
    }

    #[test]
    fn test_parse_python_class() {
        let source = r#"
class Calculator:
    def add(self, a, b):
        return a + b
"#;
        let parser = ASTParser::new_python().unwrap();
        let entities = parser.parse_file("test.py", source).unwrap();

        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].entity_type, EntityType::Class);
        assert_eq!(entities[0].name, "Calculator");
        assert_eq!(entities[1].entity_type, EntityType::Function);
        assert_eq!(entities[1].name, "add");
    }

    #[test]
    fn test_parse_malformed_code() {
        let source = "def broken(\n";  // Incomplete function
        let parser = ASTParser::new_python().unwrap();
        let result = parser.parse_file("test.py", source);

        // Should handle gracefully, not panic
        assert!(result.is_ok());
    }
}
```

### Graph Construction Tests

```rust
// tests/unit/graph/builder_tests.rs
#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn test_build_simple_graph() {
        let repo_path = PathBuf::from("tests/fixtures/simple_repo");
        let builder = GraphBuilder::new();
        let graph = builder.build(&repo_path).unwrap();

        // Verify nodes created
        assert!(graph.node_count() > 0);

        // Verify directory node exists
        let dir_nodes: Vec<_> = graph.nodes()
            .filter(|n| n.node_type == NodeType::Directory)
            .collect();
        assert!(!dir_nodes.is_empty());

        // Verify file node exists
        let file_nodes: Vec<_> = graph.nodes()
            .filter(|n| n.node_type == NodeType::File)
            .collect();
        assert!(!file_nodes.is_empty());
    }

    #[test]
    fn test_graph_edges_created() {
        let repo_path = PathBuf::from("tests/fixtures/simple_repo");
        let builder = GraphBuilder::new();
        let graph = builder.build(&repo_path).unwrap();

        // Verify contain edges
        let contain_edges: Vec<_> = graph.edges()
            .filter(|e| e.edge_type == EdgeType::Contain)
            .collect();
        assert!(!contain_edges.is_empty());

        // Verify import edges (if imports exist in fixture)
        let import_edges: Vec<_> = graph.edges()
            .filter(|e| e.edge_type == EdgeType::Import)
            .collect();
        // May be 0 if fixture has no imports
    }

    #[test]
    fn test_incremental_update() {
        // v0.2.0 feature, placeholder test
        // TODO: Implement incremental update in v0.2.0
    }
}
```

### Property-Based Tests (Graph Invariants)

```rust
// tests/unit/graph/property_tests.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_graph_acyclic_property(
        edges in prop::collection::vec(
            (0usize..100, 0usize..100),
            0..200
        )
    ) {
        // Build directed graph from edges
        let mut graph = Graph::new();
        for (from, to) in edges {
            graph.add_edge(from, to, EdgeType::Invoke);
        }

        // Property: Contain edges should form a tree (no cycles)
        let contain_subgraph = graph.filter_edges(EdgeType::Contain);
        assert!(!has_cycle(&contain_subgraph), "Contain edges must be acyclic");
    }

    #[test]
    fn test_node_name_uniqueness(
        names in prop::collection::vec(
            "[a-zA-Z_][a-zA-Z0-9_]*",
            1..50
        )
    ) {
        let mut builder = GraphBuilder::new();
        for name in &names {
            builder.add_function_node(name, 0..1);
        }
        let graph = builder.build_graph();

        // Property: Node IDs should be unique
        let node_ids: Vec<_> = graph.nodes().map(|n| n.id).collect();
        let unique_ids: HashSet<_> = node_ids.iter().collect();
        assert_eq!(node_ids.len(), unique_ids.len(), "Node IDs must be unique");
    }
}
```

### BM25 Index Tests

```rust
// tests/unit/index/bm25_tests.rs
#[cfg(test)]
mod bm25_tests {
    use super::*;

    #[test]
    fn test_bm25_scoring() {
        let mut index = BM25Index::new(1.5, 0.75);  // LocAgent params

        // Add documents
        index.add_document("doc1", "function add returns sum");
        index.add_document("doc2", "function multiply returns product");
        index.add_document("doc3", "class Calculator implements add");

        // Search
        let results = index.search("add", 10);

        // Verify results
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].doc_id, "doc1");  // Best match
        assert!(results[0].score > results[1].score);
    }

    #[test]
    fn test_bm25_parameters() {
        // Verify LocAgent parity: k1=1.5, b=0.75
        let index = BM25Index::new(1.5, 0.75);
        assert_eq!(index.k1, 1.5);
        assert_eq!(index.b, 0.75);
    }

    #[test]
    fn test_empty_query() {
        let index = BM25Index::new(1.5, 0.75);
        let results = index.search("", 10);
        assert!(results.is_empty());
    }
}
```

### Name Index Tests

```rust
// tests/unit/index/name_index_tests.rs
#[cfg(test)]
mod name_index_tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let mut index = NameIndex::new();
        index.insert("Calculator", NodeID(1));
        index.insert("add", NodeID(2));

        let results = index.exact_match("Calculator");
        assert!(results.is_some());
        assert_eq!(results.unwrap()[0], NodeID(1));
    }

    #[test]
    fn test_prefix_match() {
        let mut index = NameIndex::new();
        index.insert("add", NodeID(1));
        index.insert("add_numbers", NodeID(2));
        index.insert("subtract", NodeID(3));

        let results = index.prefix_match("add", 10);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&NodeID(1)));
        assert!(results.contains(&NodeID(2)));
    }

    #[test]
    fn test_case_sensitivity() {
        let mut index = NameIndex::new();
        index.insert("MyClass", NodeID(1));

        assert!(index.exact_match("MyClass").is_some());
        assert!(index.exact_match("myclass").is_none());  // Case-sensitive
    }
}
```

### CLI Command Tests

```rust
// tests/unit/cli/search_tests.rs
#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn test_search_command_parsing() {
        let args = vec!["cds", "search", "Calculator", "--type", "class", "--limit", "5"];
        let cmd = parse_command(args).unwrap();

        assert_eq!(cmd.query, "Calculator");
        assert_eq!(cmd.entity_type, Some("class"));
        assert_eq!(cmd.limit, 5);
    }

    #[test]
    fn test_search_output_json() {
        let results = vec![
            SearchResult { file: "calc.py", name: "Calculator", score: 0.9 },
        ];

        let output = format_json(&results);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["results"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["results"][0]["name"], "Calculator");
    }

    #[test]
    fn test_search_invalid_args() {
        let args = vec!["cds", "search"];  // Missing query
        let result = parse_command(args);
        assert!(result.is_err());
    }
}
```

### Service Layer Tests

```rust
// tests/unit/service/handler_tests.rs
#[cfg(test)]
mod handler_tests {
    use super::*;

    #[tokio::test]
    async fn test_search_handler() {
        let index = MockIndexService::new();
        let handler = SearchHandler::new(index);

        let request = json!({
            "query": "Calculator",
            "entity_type": "class",
            "limit": 5
        });

        let response = handler.handle(request).await.unwrap();
        assert!(response["results"].as_array().unwrap().len() <= 5);
    }

    #[tokio::test]
    async fn test_error_handling() {
        let handler = SearchHandler::new(MockIndexService::failing());

        let request = json!({ "query": "test" });
        let result = handler.handle(request).await;

        assert!(result.is_err());
        // Verify error format matches JSON-RPC spec
    }
}
```

## Test Fixtures

### Sample Repository (tests/fixtures/simple_repo/)

```tree
tests/fixtures/simple_repo/
├── src/
│   ├── calculator.py
│   │   class Calculator:
│   │       def add(self, a, b): ...
│   │       def subtract(self, a, b): ...
│   └── utils.py
│       def helper_function(): ...
└── tests/
    └── test_calculator.py
```

### Golden Outputs (tests/fixtures/golden/)

```json
// tests/fixtures/golden/simple_repo_graph.json
{
  "nodes": [
    {"id": 1, "type": "directory", "name": "src"},
    {"id": 2, "type": "file", "name": "calculator.py"},
    {"id": 3, "type": "class", "name": "Calculator"},
    {"id": 4, "type": "function", "name": "add"}
  ],
  "edges": [
    {"from": 1, "to": 2, "type": "contain"},
    {"from": 2, "to": 3, "type": "contain"},
    {"from": 3, "to": 4, "type": "contain"}
  ]
}
```

## CI/CD Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Unit Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run unit tests
        run: cargo test --all --lib

      - name: Check coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Lcov --output-dir coverage
          # Fail if coverage <80%
          coverage_pct=$(grep -Po '(?<=lines......: )\d+\.\d+' coverage/lcov.info | head -1)
          if (( $(echo "$coverage_pct < 80" | bc -l) )); then
            echo "Coverage $coverage_pct% is below 80% threshold"
            exit 1
          fi
```

## Acceptance Criteria

- [ ] >80% code coverage for cds-index crate
- [ ] >80% code coverage for cds-tools crate
- [ ] All unit tests pass in CI/CD pipeline
- [ ] Property-based tests validate graph invariants
- [ ] Test execution time <5 minutes
- [ ] Coverage report generated on every PR

**Related**: [00-overview.md](00-overview.md), [02-integration.md](02-integration.md), [../02-index-core/](../02-index-core/)
