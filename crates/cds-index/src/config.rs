//! Configuration management for CDS-Index Service

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexServiceConfig {
    pub graph_index_dir: PathBuf,
    pub bm25_index_dir: PathBuf,
    pub port: u16,
    pub host: String,
    pub log_level: String,
}

impl IndexServiceConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let graph_index_dir = std::env::var("GRAPH_INDEX_DIR")
            .context("GRAPH_INDEX_DIR not set")?
            .into();

        let bm25_index_dir = std::env::var("BM25_INDEX_DIR")
            .context("BM25_INDEX_DIR not set")?
            .into();

        let port = std::env::var("INDEX_SERVICE_PORT")
            .unwrap_or_else(|_| "3030".to_string())
            .parse()
            .context("Invalid INDEX_SERVICE_PORT")?;

        let host = std::env::var("INDEX_SERVICE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            graph_index_dir,
            bm25_index_dir,
            port,
            host,
            log_level,
        })
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate port range
        if self.port < 1024 {
            anyhow::bail!("INDEX_SERVICE_PORT must be >= 1024");
        }

        // Create directories if they don't exist
        if !self.graph_index_dir.exists() {
            std::fs::create_dir_all(&self.graph_index_dir)
                .context("Failed to create GRAPH_INDEX_DIR")?;
        }

        if !self.bm25_index_dir.exists() {
            std::fs::create_dir_all(&self.bm25_index_dir)
                .context("Failed to create BM25_INDEX_DIR")?;
        }

        Ok(())
    }
}
