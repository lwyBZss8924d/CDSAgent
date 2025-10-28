//! Behavior edge processing
//!
//! This module handles construction of behavior edges (Invoke, Inherit) in the code graph.
//! It connects entities (functions, classes) based on their runtime behavior:
//! - Invoke edges: Function calls, decorator applications
//! - Inherit edges: Class inheritance relationships
//!
//! The module is language-agnostic in its orchestration, delegating language-specific
//! AST operations to the `python` submodule.

use crate::graph::builder::python::ast_utils::{
    collect_base_names, collect_class_init_calls, collect_decorator_calls, find_entity_ast,
    visit_block, EntityAstRef,
};
use crate::graph::builder::state::BuilderState;
use crate::graph::EdgeKind;
use petgraph::graph::NodeIndex as GraphNodeIndex;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Process behavior edges for all entities in all files
///
/// This is the main entry point for behavior edge construction. It:
/// 1. Iterates over all files in the repository
/// 2. Parses each file's AST
/// 3. Builds an alias map for import resolution
/// 4. For each entity (function/class) in the file:
///    - Extracts calls, decorators, or base classes from AST
///    - Resolves names to graph nodes via alias map
///    - Adds Invoke or Inherit edges to the graph
///
/// # Deduplication
///
/// Uses `BuilderState::behavior_edge_cache` to prevent duplicate edges.
///
/// # Language Support
///
/// Currently Python-only. The orchestration logic here is language-agnostic,
/// while language-specific AST operations are delegated to `python::ast_utils`.
pub(in crate::graph::builder) fn process_behavior_edges(state: &mut BuilderState) {
    // Collect all file paths to avoid borrow checker issues
    let rel_paths: Vec<std::path::PathBuf> = state.file_sources.keys().cloned().collect();

    for rel_path in rel_paths {
        // Get file node
        let Some(&file_idx) = state.file_nodes.get(&rel_path) else {
            continue;
        };

        // Parse module AST (using import module's helper)
        let Some(module_ast) = crate::graph::builder::imports::parse_module_ast(state, &rel_path)
        else {
            continue;
        };

        // Build alias map for this file (using import module's helper)
        let alias_map = crate::graph::builder::imports::build_alias_map(state, file_idx);

        // Get entities in this file
        let entity_indices = match state.file_entities.get(&rel_path) {
            Some(list) => list.clone(),
            None => continue,
        };

        // Process each entity's behavior edges
        for entity_idx in entity_indices {
            process_entity_behavior_edges(state, entity_idx, &rel_path, &module_ast, &alias_map);
        }
    }
}

/// Process behavior edges for a single entity
///
/// Extracts calls/bases from the entity's AST and connects edges.
fn process_entity_behavior_edges(
    state: &mut BuilderState,
    entity_idx: GraphNodeIndex,
    rel_path: &Path,
    module_ast: &std::rc::Rc<rustpython_parser::ast::Suite>,
    alias_map: &HashMap<String, Vec<GraphNodeIndex>>,
) {
    // Get entity segments (e.g., ["MyClass", "my_method"])
    let Some(segments) = state.entity_segments.get(&entity_idx) else {
        return;
    };

    // Find entity's AST node
    let Some(ast_ref) = find_entity_ast(module_ast.as_ref(), segments) else {
        return;
    };

    // Extract behavior based on entity type
    match ast_ref {
        EntityAstRef::Function(func) => {
            // Collect calls in function body
            let mut calls = Vec::new();
            visit_block(&func.body, &mut calls);
            collect_decorator_calls(&func.decorator_list, &mut calls);

            // Connect Invoke edges
            connect_behavior_edges(
                state,
                entity_idx,
                rel_path,
                alias_map,
                &calls,
                EdgeKind::Invoke,
            );
        }
        EntityAstRef::AsyncFunction(func) => {
            // Collect calls in async function body
            let mut calls = Vec::new();
            visit_block(&func.body, &mut calls);
            collect_decorator_calls(&func.decorator_list, &mut calls);

            // Connect Invoke edges
            connect_behavior_edges(
                state,
                entity_idx,
                rel_path,
                alias_map,
                &calls,
                EdgeKind::Invoke,
            );
        }
        EntityAstRef::Class(class_def) => {
            // Collect base classes (inheritance)
            let bases = collect_base_names(class_def);
            connect_behavior_edges(
                state,
                entity_idx,
                rel_path,
                alias_map,
                &bases,
                EdgeKind::Inherit,
            );

            // Collect calls in __init__ method
            let init_calls = collect_class_init_calls(class_def);
            if !init_calls.is_empty() {
                connect_behavior_edges(
                    state,
                    entity_idx,
                    rel_path,
                    alias_map,
                    &init_calls,
                    EdgeKind::Invoke,
                );
            }
        }
    }
}

/// Connect behavior edges from caller to targets
///
/// For each name in `names`:
/// 1. Resolves the name to target nodes using alias map + file symbols
/// 2. Adds an edge of `kind` from `caller_idx` to each target
/// 3. Deduplicates edges using `state.behavior_edge_cache`
///
/// # Parameters
///
/// - `caller_idx`: Source node (the entity making the call/inheriting)
/// - `rel_path`: File path of the caller (for symbol lookup)
/// - `alias_map`: Name-to-node mapping from imports
/// - `names`: List of names to resolve (function names, class names)
/// - `kind`: Edge type (Invoke or Inherit)
fn connect_behavior_edges(
    state: &mut BuilderState,
    caller_idx: GraphNodeIndex,
    rel_path: &Path,
    alias_map: &HashMap<String, Vec<GraphNodeIndex>>,
    names: &[String],
    kind: EdgeKind,
) {
    let mut seen_targets = HashSet::new();

    for name in names {
        // Resolve name to target node(s)
        let targets =
            crate::graph::builder::imports::resolve_targets(state, rel_path, alias_map, name);

        for target_idx in targets {
            // Deduplicate: only add edge if not already added
            if seen_targets.insert(target_idx)
                && state
                    .behavior_edge_cache
                    .insert((caller_idx, target_idx, kind))
            {
                // Add edge to graph
                state.graph.add_edge(caller_idx, target_idx, kind);
            }
        }
    }
}
