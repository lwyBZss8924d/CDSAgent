# Sub-Issue 02.02: Sparse Index - Hierarchical Name/ID + BM25 Search

**Priority**: P0 (Critical Path - Foundation)
**Status**: ☐ Not Started
**Owner**: Rust Dev 2
**Parent**: [02-index-core/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-02 §2.2](../../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-06 §2.2](../../../prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md)
**Timing**: Phase 1-2, Week 3-4

---

## Objective

Implement two-tier hierarchical sparse indexing that combines fast name/ID lookup (upper index) with BM25 content search (lower index), maintaining exact algorithmic fidelity with LocAgent's SearchEntity tool.

## Scope

**In Scope**:

- HashMap-based upper index (name/ID exact + prefix matching)
- BM25 lower index (tantivy integration or custom implementation)
- Hierarchical search strategy (upper → BM25 fallback)
- Validate against LocAgent search results (50 queries, ≥90% overlap)

**Out of Scope (v0.2.0)**:

- Semantic/vector search
- Cross-repository federated search
- Real-time incremental updates

---

## Dependencies

- **Requires**: [01-graph-build.md](01-graph-build.md) (graph structure must exist)
- **Blocks**: [03-service-layer.md](03-service-layer.md) (service needs search API)
- **Validates With**: [../06-refactor-parity.md](../06-refactor-parity.md)

---

## Implementation Tasks

### Week 3, Day 1-2: Upper Index (Name/ID)

Task 1: Name/ID Index Structure

```rust
// cds_sparse_index/src/name_index.rs
use std::collections::HashMap;
use crate::graph::{NodeID, EntityType};

pub struct NameIndex {
    // Exact match: name → list of entity IDs
    exact_map: HashMap<String, Vec<NodeID>>,
    // Prefix search support (optional: use trie or sorted keys)
    sorted_keys: Vec<String>,
}

impl NameIndex {
    pub fn new() -> Self {
        Self {
            exact_map: HashMap::new(),
            sorted_keys: Vec::new(),
        }
    }

    pub fn insert(&mut self, name: String, entity_id: NodeID) {
        self.exact_map.entry(name.clone()).or_insert_with(Vec::new).push(entity_id);
    }

    pub fn finalize(&mut self) {
        // Sort keys for prefix search
        self.sorted_keys = self.exact_map.keys().cloned().collect();
        self.sorted_keys.sort();
    }

    pub fn exact_match(&self, name: &str) -> Option<&Vec<NodeID>> {
        self.exact_map.get(name)
    }

    pub fn prefix_match(&self, prefix: &str, limit: usize) -> Vec<NodeID> {
        let mut results = Vec::new();
        for key in &self.sorted_keys {
            if key.starts_with(prefix) {
                if let Some(ids) = self.exact_map.get(key) {
                    results.extend(ids.iter().cloned());
                    if results.len() >= limit {
                        break;
                    }
                }
            }
        }
        results.truncate(limit);
        results
    }
}
```

**Acceptance**:

- [ ] Exact match returns all entities with that name
- [ ] Prefix match returns all entities starting with prefix
- [ ] O(1) average case for exact match
- [ ] Prefix search uses sorted keys or trie

---

Task 2: Entity Type Filtering

```rust
// cds_sparse_index/src/name_index.rs (extended)
use crate::graph::CodeGraph;

pub struct NameIndexWithGraph<'a> {
    index: NameIndex,
    graph: &'a CodeGraph,
}

impl<'a> NameIndexWithGraph<'a> {
    pub fn search(
        &self,
        query: &str,
        entity_type: Option<EntityType>,
        limit: usize,
    ) -> Vec<NodeID> {
        let candidates = if query.ends_with('*') {
            // Prefix search
            let prefix = &query[..query.len() - 1];
            self.index.prefix_match(prefix, limit * 2)
        } else {
            // Exact match
            self.index.exact_match(query).map(|v| v.clone()).unwrap_or_default()
        };

        // Filter by entity type if specified
        if let Some(etype) = entity_type {
            candidates
                .into_iter()
                .filter(|id| {
                    self.graph.nodes.get(id).map(|n| n.entity_type == etype).unwrap_or(false)
                })
                .take(limit)
                .collect()
        } else {
            candidates.into_iter().take(limit).collect()
        }
    }
}
```

**Acceptance**:

- [ ] Can filter results by EntityType (Directory/File/Class/Function)
- [ ] Wildcard queries (e.g., "Auth*") trigger prefix search
- [ ] Type filter applied after name matching

---

### Week 3, Day 3-5: BM25 Lower Index

Task 3: BM25 Implementation Choice

Option A: Tantivy Integration

```rust
// cds_sparse_index/src/bm25_tantivy.rs
use tantivy::{Index, IndexWriter, schema::*, collector::TopDocs};
use tantivy::query::QueryParser;

pub struct BM25Index {
    index: Index,
    reader: tantivy::IndexReader,
    schema: Schema,
}

impl BM25Index {
    pub fn new() -> tantivy::Result<Self> {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("entity_id", STRING | STORED);
        schema_builder.add_text_field("name", TEXT);
        schema_builder.add_text_field("content", TEXT);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());
        let reader = index.reader()?;

        Ok(Self { index, reader, schema })
    }

    pub fn add_entity(&mut self, entity_id: &str, name: &str, content: &str) -> tantivy::Result<()> {
        let mut writer = self.index.writer(50_000_000)?;
        let entity_id_field = self.schema.get_field("entity_id").unwrap();
        let name_field = self.schema.get_field("name").unwrap();
        let content_field = self.schema.get_field("content").unwrap();

        writer.add_document(doc!(
            entity_id_field => entity_id,
            name_field => name,
            content_field => content
        ))?;
        writer.commit()?;
        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> tantivy::Result<Vec<(String, f32)>> {
        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.schema.get_field("name").unwrap(), self.schema.get_field("content").unwrap()],
        );
        let query = query_parser.parse_query(query)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let results = top_docs
            .into_iter()
            .map(|(score, doc_address)| {
                let doc = searcher.doc(doc_address).unwrap();
                let entity_id = doc
                    .get_first(self.schema.get_field("entity_id").unwrap())
                    .unwrap()
                    .as_text()
                    .unwrap()
                    .to_string();
                (entity_id, score)
            })
            .collect();

        Ok(results)
    }
}
```

Option B: Custom BM25

```rust
// cds_sparse_index/src/bm25_custom.rs
use std::collections::HashMap;

pub struct CustomBM25 {
    inverted_index: HashMap<String, Vec<Posting>>,
    doc_lengths: HashMap<String, usize>,
    avg_doc_length: f32,
    k1: f32,
    b: f32,
}

struct Posting {
    entity_id: String,
    term_freq: usize,
    positions: Vec<usize>,
}

impl CustomBM25 {
    pub fn new(k1: f32, b: f32) -> Self {
        Self {
            inverted_index: HashMap::new(),
            doc_lengths: HashMap::new(),
            avg_doc_length: 0.0,
            k1,
            b,
        }
    }

    pub fn add_document(&mut self, entity_id: String, tokens: Vec<String>) {
        let doc_len = tokens.len();
        self.doc_lengths.insert(entity_id.clone(), doc_len);

        let mut term_freqs: HashMap<String, usize> = HashMap::new();
        for (pos, token) in tokens.iter().enumerate() {
            *term_freqs.entry(token.clone()).or_insert(0) += 1;
        }

        for (term, freq) in term_freqs {
            self.inverted_index
                .entry(term)
                .or_insert_with(Vec::new)
                .push(Posting {
                    entity_id: entity_id.clone(),
                    term_freq: freq,
                    positions: vec![], // Simplified: omit positions for v0.1.0
                });
        }
    }

    pub fn finalize(&mut self) {
        let total_len: usize = self.doc_lengths.values().sum();
        self.avg_doc_length = total_len as f32 / self.doc_lengths.len() as f32;
    }

    pub fn search(&self, query_tokens: Vec<String>, limit: usize) -> Vec<(String, f32)> {
        let mut scores: HashMap<String, f32> = HashMap::new();
        let num_docs = self.doc_lengths.len() as f32;

        for term in query_tokens {
            if let Some(postings) = self.inverted_index.get(&term) {
                let idf = ((num_docs - postings.len() as f32 + 0.5) / (postings.len() as f32 + 0.5)).ln();

                for posting in postings {
                    let doc_len = *self.doc_lengths.get(&posting.entity_id).unwrap() as f32;
                    let tf = posting.term_freq as f32;
                    let norm = 1.0 - self.b + self.b * (doc_len / self.avg_doc_length);
                    let score = idf * (tf * (self.k1 + 1.0)) / (tf + self.k1 * norm);
                    *scores.entry(posting.entity_id.clone()).or_insert(0.0) += score;
                }
            }
        }

        let mut ranked: Vec<(String, f32)> = scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        ranked.truncate(limit);
        ranked
    }
}
```

**Decision Criteria**:

- [ ] Prototype both implementations (Week 3, Day 3)
- [ ] Validate accuracy against LocAgent on 50 sample queries
- [ ] Choose tantivy if accuracy within 5% of LocAgent; otherwise custom BM25
- [ ] Document decision in [../06-refactor-parity.md](../06-refactor-parity.md)

**Acceptance** (from PRD-02 FR-HI-2):

- [ ] Returns entities ranked by BM25 score
- [ ] Query latency <500ms for typical codebases (<10K files)
- [ ] BM25 parameters: k1=1.5, b=0.75 (LocAgent defaults)
- [ ] Tokenization matches LocAgent (camelCase/snake_case splitting)

---

### Week 4: Hierarchical Search

Task 4: Hierarchical Search Strategy

```rust
// cds_sparse_index/src/search.rs
use crate::name_index::NameIndexWithGraph;
use crate::bm25::BM25Index;
use crate::graph::{CodeGraph, NodeID, EntityType};

pub struct HierarchicalSearcher<'a> {
    name_index: NameIndexWithGraph<'a>,
    bm25_index: BM25Index,
    graph: &'a CodeGraph,
}

pub struct SearchOptions {
    pub limit: usize,
    pub entity_type: Option<EntityType>,
    pub upper_threshold: usize, // Minimum upper results before BM25 fallback
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            limit: 10,
            entity_type: None,
            upper_threshold: 5,
        }
    }
}

pub struct EntityMatch {
    pub entity_id: NodeID,
    pub name: String,
    pub score: f32,
    pub source: SearchSource,
}

#[derive(Debug, Clone, Copy)]
pub enum SearchSource {
    UpperIndex,  // Exact or prefix match
    BM25,        // Content search
}

impl<'a> HierarchicalSearcher<'a> {
    pub fn search(&self, query: &str, options: SearchOptions) -> Vec<EntityMatch> {
        // Step 1: Try upper index (name/ID)
        let upper_results = self.name_index.search(query, options.entity_type, options.limit);

        // Step 2: If upper results insufficient, fallback to BM25
        let mut matches = Vec::new();

        if upper_results.len() >= options.upper_threshold {
            // Sufficient results from upper index
            for entity_id in upper_results {
                if let Some(node) = self.graph.nodes.get(&entity_id) {
                    matches.push(EntityMatch {
                        entity_id: entity_id.clone(),
                        name: node.name.clone(),
                        score: 1.0, // Upper index exact match = highest score
                        source: SearchSource::UpperIndex,
                    });
                }
            }
        } else {
            // Fallback to BM25 content search
            let bm25_results = self.bm25_index.search(query, options.limit).unwrap_or_default();

            for (entity_id, score) in bm25_results {
                if let Some(node) = self.graph.nodes.get(&entity_id) {
                    // Filter by entity type if specified
                    if let Some(etype) = options.entity_type {
                        if node.entity_type != etype {
                            continue;
                        }
                    }
                    matches.push(EntityMatch {
                        entity_id: entity_id.clone(),
                        name: node.name.clone(),
                        score,
                        source: SearchSource::BM25,
                    });
                }
            }
        }

        // Step 3: Deduplicate and rank
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        matches.truncate(options.limit);
        matches
    }
}
```

**Acceptance** (from PRD-02 FR-HI-3):

- [ ] Upper index tried first for keyword queries
- [ ] BM25 fallback when upper results < threshold (default: 5)
- [ ] Combined results ranked by score (upper=1.0 > BM25 scores)
- [ ] Deduplication if entity appears in both indices

---

## Validation Against LocAgent

### Parity Test (from PRD-06 §2.2)

```rust
// tests/parity/search_parity_test.rs
use std::collections::HashSet;

#[test]
fn test_hierarchical_search_parity() {
    let graph = build_repo_graph("tmp/LocAgent").unwrap();
    let searcher = HierarchicalSearcher::new(&graph);

    // Load 50 benchmark queries
    let queries = load_benchmark_queries("tests/fixtures/parity/search_queries.jsonl");

    let mut overlap_scores = Vec::new();

    for query in queries {
        let cds_results = searcher.search(&query.text, SearchOptions::default());
        let cds_top10: HashSet<_> = cds_results.iter().take(10).map(|m| &m.entity_id).collect();

        let locagent_top10: HashSet<_> = query.expected_results.iter().collect();

        let overlap = cds_top10.intersection(&locagent_top10).count();
        let overlap_pct = (overlap as f32 / 10.0) * 100.0;
        overlap_scores.push(overlap_pct);

        if overlap_pct < 80.0 {
            eprintln!(
                "Low overlap for query '{}': {}%\n  CDS: {:?}\n  LocAgent: {:?}",
                query.text, overlap_pct, cds_top10, locagent_top10
            );
        }
    }

    let avg_overlap = overlap_scores.iter().sum::<f32>() / overlap_scores.len() as f32;
    assert!(avg_overlap >= 90.0, "Average overlap {}% < 90%", avg_overlap);
}
```

**Baseline Generation**:

```bash
# Run LocAgent to generate search result baselines
cd tmp/LocAgent
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
results = {}

for query in queries:
    top10 = retriever.search(query, k=10)
    results[query] = [r['entity_id'] for r in top10]

with open('../../tests/fixtures/parity/search_queries.jsonl', 'w') as f:
    for query, entity_ids in results.items():
        json.dump({'text': query, 'expected_results': entity_ids}, f)
        f.write('\n')
"
```

---

## Acceptance Criteria (from PRD-02 §2.2, PRD-06 §5.1)

### Must-Pass

- [ ] Upper index: O(1) exact match, prefix search returns all matches
- [ ] BM25 index: Search latency <500ms p95 for 10K files
- [ ] Hierarchical search: Top-10 overlap ≥90% with LocAgent on 50 queries
- [ ] Unit test coverage >95% for `cds_sparse_index` crate
- [ ] Passes `cargo clippy` with zero warnings

### Performance

- [ ] Upper index lookup: <10ms
- [ ] BM25 search: <500ms p95
- [ ] Memory usage: <500MB for 10K files

---

## Testing Strategy

### Unit Tests

```rust
// cds_sparse_index/tests/name_index_test.rs
#[test]
fn test_exact_match() {
    let mut index = NameIndex::new();
    index.insert("MyClass".to_string(), "id1".to_string());
    index.insert("MyClass".to_string(), "id2".to_string());
    index.finalize();

    let results = index.exact_match("MyClass").unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.contains(&"id1".to_string()));
}

#[test]
fn test_prefix_match() {
    let mut index = NameIndex::new();
    index.insert("AuthService".to_string(), "id1".to_string());
    index.insert("AuthController".to_string(), "id2".to_string());
    index.insert("UserAuth".to_string(), "id3".to_string());
    index.finalize();

    let results = index.prefix_match("Auth", 10);
    assert_eq!(results.len(), 2); // AuthService, AuthController
}

#[test]
fn test_bm25_ranking() {
    let mut bm25 = CustomBM25::new(1.5, 0.75);
    bm25.add_document("doc1".to_string(), vec!["sanitize".to_string(), "input".to_string()]);
    bm25.add_document("doc2".to_string(), vec!["clean".to_string(), "data".to_string()]);
    bm25.finalize();

    let results = bm25.search(vec!["sanitize".to_string()], 10);
    assert_eq!(results[0].0, "doc1"); // doc1 should rank higher
}
```

### Integration Tests

- [ ] Index LocAgent repo, validate search results vs LocAgent baseline
- [ ] Test hierarchical fallback logic (upper → BM25)

---

## Open Questions & Risks

### Q1: Tantivy vs Custom BM25

**Question**: Which implementation achieves better parity with LocAgent?
**Decision**: Prototype both (Week 3, Day 3), compare accuracy on 50 queries
**Risk**: Tantivy tokenization may differ from LocAgent's custom tokenizer
**Mitigation**: Implement custom tokenizer compatible with LocAgent if needed

### Q2: Upper Index Threshold

**Question**: What's the optimal threshold for BM25 fallback?
**Decision**: Default to 5 (LocAgent behavior), make configurable
**Validation**: Compare fallback behavior on benchmark queries

### Q3: Memory vs Disk Storage

**Risk**: In-memory BM25 index may exceed memory limits for large repos
**Mitigation**: Use tantivy's on-disk storage, benchmark memory usage
**Escalation**: If memory >2GB for 10K files, implement mmap or streaming

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **Depends On**: [01-graph-build.md](01-graph-build.md)
- **Blocks**: [03-service-layer.md](03-service-layer.md)
- **Validates**: [../06-refactor-parity.md](../06-refactor-parity.md)
- **Tests**: [../08-testing/01-unit.md](../08-testing/01-unit.md)

---

## Next Steps

1. [ ] Implement NameIndex with exact and prefix matching (Day 1)
2. [ ] Prototype both tantivy and custom BM25 (Day 3)
3. [ ] Run accuracy comparison on 50 queries (Day 4)
4. [ ] Implement hierarchical search strategy (Day 5)
5. [ ] Run parity tests against LocAgent (Week 4)
6. [ ] Review with Rust Lead before service layer integration

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, awaiting graph build completion
