// classifier.rs - Query Classification for Selective LLM Application
//
// Thread-21: Selective LLM Integration
// Purpose: Decide when to apply LLM re-ranking (avoid universal overhead)
// Based on: Thread-20 batch test findings (42.86% effective rate)

/// Query classifier for selective LLM re-ranking decisions
///
/// # Thread-20 Decision Matrix
/// Apply LLM ONLY if ALL of:
/// 1. Query has entity keywords ("parameter", "docstring", "logic", etc.)
/// 2. BM25 top score < 25.0 (low confidence)
/// 3. BM25 score gap < 5.0 (flat distribution)
/// 4. Likely SEVERE case (baseline <30% overlap)
///
/// # Expected Accuracy
/// ~75% (based on Thread-20 batch test win rate for SEVERE queries)
#[derive(Clone, Debug)]
pub struct QueryClassifier {
    entity_keywords: Vec<String>,
}

impl QueryClassifier {
    /// Create new query classifier with default entity keywords
    pub fn new() -> Self {
        Self {
            entity_keywords: vec![
                // SEVERE query indicators (Thread-20 analysis)
                "parameter".to_string(),
                "docstring".to_string(),
                "logic".to_string(),
                "method".to_string(),
                "class".to_string(),
                "function".to_string(),
                "constant".to_string(),
                "attribute".to_string(),
                "variable".to_string(),
                "field".to_string(),
                "property".to_string(),
                "decorator".to_string(),
            ],
        }
    }

    /// Determine if LLM re-ranking should be applied
    ///
    /// # Arguments
    /// * `query` - Search query text
    /// * `bm25_top_score` - Highest BM25 score from results
    /// * `bm25_score_gap` - Gap between top and 10th result
    /// * `result_count` - Number of BM25 results returned
    ///
    /// # Returns
    /// `true` if LLM re-ranking should be applied (selective, not universal)
    ///
    /// # Thread-20 Validation
    /// - 3/7 queries (42.86%) benefited from LLM re-ranking
    /// - 75% win rate on SEVERE entity queries
    /// - 0% win rate on MODERATE/MILD concept queries
    pub fn should_rerank(
        &self,
        query: &str,
        bm25_top_score: f32,
        bm25_score_gap: f32,
        result_count: usize,
    ) -> bool {
        // Minimum requirements
        if result_count == 0 {
            return false; // No candidates to re-rank
        }

        // Criterion 1: Query has entity keywords (Thread-20 finding)
        if !self.has_entity_keywords(query) {
            return false; // Skip general concept queries ("version", "helper", "metadata")
        }

        // Criterion 2: Low BM25 confidence (top score < 25.0)
        if bm25_top_score >= 25.0 {
            return false; // High confidence BM25 results don't need LLM
        }

        // Criterion 3: Flat score distribution (gap < 5.0)
        // This indicates BM25 is uncertain about ranking
        if bm25_score_gap >= 5.0 {
            return false; // Clear winner, no need for LLM
        }

        // Criterion 4: Likely SEVERE case (heuristic)
        if !self.is_severe_case(bm25_top_score, bm25_score_gap) {
            return false; // MODERATE/MILD cases don't benefit from LLM
        }

        // All criteria met → Apply LLM re-ranking
        true
    }

    /// Check if query contains entity keywords (not general concepts)
    ///
    /// # Thread-20 Finding
    /// - Entity queries: "parameter", "docstring", "logic" → 75% win rate
    /// - Concept queries: "version", "helper", "metadata" → 0% win rate
    fn has_entity_keywords(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.entity_keywords
            .iter()
            .any(|keyword| query_lower.contains(keyword))
    }

    /// Heuristic to detect SEVERE cases (baseline <30% likely)
    ///
    /// # Logic
    /// - Very low score (<20.0) = definitely SEVERE
    /// - Low score + flat distribution = likely SEVERE
    /// - Otherwise = MODERATE or better
    fn is_severe_case(&self, bm25_top_score: f32, bm25_score_gap: f32) -> bool {
        // Very low top score = definitely SEVERE
        if bm25_top_score < 20.0 {
            return true;
        }

        // Low score + flat distribution = likely SEVERE
        if bm25_top_score < 25.0 && bm25_score_gap < 3.0 {
            return true;
        }

        // Otherwise, likely MODERATE or better (don't apply LLM)
        false
    }
}

impl Default for QueryClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_keyword_detection() {
        let classifier = QueryClassifier::new();

        // Entity queries (should pass)
        assert!(classifier.has_entity_keywords("RidgeClassifierCV parameter"));
        assert!(classifier.has_entity_keywords("detect docstring constant"));
        assert!(classifier.has_entity_keywords("cross-validation logic"));

        // Concept queries (should fail)
        assert!(!classifier.has_entity_keywords("get_versions helper"));
        assert!(!classifier.has_entity_keywords("build metadata"));
        assert!(!classifier.has_entity_keywords("version utility"));
    }

    #[test]
    fn test_severe_case_detection() {
        let classifier = QueryClassifier::new();

        // Very low score = SEVERE
        assert!(classifier.is_severe_case(15.0, 10.0));

        // Low score + flat distribution = SEVERE
        assert!(classifier.is_severe_case(23.0, 2.5));

        // High score = NOT severe
        assert!(!classifier.is_severe_case(30.0, 10.0));

        // Low score but high gap = NOT severe
        assert!(!classifier.is_severe_case(23.0, 8.0));
    }

    #[test]
    fn test_should_rerank_all_criteria() {
        let classifier = QueryClassifier::new();

        // Thread-20 winning query: pytest/1 (_pytest.rewrite detect docstring constant)
        // Expected: TRUE (entity keyword + low score + flat distribution + SEVERE)
        assert!(classifier.should_rerank(
            "_pytest.rewrite detect docstring constant",
            18.5, // Low BM25 confidence
            2.3,  // Flat distribution
            10    // Multiple candidates
        ));

        // Thread-20 losing query: matplotlib/4 (build metadata for Matplotlib version)
        // Expected: FALSE (no entity keywords)
        assert!(!classifier.should_rerank(
            "build metadata for Matplotlib version",
            22.0, // Low score
            2.5,  // Flat distribution
            10
        ));

        // High confidence BM25 (requests repo baseline)
        // Expected: FALSE (top score >= 25.0)
        assert!(!classifier.should_rerank(
            "parameter validation logic",
            35.0, // High confidence
            8.0,
            10
        ));
    }

    #[test]
    fn test_no_candidates() {
        let classifier = QueryClassifier::new();

        // Empty results = no re-ranking
        assert!(!classifier.should_rerank("any query", 0.0, 0.0, 0));
    }
}
