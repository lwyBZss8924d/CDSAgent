//! CDS-Index: Graph-based code indexing and search
//!
//! This crate implements the core indexing functionality for CDSAgent,
//! including:
//! - Graph-based code structure representation (dependency graph)
//! - Hierarchical sparse indexing (name/ID + BM25 content search)
//! - JSON-RPC service layer for remote access
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │     JSON-RPC Service (Axum)         │
//! └──────────────┬──────────────────────┘
//!                │
//!      ┌─────────┴─────────┐
//!      │                   │
//! ┌────▼────┐      ┌───────▼──────┐
//! │  Graph  │      │ Sparse Index │
//! │ Builder │      │ (Name + BM25)│
//! └─────────┘      └──────────────┘
//! ```

pub mod config;
pub mod graph;
pub mod index;
pub mod persistence;
pub mod service;

pub use config::IndexServiceConfig;
