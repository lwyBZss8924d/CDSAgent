//! Sparse Index - Unified hierarchical search combining NameIndex + BM25Index
//!
//! The `SparseIndex` provides a two-tier search architecture:
//! - **Upper tier**: Fast exact/prefix matching via `NameIndex` (<1μs)
//! - **Lower tier**: Full-text BM25 search for complex queries (<10ms expected)
//!
//! Search strategy (hierarchical fallback):
//! 1. Try exact_match() → return immediately if limit satisfied
//! 2. Try prefix_match() → merge results, return if limit satisfied
//! 3. Try BM25 search() → fill remaining slots
//!
//! This design optimizes for common patterns (name-based queries) while
//! maintaining semantic search capability for complex queries.

use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

use crate::graph::{DependencyGraph, NodeKind};

use super::bm25::{AnalyzerConfig, Bm25Index};
use super::name_index::NameIndex;

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
pub struct SparseIndex {
    graph: DependencyGraph,
    upper: NameIndex,
    lower: Bm25Index,
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

        Ok(Self { graph, upper, lower })
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
                if seen_ids.insert(node.id.clone()) {
                    results.push(SearchResult {
                        entity_id: node.id.clone(),
                        name: Some(entry.name.to_string()),
                        path: node
                            .file_path
                            .as_ref()
                            .and_then(|p| p.to_str())
                            .unwrap_or("")
                            .to_string(),
                        kind: entry.kind,
                        score: 1.0, // Exact match gets perfect score
                        matched_terms: Vec::new(),
                    });
                    if results.len() >= limit {
                        return Ok(results);
                    }
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
                    if seen_ids.insert(node.id.clone()) {
                        results.push(SearchResult {
                            entity_id: node.id.clone(),
                            name: Some(entry.name.to_string()),
                            path: node
                                .file_path
                                .as_ref()
                                .and_then(|p| p.to_str())
                                .unwrap_or("")
                                .to_string(),
                            kind: entry.kind,
                            score: 0.9, // Prefix match gets slightly lower score
                            matched_terms: Vec::new(),
                        });
                        if results.len() >= limit {
                            return Ok(results);
                        }
                    }
                }
            }
        }

        // Phase 3: BM25 full-text search fallback
        if results.len() < limit {
            let remaining = limit - results.len();
            // Request more results from BM25 to account for deduplication
            let oversample = (remaining * 2).max(remaining + 10);
            let bm25_results = self.lower.search(query, oversample, kind_filter)?;

            for bm25_result in bm25_results {
                // Add unique result
                if seen_ids.insert(bm25_result.entity_id.clone()) {
                    results.push(SearchResult::from(bm25_result));
                    if results.len() >= limit {
                        break;
                    }
                }
            }
        }

        Ok(results)
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
        user_auth
            .attributes
            .insert("docstring".to_string(), "Handles user authentication".to_string());
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
}
