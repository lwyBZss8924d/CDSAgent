//! Graph-based code structure representation
//!
//! Implements the dependency graph from LocAgent:
//! - 4 node types: directory, file, class, function
//! - 4 edge types: contain, import, invoke, inherit

pub mod builder;
pub mod parser;
pub mod traversal;

// Placeholder types - to be implemented
pub struct DependencyGraph;
pub struct GraphBuilder;
