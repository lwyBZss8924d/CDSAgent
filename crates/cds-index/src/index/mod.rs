//! Hierarchical sparse indexing
//!
//! Two-tier index structure:
//! - Upper: Name/ID HashMap with prefix matching (NameIndex)
//! - Lower: BM25 content search (Bm25Index)
//! - Unified: SparseIndex combining both tiers with hierarchical fallback

pub mod bm25;
pub mod name_index;
pub mod sparse_index;
mod stop_words;
pub mod tokenizer;

// BM25 exports (with renamed SearchResult to avoid conflict)
pub use bm25::{
    register_code_analyzer, AnalyzerConfig, Bm25Document, Bm25Index,
    SearchResult as Bm25SearchResult, CODE_ANALYZER_NAME,
};

// NameIndex exports
pub use name_index::{NameEntry, NameIndex, NameIndexBuilder, NameIndexStats};

// SparseIndex exports (primary search interface)
pub use sparse_index::{SearchResult, SparseIndex};

// Utility exports
pub use stop_words::DEFAULT_STOP_WORDS;
pub use tokenizer::{TantivyCodeTokenizer, TokenizedToken, Tokenizer};
