//! Graph-based code structure representation
//!
//! Implements the dependency graph from LocAgent:
//! - 4 node types: directory, file, class, function
//! - 4 edge types: contain, import, invoke, inherit

pub mod builder;
pub mod parser;
pub mod traversal;

pub use builder::{
    GraphBuildStats, GraphBuilder, GraphBuilderConfig, GraphBuilderResult, GraphError,
};
pub use parser::{
    ImportDirective, ImportEntity, ModuleSpecifier, ParsedEntity, ParserError, PythonParser,
};
pub use traversal::{bfs_traversal, TraversalFilter};

use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Public alias for callers that interact with node indices.
pub type GraphNodeIndex = NodeIndex;
/// Public alias for callers that need to inspect the underlying graph storage.
pub type GraphStorage = StableDiGraph<GraphNode, GraphEdge>;

/// Supported entity/node kinds as defined in PRD-02 §2.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Directory,
    File,
    Class,
    Function,
}

/// Supported edge kinds as defined in PRD-02 FR-CG-2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Contain,
    Import,
    Invoke,
    Inherit,
}

/// Line-based source range (1-indexed) for parity comparisons with LocAgent output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRange {
    pub start_line: u32,
    pub end_line: u32,
}

impl SourceRange {
    /// Creates a new [`SourceRange`] using inclusive start/end 1-based line numbers.
    pub fn new(start_line: u32, end_line: u32) -> Self {
        Self {
            start_line,
            end_line,
        }
    }
}

/// Node metadata stored inside the dependency graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: NodeKind,
    pub display_name: String,
    pub file_path: Option<PathBuf>,
    pub range: Option<SourceRange>,
    pub attributes: HashMap<String, String>,
}

impl GraphNode {
    /// Constructs a directory node used for folders in the repository hierarchy.
    pub fn directory(id: String, display_name: String, file_path: Option<PathBuf>) -> Self {
        Self {
            id,
            kind: NodeKind::Directory,
            display_name,
            file_path,
            range: None,
            attributes: HashMap::new(),
        }
    }

    /// Constructs a file node that owns the absolute source path.
    pub fn file(id: String, display_name: String, file_path: PathBuf) -> Self {
        Self {
            id,
            kind: NodeKind::File,
            display_name,
            file_path: Some(file_path),
            range: None,
            attributes: HashMap::new(),
        }
    }

    /// Constructs a class or function node with the given metadata.
    pub fn entity(
        id: String,
        kind: NodeKind,
        display_name: String,
        file_path: PathBuf,
        range: Option<SourceRange>,
    ) -> Self {
        Self {
            id,
            kind,
            display_name,
            file_path: Some(file_path),
            range,
            attributes: HashMap::new(),
        }
    }
}

/// Graph wrapper that keeps a stable mapping between node ids and indices.
#[derive(Debug, Default)]
pub struct DependencyGraph {
    graph: GraphStorage,
    id_lookup: HashMap<String, GraphNodeIndex>,
}

impl DependencyGraph {
    /// Creates an empty dependency graph.
    pub fn new() -> Self {
        Self {
            graph: GraphStorage::default(),
            id_lookup: HashMap::new(),
        }
    }

    /// Inserts a node into the graph, reusing an existing index when the id already exists.
    pub fn add_node(&mut self, node: GraphNode) -> GraphNodeIndex {
        if let Some(&idx) = self.id_lookup.get(&node.id) {
            return idx;
        }

        let idx = self.graph.add_node(node.clone());
        self.id_lookup.insert(node.id.clone(), idx);
        idx
    }

    /// Adds an edge of the given relation kind between two nodes.
    pub fn add_edge(
        &mut self,
        source: GraphNodeIndex,
        target: GraphNodeIndex,
        relation: EdgeKind,
    ) -> EdgeIndex {
        self.graph
            .add_edge(source, target, GraphEdge::new(relation))
    }

    /// Adds an edge with an optional import alias payload.
    pub fn add_edge_with_alias(
        &mut self,
        source: GraphNodeIndex,
        target: GraphNodeIndex,
        relation: EdgeKind,
        alias: Option<String>,
    ) -> EdgeIndex {
        self.graph
            .add_edge(source, target, GraphEdge::with_alias(relation, alias))
    }

    /// Looks up a node index by its fully-qualified id.
    pub fn get_index(&self, id: &str) -> Option<GraphNodeIndex> {
        self.id_lookup.get(id).copied()
    }

    /// Returns the total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns the total number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Retrieves the node metadata associated with the given index.
    pub fn node(&self, idx: GraphNodeIndex) -> Option<&GraphNode> {
        self.graph.node_weight(idx)
    }

    /// Immutable access to the underlying petgraph storage (for traversal).
    pub fn graph(&self) -> &GraphStorage {
        &self.graph
    }

    /// Mutable access to the underlying petgraph storage (for post-processing).
    pub fn graph_mut(&mut self) -> &mut GraphStorage {
        &mut self.graph
    }

    /// Consumes the wrapper and returns the raw petgraph storage.
    pub fn into_graph(self) -> GraphStorage {
        self.graph
    }
}

/// Edge metadata capturing relation kind plus optional alias (for imports).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub kind: EdgeKind,
    pub alias: Option<String>,
}

impl GraphEdge {
    /// Creates a new edge without alias metadata.
    pub fn new(kind: EdgeKind) -> Self {
        Self { kind, alias: None }
    }

    /// Creates a new edge with optional alias metadata (used for imports).
    pub fn with_alias(kind: EdgeKind, alias: Option<String>) -> Self {
        Self { kind, alias }
    }
}
