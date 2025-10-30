//! Language configuration and abstraction layer
//!
//! This module provides a placeholder for future language abstraction (v0.2.0).
//! Currently contains basic language configuration.

/// Language configuration (currently for Python only)
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub name: &'static str,
    pub file_extensions: &'static [&'static str],
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            name: "python",
            file_extensions: &["py"],
        }
    }
}
