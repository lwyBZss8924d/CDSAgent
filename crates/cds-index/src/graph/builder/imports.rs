//! Import edge building
//!
//! This module contains logic for processing import directives and building
//! import edges in the dependency graph. It handles:
//! - Module imports (import X)
//! - From-imports (from X import Y)
//! - Wildcard imports (from X import *)
//! - Deferred attribute resolution
//! - Module alias tracking

use super::python::ast_utils::{finalize_module_path, module_components};
use super::state::{BuilderState, DeferredAttributeImport, ExportSource, PendingWildcardExport};
use crate::graph::{
    EdgeKind, GraphNodeIndex, ImportDirective, ImportEntity, ModuleSpecifier, NodeKind,
};
use petgraph::visit::EdgeRef;
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::warn;

/// Main entry point for processing pending imports
///
/// This function is called after all files have been parsed. It:
/// 1. Processes all pending import directives
/// 2. Resolves wildcard exports
/// 3. Resolves deferred attribute imports
pub(in crate::graph::builder) fn process_pending_imports(state: &mut BuilderState) {
    let pending = std::mem::take(&mut state.pending_imports);
    for (rel_path, directives) in pending {
        let Some(&file_idx) = state.file_nodes.get(&rel_path) else {
            continue;
        };
        for directive in directives {
            match directive {
                ImportDirective::Module { module, alias } => {
                    if let Some(path) = resolve_module_spec(state, &rel_path, &module) {
                        add_file_import_edge(state, file_idx, &path, alias.as_deref());
                        let alias_name = alias.clone().or_else(|| module.segments.last().cloned());
                        if let Some(alias_value) = alias_name {
                            record_module_alias(state, &rel_path, alias_value, path);
                        }
                    }
                }
                ImportDirective::FromModule { module, entities } => {
                    process_from_import(state, file_idx, &rel_path, &module, &entities);
                }
            }
        }
    }

    resolve_pending_wildcard_exports(state);
    resolve_deferred_attribute_imports(state);
}

fn process_from_import(
    state: &mut BuilderState,
    file_idx: GraphNodeIndex,
    source_path: &Path,
    module: &ModuleSpecifier,
    entities: &[ImportEntity],
) {
    let base_module_path = resolve_module_spec(state, source_path, module);
    for entity in entities {
        if entity.is_wildcard {
            if let Some(ref module_path) = base_module_path {
                add_file_import_edge(state, file_idx, module_path, None);
                expand_wildcard_import(state, file_idx, module_path);
                if !add_wildcard_export_edges(state, file_idx, module_path) {
                    enqueue_wildcard_export(state, file_idx, module_path);
                }
            }
            continue;
        }

        let extended_spec = ModuleSpecifier::new(
            module.level,
            module
                .segments
                .iter()
                .cloned()
                .chain(split_entity_segments(&entity.name))
                .collect(),
        );

        if let Some(path) = resolve_module_spec(state, source_path, &extended_spec) {
            add_file_import_edge(state, file_idx, &path, entity.alias.as_deref());
            let alias_name = entity.alias.clone().unwrap_or_else(|| entity.name.clone());
            record_module_alias(state, source_path, alias_name, path);
            continue;
        }

        if let Some(ref module_path) = base_module_path {
            let _ = add_attribute_import_edge(
                state,
                file_idx,
                module_path,
                &entity.name,
                entity.alias.as_deref(),
            );
        }
    }
}

fn add_wildcard_export_edges(
    state: &mut BuilderState,
    source_idx: GraphNodeIndex,
    module_path: &Path,
) -> bool {
    let has_explicit_exports = state
        .module_exports
        .get(module_path)
        .map(|info| !info.names.is_empty() || !info.sources.is_empty())
        .unwrap_or(false);
    if !has_explicit_exports {
        return false;
    }

    let exports = resolve_exports(state, module_path);
    let mut added = false;
    for name in exports {
        if add_attribute_import_edge(state, source_idx, module_path, &name, None) {
            added = true;
        }
    }
    added
}

fn add_file_import_edge(
    state: &mut BuilderState,
    source_idx: GraphNodeIndex,
    target_path: &Path,
    alias: Option<&str>,
) {
    if let Some(&target_idx) = state.file_nodes.get(target_path) {
        add_import_edge_if_absent(state, source_idx, target_idx, alias);
    }
}

fn add_attribute_import_edge(
    state: &mut BuilderState,
    source_idx: GraphNodeIndex,
    module_path: &Path,
    name: &str,
    alias: Option<&str>,
) -> bool {
    if try_add_attribute_import_edge(state, source_idx, module_path, name, alias) {
        return true;
    }

    state
        .deferred_attribute_imports
        .push(DeferredAttributeImport {
            source_idx,
            module_path: module_path.to_path_buf(),
            name: name.to_string(),
            alias: alias.map(|value| value.to_string()),
        });
    false
}

fn try_add_attribute_import_edge(
    state: &mut BuilderState,
    source_idx: GraphNodeIndex,
    module_path: &Path,
    name: &str,
    alias: Option<&str>,
) -> bool {
    if let Some(target_idx) = resolve_attribute_target(state, module_path, name) {
        add_import_edge_if_absent(state, source_idx, target_idx, alias);
        return true;
    }
    false
}

fn resolve_attribute_target(
    state: &mut BuilderState,
    module_path: &Path,
    name: &str,
) -> Option<GraphNodeIndex> {
    let module_id = super::state::normalized_path(module_path);
    let suffix = split_entity_segments(name).join("::");
    let debug = std::env::var_os("PARITY_DEBUG").is_some();
    if !suffix.is_empty() {
        let candidate_id = format!("{}::{}", module_id, suffix);
        if let Some(idx) = state.graph.get_index(&candidate_id) {
            return Some(idx);
        }
        if debug && module_id == "util/process_output.py" && name == "merge_sample_locations" {
            println!(
                "[PARITY DEBUG] candidate {} missing for {}",
                candidate_id, module_id
            );
        }
    }

    let Some(&module_idx) = state.file_nodes.get(module_path) else {
        return None;
    };

    if let Some(symbols) = state.file_symbols.get(module_path) {
        if let Some(&idx) = symbols.get(name) {
            return Some(idx);
        }
        if debug && module_id == "util/process_output.py" && name == "merge_sample_locations" {
            println!(
                "[PARITY DEBUG] symbols for {} did not contain {} (keys: {:?})",
                module_id,
                name,
                symbols.keys().take(5).collect::<Vec<_>>()
            );
        }
    }

    let alias_map = build_alias_map(state, module_idx);
    let targets = resolve_targets(state, module_path, &alias_map, name);
    targets.into_iter().next()
}

fn resolve_deferred_attribute_imports(state: &mut BuilderState) {
    if state.deferred_attribute_imports.is_empty() {
        return;
    }

    let mut remaining = Vec::new();
    let mut progress = true;
    let mut attempts = 0;

    while progress && attempts < 4 {
        progress = false;
        attempts += 1;
        let pending = std::mem::take(&mut state.deferred_attribute_imports);
        for entry in pending {
            if try_add_attribute_import_edge(
                state,
                entry.source_idx,
                &entry.module_path,
                &entry.name,
                entry.alias.as_deref(),
            ) {
                progress = true;
            } else {
                remaining.push(entry);
            }
        }

        state.deferred_attribute_imports = remaining;
        remaining = Vec::new();
    }

    let pending = std::mem::take(&mut state.deferred_attribute_imports);
    let mut still_unresolved = Vec::new();
    let debug = std::env::var_os("PARITY_DEBUG").is_some();
    for entry in pending {
        if let Some(&module_idx) = state.file_nodes.get(&entry.module_path) {
            let alias_cow = entry
                .alias
                .as_deref()
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(entry.name.clone()));
            add_import_edge_if_absent(
                state,
                entry.source_idx,
                module_idx,
                Some(alias_cow.as_ref()),
            );
            if debug {
                if let Some(source_node) = state.graph.node(entry.source_idx) {
                    println!(
                        "[PARITY DEBUG] Fallback import {} -> {} (module file)",
                        source_node.id,
                        super::state::normalized_path(&entry.module_path)
                    );
                }
            }
        } else {
            still_unresolved.push(entry);
        }
    }

    if debug {
        for entry in &still_unresolved {
            if let Some(source_node) = state.graph.node(entry.source_idx) {
                println!(
                    "[PARITY DEBUG] Deferred import unresolved {} -> {}::{}",
                    source_node.id,
                    super::state::normalized_path(&entry.module_path),
                    entry.name
                );
            }
        }
    }

    state.deferred_attribute_imports = still_unresolved;
}

fn resolve_pending_wildcard_exports(state: &mut BuilderState) {
    if state.pending_wildcard_exports.is_empty() {
        return;
    }

    let mut remaining = Vec::new();
    let mut progress = true;
    let mut attempts = 0;

    while progress && attempts < 4 {
        progress = false;
        attempts += 1;
        let pending = std::mem::take(&mut state.pending_wildcard_exports);
        for entry in pending {
            if add_wildcard_export_edges(state, entry.source_idx, &entry.module_path) {
                progress = true;
            } else {
                remaining.push(entry);
            }
        }

        state.pending_wildcard_exports = remaining;
        remaining = Vec::new();
    }
}

fn record_module_alias(
    state: &mut BuilderState,
    source_path: &Path,
    alias: String,
    target_path: PathBuf,
) {
    if alias.is_empty() {
        return;
    }
    let entry = state
        .module_aliases
        .entry(source_path.to_path_buf())
        .or_default();
    entry.insert(alias, target_path.clone());
    if let Some(stem) = target_path.file_stem().and_then(|stem| stem.to_str()) {
        entry
            .entry(stem.to_string())
            .or_insert_with(|| target_path.clone());
    }
    state.resolved_exports.clear();
}

fn expand_wildcard_import(
    state: &mut BuilderState,
    source_idx: GraphNodeIndex,
    module_path: &Path,
) {
    if let Some(source_path) = state.file_index_lookup.get(&source_idx) {
        let entry = state
            .wildcard_imports
            .entry(source_path.clone())
            .or_default();
        if !entry.iter().any(|path| path == module_path) {
            entry.push(module_path.to_path_buf());
        }
    }
}

fn enqueue_wildcard_export(
    state: &mut BuilderState,
    source_idx: GraphNodeIndex,
    module_path: &Path,
) {
    if state
        .pending_wildcard_exports
        .iter()
        .any(|entry| entry.source_idx == source_idx && entry.module_path == module_path)
    {
        return;
    }
    state.pending_wildcard_exports.push(PendingWildcardExport {
        source_idx,
        module_path: module_path.to_path_buf(),
    });
}

fn resolve_exports(state: &mut BuilderState, module_path: &Path) -> HashSet<String> {
    if let Some(cached) = state.resolved_exports.get(module_path) {
        return cached.clone();
    }

    let mut visited = HashSet::new();
    let mut result = HashSet::new();
    resolve_exports_recursive(state, module_path, &mut visited, &mut result);
    state
        .resolved_exports
        .insert(module_path.to_path_buf(), result.clone());
    result
}

fn resolve_exports_recursive(
    state: &mut BuilderState,
    module_path: &Path,
    visited: &mut HashSet<PathBuf>,
    result: &mut HashSet<String>,
) {
    if !visited.insert(module_path.to_path_buf()) {
        return;
    }

    let mut added = false;
    if let Some(info) = state.module_exports.get(module_path).cloned() {
        for name in &info.names {
            if result.insert(name.clone()) {
                added = true;
            }
        }
        for source in info.sources {
            for target in resolve_export_source(state, module_path, &source) {
                resolve_exports_recursive(state, &target, visited, result);
                added = true;
            }
        }
    }

    if !added {
        if let Some(symbols) = state.file_symbols.get(module_path) {
            for name in symbols.keys() {
                if !name.contains("::") {
                    result.insert(name.clone());
                }
            }
        }
    }
}

fn resolve_export_source(
    state: &mut BuilderState,
    module_path: &Path,
    source: &ExportSource,
) -> Vec<PathBuf> {
    match source {
        ExportSource::Module(spec) => resolve_module_spec(state, module_path, spec)
            .into_iter()
            .collect(),
        ExportSource::Alias(name) => {
            let mut paths = Vec::new();
            if let Some(map) = state.module_aliases.get(module_path) {
                if let Some(target) = map.get(name) {
                    paths.push(target.clone());
                }
            }
            if paths.is_empty() {
                let spec = ModuleSpecifier::new(0, vec![name.clone()]);
                if let Some(path) = resolve_module_spec(state, module_path, &spec) {
                    paths.push(path);
                }
            }
            paths
        }
    }
}

fn add_import_edge_if_absent(
    state: &mut BuilderState,
    source_idx: GraphNodeIndex,
    target_idx: GraphNodeIndex,
    alias: Option<&str>,
) {
    let exists = state.graph.graph().edges(source_idx).any(|edge| {
        edge.weight().kind == EdgeKind::Import
            && edge.target() == target_idx
            && edge.weight().alias.as_deref() == alias
    });
    if !exists {
        state.graph.add_edge_with_alias(
            source_idx,
            target_idx,
            EdgeKind::Import,
            alias.map(|value| value.to_string()),
        );
    }
}

fn resolve_module_spec(
    state: &BuilderState,
    current_file: &Path,
    spec: &ModuleSpecifier,
) -> Option<PathBuf> {
    let mut components = if spec.level == 0 {
        Vec::new()
    } else {
        module_components(current_file)
    };

    if spec.level > components.len() {
        return None;
    }
    for _ in 0..spec.level {
        components.pop();
    }

    for segment in &spec.segments {
        if !segment.is_empty() {
            components.push(segment.clone());
        }
    }

    finalize_module_path(&components, &state.file_nodes)
}

pub(in crate::graph::builder) fn parse_module_ast(
    state: &mut BuilderState,
    rel_path: &Path,
) -> Option<Rc<Suite>> {
    if let Some(existing) = state.parsed_modules.get(rel_path) {
        return Some(existing.clone());
    }

    let source = state.file_sources.get(rel_path)?;
    match Suite::parse(source, rel_path.to_string_lossy().as_ref()) {
        Ok(suite) => {
            let rc = Rc::new(suite);
            state
                .parsed_modules
                .insert(rel_path.to_path_buf(), rc.clone());
            Some(rc)
        }
        Err(err) => {
            warn!("Failed to parse Python AST for {:?}: {err}", rel_path);
            None
        }
    }
}

pub(in crate::graph::builder) fn build_alias_map(
    state: &mut BuilderState,
    file_idx: GraphNodeIndex,
) -> HashMap<String, Vec<GraphNodeIndex>> {
    let mut aliases: HashMap<String, Vec<GraphNodeIndex>> = HashMap::new();
    let mut visited = HashSet::new();
    collect_callee_candidates(state, file_idx, &mut visited, &mut aliases);

    if let Some(rel_path) = state.file_index_lookup.get(&file_idx) {
        if let Some(modules) = state.wildcard_imports.get(rel_path).cloned() {
            for module_path in modules {
                let exports = resolve_exports(state, &module_path);
                for name in exports {
                    if let Some(target_idx) = resolve_attribute_target(state, &module_path, &name) {
                        insert_alias(&mut aliases, name, target_idx);
                    }
                }
            }
        }
    }
    aliases
}

fn insert_alias(
    aliases: &mut HashMap<String, Vec<GraphNodeIndex>>,
    name: String,
    target: GraphNodeIndex,
) {
    if name.is_empty() {
        return;
    }
    let entry = aliases.entry(name).or_default();
    if !entry.contains(&target) {
        entry.push(target);
    }
}

fn collect_callee_candidates(
    state: &BuilderState,
    file_idx: GraphNodeIndex,
    visited: &mut HashSet<GraphNodeIndex>,
    aliases: &mut HashMap<String, Vec<GraphNodeIndex>>,
) {
    if !visited.insert(file_idx) {
        return;
    }

    if let Some(rel_path) = state.file_index_lookup.get(&file_idx) {
        if let Some(symbols) = state.file_symbols.get(rel_path) {
            for (name, &idx) in symbols {
                insert_alias(aliases, name.clone(), idx);
            }
        }
    }

    for edge in state.graph.graph().edges(file_idx) {
        if edge.weight().kind != EdgeKind::Import {
            continue;
        }
        let target = edge.target();
        if let Some(alias) = &edge.weight().alias {
            insert_alias(aliases, alias.clone(), target);
        }
        let Some(node) = state.graph.node(target) else {
            continue;
        };

        match node.kind {
            NodeKind::File => {
                if let Some(path) = node.file_path.as_ref() {
                    if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                        insert_alias(aliases, stem.to_string(), target);
                    }
                }
                collect_callee_candidates(state, target, visited, aliases);
            }
            NodeKind::Class => {
                insert_alias(aliases, node.display_name.clone(), target);
                collect_enclosed_entities(state, target, aliases);
            }
            NodeKind::Function => {
                insert_alias(aliases, node.display_name.clone(), target);
            }
            NodeKind::Directory => {}
        }
    }
}

fn collect_enclosed_entities(
    state: &BuilderState,
    parent_idx: GraphNodeIndex,
    aliases: &mut HashMap<String, Vec<GraphNodeIndex>>,
) {
    for edge in state.graph.graph().edges(parent_idx) {
        if edge.weight().kind != EdgeKind::Contain {
            continue;
        }
        let child = edge.target();
        let Some(node) = state.graph.node(child) else {
            continue;
        };
        match node.kind {
            NodeKind::Function => {
                insert_alias(aliases, node.display_name.clone(), child);
            }
            NodeKind::Class => {
                insert_alias(aliases, node.display_name.clone(), child);
                collect_enclosed_entities(state, child, aliases);
            }
            _ => {}
        }
    }
}

pub(in crate::graph::builder) fn resolve_targets(
    state: &BuilderState,
    rel_path: &Path,
    alias_map: &HashMap<String, Vec<GraphNodeIndex>>,
    name: &str,
) -> Vec<GraphNodeIndex> {
    let mut result = Vec::new();
    if let Some(entries) = alias_map.get(name) {
        result.extend(entries.iter().copied());
    }

    if let Some(symbols) = state.file_symbols.get(rel_path) {
        if let Some(&idx) = symbols.get(name) {
            if !result.contains(&idx) {
                result.push(idx);
            }
        }
    }

    result
}

fn split_entity_segments(value: &str) -> Vec<String> {
    value
        .split('.')
        .map(|segment| segment.trim())
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_string())
        .collect()
}
