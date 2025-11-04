//! Sparse Index - Unified hierarchical search combining NameIndex + BM25Index
//!
//! The `SparseIndex` provides a two-tier search architecture:
//! - **Upper tier**: Fast exact/prefix matching via `NameIndex` (<1μs)
//! - **Lower tier**: Full-text BM25 search for complex queries (<10ms expected)
//! - **Optional**: LLM re-ranking for SEVERE queries (Thread-21, feature-flagged)
//!
//! Search strategy (hierarchical fallback):
//! 1. Try exact_match() → return immediately if limit satisfied
//! 2. Try prefix_match() → merge results, return if limit satisfied
//! 3. Try BM25 search() → fill remaining slots
//! 4. [Optional] Apply selective LLM re-ranking (15-25% of queries)
//!
//! This design optimizes for common patterns (name-based queries) while
//! maintaining semantic search capability for complex queries.

// Thread-21: Selective LLM Integration (sub-modules)
mod classifier;

#[cfg(feature = "llm-reranking")]
mod llm_reranker;

use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::graph::{DependencyGraph, NodeKind};

use super::bm25::{AnalyzerConfig, Bm25Index};
use super::name_index::NameIndex;

use classifier::QueryClassifier;

#[cfg(feature = "llm-reranking")]
use llm_reranker::LlmReranker;

/// Unified search result combining upper and lower index results
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub entity_id: String,
    pub name: Option<String>,
    pub path: String,
    pub kind: NodeKind,
    pub score: f32,
    pub matched_terms: Vec<String>,
}

// Note: We can't implement From<NameEntry> because NameEntry only has
// node_id, not the full entity_id and path. Those are retrieved from the graph.

impl From<super::bm25::SearchResult> for SearchResult {
    fn from(result: super::bm25::SearchResult) -> Self {
        SearchResult {
            entity_id: result.entity_id,
            name: result.name,
            path: result.path,
            kind: result.kind,
            score: result.score,
            matched_terms: result.matched_terms,
        }
    }
}

/// Two-tier sparse index combining name-based and BM25 search
/// Thread-21: Extended with optional LLM re-ranking (feature-flagged)
pub struct SparseIndex {
    graph: DependencyGraph,
    upper: NameIndex,
    lower: Bm25Index,

    // Thread-21: Query classification for selective LLM application
    classifier: QueryClassifier,

    // Thread-21: Optional LLM re-ranker (feature-gated)
    #[cfg(feature = "llm-reranking")]
    llm_reranker: Option<LlmReranker>,
}

impl SparseIndex {
    /// Builds a sparse index from graph entities.
    ///
    /// This method creates both the upper (name-based) and lower (BM25) indices
    /// from the same graph, ensuring consistent entity coverage across both tiers.
    ///
    /// # Arguments
    ///
    /// * `graph` - The dependency graph to index
    /// * `base_path` - Base directory for index storage (will create subdirs)
    /// * `config` - Analyzer configuration for BM25 tokenization
    ///
    /// # Returns
    ///
    /// A `SparseIndex` ready for hierarchical searching
    ///
    /// # Errors
    ///
    /// Returns an error if either index creation fails
    pub fn from_graph(
        graph: DependencyGraph,
        base_path: impl AsRef<Path>,
        config: AnalyzerConfig,
    ) -> Result<Self> {
        let base = base_path.as_ref();

        // Build upper index (name-based, in-memory)
        let upper = NameIndex::from_graph(&graph);

        // Build lower index (BM25, on-disk)
        let bm25_path = base.join("bm25");
        let lower = Bm25Index::from_graph(&graph, &bm25_path, config)?;

        // Thread-21: Initialize query classifier (always enabled)
        let classifier = QueryClassifier::new();

        // Thread-21: Initialize optional LLM re-ranker (feature-gated)
        #[cfg(feature = "llm-reranking")]
        let llm_reranker = LlmReranker::new().ok(); // Graceful fallback if script not found

        Ok(Self {
            graph,
            upper,
            lower,
            classifier,
            #[cfg(feature = "llm-reranking")]
            llm_reranker,
        })
    }

    /// Performs hierarchical search with fallback strategy.
    ///
    /// Search flow:
    /// 1. Try exact match on entity names
    /// 2. If slots remain, try prefix match
    /// 3. If slots still remain, use BM25 full-text search
    ///
    /// Results are deduplicated by entity_id to prevent duplicates
    /// across tiers.
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string
    /// * `limit` - Maximum number of results to return
    /// * `kind_filter` - Optional filter by entity kind (Class, Function, etc.)
    ///
    /// # Returns
    ///
    /// A vector of `SearchResult` ordered by relevance (exact > prefix > BM25)
    pub fn search(
        &self,
        query: &str,
        limit: usize,
        kind_filter: Option<&[NodeKind]>,
    ) -> Result<Vec<SearchResult>> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let mut results = Vec::with_capacity(limit);
        let mut seen_ids: HashSet<String> = HashSet::new();
        let mut path_index: HashMap<String, usize> = HashMap::new();

        // Convert kind filter for upper index (takes Option<NodeKind>, not slice)
        // If multiple kinds provided, we use None (no filter) for upper index
        // since it can only filter by single kind
        let upper_kind_filter = kind_filter.and_then(|kinds| {
            if kinds.len() == 1 {
                Some(kinds[0])
            } else {
                None // Multiple kinds, can't filter in upper index
            }
        });

        // Phase 1: Try exact match
        let exact_matches = self.upper.exact_match(query, upper_kind_filter, limit);
        for entry in exact_matches {
            // Post-filter for multiple kinds if needed
            if let Some(kinds) = kind_filter {
                if !kinds.contains(&entry.kind) {
                    continue;
                }
            }
            // Convert NameEntry to SearchResult via graph lookup
            if let Some(node) = self.graph.node(entry.node_id) {
                let path = node
                    .file_path
                    .as_ref()
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .to_string();
                let result = SearchResult {
                    entity_id: node.id.clone(),
                    name: Some(entry.name.to_string()),
                    path,
                    kind: entry.kind,
                    score: 1.0, // Exact match gets perfect score
                    matched_terms: Vec::new(),
                };
                Self::insert_or_update(&mut results, &mut seen_ids, &mut path_index, result, limit);
                if results.len() >= limit {
                    return Ok(results);
                }
            }
        }

        // Phase 2: Try prefix match
        if results.len() < limit {
            let remaining = limit - results.len();
            let prefix_matches = self.upper.prefix_match(query, upper_kind_filter, remaining);
            for entry in prefix_matches {
                // Post-filter for multiple kinds if needed
                if let Some(kinds) = kind_filter {
                    if !kinds.contains(&entry.kind) {
                        continue;
                    }
                }
                // Convert NameEntry to SearchResult via graph lookup
                if let Some(node) = self.graph.node(entry.node_id) {
                    let path = node
                        .file_path
                        .as_ref()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string();
                    let result = SearchResult {
                        entity_id: node.id.clone(),
                        name: Some(entry.name.to_string()),
                        path,
                        kind: entry.kind,
                        score: 0.9, // Prefix match gets slightly lower score
                        matched_terms: Vec::new(),
                    };
                    Self::insert_or_update(
                        &mut results,
                        &mut seen_ids,
                        &mut path_index,
                        result,
                        limit,
                    );
                    if results.len() >= limit {
                        return Ok(results);
                    }
                }
            }
        }

        // Phase 3: BM25 full-text search fallback
        if results.len() < limit {
            let remaining = limit - results.len();
            // Request more results from BM25 to account for deduplication
            let oversample = (remaining * 6).max(remaining + 25);
            let bm25_results = self.lower.search(query, oversample, kind_filter)?;

            for bm25_result in bm25_results {
                let result = SearchResult::from(bm25_result);
                Self::insert_or_update(&mut results, &mut seen_ids, &mut path_index, result, limit);
                if results.len() >= limit {
                    break;
                }
            }
        }

        // Phase 4 (Optional): Selective LLM re-ranking (Thread-21)
        // Only apply if feature enabled AND classifier determines it's beneficial
        #[cfg(feature = "llm-reranking")]
        if !results.is_empty() && self.llm_reranker.is_some() {
            // Calculate BM25 statistics for classification
            let bm25_top_score = results.first().map(|r| r.score).unwrap_or(0.0);
            let bm25_score_gap = if results.len() >= 10 {
                bm25_top_score - results.get(9).map(|r| r.score).unwrap_or(0.0)
            } else {
                bm25_top_score
            };

            // Query classifier: should we apply LLM re-ranking?
            if self.classifier.should_rerank(query, bm25_top_score, bm25_score_gap, results.len()) {
                // Apply LLM re-ranking (graceful fallback on error)
                if let Some(ref reranker) = self.llm_reranker {
                    if let Ok(reranked) = reranker.rerank(query, &results) {
                        // Update results with LLM-adjusted scores
                        results = reranked;
                    }
                    // On error, silently fall back to BM25 results (already logged in reranker)
                }
            }
        }

        Ok(results)
    }

    fn insert_or_update(
        results: &mut Vec<SearchResult>,
        seen_ids: &mut HashSet<String>,
        path_index: &mut HashMap<String, usize>,
        result: SearchResult,
        limit: usize,
    ) {
        let path_key = result.path.clone();
        let boost = 1.0;
        let penalty = 1.0;
        let mut result = result;
        result.score *= boost;
        result.score /= penalty;

        if path_index.contains_key(&path_key) {
            return;
        }

        if seen_ids.contains(&result.entity_id) {
            return;
        }

        if results.len() < limit {
            seen_ids.insert(result.entity_id.clone());
            path_index.insert(path_key, results.len());
            results.push(result);
        }
    }

    /// Returns the underlying name index (useful for testing/debugging)
    pub fn upper(&self) -> &NameIndex {
        &self.upper
    }

    /// Returns the underlying BM25 index (useful for testing/debugging)
    pub fn lower(&self) -> &Bm25Index {
        &self.lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{GraphNode, SourceRange};
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn sparse_index_hierarchical_search() -> Result<()> {
        let mut graph = DependencyGraph::new();

        // Add a class with exact name "User"
        let mut user_class = GraphNode::entity(
            "repo::models.py::User".to_string(),
            NodeKind::Class,
            "User".to_string(),
            PathBuf::from("models.py"),
            Some(SourceRange::new(1, 10)),
        );
        user_class
            .attributes
            .insert("docstring".to_string(), "User model".to_string());
        graph.add_node(user_class);

        // Add a function with prefix "UserAuth"
        let mut user_auth = GraphNode::entity(
            "repo::auth.py::UserAuthenticator".to_string(),
            NodeKind::Class,
            "UserAuthenticator".to_string(),
            PathBuf::from("auth.py"),
            Some(SourceRange::new(1, 20)),
        );
        user_auth.attributes.insert(
            "docstring".to_string(),
            "Handles user authentication".to_string(),
        );
        graph.add_node(user_auth);

        // Add a function that only matches via BM25 (no name overlap)
        let mut auth_func = GraphNode::entity(
            "repo::auth.py::validate_credentials".to_string(),
            NodeKind::Function,
            "validate_credentials".to_string(),
            PathBuf::from("auth.py"),
            Some(SourceRange::new(25, 40)),
        );
        auth_func.attributes.insert(
            "docstring".to_string(),
            "Validate user login credentials".to_string(),
        );
        graph.add_node(auth_func);

        let dir = tempdir()?;
        let index = SparseIndex::from_graph(graph, dir.path(), AnalyzerConfig::default())?;

        // Test 1: Exact match should return first (highest priority)
        let results = index.search("User", 10, None)?;
        assert!(!results.is_empty(), "Should find results for 'User'");
        assert_eq!(
            results[0].entity_id, "repo::models.py::User",
            "Exact match should be first"
        );
        assert_eq!(
            results[0].score, 1.0,
            "Exact match should have perfect score"
        );

        // Test 2: Kind filtering works across all tiers
        let class_only = index.search("User", 10, Some(&[NodeKind::Class]))?;
        assert!(
            class_only.iter().all(|r| r.kind == NodeKind::Class),
            "Should only return classes"
        );

        // Test 3: BM25 fallback finds content-based matches
        let cred_results = index.search("credentials", 10, None)?;
        assert!(
            cred_results
                .iter()
                .any(|r| r.entity_id == "repo::auth.py::validate_credentials"),
            "BM25 should find 'validate_credentials' via docstring match"
        );

        Ok(())
    }

    #[test]
    fn sparse_index_deduplicates_results() -> Result<()> {
        let mut graph = DependencyGraph::new();

        // Add entity that matches both name and content
        let mut user = GraphNode::entity(
            "repo::user.py::UserManager".to_string(),
            NodeKind::Class,
            "UserManager".to_string(),
            PathBuf::from("user.py"),
            Some(SourceRange::new(1, 50)),
        );
        user.attributes
            .insert("docstring".to_string(), "Manages user accounts".to_string());
        graph.add_node(user);

        let dir = tempdir()?;
        let index = SparseIndex::from_graph(graph, dir.path(), AnalyzerConfig::default())?;

        // Search for "user" - should match both name (prefix) and content (BM25)
        // But deduplication should prevent duplicate entries
        let results = index.search("user", 10, None)?;

        // Verify no duplicates
        let unique_ids: HashSet<_> = results.iter().map(|r| &r.entity_id).collect();
        assert_eq!(
            results.len(),
            unique_ids.len(),
            "Results should not contain duplicates"
        );

        Ok(())
    }

    #[test]
    fn sparse_index_initializes_classifier() -> Result<()> {
        // Thread-21: Verify that SparseIndex initializes with classifier
        let mut graph = DependencyGraph::new();

        // Add minimal entity for index construction
        let user = GraphNode::entity(
            "repo::test.py::TestClass".to_string(),
            NodeKind::Class,
            "TestClass".to_string(),
            PathBuf::from("test.py"),
            Some(SourceRange::new(1, 10)),
        );
        graph.add_node(user);

        let dir = tempdir()?;
        let index = SparseIndex::from_graph(graph, dir.path(), AnalyzerConfig::default())?;

        // Verify search works (classifier is initialized internally)
        let results = index.search("TestClass", 10, None)?;
        assert_eq!(results.len(), 1, "Should find the test class");
        assert_eq!(results[0].entity_id, "repo::test.py::TestClass");

        Ok(())
    }

    #[test]
    #[cfg(not(feature = "llm-reranking"))]
    fn sparse_index_works_without_llm_feature() -> Result<()> {
        // Thread-21: Verify SparseIndex works WITHOUT llm-reranking feature
        let mut graph = DependencyGraph::new();

        let user = GraphNode::entity(
            "repo::test.py::TestFunc".to_string(),
            NodeKind::Function,
            "TestFunc".to_string(),
            PathBuf::from("test.py"),
            Some(SourceRange::new(1, 10)),
        );
        graph.add_node(user);

        let dir = tempdir()?;
        let index = SparseIndex::from_graph(graph, dir.path(), AnalyzerConfig::default())?;

        // Search should work normally without LLM
        let results = index.search("TestFunc", 10, None)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entity_id, "repo::test.py::TestFunc");

        Ok(())
    }
}
