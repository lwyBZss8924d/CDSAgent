//! Hierarchical sparse indexing
//!
//! Two-tier index structure:
//! - Upper: Name/ID HashMap with prefix matching
//! - Lower: BM25 content search (tantivy)

pub mod bm25;
pub mod name_index;

pub use name_index::{NameEntry, NameIndex, NameIndexBuilder, NameIndexStats};
