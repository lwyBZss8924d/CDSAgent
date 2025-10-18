//! CDS-Index Service - JSON-RPC server for code indexing
//!
//! Runs the Index Service as a long-running daemon, exposing:
//! - /rpc - JSON-RPC 2.0 endpoint
//! - /health - Health check endpoint
//! - /metrics - Prometheus metrics

use anyhow::Result;
use cds_index::IndexServiceConfig;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("cds_index=info".parse()?),
        )
        .init();

    info!("Starting CDS-Index Service...");

    // Load configuration
    let config = IndexServiceConfig::from_env()?;
    config.validate()?;

    info!("Configuration loaded: {:?}", config);

    // TODO: Build or load graph index
    // TODO: Build or load BM25 index
    // TODO: Set up JSON-RPC server with axum
    // TODO: Notify systemd (if running under systemd)

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    info!("CDS-Index Service will listen on {}", addr);

    // Placeholder - will be replaced with actual server
    info!("Service bootstrap complete (placeholder)");

    Ok(())
}
