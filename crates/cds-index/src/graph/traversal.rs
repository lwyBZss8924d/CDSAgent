//! Graph traversal and dependency exploration utilities.

use crate::graph::{DependencyGraph, EdgeKind, GraphNodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct TraversalFilter {
    pub max_depth: usize,
    pub relations: Vec<EdgeKind>,
}

impl Default for TraversalFilter {
    fn default() -> Self {
        Self {
            max_depth: 1,
            relations: Vec::new(),
        }
    }
}

/// Breadth-first traversal constrained by relation types and depth (mirrors LocAgent BFS).
pub fn bfs_traversal(
    graph: &DependencyGraph,
    start: GraphNodeIndex,
    filter: &TraversalFilter,
) -> Vec<GraphNodeIndex> {
    let mut results = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back((start, 0usize));
    results.push(start);

    let allowed = if filter.relations.is_empty() {
        None
    } else {
        Some(filter.relations.iter().copied().collect::<HashSet<_>>())
    };

    while let Some((node, depth)) = queue.pop_front() {
        if depth >= filter.max_depth {
            continue;
        }

        for edge in graph.graph().edges(node) {
            let weight = edge.weight();
            if let Some(ref allow) = allowed {
                if !allow.contains(&weight.kind) {
                    continue;
                }
            }

            let neighbor = edge.target();
            if visited.insert(neighbor) {
                results.push(neighbor);
                queue.push_back((neighbor, depth + 1));
            }
        }
    }

    results
}
