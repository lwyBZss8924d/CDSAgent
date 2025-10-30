# LocAgent Parity Validation Methodology

**Version**: 1.0
**Date**: 2025-10-20
**Status**: Active
**Owner**: Rust Lead + All Rust Developers

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Module Mapping](#2-module-mapping)
3. [Algorithm Preservation](#3-algorithm-preservation)
4. [Output Format Preservation](#4-output-format-preservation)
5. [Performance Validation](#5-performance-validation)
6. [Unit Test Coverage](#6-unit-test-coverage)
7. [Continuous Validation Strategy](#7-continuous-validation-strategy)
8. [Automated Regression Tests](#8-automated-regression-tests)
9. [Troubleshooting](#9-troubleshooting)

---

## 1. Introduction

### 1.1 Purpose

This document establishes the **Standard Operating Procedure (SOP)** for validating that CDSAgent's Rust refactoring maintains algorithmic fidelity with LocAgent's Python implementation while achieving 2-5x performance improvements.

**Parity** means:

- ✅ **Same Results**: Graph structure, search rankings, traversal outputs match LocAgent
- ✅ **Same Algorithms**: BFS traversal, BM25 scoring, tree-sitter parsing logic preserved
- ✅ **Same Behavior**: Edge cases, error handling, output formatting identical
- ✅ **Better Performance**: 2-5x faster execution, lower memory usage

### 1.2 Scope

This methodology applies to:

- **All Rust crates**: `cds-index`, `cds-tools`, `cds-agent`
- **All development phases**: Week 2 (graph) through Week 10 (production)
- **All team members**: Every PR must pass parity checks before merge

### 1.3 Success Criteria

Parity is considered **achieved** when:

| Metric | Threshold | Validation Method |
|--------|-----------|-------------------|
| Graph structure variance | ≤2% node/edge count difference | Automated script |
| Search result overlap@10 | ≥90% (9/10 results match) | 50 benchmark queries |
| Traversal exact match | 100% (10/10 scenarios) | JSONL diff |
| Performance speedup | 2-5x faster than Python | Criterion benchmarks |

---

## 2. Module Mapping

### 2.1 LocAgent → CDSAgent Module Map

This mapping ensures every LocAgent Python module has a Rust equivalent:

| LocAgent Module (Python) | CDSAgent Equivalent (Rust) | Owner | Status |
|--------------------------|---------------------------|-------|--------|
| `dependency_graph/build_graph.py` | `crates/cds-index/src/graph/builder.rs` | Rust Dev 1 | ☐ Not Started |
| `dependency_graph/traverse_graph.py` | `crates/cds-index/src/graph/traversal.rs` | Rust Dev 1 | ☐ Not Started |
| `repo_index/codeblocks/parser/` | `crates/cds-index/src/graph/ast_parser/` | Rust Dev 1 | ☐ Not Started |
| `build_bm25_index.py` | `crates/cds-index/src/index/bm25.rs` | Rust Dev 2 | ☐ Not Started |
| `repo_index/index/code_index.py` | `crates/cds-index/src/index/search.rs` | Rust Dev 2 | ☐ Not Started |
| `plugins/location_tools/` | `crates/cds-tools/src/commands/` | Rust Dev 2 | ☐ Not Started |

**Validation**: Before completing any module, verify:

```bash
# Check that Rust module exposes same functionality
cargo test --package cds-index --test module_api_tests
```

### 2.2 Tree-sitter Query Reuse

**Decision**: Reuse LocAgent's `.scm` query files verbatim for AST parsing.

**Rationale**:

- Guarantees identical entity extraction logic
- Avoids reimplementation errors
- Proven by LocAgent paper benchmarks

**Implementation**:

```rust
// crates/cds-index/src/graph/ast_parser/queries/python.scm
// (Copied directly from tmp/LocAgent/repo_index/codeblocks/parser/queries/python.scm)

const PYTHON_CLASS_QUERY: &str = include_str!("queries/python.scm");
```

**Validation**:

```bash
# Verify .scm files match LocAgent exactly
diff crates/cds-index/src/graph/ast_parser/queries/python.scm \
     tmp/LocAgent/repo_index/codeblocks/parser/queries/python.scm
```

---

## 3. Algorithm Preservation

This section documents **exact algorithms** from LocAgent that must be preserved in Rust.

### 3.1 Graph Construction Algorithm

#### 3.1.1 Directory Traversal (LocAgent `build_graph.py::build_repo_graph()`)

**Python Implementation** (simplified):

```python
def build_repo_graph(repo_path):
    graph = Graph()
    for root, dirs, files in os.walk(repo_path):
        for file in files:
            if file.endswith('.py'):
                parse_file(file, graph)
    return graph
```

**Rust Implementation** (parity-preserving):

```rust
use walkdir::WalkDir;

pub fn build_repo_graph(repo_path: &Path) -> Result<CodeGraph> {
    let mut graph = CodeGraph::new();

    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("py"))
    {
        parse_file(entry.path(), &mut graph)?;
    }

    Ok(graph)
}
```

**Validation Checklist**:

- [ ] Same directory walk order (alphabetical by default)
- [ ] Same file filtering (`.py` extension only)
- [ ] Same error handling (skip unreadable files, log warning)

**Test**:

```bash
cargo test --test graph_builder_tests::test_directory_traversal_order
```

#### 3.1.2 Entity Extraction (LocAgent tree-sitter queries)

**LocAgent Query** (`repo_index/codeblocks/parser/queries/python.scm`):

```scheme
(class_definition
  name: (identifier) @class.name
  body: (block) @class.body) @class.def

(function_definition
  name: (identifier) @function.name
  parameters: (parameters) @function.params
  body: (block) @function.body) @function.def
```

**Rust Implementation**:

```rust
const PYTHON_QUERY: &str = include_str!("queries/python.scm");

pub fn extract_entities(tree: &Tree, source: &str) -> Vec<Entity> {
    let query = Query::new(tree.language(), PYTHON_QUERY).unwrap();
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    // Extract entities from matches following LocAgent logic exactly
}
```

**Validation Checklist**:

- [ ] Nested class extraction matches LocAgent
- [ ] Function within class matches LocAgent
- [ ] Qualified name format matches LocAgent (`module.Class.method`)

**Test**:

```bash
# Parse LocAgent's own codebase, compare entity counts
cargo test --test graph_builder_tests::test_locagent_repo_entity_count
# Expected: ~1200 entities (match Python output)
```

#### 3.1.3 Edge Creation (contain, import, invoke, inherit)

**LocAgent Logic**:

1. **Contain**: Parent directory/file/class → child file/class/function
2. **Import**: Module → imported module (from `import` statements)
3. **Invoke**: Function → called function (from function call AST nodes)
4. **Inherit**: Class → parent class (from class definition bases)

**Rust Implementation**:

Preserve LocAgent's edge creation order and logic exactly. See PRD-06 §2.1 for detailed algorithm.

**Validation**:

```bash
# Compare edge counts by type
cargo test --test graph_builder_tests::test_edge_type_counts
# Expected: contain > invoke > import > inherit (same proportions as LocAgent)
```

### 3.2 BM25 Indexing Algorithm

#### 3.2.1 Tokenization (LocAgent `repo_index/utils/tokenizer.py`)

**Python Implementation** (simplified):

```python
def tokenize_code(code):
    # Split camelCase: "getUserName" → ["get", "User", "Name"]
    tokens = re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?=[A-Z]|$)|\d+', code)

    # Split snake_case: "get_user_name" → ["get", "user", "name"]
    tokens = [t.split('_') for t in tokens]
    tokens = [item for sublist in tokens for item in sublist]  # Flatten

    # Remove stop words
    tokens = [t.lower() for t in tokens if t.lower() not in STOP_WORDS]

    return tokens
```

**Rust Implementation**:

```rust
use regex::Regex;

lazy_static! {
    static ref CAMEL_CASE_RE: Regex = Regex::new(r"[A-Z]?[a-z]+|[A-Z]+(?=[A-Z]|$)|\d+").unwrap();
    static ref STOP_WORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("the"); s.insert("a"); s.insert("is"); // ... (match LocAgent's list exactly)
        s
    };
}

pub fn tokenize_code(code: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for token in CAMEL_CASE_RE.find_iter(code) {
        for sub in token.as_str().split('_') {
            let lower = sub.to_lowercase();
            if !STOP_WORDS.contains(lower.as_str()) {
                tokens.push(lower);
            }
        }
    }

    tokens
}
```

**Validation Checklist**:

- [ ] camelCase splitting matches LocAgent (`getUserName` → `["get", "user", "name"]`)
- [ ] snake_case splitting matches LocAgent (`get_user_name` → `["get", "user", "name"]`)
- [ ] Stop word list matches LocAgent exactly
- [ ] Lowercasing applied consistently

**Test**:

```bash
cargo test --test tokenizer_tests::test_camel_case_splitting
cargo test --test tokenizer_tests::test_snake_case_splitting
cargo test --test tokenizer_tests::test_stop_words
```

#### 3.2.2 BM25 Scoring (LocAgent `repo_index/index/bm25.py`)

**Parameters** (from PRD-06 §2.2):

- **k1**: 1.5 (term frequency saturation)
- **b**: 0.75 (length normalization)

**Formula**:

$$
\text{BM25}(D, Q) = \sum_{i=1}^{n} \text{IDF}(q_i) \cdot \frac{f(q_i, D) \cdot (k_1 + 1)}{f(q_i, D) + k_1 \cdot (1 - b + b \cdot \frac{|D|}{\text{avgdl}})}
$$

**Rust Implementation**:

```rust
pub struct BM25Index {
    inverted_index: HashMap<String, Vec<(String, f32)>>,  // term → [(doc_id, tf), ...]
    doc_lengths: HashMap<String, usize>,
    avg_doc_len: f32,
    k1: f32,
    b: f32,
}

impl BM25Index {
    pub fn new() -> Self {
        Self {
            inverted_index: HashMap::new(),
            doc_lengths: HashMap::new(),
            avg_doc_len: 0.0,
            k1: 1.5,  // Match LocAgent exactly
            b: 0.75,  // Match LocAgent exactly
        }
    }

    pub fn search(&self, query: &str) -> Vec<(String, f32)> {
        // Implement BM25 formula exactly as LocAgent
    }
}
```

**Validation Checklist**:

- [ ] IDF calculation matches LocAgent (log formula)
- [ ] TF normalization matches LocAgent
- [ ] Length normalization matches LocAgent
- [ ] Final scores within ±0.01 of LocAgent (floating-point tolerance)

**Test**:

```bash
# Compare BM25 scores on 10 sample queries
cargo test --test bm25_tests::test_score_parity_with_locagent
# Expected: All scores within 0.01 tolerance
```

### 3.3 Graph Traversal Algorithm

#### 3.3.1 BFS Traversal (LocAgent `dependency_graph/traverse_graph.py`)

**Python Implementation** (simplified):

```python
def traverse_graph(graph, start, depth, relations, entity_types):
    queue = [(start, 0)]
    visited = set()
    result = []

    while queue:
        node, d = queue.pop(0)
        if d > depth or node in visited:
            continue

        visited.add(node)
        result.append(node)

        for edge in graph.edges_from(node):
            if edge.type in relations:
                queue.append((edge.target, d + 1))

    return result
```

**Rust Implementation**:

```rust
use std::collections::{VecDeque, HashSet};

pub fn traverse_graph(
    graph: &CodeGraph,
    start: &NodeID,
    depth: usize,
    relations: &[RelationType],
    entity_types: &[EntityType],
) -> Vec<NodeID> {
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), 0));

    let mut visited = HashSet::new();
    let mut result = Vec::new();

    while let Some((node, d)) = queue.pop_front() {
        if d > depth || visited.contains(&node) {
            continue;
        }

        visited.insert(node.clone());
        result.push(node.clone());

        for edge in graph.edges_from(&node) {
            if relations.contains(&edge.relation) {
                queue.push_back((edge.target.clone(), d + 1));
            }
        }
    }

    result
}
```

**Validation Checklist**:

- [ ] BFS queue-based (not DFS)
- [ ] Depth limit enforced correctly
- [ ] Relation filtering matches LocAgent
- [ ] Entity type filtering matches LocAgent
- [ ] Visited set prevents cycles

**Test**:

```bash
# Run 10 traversal scenarios, compare node sets
cargo test --test traverse_tests::test_traversal_parity
# Expected: 10/10 exact matches
```

---

## 4. Output Format Preservation

### 4.1 Fold Snippet Format

**LocAgent Format**:

```text
{type} {name} - {file}:{line}
```

**Example**:

```text
function parse_file - dependency_graph/build_graph.py:42
```

**Rust Implementation**:

```rust
pub fn format_fold(entity: &Entity) -> String {
    format!("{} {} - {}:{}",
        entity.entity_type,
        entity.name,
        entity.file_path,
        entity.line_range.0  // Start line
    )
}
```

**Validation**:

```bash
# Generate fold snippets, compare character-by-character
cargo test --test format_tests::test_fold_snippet_parity
```

### 4.2 Preview Snippet Format

**LocAgent Format**:

- Signature line
- First 5 lines of code body

**Example**:

```python
def parse_file(file_path, graph):
    \"\"\"Parse a Python file and add entities to graph.\"\"\"
    with open(file_path, 'r') as f:
        source = f.read()
    tree = parser.parse(source)
```

**Rust Implementation**:

```rust
pub fn format_preview(entity: &Entity) -> String {
    let lines: Vec<&str> = entity.code.lines().collect();
    let preview_lines = lines.iter().take(5).cloned().collect::<Vec<_>>();
    preview_lines.join("\n")
}
```

**Validation**:

```bash
# Compare preview line counts and content
cargo test --test format_tests::test_preview_snippet_parity
```

### 4.3 Tree Format

**LocAgent Format**:

```text
src/
├─[contain]→ dependency_graph/
│   ├─[contain]→ build_graph.py
│   │   ├─[contain]→ function build_repo_graph
│   │   └─[contain]→ function parse_file
│   └─[contain]→ traverse_graph.py
└─[contain]→ repo_index/
```

**Rust Implementation**:

Use `├─`, `│`, `└─` box-drawing characters exactly as LocAgent.

**Validation**:

```bash
# Generate tree output, diff character-by-character
cargo test --test format_tests::test_tree_format_parity
```

---

## 5. Performance Validation

### 5.1 Target Metrics (from PRD-06 §5.3)

| Metric | LocAgent (Python) Baseline | CDSAgent (Rust) Target | Speedup |
|--------|---------------------------|----------------------|---------|
| Index 1K files | ~5s | <3s | 1.6x faster |
| Search query | ~200ms | <100ms | 2x faster |
| Traverse 2-hop | ~500ms | <200ms | 2.5x faster |
| Memory (10K files) | ~3GB | <2GB | 1.5x improvement |

### 5.2 Benchmark Setup

**Environment**:

- **Hardware**: Same machine for Python vs Rust benchmarks
- **Warmup**: Run 10 iterations before measuring
- **Iterations**: Average of 100 runs for search/traverse, 10 runs for indexing

**Python Baseline Extraction**:

```bash
# Index benchmark
cd tmp/LocAgent
time python dependency_graph/build_graph.py \
  --repo_path tests/fixtures/parity/sample_repos/pytest \
  --output /tmp/graph.json

# Search benchmark
hyperfine --warmup 10 --runs 100 \
  'python plugins/location_tools/locationtools.py search_entities --query "User"'

# Traverse benchmark
hyperfine --warmup 10 --runs 100 \
  'python plugins/location_tools/locationtools.py traverse_graph --start "src/" --depth 2'
```

**Rust Benchmark**:

```bash
# Index benchmark
cargo bench --bench graph_build_bench

# Search benchmark
cargo bench --bench search_bench

# Traverse benchmark
cargo bench --bench traverse_bench
```

### 5.3 Performance Test Automation

**Criterion Benchmark Example**:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_search_query(c: &mut Criterion) {
    let index = setup_test_index();

    c.bench_function("search_user_query", |b| {
        b.iter(|| {
            index.search(black_box("User"))
        });
    });
}

criterion_group!(benches, bench_search_query);
criterion_main!(benches);
```

**Validation**:

```bash
# Run all performance benchmarks, compare to baselines
cargo bench --all
# Expected: All metrics meet or exceed targets
```

---

## 6. Unit Test Coverage

### 6.1 Coverage Targets

| Crate | Target Coverage | Minimum Coverage | Validation |
|-------|----------------|------------------|------------|
| `cds-index` (core) | >95% | 90% | `cargo tarpaulin` |
| `cds-tools` (CLI) | >85% | 80% | `cargo tarpaulin` |
| `cds-agent` (TS) | >80% | 75% | `bun test --coverage` |

### 6.2 Coverage Measurement

**Rust**:

```bash
# Install tarpaulin (if not already installed)
cargo install cargo-tarpaulin

# Run coverage for all crates
cargo tarpaulin --all-features --workspace --timeout 300 --out Html --output-dir coverage/

# Check specific crate
cargo tarpaulin --package cds-index --out Stdout
```

**TypeScript**:

```bash
# Run Jest with coverage
cd cds-agent
bun test --coverage

# Check coverage thresholds
bun test --coverage --coverageReporters=text-summary
```

### 6.3 Coverage Gates

**PR Merge Requirements**:

- [ ] All new code has unit tests
- [ ] Coverage does not decrease (compared to main branch)
- [ ] Core modules (graph, index) maintain >95% coverage

**CI Integration**:

```yaml
# .github/workflows/coverage.yml
- name: Run coverage check
  run: cargo tarpaulin --all --out Xml
- name: Upload to Codecov
  uses: codecov/codecov-action@v3
- name: Fail if coverage < 90%
  run: |
    COVERAGE=$(cargo tarpaulin --all --out Stdout | grep "Coverage" | awk '{print $2}')
    if [ "${COVERAGE%\%}" -lt 90 ]; then exit 1; fi
```

---

## 7. Continuous Validation Strategy

### 7.1 Phase-Gated Checkpoints

#### Checkpoint 1: Phase 1 - Week 2 (Graph Construction)

**Required**:

- [ ] Graph builder parity validated on LocAgent's own codebase
- [ ] Entity count variance ≤2%
- [ ] Edge count variance ≤2%
- [ ] Qualified name format matches LocAgent exactly

**Validation Command**:

```bash
cargo test --test graph_parity_tests
./scripts/parity-check.sh --graph
```

**Sign-off**: Rust Lead must approve before T-02-02 (Sparse Index) can start.

#### Checkpoint 2: Phase 2 - Week 5 (BM25 Search + Traversal)

**Required**:

- [ ] BM25 search results validated (50 queries, ≥90% overlap@10)
- [ ] Traversal outputs match LocAgent exactly (10/10 samples)
- [ ] Performance targets for search (<100ms) and traverse (<200ms) met

**Validation Command**:

```bash
cargo test --test search_parity_tests
cargo test --test traverse_parity_tests
./scripts/parity-check.sh --search --traverse
cargo bench --bench search_bench --bench traverse_bench
```

**Sign-off**: Rust Lead + Tech Lead must approve before T-04-01 (Agent SDK) can start.

#### Checkpoint 3: Phase 3 - Week 7 (Performance Targets)

**Required**:

- [ ] Index build time <3s for 1K files
- [ ] Search latency <100ms p95
- [ ] Traverse latency <200ms p95
- [ ] Memory usage <2GB for 10K files

**Validation Command**:

```bash
cargo bench --all
./scripts/parity-check.sh --performance
```

**Sign-off**: Rust Lead must approve before Phase 4 (Production) starts.

#### Checkpoint 4: Phase 4 - Week 10 (SWE-bench Lite Parity)

**Required**:

- [ ] Full SWE-bench Lite benchmark run
- [ ] Accuracy@5 ≥80% (matching LocAgent's published results)
- [ ] All parity tests passing (graph, search, traverse, performance)

**Validation Command**:

```bash
cargo test --all --release
./scripts/parity-check.sh --all
./scripts/run-swe-bench-lite.sh
```

**Sign-off**: Rust Lead + Tech Lead + PM must approve for v0.1.0-MVP release.

### 7.2 Weekly Parity Reviews

**Schedule**: Every Friday, 2pm (30 minutes)

**Attendees**: Rust Lead, All Rust Developers, Tech Lead

**Agenda**:

1. Review parity test results from the week
2. Discuss any deviations from LocAgent (>5% variance)
3. Update golden outputs if LocAgent baseline changed
4. Plan fixes for failing parity tests

**Artifacts**:

- Weekly parity report (generated by `./scripts/parity-check.sh --report`)
- Updated TODO.yaml with parity status

---

## 8. Automated Regression Tests

### 8.1 Parity Check Script Usage

**Script**: `scripts/parity-check.sh`

**Usage**:

```bash
# Run all parity checks
./scripts/parity-check.sh --all

# Run specific checks
./scripts/parity-check.sh --graph
./scripts/parity-check.sh --search
./scripts/parity-check.sh --traverse
./scripts/parity-check.sh --performance

# Generate report (for weekly reviews)
./scripts/parity-check.sh --all --report > parity-report-$(date +%Y-%m-%d).md
```

**Output Format**:

```text
✅ Graph Parity Check: PASSED
   - Node count: 1234 (expected: 1234, variance: 0%)
   - Edge count: 5678 (expected: 5678, variance: 0%)

✅ Search Parity Check: PASSED
   - Overlap@10: 48/50 queries (96% match)
   - Top-5 rank order: 50/50 exact matches (100%)

❌ Traverse Parity Check: FAILED
   - Exact matches: 9/10 scenarios
   - FAILED: scenario "inherit_depth_2" (expected 15 nodes, got 14)

⚠️  Performance Check: WARNING
   - Index build: 3.2s (target: <3s) - MARGINAL
   - Search latency: 85ms (target: <100ms) - PASSED
   - Traverse latency: 180ms (target: <200ms) - PASSED
```

### 8.2 CI Integration

**GitHub Actions Workflow** (`.github/workflows/parity-check.yml`):

```yaml
name: Parity Validation

on:
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday

jobs:
  parity-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run parity checks
        run: |
          chmod +x scripts/parity-check.sh
          ./scripts/parity-check.sh --all

      - name: Fail if parity drops
        run: |
          if [ $? -ne 0 ]; then
            echo "❌ Parity check failed. Please review and fix."
            exit 1
          fi
```

**PR Merge Requirements**:

- [ ] Parity check CI job passes
- [ ] No regressions in graph/search/traverse parity
- [ ] Performance does not degrade

---

## 9. Troubleshooting

### 9.1 Common Issues

#### Issue: Graph variance >2%

**Symptoms**:

```text
❌ Graph Parity Check: FAILED
   - Node count: 1250 (expected: 1234, variance: 1.3%)
   - Edge count: 5800 (expected: 5678, variance: 2.1%)
```

**Debugging Steps**:

1. **Check directory traversal order**:

   ```bash
   # Compare file lists
   diff <(python tmp/LocAgent/scripts/list_files.py) \
        <(cargo run --bin cds-list-files)
   ```

2. **Compare tree-sitter queries**:

   ```bash
   diff crates/cds-index/src/graph/ast_parser/queries/python.scm \
        tmp/LocAgent/repo_index/codeblocks/parser/queries/python.scm
   ```

3. **Debug entity extraction**:

   ```bash
   # Enable debug logging
   RUST_LOG=debug cargo test --test graph_builder_tests::test_entity_extraction
   ```

4. **Inspect specific files with mismatches**:

   ```bash
   # Find files with different entity counts
   ./scripts/debug-graph-diff.sh
   ```

**Resolution**:

- Fix tree-sitter query if different from LocAgent
- Ensure qualified name format matches LocAgent exactly
- Check for edge cases (nested classes, decorators, etc.)

#### Issue: Search overlap@10 <90%

**Symptoms**:

```text
❌ Search Parity Check: FAILED
   - Overlap@10: 40/50 queries (80% match)
   - Failing queries: "sanitize input", "parse_ast", ...
```

**Debugging Steps**:

1. **Check tokenization**:

   ```bash
   cargo test --test tokenizer_tests -- --nocapture
   # Compare tokenized output with LocAgent
   ```

2. **Verify BM25 parameters**:

   ```rust
   // Ensure k1=1.5, b=0.75
   assert_eq!(index.k1, 1.5);
   assert_eq!(index.b, 0.75);
   ```

3. **Compare BM25 scores**:

   ```bash
   # Run search with score logging
   RUST_LOG=debug cargo run --bin cds search "sanitize input" --repo . --debug
   # Compare with LocAgent scores
   ```

4. **Check stop word list**:

   ```bash
   diff crates/cds-index/src/index/stop_words.txt \
        tmp/LocAgent/repo_index/utils/stop_words.txt
   ```

**Resolution**:

- Fix tokenization to match LocAgent's regex
- Ensure stop word list is identical
- Verify IDF calculation formula
- Check for floating-point precision issues (use 0.01 tolerance)

#### Issue: Traversal exact match failed

**Symptoms**:

```text
❌ Traverse Parity Check: FAILED
   - Exact matches: 9/10 scenarios
   - FAILED: scenario "invoke_depth_2" (expected 20 nodes, got 18)
```

**Debugging Steps**:

1. **Visualize graph difference**:

   ```bash
   ./scripts/visualize-graph-diff.sh invoke_depth_2
   # Outputs: golden_graph.dot, actual_graph.dot
   dot -Tpng golden_graph.dot -o golden.png
   dot -Tpng actual_graph.dot -o actual.png
   ```

2. **Check BFS traversal logic**:

   ```bash
   cargo test --test traverse_tests::test_bfs_order -- --nocapture
   ```

3. **Verify relation filtering**:

   ```bash
   # Ensure "invoke" edges are being followed
   cargo test --test traverse_tests::test_relation_filter
   ```

4. **Compare visited sets**:

   ```bash
   # Log visited nodes
   RUST_LOG=trace cargo test --test traverse_tests::test_invoke_depth_2
   ```

**Resolution**:

- Fix BFS queue handling (use `VecDeque`)
- Ensure relation filtering matches LocAgent
- Check for off-by-one errors in depth limit
- Verify cycle detection (visited set)

### 9.2 Escalation Path

**If parity cannot be achieved after debugging:**

1. **Document the issue**:
   - Create GitHub issue with `parity` label
   - Include: failing test, expected vs actual output, debugging steps tried
   - Tag: Rust Lead + Tech Lead

2. **Consult LocAgent paper/code**:
   - Review relevant sections of arXiv-2503.09089v2
   - Check LocAgent GitHub issues for similar problems

3. **Team discussion**:
   - Schedule emergency parity review meeting
   - Decide: fix Rust code, update baseline, or accept deviation with justification

4. **Update documentation**:
   - If accepting deviation: document in `CHANGELOG.md` with rationale
   - Update parity thresholds if necessary

---

## Appendix A: Parity Test Examples

### Example 1: Graph Parity Test

```rust
#[test]
fn test_graph_parity_locagent_repo() {
    // Load golden output
    let golden: GraphStructure = serde_json::from_str(
        include_str!("../../tests/fixtures/parity/golden_outputs/graph_locagent.json")
    ).unwrap();

    // Build graph with CDSAgent
    let graph = build_repo_graph(Path::new("tmp/LocAgent")).unwrap();

    // Compare node counts
    assert_eq!(graph.node_count(), golden.total_nodes);

    // Compare edge counts
    assert_eq!(graph.edge_count(), golden.total_edges);

    // Compare node counts by type
    for (entity_type, expected_count) in &golden.node_counts_by_type {
        let actual_count = graph.nodes_by_type(entity_type).len();
        let variance = ((actual_count as f64 - *expected_count as f64).abs() / *expected_count as f64) * 100.0;
        assert!(variance <= 2.0, "Node count variance for {:?}: {:.2}%", entity_type, variance);
    }
}
```

### Example 2: Search Parity Test

```rust
#[test]
fn test_search_parity_50_queries() {
    let index = setup_test_index();

    // Load golden queries
    let queries: Vec<SearchQuery> = read_jsonl("tests/fixtures/parity/golden_outputs/search_queries.jsonl");

    let mut matches = 0;
    for query in &queries {
        let results = index.search(&query.query).take(10).collect::<Vec<_>>();

        // Compare top-10 results
        let overlap = calculate_overlap(&results, &query.top_10_results);
        if overlap >= 0.9 {
            matches += 1;
        }
    }

    assert!(matches >= 45, "Search overlap@10: {}/50 (expected ≥45)", matches);
}
```

---

## 10. Baseline Extraction CLI

### 10.1 Overview

The `swe-lite` CLI provides a unified interface for extracting parity baselines from LocAgent and SWE-bench Lite repositories. All operations use `uv` for dependency management.

### 10.2 Installation

```bash
# Ensure uv is installed
curl -LsSf https://astral.sh/uv/install.sh | sh

# Verify environment
./scripts/swe-lite check
```

### 10.3 Complete Workflow

```bash
# Step 1: Select 5 diverse SWE-bench Lite instances
./scripts/swe-lite select
# Output: tests/fixtures/parity/swe-bench-lite/samples.yaml

# Step 2: Fetch repositories
./scripts/swe-lite fetch
# Output: .artifacts/tmp/swe-bench-lite/<instance_id>/

# Step 3: Extract all baselines
./scripts/swe-lite baseline all
# Outputs:
#   - tests/fixtures/parity/golden_outputs/graph_*.json
#   - tests/fixtures/parity/golden_outputs/search_queries.jsonl
#   - tests/fixtures/parity/golden_outputs/traverse_samples.jsonl
#   - tests/fixtures/parity/golden_outputs/performance_baselines.json
```

### 10.4 Individual Baseline Types

```bash
# Extract only graph baselines (nodes + edges)
./scripts/swe-lite baseline graph

# Extract only search baselines (50 queries × N repos)
./scripts/swe-lite baseline search

# Extract only traversal baselines (10 scenarios × N repos)
./scripts/swe-lite baseline traverse

# Extract only performance baselines (timing + memory)
./scripts/swe-lite baseline perf
```

### 10.5 Output Locations

| Artifact | Location | Description |
|----------|----------|-------------|
| Instance metadata | `tests/fixtures/parity/swe-bench-lite/samples.yaml` | Instance IDs, repos, commits |
| Repositories | `.artifacts/tmp/swe-bench-lite/<id>/` | Ephemeral (gitignored) |
| Graph baselines | `tests/fixtures/parity/golden_outputs/graph_*.json` | Full nodes + edges |
| Search baselines | `tests/fixtures/parity/golden_outputs/search_queries.jsonl` | 50 queries/repo |
| Traversal baselines | `tests/fixtures/parity/golden_outputs/traverse_samples.jsonl` | 10 scenarios/repo |
| Performance baselines | `tests/fixtures/parity/golden_outputs/performance_baselines.json` | Timing + memory |

### 10.6 Regenerating Baselines

When LocAgent is updated or you need to refresh baselines:

```bash
# Re-extract LocAgent baseline only
./scripts/swe-lite baseline graph

# Full regeneration (all repos, all baseline types)
rm -rf tests/fixtures/parity/golden_outputs/*.{json,jsonl}
./scripts/swe-lite baseline all
```

### 10.7 Underlying Python Scripts

The CLI wraps these scripts (for advanced usage):

| Script | Purpose |
|--------|---------|
| `select-swe-bench-instances.py` | Select diverse instances from dataset |
| `fetch-swe-bench-lite.py` | Clone repos at specific commits |
| `extract-parity-baseline.py` | Extract graph with nodes/edges |
| `extract-search-baseline.py` | Extract BM25 search results |
| `extract-traverse-baseline.py` | Extract graph traversal results |
| `benchmark-performance.py` | Measure build/search/traverse metrics |

All scripts use `uv run` for dependency management and set `PYTHONPATH` to include LocAgent.

### 10.8 Troubleshooting

**Problem**: `uv: command not found`

```bash
# Install uv
curl -LsSf https://astral.sh/uv/install.sh | sh
source ~/.bashrc  # or ~/.zshrc
```

**Problem**: `LocAgent not found`

```bash
# Ensure LocAgent is present
ls -la tmp/LocAgent/
# Should show dependency_graph/, plugins/, etc.
```

**Problem**: `datasets module not found`

```bash
# Install dataset dependencies
cd tmp/LocAgent
uv pip install datasets pyyaml gitpython psutil
```

**Problem**: Graph baseline too large (>5MB)

```bash
# Use --max-files to limit extraction
python scripts/extract-parity-baseline.py \
    --repo-path .artifacts/tmp/swe-bench-lite/<id> \
    --repo-name <id> \
    --output tests/fixtures/parity/golden_outputs/graph_<id>.json \
    --max-files 500
```

---

## 10. Known Limitations

### 10.1 Search & Performance Baseline Extraction (LocAgent Dependency Issue)

**Status**: Partial baselines available (LocAgent only, SWE-bench repos blocked)
**Impact**: Does NOT block CDSAgent development
**Resolution**: CDSAgent Rust implementation will not have this limitation

#### Problem Description

During T-06-01 Phase 2 baseline extraction, we discovered that `llama-index` (v0.11.22) `SimpleDirectoryReader` has a validation bug that prevents search and performance baseline extraction for SWE-bench repos with code in subdirectories.

**Technical Details**:

```python
# LocAgent's bm25_retriever.py uses:
reader = SimpleDirectoryReader(
    input_dir=repo_path,
    required_exts=['.py'],  # ← Validates at root level BEFORE recursing
    recursive=True,
)
```

For repos where Python code is in subdirectories (e.g., `django/django/`, `sklearn/sklearn/`), the validator fails:

```text
ValueError: No files found in /path/to/repo.
```

**Affected Baselines**:

- ❌ `search_queries.jsonl`: Only contains LocAgent data (1/6 repos)
- ❌ `performance_baselines.json`: Only contains LocAgent data (1/6 repos)
- ✅ `graph_*.json`: Complete for all 6 repos (most critical)
- ✅ `traverse_samples.jsonl`: Complete for all 60 scenarios

#### Why This Doesn't Block Development

1. **Graph baselines (core parity validation)**: ✅ Complete
   - T-02-01 Graph Builder primarily validated against graph structure
   - Node/edge counts, type distributions, qualified names all captured

2. **Traverse baselines (important)**: ✅ Complete
   - 60 scenarios (10 × 6 repos) for graph traversal validation
   - Covers all edge types: contains, imports, invokes, inherits

3. **Search baselines (secondary)**:
   - Used for BM25 overlap@10 validation (nice-to-have metric)
   - CDSAgent will implement BM25 independently in Rust using `tantivy` or similar
   - Can run live comparison: CDSAgent search vs LocAgent search (both systems running)

4. **Performance baselines (supplementary)**:
   - T-08-04 will generate CDSAgent's own performance benchmarks
   - Comparison will be live: CDSAgent vs LocAgent on same repos

#### Workarounds Attempted

1. ❌ **Package directory detection**: llama-index validates before accepting subdirectory paths
2. ❌ **Dummy file creation**: Created `.py` file at root, but llama-index still fails (caching or dotfile exclusion)
3. ❌ **Path preprocessing**: Cannot modify LocAgent's code without maintenance burden
4. ✅ **Accepted limitation**: Document and proceed; not a blocker

#### Impact on Milestone M1 & M2

**M1 (API Contracts & Parity)**:

- T-06-01 acceptance criteria adjusted:
  - ✅ Graph baselines: Complete (core requirement)
  - ✅ Traverse baselines: Complete (important)
  - ⚠️ Search/perf baselines: Partial (documented limitation)

**M2 (Core Indexing Prototype)**:

- T-02-01 Graph Builder: ✅ No impact (uses graph baselines)
- T-02-02 Sparse Index: ✅ No impact (Rust BM25, no llama-index dependency)
- T-02-04 Serialization: ✅ No impact

#### CDSAgent Implementation Plan

**T-02-02 BM25 Search Implementation** (Rust):

```rust
// crates/cds-index/src/index/bm25.rs
use walkdir::WalkDir;

pub fn build_from_repo(repo_path: &Path) -> Result<BM25Index> {
    // Native Rust directory traversal - no llama-index limitation
    let py_files: Vec<PathBuf> = WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some("py"))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Works for ALL repos including SWE-bench ✅
    build_index(py_files)
}
```

**T-08-03 Parity Validation Strategy**:

```bash
# Primary validation (graph structure)
./scripts/parity-check.sh graph django__django-10914
# Uses: tests/fixtures/parity/golden_outputs/graph_*.json ✅

# Traverse validation
./scripts/parity-check.sh traverse django__django-10914
# Uses: tests/fixtures/parity/golden_outputs/traverse_samples.jsonl ✅

# Search validation (live comparison)
cds-tools search "query" --repo django__django-10914 > cdsagent_results.json
python tmp/LocAgent/auto_search_main.py --query "query" > locagent_results.json
./scripts/compare-search-results.py cdsagent_results.json locagent_results.json
# Compares live runs, no baseline needed ✅
```

#### Resolution Timeline

- **Short-term (M1-M2)**: Documented limitation, proceed with available baselines
- **Medium-term (M3-M4)**: CDSAgent BM25 implementation complete, live validation possible
- **Long-term (M5)**: Optional: Create custom baseline extractor without llama-index if needed for regression testing

#### Documentation Updates

- ✅ `tests/fixtures/parity/golden_outputs/README.md`: Detailed explanation
- ✅ `docs/parity-validation-methodology.md`: This section
- ✅ Task spec: `spacs/tasks/0.1.0-mvp/06-refactor-parity/T-06-01-parity-methodology.md`
- ✅ TODO.yaml: Acceptance criteria adjusted

#### References

- llama-index issue: `SimpleDirectoryReader` validates `required_exts` at root before recursion
- LocAgent source: `tmp/LocAgent/plugins/location_tools/retriever/bm25_retriever.py:72`
- Baseline files: `tests/fixtures/parity/golden_outputs/`

---

End of Parity Validation Methodology

**Next Steps**:

1. Run `./scripts/swe-lite check` to verify environment
2. Extract baselines: `./scripts/swe-lite select && ./scripts/swe-lite fetch && ./scripts/swe-lite baseline all`
3. Use this methodology when implementing T-02-01 (Graph Builder)
4. Run `./scripts/parity-check.sh` before every PR
5. Update this document as we discover new validation needs

**Questions?** Contact Rust Lead or open an issue with `parity` label.
