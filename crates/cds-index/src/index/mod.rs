//! Hierarchical sparse indexing
//!
//! Two-tier index structure:
//! - Upper: Name/ID HashMap with prefix matching
//! - Lower: BM25 content search (tantivy)

pub mod bm25;
pub mod name_index;
mod stop_words;
pub mod tokenizer;

pub use bm25::{
    register_code_analyzer, AnalyzerConfig, Bm25Document, Bm25Index, SearchResult,
    CODE_ANALYZER_NAME,
};
pub use name_index::{NameEntry, NameIndex, NameIndexBuilder, NameIndexStats};
pub use stop_words::DEFAULT_STOP_WORDS;
pub use tokenizer::{TantivyCodeTokenizer, TokenizedToken, Tokenizer};
