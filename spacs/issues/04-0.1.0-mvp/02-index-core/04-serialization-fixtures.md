# Sub-Issue 02.04: Serialization & Test Fixtures

**Priority**: P1 (Critical Path - Validation)
**Status**: ☐ Not Started
**Owner**: Rust Dev 2 (parallel with sparse index work)
**Parent**: [02-index-core/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-02 §2.3](../../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-06 §6](../../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md)
**Timing**: Phase 1-2, Week 2-4 (parallel workstream)

---

## Objective

Implement persistent storage for graph and indices, create test fixtures for parity validation, and ensure data integrity across serialization/deserialization cycles.

## Scope

**In Scope**:

- Index file format (JSON + binary) compatible with LocAgent structure
- Serialization/deserialization for `CodeGraph`, `NameIndex`, `BM25Index`
- Test fixtures (sample repos, golden outputs from LocAgent)
- Parity test data generation scripts
- Concurrent read access support

**Out of Scope (v0.2.0)**:

- Incremental update persistence
- Distributed storage backends
- Index compression/optimization

---

## Dependencies

- **Requires**: [01-graph-build.md](01-graph-build.md) (graph structures), [02-sparse-index.md](02-sparse-index.md) (index structures)
- **Feeds Into**: [../06-refactor-parity.md](../06-refactor-parity.md) (parity validation), [../08-testing/](../08-testing/) (all test suites)
- **Parallel With**: [02-sparse-index.md](02-sparse-index.md) (can start early with graph serialization)

---

## Implementation Tasks

### Week 2: Graph Serialization

Task 1: Index File Format

```rust
// cds_storage/src/format.rs
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

/// Index directory structure (compatible with LocAgent)
/// <REPO_INDEX_DIR>/
///   ├── graph/
///   │   ├── nodes.json
///   │   ├── edges.json
///   │   └── metadata.json
///   ├── indices/
///   │   ├── name_index.json
///   │   ├── bm25/ (tantivy format)
///   │   └── entity_dict.json
///   └── config.toml

#[derive(Serialize, Deserialize)]
pub struct IndexMetadata {
    pub version: String,
    pub created_at: String,
    pub repo_path: PathBuf,
    pub node_count: usize,
    pub edge_count: usize,
    pub languages: Vec<String>,
}

impl IndexMetadata {
    pub fn new(graph: &CodeGraph, repo_path: PathBuf) -> Self {
        Self {
            version: "v0.1.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            repo_path,
            node_count: graph.nodes.len(),
            edge_count: graph.edges.len(),
            languages: vec!["python".to_string()],
        }
    }
}

pub struct IndexPaths {
    pub root: PathBuf,
    pub graph_dir: PathBuf,
    pub indices_dir: PathBuf,
}

impl IndexPaths {
    pub fn new(repo_path: &Path) -> Self {
        let root = repo_path.join(".cds-index");
        Self {
            graph_dir: root.join("graph"),
            indices_dir: root.join("indices"),
            root,
        }
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.graph_dir)?;
        std::fs::create_dir_all(&self.indices_dir)?;
        std::fs::create_dir_all(self.indices_dir.join("bm25"))?;
        Ok(())
    }
}
```

**Acceptance**:

- [ ] Directory structure matches LocAgent's `graph_index_v2.3/` format
- [ ] Metadata includes version, timestamps, stats
- [ ] Compatible with `GRAPH_INDEX_DIR` environment variable

---

Task 2: Graph Serialization

```rust
// cds_storage/src/serializer.rs
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use serde_json;

use crate::graph::CodeGraph;
use crate::format::{IndexMetadata, IndexPaths};

pub fn serialize_graph(graph: &CodeGraph, repo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let paths = IndexPaths::new(repo_path);
    paths.ensure_dirs()?;

    // Serialize nodes
    let nodes_file = File::create(paths.graph_dir.join("nodes.json"))?;
    let mut writer = BufWriter::new(nodes_file);
    serde_json::to_writer_pretty(&mut writer, &graph.nodes)?;
    writer.flush()?;

    // Serialize edges
    let edges_file = File::create(paths.graph_dir.join("edges.json"))?;
    let mut writer = BufWriter::new(edges_file);
    serde_json::to_writer_pretty(&mut writer, &graph.edges)?;
    writer.flush()?;

    // Serialize metadata
    let metadata = IndexMetadata::new(graph, repo_path.to_path_buf());
    let metadata_file = File::create(paths.graph_dir.join("metadata.json"))?;
    serde_json::to_writer_pretty(metadata_file, &metadata)?;

    println!(
        "Serialized graph: {} nodes, {} edges → {}",
        graph.nodes.len(),
        graph.edges.len(),
        paths.root.display()
    );

    Ok(())
}

pub fn deserialize_graph(repo_path: &Path) -> Result<CodeGraph, Box<dyn std::error::Error>> {
    let paths = IndexPaths::new(repo_path);

    // Check version compatibility
    let metadata_file = File::open(paths.graph_dir.join("metadata.json"))?;
    let metadata: IndexMetadata = serde_json::from_reader(metadata_file)?;
    if !metadata.version.starts_with("v0.1") {
        return Err(format!("Incompatible index version: {}", metadata.version).into());
    }

    // Deserialize nodes
    let nodes_file = File::open(paths.graph_dir.join("nodes.json"))?;
    let nodes = serde_json::from_reader(nodes_file)?;

    // Deserialize edges
    let edges_file = File::open(paths.graph_dir.join("edges.json"))?;
    let edges = serde_json::from_reader(edges_file)?;

    let graph = CodeGraph { nodes, edges };

    println!(
        "Loaded graph: {} nodes, {} edges from {}",
        graph.nodes.len(),
        graph.edges.len(),
        paths.root.display()
    );

    Ok(graph)
}
```

**Acceptance** (from PRD-02 FR-GS-1):

- [ ] Can serialize and deserialize graph without data loss
- [ ] Index loads in <2s for 10K file repository
- [ ] Version checking prevents loading incompatible indices
- [ ] JSON format is human-readable for debugging

---

### Week 3: Index Serialization

Task 3: Name Index Persistence

```rust
// cds_storage/src/index_storage.rs
use std::fs::File;
use std::path::Path;
use serde_json;

use crate::sparse_index::NameIndex;
use crate::format::IndexPaths;

pub fn save_name_index(index: &NameIndex, repo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let paths = IndexPaths::new(repo_path);
    let file = File::create(paths.indices_dir.join("name_index.json"))?;
    serde_json::to_writer_pretty(file, index)?;
    Ok(())
}

pub fn load_name_index(repo_path: &Path) -> Result<NameIndex, Box<dyn std::error::Error>> {
    let paths = IndexPaths::new(repo_path);
    let file = File::open(paths.indices_dir.join("name_index.json"))?;
    let index = serde_json::from_reader(file)?;
    Ok(index)
}
```

**Acceptance**:

- [ ] Name index persists exact/prefix mappings
- [ ] Deserialization rebuilds sorted keys for prefix search
- [ ] Load time <500ms for 10K entities

---

Task 4: BM25 Index Persistence

```rust
// BM25 persistence handled by tantivy (if using tantivy)
// Custom BM25 needs manual serialization

pub fn save_bm25_index(index: &BM25Index, repo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // If using tantivy: index persists automatically to disk
    // If custom BM25: serialize inverted index + doc lengths
    let paths = IndexPaths::new(repo_path);
    let bm25_dir = paths.indices_dir.join("bm25");

    if let BM25Index::Custom(custom) = index {
        let inverted_file = File::create(bm25_dir.join("inverted_index.json"))?;
        serde_json::to_writer(inverted_file, &custom.inverted_index)?;

        let doc_lengths_file = File::create(bm25_dir.join("doc_lengths.json"))?;
        serde_json::to_writer(doc_lengths_file, &custom.doc_lengths)?;
    }

    Ok(())
}
```

**Acceptance**:

- [ ] BM25 index persists inverted index + document statistics
- [ ] Tantivy native format used if applicable
- [ ] Load time <1s for 10K entities

---

### Week 2-4: Test Fixtures

Task 5: Fixture Repository Structure

```tree
tests/fixtures/
├── parity/
│   ├── locagent_repo/           # LocAgent's own codebase (ground truth)
│   ├── sample_repos/            # 5 repos from SWE-bench Lite
│   │   ├── repo1/
│   │   ├── repo2/
│   │   └── ...
│   ├── golden_outputs/
│   │   ├── locagent_graph.json          # Expected graph from LocAgent
│   │   ├── graph_node_counts.json       # Node counts by type
│   │   ├── graph_edge_counts.json       # Edge counts by type
│   │   ├── search_queries.jsonl         # 50 queries + expected top-10
│   │   └── traverse_samples.jsonl       # 10 traversal cases + outputs
│   ├── locagent_version.txt     # LocAgent commit hash
│   └── README.md                # How to regenerate golden outputs
└── unit_tests/
    ├── simple_class.py          # Minimal test case
    ├── nested_functions.py      # Nested function test
    ├── inheritance.py           # Class inheritance test
    └── ...
```

**Acceptance**:

- [ ] Fixture repos cover edge cases (nested classes, imports, inheritance)
- [ ] Golden outputs generated from LocAgent baseline
- [ ] README documents regeneration steps

---

Task 6: Golden Output Generation

```bash
# tests/fixtures/parity/generate_baseline.sh

#!/bin/bash
set -e

echo "Generating LocAgent baseline outputs..."

cd tmp/LocAgent

# 1. Build graph and extract counts
python -c "
from dependency_graph.build_graph import build_repo_graph
import json

graph = build_repo_graph('.')
node_counts = {}
for node in graph['nodes']:
    ntype = node['entity_type']
    node_counts[ntype] = node_counts.get(ntype, 0) + 1

edge_counts = {}
for edge in graph['edges']:
    etype = edge['relation']
    edge_counts[etype] = edge_counts.get(etype, 0) + 1

with open('../../tests/fixtures/parity/golden_outputs/graph_node_counts.json', 'w') as f:
    json.dump(node_counts, f, indent=2)

with open('../../tests/fixtures/parity/golden_outputs/graph_edge_counts.json', 'w') as f:
    json.dump(edge_counts, f, indent=2)

with open('../../tests/fixtures/parity/golden_outputs/locagent_graph.json', 'w') as f:
    json.dump(graph, f, indent=2)

print(f'Nodes: {sum(node_counts.values())}')
print(f'Edges: {sum(edge_counts.values())}')
"

# 2. Generate search baseline
python -c "
from plugins.location_tools.retriever.bm25_retriever import BM25Retriever
import json

queries = [
    'sanitize user input',
    'authentication handler',
    'database connection',
    # ... (50 queries total)
]

retriever = BM25Retriever('.')
with open('../../tests/fixtures/parity/golden_outputs/search_queries.jsonl', 'w') as f:
    for query in queries:
        top10 = retriever.search(query, k=10)
        result = {
            'text': query,
            'expected_results': [r['entity_id'] for r in top10]
        }
        f.write(json.dumps(result) + '\n')
"

# 3. Document LocAgent version
git rev-parse HEAD > ../../tests/fixtures/parity/locagent_version.txt

echo "Baseline generation complete!"
```

**Acceptance**:

- [ ] Script generates all golden outputs
- [ ] LocAgent version documented for reproducibility
- [ ] Outputs validated manually before committing

---

### Week 4: Concurrent Access

Task 7: Read-Write Lock Support

```rust
// cds_storage/src/concurrent.rs
use std::sync::{Arc, RwLock};
use crate::graph::CodeGraph;

pub struct SharedGraph {
    inner: Arc<RwLock<CodeGraph>>,
}

impl SharedGraph {
    pub fn new(graph: CodeGraph) -> Self {
        Self {
            inner: Arc::new(RwLock::new(graph)),
        }
    }

    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&CodeGraph) -> R,
    {
        let graph = self.inner.read().unwrap();
        f(&graph)
    }

    pub fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut CodeGraph) -> R,
    {
        let mut graph = self.inner.write().unwrap();
        f(&mut graph)
    }
}
```

**Acceptance** (from PRD-02 FR-GS-2):

- [ ] Multiple concurrent `cds search` commands succeed
- [ ] Read latency unaffected by concurrent readers
- [ ] Writes lock only affected portions (deferred to v0.2.0 for granular locking)

---

## Acceptance Criteria (from PRD-02 §2.3, PRD-06 §6)

### Must-Pass

- [ ] Can serialize and deserialize graph without data loss
- [ ] Index loads in <2s for 10K file repository
- [ ] Supports version checking (detect incompatible formats)
- [ ] Multiple concurrent reads succeed without blocking
- [ ] Parity tests pass (see [../06-refactor-parity.md](../06-refactor-parity.md))

### File Format

- [ ] Directory structure matches LocAgent's layout
- [ ] JSON files are human-readable
- [ ] Binary formats (if used) documented

### Test Fixtures

- [ ] Golden outputs generated from LocAgent baseline
- [ ] Cover all node/edge types
- [ ] Include 50 search queries with expected results
- [ ] Include 10 traversal cases with expected outputs

---

## Testing Strategy

### Unit Tests

```rust
// cds_storage/tests/serialization_test.rs
#[test]
fn test_graph_roundtrip() {
    let original_graph = build_test_graph();
    let temp_dir = tempdir().unwrap();

    serialize_graph(&original_graph, temp_dir.path()).unwrap();
    let loaded_graph = deserialize_graph(temp_dir.path()).unwrap();

    assert_eq!(original_graph.nodes.len(), loaded_graph.nodes.len());
    assert_eq!(original_graph.edges.len(), loaded_graph.edges.len());

    // Verify nodes match
    for (id, node) in &original_graph.nodes {
        let loaded_node = loaded_graph.nodes.get(id).unwrap();
        assert_eq!(node.name, loaded_node.name);
        assert_eq!(node.entity_type, loaded_node.entity_type);
    }
}

#[test]
fn test_version_compatibility() {
    let temp_dir = tempdir().unwrap();
    let paths = IndexPaths::new(temp_dir.path());
    paths.ensure_dirs().unwrap();

    // Write incompatible version
    let metadata = IndexMetadata {
        version: "v99.0.0".to_string(),
        ..Default::default()
    };
    let file = File::create(paths.graph_dir.join("metadata.json")).unwrap();
    serde_json::to_writer(file, &metadata).unwrap();

    // Attempt to load
    let result = deserialize_graph(temp_dir.path());
    assert!(result.is_err());
}
```

### Integration Tests

- [ ] Serialize LocAgent repo, reload, and verify node/edge counts
- [ ] Generate golden outputs, load in CDSAgent, compare results

---

## Open Questions & Risks

### Q1: Binary vs JSON

**Question**: Use binary format (bincode) for performance or JSON for readability?
**Decision**: JSON for v0.1.0 (easier debugging), binary optimization in v0.2.0 if needed
**Validation**: Benchmark load times on 10K file repo

### Q2: Index Caching

**Risk**: Repeated loads may be slow
**Mitigation**: v0.1.0 loads once at service startup; v0.2.0 adds memory mapping
**Tracking**: [10-extensibility.md](../10-extensibility.md) v0.2.0 backlog

### Q3: Fixture Maintenance

**Risk**: LocAgent updates may invalidate golden outputs
**Mitigation**: Document LocAgent version, regenerate fixtures when upgrading
**Escalation**: If parity drops, investigate LocAgent changes and update CDSAgent

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **Depends On**: [01-graph-build.md](01-graph-build.md), [02-sparse-index.md](02-sparse-index.md)
- **Feeds Into**: [../06-refactor-parity.md](../06-refactor-parity.md), [../08-testing/](../08-testing/)
- **Tests**: [../08-testing/01-unit.md](../08-testing/01-unit.md), [../08-testing/04-benchmark.md](../08-testing/04-benchmark.md)

---

## Next Steps

1. [ ] Define index file format and directory structure (Week 2)
2. [ ] Implement graph serialization/deserialization (Week 2)
3. [ ] Create test fixture directory structure (Week 2)
4. [ ] Generate LocAgent golden outputs (Week 3)
5. [ ] Implement index persistence (name + BM25) (Week 3-4)
6. [ ] Add concurrent access support (Week 4)
7. [ ] Validate parity tests against golden outputs

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, fixture structure defined
