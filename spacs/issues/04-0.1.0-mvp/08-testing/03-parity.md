# Sub-Issue 08.03: Parity Validation - LocAgent Behavior Alignment

**Priority**: P1
**Owner**: Rust Dev 1 + QA Lead
**Timing**: Phase 4, Week 9
**PRD Reference**: [PRD-08 §2.3, §4.3](../../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md), [PRD-06 §2](../../../prd/0.1.0-MVP-PRDs-v0/06-refactor-parity.md)

## Objective

Validate CDSAgent produces equivalent outputs to LocAgent on SWE-bench Lite subset (50 instances) with algorithm parity, output format alignment, and zero regressions.

## Key Implementations

### Graph Structure Parity

```rust
// tests/parity/graph_parity_test.rs
#[cfg(test)]
mod graph_parity_tests {
    use super::*;
    use serde_json::Value;
    use std::collections::HashSet;

    #[test]
    fn test_graph_structure_matches_locagent() {
        // Load LocAgent golden output
        let locagent_graph: Value = load_golden("simple_repo_graph.json");

        // Build graph with CDSAgent
        let repo_path = PathBuf::from("tests/fixtures/simple_repo");
        let builder = GraphBuilder::new();
        let cds_graph = builder.build(&repo_path).unwrap();

        // Compare node counts by type
        for node_type in &["directory", "file", "class", "function"] {
            let locagent_count = count_nodes(&locagent_graph, node_type);
            let cds_count = cds_graph.nodes()
                .filter(|n| n.node_type.to_string() == *node_type)
                .count();

            assert_eq!(
                cds_count,
                locagent_count,
                "Node count mismatch for type: {}",
                node_type
            );
        }

        // Compare edge counts by type
        for edge_type in &["contain", "import", "invoke", "inherit"] {
            let locagent_count = count_edges(&locagent_graph, edge_type);
            let cds_count = cds_graph.edges()
                .filter(|e| e.edge_type.to_string() == *edge_type)
                .count();

            assert_eq!(
                cds_count,
                locagent_count,
                "Edge count mismatch for type: {}",
                edge_type
            );
        }
    }

    #[test]
    fn test_node_naming_matches_locagent() {
        let locagent_graph: Value = load_golden("simple_repo_graph.json");
        let repo_path = PathBuf::from("tests/fixtures/simple_repo");
        let cds_graph = GraphBuilder::new().build(&repo_path).unwrap();

        // Extract node names from both graphs
        let locagent_names: HashSet<String> = locagent_graph["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n["name"].as_str().unwrap().to_string())
            .collect();

        let cds_names: HashSet<String> = cds_graph.nodes()
            .map(|n| n.name.clone())
            .collect();

        // Compare
        let diff: HashSet<_> = locagent_names.symmetric_difference(&cds_names).collect();
        assert!(
            diff.is_empty(),
            "Node name mismatch: {:?}",
            diff
        );
    }

    fn load_golden(filename: &str) -> Value {
        let path = format!("tests/fixtures/golden/{}", filename);
        let content = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    fn count_nodes(graph: &Value, node_type: &str) -> usize {
        graph["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|n| n["type"].as_str().unwrap() == node_type)
            .count()
    }

    fn count_edges(graph: &Value, edge_type: &str) -> usize {
        graph["edges"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|e| e["type"].as_str().unwrap() == edge_type)
            .count()
    }
}
```

### BM25 Scoring Parity

```rust
// tests/parity/bm25_parity_test.rs
#[cfg(test)]
mod bm25_parity_tests {
    use super::*;

    #[test]
    fn test_bm25_scores_within_5_percent() {
        // Load LocAgent BM25 scores
        let locagent_scores: HashMap<String, f32> = load_golden_scores("bm25_scores.json");

        // Build BM25 index with CDSAgent
        let index = BM25Index::new(1.5, 0.75);  // LocAgent params
        // ... populate index with same documents as LocAgent ...

        // Compare scores for each query
        for (query, locagent_score) in locagent_scores {
            let cds_results = index.search(&query, 1);
            let cds_score = cds_results.get(0).map(|r| r.score).unwrap_or(0.0);

            let diff_pct = ((cds_score - locagent_score).abs() / locagent_score) * 100.0;

            assert!(
                diff_pct < 5.0,
                "BM25 score for '{}' differs by {:.2}% (LocAgent: {:.4}, CDS: {:.4})",
                query,
                diff_pct,
                locagent_score,
                cds_score
            );
        }
    }

    #[test]
    fn test_bm25_ranking_order_matches() {
        let query = "function calculate sum";
        let locagent_ranking: Vec<String> = load_golden_ranking("bm25_ranking.json", query);

        let index = BM25Index::new(1.5, 0.75);
        // ... populate index ...

        let cds_results = index.search(query, 10);
        let cds_ranking: Vec<String> = cds_results.iter().map(|r| r.doc_id.clone()).collect();

        // Top 5 results should match (order may vary slightly)
        let locagent_top5: HashSet<_> = locagent_ranking.iter().take(5).collect();
        let cds_top5: HashSet<_> = cds_ranking.iter().take(5).collect();

        assert_eq!(
            locagent_top5,
            cds_top5,
            "Top 5 BM25 results differ"
        );
    }

    fn load_golden_scores(filename: &str) -> HashMap<String, f32> {
        // Load from fixture
        unimplemented!()
    }

    fn load_golden_ranking(filename: &str, query: &str) -> Vec<String> {
        unimplemented!()
    }
}
```

### Output Format Parity

```typescript
// tests/parity/output-parity.test.ts
import { describe, it, expect } from 'bun:test';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

describe('Output Format Parity', () => {
  it('should match LocAgent JSON structure', async () => {
    // Load LocAgent golden output
    const locagentOutput = await loadGolden('search_output.json');

    // Run CDSAgent search
    const { stdout } = await execAsync('cds search "Calculator" --type class --format json');
    const cdsOutput = JSON.parse(stdout);

    // Compare structure
    expect(cdsOutput).toHaveProperty('results');
    expect(cdsOutput).toHaveProperty('query');
    expect(cdsOutput).toHaveProperty('total');

    // Compare result schema
    const locagentResult = locagentOutput.results[0];
    const cdsResult = cdsOutput.results[0];

    expect(Object.keys(cdsResult).sort()).toEqual(Object.keys(locagentResult).sort());
  });

  it('should match LocAgent tree format', async () => {
    const locagentTree = await loadGolden('traverse_tree.txt');

    const { stdout } = await execAsync('cds traverse "test.py::Calculator" --format tree');

    // Verify tree structure markers
    expect(stdout).toContain('├─');  // Branch
    expect(stdout).toContain('└─');  // Last branch
    expect(stdout).toContain('[invoke]→');  // Edge label format

    // Verify indentation consistency
    const lines = stdout.split('\n').filter(l => l.trim());
    const locagentLines = locagentTree.split('\n').filter(l => l.trim());

    expect(lines.length).toEqual(locagentLines.length);
  });

  it('should produce identical reasoning chain structure', async () => {
    const locagentReasoning = await loadGolden('agent_reasoning.json');

    // Run agent workflow
    const agent = new AgentRunner({ /* ... */ });
    const result = await agent.runTask('Find sanitize functions');

    // Verify CoT steps present
    expect(result.reasoning).toContain('Extract keywords');
    expect(result.reasoning).toContain('Search');
    expect(result.reasoning).toContain('Traverse');
    expect(result.reasoning).toContain('Synthesize');

    // Verify output structure matches
    expect(result).toHaveProperty('locations');
    expect(result.locations[0]).toHaveProperty('file');
    expect(result.locations[0]).toHaveProperty('line_range');
    expect(result.locations[0]).toHaveProperty('reason');
  });
});

async function loadGolden(filename: string) {
  const content = await Bun.file(`tests/fixtures/golden/${filename}`).text();
  return filename.endsWith('.json') ? JSON.parse(content) : content;
}
```

### SWE-bench Lite Subset Evaluation

```python
# tests/parity/swe_bench_parity.py
"""
Compare CDSAgent and LocAgent on SWE-bench Lite subset (50 instances)
"""

import json
import subprocess
from pathlib import Path
from datasets import load_dataset

def run_locagent(instance):
    """Run LocAgent on single instance, return locations"""
    # Call LocAgent (assumes tmp/LocAgent available)
    result = subprocess.run(
        ["python", "tmp/LocAgent/auto_search_main.py", "--instance", instance["instance_id"]],
        capture_output=True,
        text=True
    )
    return json.loads(result.stdout)

def run_cdsagent(instance):
    """Run CDSAgent on single instance, return locations"""
    # Call CDSAgent via agent
    result = subprocess.run(
        ["bun", "run", "cds-agent/src/main.ts", "--task", instance["problem_statement"]],
        capture_output=True,
        text=True
    )
    return json.loads(result.stdout)

def calculate_file_acc_at_k(predictions, labels, k=5):
    """Calculate File Acc@k metric"""
    hits = 0
    for pred, label in zip(predictions, labels):
        pred_files = set([loc["file"] for loc in pred["locations"][:k]])
        label_files = set(label["files"])
        if pred_files & label_files:  # Intersection non-empty
            hits += 1
    return hits / len(predictions)

def main():
    # Load SWE-bench Lite subset (first 50 instances)
    dataset = load_dataset("czlll/SWE-bench_Lite", split="test[:50]")

    locagent_results = []
    cdsagent_results = []
    labels = []

    for instance in dataset:
        print(f"Evaluating {instance['instance_id']}...")

        # Run both agents
        locagent_out = run_locagent(instance)
        cdsagent_out = run_cdsagent(instance)

        locagent_results.append(locagent_out)
        cdsagent_results.append(cdsagent_out)
        labels.append({"files": instance["gold_files"]})

    # Calculate metrics
    locagent_acc = calculate_file_acc_at_k(locagent_results, labels, k=5)
    cdsagent_acc = calculate_file_acc_at_k(cdsagent_results, labels, k=5)

    print(f"\n=== Results ===")
    print(f"LocAgent File Acc@5: {locagent_acc:.2%}")
    print(f"CDSAgent File Acc@5: {cdsagent_acc:.2%}")
    print(f"Difference: {abs(locagent_acc - cdsagent_acc):.2%}")

    # Parity check: within 5% acceptable
    assert abs(locagent_acc - cdsagent_acc) < 0.05, "Accuracy differs by >5%"

    # Save results
    with open("tests/parity/results.json", "w") as f:
        json.dump({
            "locagent_acc": locagent_acc,
            "cdsagent_acc": cdsagent_acc,
            "results": {
                "locagent": locagent_results,
                "cdsagent": cdsagent_results,
            }
        }, f, indent=2)

    print("Parity validation passed!")

if __name__ == "__main__":
    main()
```

### Regression Detection

```rust
// tests/parity/regression_test.rs
#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn test_no_regressions_vs_golden_outputs() {
        let golden_dir = PathBuf::from("tests/fixtures/golden");

        for entry in std::fs::read_dir(golden_dir).unwrap() {
            let entry = entry.unwrap();
            let filename = entry.file_name().to_str().unwrap().to_string();

            if !filename.ends_with("_output.json") {
                continue;
            }

            println!("Checking regression for: {}", filename);

            let golden: Value = load_golden(&filename);
            let current = run_cdsagent_for_fixture(&filename);

            assert_eq!(
                golden,
                current,
                "Regression detected in {}",
                filename
            );
        }
    }

    fn run_cdsagent_for_fixture(filename: &str) -> Value {
        // Re-run CDSAgent with same inputs as golden fixture
        unimplemented!()
    }
}
```

## Golden Output Generation

```shell
#!/bin/bash
# tests/parity/generate_golden_outputs.sh

# Run LocAgent to generate golden outputs for parity comparison

set -e

LOCAGENT_DIR="tmp/LocAgent"
GOLDEN_DIR="tests/fixtures/golden"

mkdir -p "$GOLDEN_DIR"

echo "Generating golden outputs from LocAgent..."

# Generate graph structure
python "$LOCAGENT_DIR/dependency_graph/build_graph.py" \
  --repo-path tests/fixtures/simple_repo \
  --output "$GOLDEN_DIR/simple_repo_graph.json"

# Generate BM25 scores
python "$LOCAGENT_DIR/build_bm25_index.py" \
  --repo-path tests/fixtures/simple_repo \
  --save-scores "$GOLDEN_DIR/bm25_scores.json"

# Run sample queries
python "$LOCAGENT_DIR/auto_search_main.py" \
  --query "sanitize function" \
  --output "$GOLDEN_DIR/search_output.json"

echo "Golden outputs generated in $GOLDEN_DIR"
```

## Acceptance Criteria

- [ ] Graph structure matches LocAgent on 10 sample repos
- [ ] BM25 scores within 5% of LocAgent
- [ ] Output format (JSON, tree) matches LocAgent schema
- [ ] File Acc@5 ≥75% on SWE-bench Lite subset (50 instances)
- [ ] Zero regressions vs. golden outputs
- [ ] Parity validation runs in <30 minutes

**Related**: [00-overview.md](00-overview.md), [04-benchmark.md](04-benchmark.md), [../06-refactor-parity.md](../06-refactor-parity.md)
