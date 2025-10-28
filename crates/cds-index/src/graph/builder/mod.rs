//! Graph builder module - refactored for multi-language support
//!
//! This module is organized for future multi-language support (v0.2.0+):
//! - Generic graph building logic in top-level modules
//! - Python-specific implementation in `python/` submodule
//! - Language trait abstraction planned for v0.2.0

pub mod aliases;
pub mod behaviors;
pub mod imports;
pub mod language;
pub mod python;
pub mod state;

// Re-export public API from submodules
pub use language::LanguageConfig;
pub use state::{
    GraphBuildStats, GraphBuilder, GraphBuilderConfig, GraphBuilderResult, GraphError,
};
