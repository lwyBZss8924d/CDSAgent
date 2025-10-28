//! Graph construction from parsed AST entities.
//!
//! The builder mirrors LocAgent's `build_graph.py` pipeline:
//! 1. Walk the repository while honoring the SKIP_DIRS rules.
//! 2. Parse every Python file with tree-sitter to extract class/function nodes.
//! 3. Assemble the heterogeneous graph (directories, files, entities) with contain edges.

use crate::graph::{
    DependencyGraph, EdgeKind, GraphNode, GraphNodeIndex, ImportDirective, ImportEntity,
    ModuleSpecifier, NodeKind, ParsedEntity, ParserError, PythonParser,
};
use petgraph::visit::EdgeRef;
use rustpython_parser::ast::{self as pyast, Constant, Expr, Operator, Stmt, Suite};
use rustpython_parser::Parse;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};
use thiserror::Error;
use tracing::warn;
use walkdir::{DirEntry, WalkDir};

const SKIP_DIRS: &[&str] = &[
    ".git",
    ".github",
    ".venv",
    "venv",
    "__pycache__",
    ".pytest_cache",
    "node_modules",
    "site-packages",
    ".tox",
    ".eggs",
    "build",
    "dist",
    ".mypy_cache",
    ".hypothesis",
];

#[derive(Debug, Default, Clone)]
struct ModuleExports {
    names: HashSet<String>,
    sources: Vec<ExportSource>,
}

impl ModuleExports {
    fn is_empty(&self) -> bool {
        self.names.is_empty() && self.sources.is_empty()
    }

    fn merge(&mut self, other: &ModuleExports) {
        self.names.extend(other.names.iter().cloned());
        self.sources.extend(other.sources.iter().cloned());
    }

    fn add_name(&mut self, name: String) {
        if !name.is_empty() {
            self.names.insert(name);
        }
    }

    fn add_source(&mut self, source: ExportSource) {
        self.sources.push(source);
    }
}

#[derive(Debug, Clone)]
enum ExportSource {
    Module(ModuleSpecifier),
    Alias(String),
}

#[derive(Debug, Default)]
struct AstModuleData {
    imports: Vec<ImportDirective>,
    exports: ModuleExports,
}

impl AstModuleData {
    fn into_parts(self) -> (Vec<ImportDirective>, ModuleExports) {
        (self.imports, self.exports)
    }
}

#[derive(Debug, Clone)]
pub struct GraphBuilderConfig {
    pub follow_symlinks: bool,
}

#[derive(Debug, Clone)]
struct DeferredAttributeImport {
    source_idx: GraphNodeIndex,
    module_path: PathBuf,
    name: String,
    alias: Option<String>,
}

struct PendingWildcardExport {
    source_idx: GraphNodeIndex,
    module_path: PathBuf,
}

impl Default for GraphBuilderConfig {
    fn default() -> Self {
        Self {
            follow_symlinks: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphBuilder {
    repo_root: PathBuf,
    config: GraphBuilderConfig,
}

impl GraphBuilder {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
            config: GraphBuilderConfig::default(),
        }
    }

    pub fn with_config(repo_root: impl Into<PathBuf>, config: GraphBuilderConfig) -> Self {
        Self {
            repo_root: repo_root.into(),
            config,
        }
    }

    pub fn build(&self) -> Result<GraphBuilderResult, GraphError> {
        let mut parser = PythonParser::new()?;
        let mut state = BuilderState::new(self.repo_root.clone());

        let walker = WalkDir::new(&self.repo_root)
            .follow_links(self.config.follow_symlinks)
            .into_iter()
            .filter_entry(|entry| !should_skip(entry));

        for entry in walker {
            let entry = entry?;
            let rel_path = relative_path(&self.repo_root, entry.path());

            if entry.file_type().is_dir() {
                continue;
            }

            if is_python_file(&entry) {
                state.process_python_file(&mut parser, &rel_path, entry.path())?;
            }
        }

        state.process_pending_imports();
        state.process_behavior_edges();

        Ok(state.finish())
    }
}

#[derive(Debug, Default, Clone)]
pub struct GraphBuildStats {
    pub directories: usize,
    pub files: usize,
    pub entities: usize,
}

pub struct GraphBuilderResult {
    pub graph: DependencyGraph,
    pub stats: GraphBuildStats,
}

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("walker error: {0}")]
    Walk(#[from] walkdir::Error),
    #[error("parser error: {0}")]
    Parser(#[from] ParserError),
}

struct BuilderState {
    repo_root: PathBuf,
    graph: DependencyGraph,
    directory_nodes: HashMap<PathBuf, GraphNodeIndex>,
    file_nodes: HashMap<PathBuf, GraphNodeIndex>,
    file_index_lookup: HashMap<GraphNodeIndex, PathBuf>,
    pending_imports: HashMap<PathBuf, Vec<ImportDirective>>,
    file_sources: HashMap<PathBuf, String>,
    file_symbols: HashMap<PathBuf, HashMap<String, GraphNodeIndex>>,
    file_entities: HashMap<PathBuf, Vec<GraphNodeIndex>>,
    entity_segments: HashMap<GraphNodeIndex, Vec<String>>,
    parsed_modules: HashMap<PathBuf, Rc<Suite>>,
    behavior_edge_cache: HashSet<(GraphNodeIndex, GraphNodeIndex, EdgeKind)>,
    deferred_attribute_imports: Vec<DeferredAttributeImport>,
    pending_wildcard_exports: Vec<PendingWildcardExport>,
    module_exports: HashMap<PathBuf, ModuleExports>,
    module_aliases: HashMap<PathBuf, HashMap<String, PathBuf>>,
    resolved_exports: HashMap<PathBuf, HashSet<String>>,
    wildcard_imports: HashMap<PathBuf, Vec<PathBuf>>,
    stats: GraphBuildStats,
}

impl BuilderState {
    fn new(repo_root: PathBuf) -> Self {
        let mut graph = DependencyGraph::new();
        let repo_name = repo_root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(".")
            .to_string();
        let root_node = GraphNode::directory(".".into(), repo_name, Some(repo_root.clone()));
        let root_idx = graph.add_node(root_node);

        let mut directory_nodes = HashMap::new();
        directory_nodes.insert(PathBuf::new(), root_idx);

        Self {
            repo_root,
            graph,
            directory_nodes,
            file_nodes: HashMap::new(),
            file_index_lookup: HashMap::new(),
            pending_imports: HashMap::new(),
            file_sources: HashMap::new(),
            file_symbols: HashMap::new(),
            file_entities: HashMap::new(),
            entity_segments: HashMap::new(),
            parsed_modules: HashMap::new(),
            behavior_edge_cache: HashSet::new(),
            deferred_attribute_imports: Vec::new(),
            pending_wildcard_exports: Vec::new(),
            module_exports: HashMap::new(),
            module_aliases: HashMap::new(),
            resolved_exports: HashMap::new(),
            wildcard_imports: HashMap::new(),
            stats: GraphBuildStats {
                directories: 1,
                ..GraphBuildStats::default()
            },
        }
    }

    fn finish(self) -> GraphBuilderResult {
        GraphBuilderResult {
            graph: self.graph,
            stats: self.stats,
        }
    }

    fn process_python_file(
        &mut self,
        parser: &mut PythonParser,
        rel_path: &Path,
        absolute_path: &Path,
    ) -> Result<(), GraphError> {
        let file_idx = self.ensure_file_node(rel_path);
        let source = fs::read_to_string(absolute_path)?;
        let tree = parser.parse(&source)?;
        let entities = PythonParser::collect_entities_from_tree(&tree, &source);
        let module_data = match Suite::parse(&source, rel_path.to_string_lossy().as_ref()) {
            Ok(ast) => {
                let rc = Rc::new(ast);
                let data = collect_module_data_from_ast(rc.as_ref());
                self.parsed_modules.insert(rel_path.to_path_buf(), rc);
                data
            }
            Err(err) => {
                warn!("Failed to parse Python AST for {:?}: {err}", rel_path);
                AstModuleData {
                    imports: PythonParser::collect_imports_from_tree(&tree, &source),
                    exports: ModuleExports::default(),
                }
            }
        };
        let (directives, exports) = module_data.into_parts();
        self.pending_imports
            .entry(rel_path.to_path_buf())
            .or_default()
            .extend(directives);
        if !exports.is_empty() {
            self.module_exports
                .entry(rel_path.to_path_buf())
                .and_modify(|existing| existing.merge(&exports))
                .or_insert(exports);
            self.resolved_exports.clear();
        }
        self.file_sources
            .entry(rel_path.to_path_buf())
            .or_insert_with(|| source.clone());
        let absolute = absolute_path.to_path_buf();
        self.add_entities(file_idx, rel_path, &absolute, entities);
        Ok(())
    }

    fn ensure_directory_node(&mut self, rel_path: &Path) -> GraphNodeIndex {
        if let Some(&idx) = self.directory_nodes.get(rel_path) {
            return idx;
        }

        let id = normalized_path(rel_path);
        let display_name = rel_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| id.clone());
        let absolute = self.repo_root.join(rel_path);
        let node = GraphNode::directory(id, display_name, Some(absolute));
        let idx = self.graph.add_node(node);
        self.directory_nodes.insert(rel_path.to_path_buf(), idx);
        self.stats.directories += 1;

        if let Some(parent) = rel_path.parent() {
            let parent_idx = self.ensure_directory_node(parent);
            self.graph.add_edge(parent_idx, idx, EdgeKind::Contain);
        }

        idx
    }

    fn ensure_file_node(&mut self, rel_path: &Path) -> GraphNodeIndex {
        if let Some(&idx) = self.file_nodes.get(rel_path) {
            return idx;
        }

        let id = normalized_path(rel_path);
        let display_name = rel_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| id.clone());
        let absolute = self.repo_root.join(rel_path);
        let node = GraphNode::file(id.clone(), display_name, absolute);
        let idx = self.graph.add_node(node);
        self.file_nodes.insert(rel_path.to_path_buf(), idx);
        self.file_index_lookup.insert(idx, rel_path.to_path_buf());
        self.stats.files += 1;

        let parent_idx = rel_path
            .parent()
            .map(|parent| self.ensure_directory_node(parent))
            .unwrap_or_else(|| self.ensure_directory_node(Path::new("")));
        self.graph.add_edge(parent_idx, idx, EdgeKind::Contain);
        idx
    }

    fn add_entities(
        &mut self,
        file_idx: GraphNodeIndex,
        rel_path: &Path,
        absolute_path: &Path,
        entities: Vec<ParsedEntity>,
    ) {
        if entities.is_empty() {
            return;
        }

        let file_id = normalized_path(rel_path);
        let mut local_lookup: HashMap<String, GraphNodeIndex> = HashMap::new();
        let symbol_table = self
            .file_symbols
            .entry(rel_path.to_path_buf())
            .or_insert_with(HashMap::new);
        symbol_table.clear();
        let entity_list = self
            .file_entities
            .entry(rel_path.to_path_buf())
            .or_insert_with(Vec::new);

        for entity in entities {
            let suffix = entity.qualified_name("::");
            let node_id = format!("{}::{}", file_id, suffix);
            let display_name = entity
                .identifier()
                .map(|name| name.to_string())
                .unwrap_or_else(|| suffix.clone());
            let node = GraphNode::entity(
                node_id.clone(),
                entity.kind,
                display_name,
                absolute_path.to_path_buf(),
                entity.range,
            );
            let node_idx = self.graph.add_node(node);
            self.entity_segments
                .insert(node_idx, entity.segments.clone());
            entity_list.push(node_idx);

            let parent_idx = if entity.segments.len() == 1 {
                file_idx
            } else {
                let parent_suffix = entity.segments[..entity.segments.len() - 1].join("::");
                let parent_id = format!("{}::{}", file_id, parent_suffix);
                local_lookup.get(&parent_id).copied().unwrap_or(file_idx)
            };

            self.graph.add_edge(parent_idx, node_idx, EdgeKind::Contain);
            local_lookup.insert(node_id, node_idx);
            self.stats.entities += 1;

            if let Some(identifier) = entity.identifier() {
                symbol_table
                    .entry(identifier.to_string())
                    .or_insert(node_idx);
            }
            symbol_table.entry(suffix.clone()).or_insert(node_idx);
        }
    }

    fn process_pending_imports(&mut self) {
        let pending = std::mem::take(&mut self.pending_imports);
        for (rel_path, directives) in pending {
            let Some(&file_idx) = self.file_nodes.get(&rel_path) else {
                continue;
            };
            for directive in directives {
                match directive {
                    ImportDirective::Module { module, alias } => {
                        if let Some(path) = self.resolve_module_spec(&rel_path, &module) {
                            self.add_file_import_edge(file_idx, &path, alias.as_deref());
                            let alias_name =
                                alias.clone().or_else(|| module.segments.last().cloned());
                            if let Some(alias_value) = alias_name {
                                self.record_module_alias(&rel_path, alias_value, path);
                            }
                        }
                    }
                    ImportDirective::FromModule { module, entities } => {
                        self.process_from_import(file_idx, &rel_path, &module, &entities);
                    }
                }
            }
        }

        self.resolve_pending_wildcard_exports();
        self.resolve_deferred_attribute_imports();
    }

    fn process_behavior_edges(&mut self) {
        let rel_paths: Vec<PathBuf> = self.file_sources.keys().cloned().collect();
        for rel_path in rel_paths {
            let Some(&file_idx) = self.file_nodes.get(&rel_path) else {
                continue;
            };
            let Some(module_ast) = self.parse_module_ast(&rel_path) else {
                continue;
            };
            let alias_map = self.build_alias_map(file_idx);
            let entity_indices = match self.file_entities.get(&rel_path) {
                Some(list) => list.clone(),
                None => continue,
            };

            for entity_idx in entity_indices {
                let Some(segments) = self.entity_segments.get(&entity_idx) else {
                    continue;
                };
                let Some(ast_ref) = find_entity_ast(module_ast.as_ref(), segments) else {
                    continue;
                };

                match ast_ref {
                    EntityAstRef::Function(func) => {
                        let mut calls = Vec::new();
                        visit_block(&func.body, &mut calls);
                        collect_decorator_calls(&func.decorator_list, &mut calls);
                        self.connect_behavior_edges(
                            entity_idx,
                            &rel_path,
                            &alias_map,
                            &calls,
                            EdgeKind::Invoke,
                        );
                    }
                    EntityAstRef::AsyncFunction(func) => {
                        let mut calls = Vec::new();
                        visit_block(&func.body, &mut calls);
                        collect_decorator_calls(&func.decorator_list, &mut calls);
                        self.connect_behavior_edges(
                            entity_idx,
                            &rel_path,
                            &alias_map,
                            &calls,
                            EdgeKind::Invoke,
                        );
                    }
                    EntityAstRef::Class(class_def) => {
                        let bases = collect_base_names(class_def);
                        self.connect_behavior_edges(
                            entity_idx,
                            &rel_path,
                            &alias_map,
                            &bases,
                            EdgeKind::Inherit,
                        );

                        let init_calls = collect_class_init_calls(class_def);
                        if !init_calls.is_empty() {
                            self.connect_behavior_edges(
                                entity_idx,
                                &rel_path,
                                &alias_map,
                                &init_calls,
                                EdgeKind::Invoke,
                            );
                        }
                    }
                }
            }
        }
    }

    fn process_from_import(
        &mut self,
        file_idx: GraphNodeIndex,
        source_path: &Path,
        module: &ModuleSpecifier,
        entities: &[ImportEntity],
    ) {
        let base_module_path = self.resolve_module_spec(source_path, module);
        for entity in entities {
            if entity.is_wildcard {
                if let Some(ref module_path) = base_module_path {
                    self.add_file_import_edge(file_idx, module_path, None);
                    self.expand_wildcard_import(file_idx, module_path);
                    if !self.add_wildcard_export_edges(file_idx, module_path) {
                        self.enqueue_wildcard_export(file_idx, module_path);
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

            if let Some(path) = self.resolve_module_spec(source_path, &extended_spec) {
                self.add_file_import_edge(file_idx, &path, entity.alias.as_deref());
                let alias_name = entity.alias.clone().unwrap_or_else(|| entity.name.clone());
                self.record_module_alias(source_path, alias_name, path);
                continue;
            }

            if let Some(ref module_path) = base_module_path {
                let _ = self.add_attribute_import_edge(
                    file_idx,
                    module_path,
                    &entity.name,
                    entity.alias.as_deref(),
                );
            }
        }
    }

    fn add_wildcard_export_edges(
        &mut self,
        source_idx: GraphNodeIndex,
        module_path: &Path,
    ) -> bool {
        let has_explicit_exports = self
            .module_exports
            .get(module_path)
            .map(|info| !info.names.is_empty() || !info.sources.is_empty())
            .unwrap_or(false);
        if !has_explicit_exports {
            return false;
        }

        let exports = self.resolve_exports(module_path);
        let mut added = false;
        for name in exports {
            if self.add_attribute_import_edge(source_idx, module_path, &name, None) {
                added = true;
            }
        }
        added
    }

    fn add_file_import_edge(
        &mut self,
        source_idx: GraphNodeIndex,
        target_path: &Path,
        alias: Option<&str>,
    ) {
        if let Some(&target_idx) = self.file_nodes.get(target_path) {
            self.add_import_edge_if_absent(source_idx, target_idx, alias);
        }
    }

    fn add_attribute_import_edge(
        &mut self,
        source_idx: GraphNodeIndex,
        module_path: &Path,
        name: &str,
        alias: Option<&str>,
    ) -> bool {
        if self.try_add_attribute_import_edge(source_idx, module_path, name, alias) {
            return true;
        }

        self.deferred_attribute_imports
            .push(DeferredAttributeImport {
                source_idx,
                module_path: module_path.to_path_buf(),
                name: name.to_string(),
                alias: alias.map(|value| value.to_string()),
            });
        false
    }

    fn try_add_attribute_import_edge(
        &mut self,
        source_idx: GraphNodeIndex,
        module_path: &Path,
        name: &str,
        alias: Option<&str>,
    ) -> bool {
        if let Some(target_idx) = self.resolve_attribute_target(module_path, name) {
            self.add_import_edge_if_absent(source_idx, target_idx, alias);
            return true;
        }
        false
    }

    fn resolve_attribute_target(
        &mut self,
        module_path: &Path,
        name: &str,
    ) -> Option<GraphNodeIndex> {
        let module_id = normalized_path(module_path);
        let suffix = split_entity_segments(name).join("::");
        let debug = std::env::var_os("PARITY_DEBUG").is_some();
        if !suffix.is_empty() {
            let candidate_id = format!("{}::{}", module_id, suffix);
            if let Some(idx) = self.graph.get_index(&candidate_id) {
                return Some(idx);
            }
            if debug && module_id == "util/process_output.py" && name == "merge_sample_locations" {
                println!(
                    "[PARITY DEBUG] candidate {} missing for {}",
                    candidate_id, module_id
                );
            }
        }

        let Some(&module_idx) = self.file_nodes.get(module_path) else {
            return None;
        };

        if let Some(symbols) = self.file_symbols.get(module_path) {
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

        let alias_map = self.build_alias_map(module_idx);
        let targets = self.resolve_targets(module_path, &alias_map, name);
        targets.into_iter().next()
    }

    fn resolve_deferred_attribute_imports(&mut self) {
        if self.deferred_attribute_imports.is_empty() {
            return;
        }

        let mut remaining = Vec::new();
        let mut progress = true;
        let mut attempts = 0;

        while progress && attempts < 4 {
            progress = false;
            attempts += 1;
            let pending = std::mem::take(&mut self.deferred_attribute_imports);
            for entry in pending {
                if self.try_add_attribute_import_edge(
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

            self.deferred_attribute_imports = remaining;
            remaining = Vec::new();
        }

        let pending = std::mem::take(&mut self.deferred_attribute_imports);
        let mut still_unresolved = Vec::new();
        let debug = std::env::var_os("PARITY_DEBUG").is_some();
        for entry in pending {
            if let Some(&module_idx) = self.file_nodes.get(&entry.module_path) {
                let alias_cow = entry
                    .alias
                    .as_deref()
                    .map(Cow::Borrowed)
                    .unwrap_or_else(|| Cow::Owned(entry.name.clone()));
                self.add_import_edge_if_absent(
                    entry.source_idx,
                    module_idx,
                    Some(alias_cow.as_ref()),
                );
                if debug {
                    if let Some(source_node) = self.graph.node(entry.source_idx) {
                        println!(
                            "[PARITY DEBUG] Fallback import {} -> {} (module file)",
                            source_node.id,
                            normalized_path(&entry.module_path)
                        );
                    }
                }
            } else {
                still_unresolved.push(entry);
            }
        }

        if debug {
            for entry in &still_unresolved {
                if let Some(source_node) = self.graph.node(entry.source_idx) {
                    println!(
                        "[PARITY DEBUG] Deferred import unresolved {} -> {}::{}",
                        source_node.id,
                        normalized_path(&entry.module_path),
                        entry.name
                    );
                }
            }
        }

        self.deferred_attribute_imports = still_unresolved;
    }

    fn resolve_pending_wildcard_exports(&mut self) {
        if self.pending_wildcard_exports.is_empty() {
            return;
        }

        let mut remaining = Vec::new();
        let mut progress = true;
        let mut attempts = 0;

        while progress && attempts < 4 {
            progress = false;
            attempts += 1;
            let pending = std::mem::take(&mut self.pending_wildcard_exports);
            for entry in pending {
                if self.add_wildcard_export_edges(entry.source_idx, &entry.module_path) {
                    progress = true;
                } else {
                    remaining.push(entry);
                }
            }

            self.pending_wildcard_exports = remaining;
            remaining = Vec::new();
        }
    }

    fn record_module_alias(&mut self, source_path: &Path, alias: String, target_path: PathBuf) {
        if alias.is_empty() {
            return;
        }
        let entry = self
            .module_aliases
            .entry(source_path.to_path_buf())
            .or_default();
        entry.insert(alias, target_path.clone());
        if let Some(stem) = target_path.file_stem().and_then(|stem| stem.to_str()) {
            entry
                .entry(stem.to_string())
                .or_insert_with(|| target_path.clone());
        }
        self.resolved_exports.clear();
    }

    fn expand_wildcard_import(&mut self, source_idx: GraphNodeIndex, module_path: &Path) {
        if let Some(source_path) = self.file_index_lookup.get(&source_idx) {
            let entry = self
                .wildcard_imports
                .entry(source_path.clone())
                .or_default();
            if !entry.iter().any(|path| path == module_path) {
                entry.push(module_path.to_path_buf());
            }
        }
    }

    fn enqueue_wildcard_export(&mut self, source_idx: GraphNodeIndex, module_path: &Path) {
        if self
            .pending_wildcard_exports
            .iter()
            .any(|entry| entry.source_idx == source_idx && entry.module_path == module_path)
        {
            return;
        }
        self.pending_wildcard_exports.push(PendingWildcardExport {
            source_idx,
            module_path: module_path.to_path_buf(),
        });
    }

    fn resolve_exports(&mut self, module_path: &Path) -> HashSet<String> {
        if let Some(cached) = self.resolved_exports.get(module_path) {
            return cached.clone();
        }

        let mut visited = HashSet::new();
        let mut result = HashSet::new();
        self.resolve_exports_recursive(module_path, &mut visited, &mut result);
        self.resolved_exports
            .insert(module_path.to_path_buf(), result.clone());
        result
    }

    fn resolve_exports_recursive(
        &mut self,
        module_path: &Path,
        visited: &mut HashSet<PathBuf>,
        result: &mut HashSet<String>,
    ) {
        if !visited.insert(module_path.to_path_buf()) {
            return;
        }

        let mut added = false;
        if let Some(info) = self.module_exports.get(module_path).cloned() {
            for name in &info.names {
                if result.insert(name.clone()) {
                    added = true;
                }
            }
            for source in info.sources {
                for target in self.resolve_export_source(module_path, &source) {
                    self.resolve_exports_recursive(&target, visited, result);
                    added = true;
                }
            }
        }

        if !added {
            if let Some(symbols) = self.file_symbols.get(module_path) {
                for name in symbols.keys() {
                    if !name.contains("::") {
                        result.insert(name.clone());
                    }
                }
            }
        }
    }

    fn resolve_export_source(&mut self, module_path: &Path, source: &ExportSource) -> Vec<PathBuf> {
        match source {
            ExportSource::Module(spec) => self
                .resolve_module_spec(module_path, spec)
                .into_iter()
                .collect(),
            ExportSource::Alias(name) => {
                let mut paths = Vec::new();
                if let Some(map) = self.module_aliases.get(module_path) {
                    if let Some(target) = map.get(name) {
                        paths.push(target.clone());
                    }
                }
                if paths.is_empty() {
                    let spec = ModuleSpecifier::new(0, vec![name.clone()]);
                    if let Some(path) = self.resolve_module_spec(module_path, &spec) {
                        paths.push(path);
                    }
                }
                paths
            }
        }
    }

    fn add_import_edge_if_absent(
        &mut self,
        source_idx: GraphNodeIndex,
        target_idx: GraphNodeIndex,
        alias: Option<&str>,
    ) {
        let exists = self.graph.graph().edges(source_idx).any(|edge| {
            edge.weight().kind == EdgeKind::Import
                && edge.target() == target_idx
                && edge.weight().alias.as_deref() == alias
        });
        if !exists {
            self.graph.add_edge_with_alias(
                source_idx,
                target_idx,
                EdgeKind::Import,
                alias.map(|value| value.to_string()),
            );
        }
    }

    fn resolve_module_spec(&self, current_file: &Path, spec: &ModuleSpecifier) -> Option<PathBuf> {
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

        finalize_module_path(&components, &self.file_nodes)
    }

    fn parse_module_ast(&mut self, rel_path: &Path) -> Option<Rc<Suite>> {
        if let Some(existing) = self.parsed_modules.get(rel_path) {
            return Some(existing.clone());
        }

        let source = self.file_sources.get(rel_path)?;
        match Suite::parse(source, rel_path.to_string_lossy().as_ref()) {
            Ok(suite) => {
                let rc = Rc::new(suite);
                self.parsed_modules
                    .insert(rel_path.to_path_buf(), rc.clone());
                Some(rc)
            }
            Err(err) => {
                warn!("Failed to parse Python AST for {:?}: {err}", rel_path);
                None
            }
        }
    }

    fn build_alias_map(
        &mut self,
        file_idx: GraphNodeIndex,
    ) -> HashMap<String, Vec<GraphNodeIndex>> {
        let mut aliases: HashMap<String, Vec<GraphNodeIndex>> = HashMap::new();
        let mut visited = HashSet::new();
        self.collect_callee_candidates(file_idx, &mut visited, &mut aliases);

        if let Some(rel_path) = self.file_index_lookup.get(&file_idx) {
            if let Some(modules) = self.wildcard_imports.get(rel_path).cloned() {
                for module_path in modules {
                    let exports = self.resolve_exports(&module_path);
                    for name in exports {
                        if let Some(target_idx) = self.resolve_attribute_target(&module_path, &name)
                        {
                            Self::insert_alias(&mut aliases, name, target_idx);
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
        &self,
        file_idx: GraphNodeIndex,
        visited: &mut HashSet<GraphNodeIndex>,
        aliases: &mut HashMap<String, Vec<GraphNodeIndex>>,
    ) {
        if !visited.insert(file_idx) {
            return;
        }

        if let Some(rel_path) = self.file_index_lookup.get(&file_idx) {
            if let Some(symbols) = self.file_symbols.get(rel_path) {
                for (name, &idx) in symbols {
                    Self::insert_alias(aliases, name.clone(), idx);
                }
            }
        }

        for edge in self.graph.graph().edges(file_idx) {
            if edge.weight().kind != EdgeKind::Import {
                continue;
            }
            let target = edge.target();
            if let Some(alias) = &edge.weight().alias {
                Self::insert_alias(aliases, alias.clone(), target);
            }
            let Some(node) = self.graph.node(target) else {
                continue;
            };

            match node.kind {
                NodeKind::File => {
                    if let Some(path) = node.file_path.as_ref() {
                        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                            Self::insert_alias(aliases, stem.to_string(), target);
                        }
                    }
                    self.collect_callee_candidates(target, visited, aliases);
                }
                NodeKind::Class => {
                    Self::insert_alias(aliases, node.display_name.clone(), target);
                    self.collect_enclosed_entities(target, aliases);
                }
                NodeKind::Function => {
                    Self::insert_alias(aliases, node.display_name.clone(), target);
                }
                NodeKind::Directory => {}
            }
        }
    }

    fn collect_enclosed_entities(
        &self,
        parent_idx: GraphNodeIndex,
        aliases: &mut HashMap<String, Vec<GraphNodeIndex>>,
    ) {
        for edge in self.graph.graph().edges(parent_idx) {
            if edge.weight().kind != EdgeKind::Contain {
                continue;
            }
            let child = edge.target();
            let Some(node) = self.graph.node(child) else {
                continue;
            };
            match node.kind {
                NodeKind::Function => {
                    Self::insert_alias(aliases, node.display_name.clone(), child);
                }
                NodeKind::Class => {
                    Self::insert_alias(aliases, node.display_name.clone(), child);
                    self.collect_enclosed_entities(child, aliases);
                }
                _ => {}
            }
        }
    }

    fn connect_behavior_edges(
        &mut self,
        caller_idx: GraphNodeIndex,
        rel_path: &Path,
        alias_map: &HashMap<String, Vec<GraphNodeIndex>>,
        names: &[String],
        kind: EdgeKind,
    ) {
        let mut seen_targets = HashSet::new();
        for name in names {
            let targets = self.resolve_targets(rel_path, alias_map, name);
            for target_idx in targets {
                if seen_targets.insert(target_idx)
                    && self
                        .behavior_edge_cache
                        .insert((caller_idx, target_idx, kind))
                {
                    self.graph.add_edge(caller_idx, target_idx, kind);
                }
            }
        }
    }

    fn resolve_targets(
        &self,
        rel_path: &Path,
        alias_map: &HashMap<String, Vec<GraphNodeIndex>>,
        name: &str,
    ) -> Vec<GraphNodeIndex> {
        let mut result = Vec::new();
        if let Some(entries) = alias_map.get(name) {
            result.extend(entries.iter().copied());
        }

        if let Some(symbols) = self.file_symbols.get(rel_path) {
            if let Some(&idx) = symbols.get(name) {
                if !result.contains(&idx) {
                    result.push(idx);
                }
            }
        }

        result
    }
}

fn collect_module_data_from_ast(suite: &Suite) -> AstModuleData {
    let mut data = AstModuleData::default();
    visit_ast_statements(suite, &mut data, true);
    data
}

fn visit_ast_statements(statements: &[Stmt], data: &mut AstModuleData, module_level: bool) {
    for stmt in statements {
        match stmt {
            pyast::Stmt::Import(import_stmt) => {
                for alias in &import_stmt.names {
                    if let Some(directive) = convert_module_import(alias) {
                        data.imports.push(directive);
                    }
                }
            }
            pyast::Stmt::ImportFrom(import_from) => {
                if let Some(directive) = convert_from_import(import_from) {
                    if module_level {
                        if let ImportDirective::FromModule { module, entities } = &directive {
                            if entities.iter().any(|entity| entity.is_wildcard) {
                                data.exports
                                    .add_source(ExportSource::Module(module.clone()));
                            }
                        }
                    }
                    data.imports.push(directive);
                }
            }
            pyast::Stmt::Assign(assign) if module_level => {
                process_all_assignment(assign, &mut data.exports);
            }
            pyast::Stmt::AugAssign(assign) if module_level => {
                process_all_augassign(assign, &mut data.exports);
            }
            _ => {}
        }

        match stmt {
            pyast::Stmt::FunctionDef(func) => visit_ast_statements(&func.body, data, false),
            pyast::Stmt::AsyncFunctionDef(func) => visit_ast_statements(&func.body, data, false),
            pyast::Stmt::ClassDef(class_def) => visit_ast_statements(&class_def.body, data, false),
            pyast::Stmt::If(stmt_if) => {
                visit_ast_statements(&stmt_if.body, data, false);
                visit_ast_statements(&stmt_if.orelse, data, false);
            }
            pyast::Stmt::For(stmt_for) => {
                visit_ast_statements(&stmt_for.body, data, false);
                visit_ast_statements(&stmt_for.orelse, data, false);
            }
            pyast::Stmt::AsyncFor(stmt_for) => {
                visit_ast_statements(&stmt_for.body, data, false);
                visit_ast_statements(&stmt_for.orelse, data, false);
            }
            pyast::Stmt::While(stmt_while) => {
                visit_ast_statements(&stmt_while.body, data, false);
                visit_ast_statements(&stmt_while.orelse, data, false);
            }
            pyast::Stmt::With(stmt_with) => visit_ast_statements(&stmt_with.body, data, false),
            pyast::Stmt::AsyncWith(stmt_with) => visit_ast_statements(&stmt_with.body, data, false),
            pyast::Stmt::Try(stmt_try) => {
                visit_ast_statements(&stmt_try.body, data, false);
                visit_ast_statements(&stmt_try.orelse, data, false);
                visit_ast_statements(&stmt_try.finalbody, data, false);
                for handler in &stmt_try.handlers {
                    let pyast::ExceptHandler::ExceptHandler(except) = handler;
                    visit_ast_statements(&except.body, data, false);
                }
            }
            pyast::Stmt::Match(stmt_match) => {
                for case in &stmt_match.cases {
                    visit_ast_statements(&case.body, data, false);
                }
            }
            _ => {}
        }
    }
}

fn process_all_assignment(assign: &pyast::StmtAssign, exports: &mut ModuleExports) {
    for target in &assign.targets {
        if matches!(target, Expr::Name(name) if name.id.as_str() == "__all__") {
            collect_exports_from_expr(&assign.value, exports);
        }
    }
}

fn process_all_augassign(assign: &pyast::StmtAugAssign, exports: &mut ModuleExports) {
    if matches!(assign.op, Operator::Add)
        && matches!(&*assign.target, Expr::Name(name) if name.id.as_str() == "__all__")
    {
        collect_exports_from_expr(&assign.value, exports);
    }
}

fn collect_exports_from_expr(expr: &Expr, exports: &mut ModuleExports) {
    match expr {
        Expr::List(list) => {
            for element in &list.elts {
                collect_exports_from_expr(element, exports);
            }
        }
        Expr::Tuple(tuple) => {
            for element in &tuple.elts {
                collect_exports_from_expr(element, exports);
            }
        }
        Expr::Set(set_expr) => {
            for element in &set_expr.elts {
                collect_exports_from_expr(element, exports);
            }
        }
        Expr::Constant(constant) => {
            if let Constant::Str(value) = &constant.value {
                exports.add_name(value.to_string());
            }
        }
        Expr::BinOp(binop) => {
            if matches!(binop.op, Operator::Add) {
                collect_exports_from_expr(&binop.left, exports);
                collect_exports_from_expr(&binop.right, exports);
            }
        }
        Expr::Attribute(attr) => {
            if attr.attr.as_str() == "__all__" {
                if let Some(segments) = attribute_segments(&attr.value) {
                    if segments.len() == 1 {
                        exports.add_source(ExportSource::Alias(segments[0].clone()));
                    } else {
                        exports.add_source(ExportSource::Module(ModuleSpecifier::new(0, segments)));
                    }
                }
            }
        }
        Expr::Name(name) => {
            exports.add_source(ExportSource::Alias(name.id.to_string()));
        }
        _ => {}
    }
}

fn attribute_segments(expr: &Expr) -> Option<Vec<String>> {
    match expr {
        Expr::Name(name) => Some(vec![name.id.to_string()]),
        Expr::Attribute(attr) => {
            let mut segments = attribute_segments(&attr.value)?;
            segments.push(attr.attr.to_string());
            Some(segments)
        }
        _ => None,
    }
}

fn convert_module_import(alias: &pyast::Alias) -> Option<ImportDirective> {
    let module_name = alias.name.to_string();
    if module_name.is_empty() {
        return None;
    }
    Some(ImportDirective::Module {
        module: ModuleSpecifier::new(0, split_entity_segments(&module_name)),
        alias: alias.asname.as_ref().map(|value| value.to_string()),
    })
}

fn convert_from_import(import_from: &pyast::StmtImportFrom) -> Option<ImportDirective> {
    let level = import_from
        .level
        .as_ref()
        .map(|value| value.to_usize())
        .unwrap_or(0);
    let module_name = import_from
        .module
        .as_ref()
        .map(|identifier| identifier.to_string())
        .unwrap_or_default();
    let module = ModuleSpecifier::new(level, split_entity_segments(&module_name));

    let mut entities = Vec::new();
    for alias in &import_from.names {
        let name = alias.name.to_string();
        if name.is_empty() {
            continue;
        }
        if name == "*" {
            entities.push(ImportEntity {
                name,
                alias: None,
                is_wildcard: true,
            });
            continue;
        }

        let alias_name = alias.asname.as_ref().map(|value| value.to_string());
        entities.push(ImportEntity {
            name,
            alias: alias_name,
            is_wildcard: false,
        });
    }

    if entities.is_empty() {
        return None;
    }

    Some(ImportDirective::FromModule { module, entities })
}

fn normalized_path(path: &Path) -> String {
    let value = path.to_string_lossy().replace('\\', "/");
    if value.is_empty() {
        ".".to_string()
    } else {
        value
    }
}

fn module_components(rel_path: &Path) -> Vec<String> {
    let mut components: Vec<String> = rel_path
        .parent()
        .map(|parent| {
            parent
                .iter()
                .map(|part| part.to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();

    if let Some(stem) = rel_path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
    {
        if stem != "__init__" {
            components.push(stem);
        }
    }

    components
}

fn split_entity_segments(value: &str) -> Vec<String> {
    value
        .split('.')
        .map(|segment| segment.trim())
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_string())
        .collect()
}

fn finalize_module_path(
    components: &[String],
    file_nodes: &HashMap<PathBuf, GraphNodeIndex>,
) -> Option<PathBuf> {
    if components.is_empty() {
        return None;
    }

    let mut candidate = PathBuf::new();
    for component in components {
        candidate.push(component);
    }

    let mut file_candidate = candidate.clone();
    file_candidate.set_extension("py");
    if file_nodes.contains_key(&file_candidate) {
        return Some(file_candidate);
    }

    let mut init_candidate = PathBuf::new();
    for component in components {
        init_candidate.push(component);
    }
    init_candidate.push("__init__.py");
    if file_nodes.contains_key(&init_candidate) {
        return Some(init_candidate);
    }

    None
}

fn relative_path(root: &Path, path: &Path) -> PathBuf {
    match path.strip_prefix(root) {
        Ok(rel) => rel.to_path_buf(),
        Err(_) => path.to_path_buf(),
    }
}

enum EntityAstRef<'a> {
    Function(&'a pyast::StmtFunctionDef),
    AsyncFunction(&'a pyast::StmtAsyncFunctionDef),
    Class(&'a pyast::StmtClassDef),
}

fn find_entity_ast<'a>(suite: &'a Suite, segments: &[String]) -> Option<EntityAstRef<'a>> {
    find_in_block(suite, segments)
}

fn find_in_block<'a>(block: &'a [Stmt], segments: &[String]) -> Option<EntityAstRef<'a>> {
    let Some((first, rest)) = segments.split_first() else {
        return None;
    };
    let target = first.as_str();

    for stmt in block {
        match stmt {
            pyast::Stmt::FunctionDef(func) if func.name.as_str() == target => {
                if rest.is_empty() {
                    return Some(EntityAstRef::Function(func));
                } else if let Some(result) = find_in_block(&func.body, rest) {
                    return Some(result);
                }
            }
            pyast::Stmt::AsyncFunctionDef(func) if func.name.as_str() == target => {
                if rest.is_empty() {
                    return Some(EntityAstRef::AsyncFunction(func));
                } else if let Some(result) = find_in_block(&func.body, rest) {
                    return Some(result);
                }
            }
            pyast::Stmt::ClassDef(class_def) if class_def.name.as_str() == target => {
                if rest.is_empty() {
                    return Some(EntityAstRef::Class(class_def));
                } else if let Some(result) = find_in_block(&class_def.body, rest) {
                    return Some(result);
                }
            }
            _ => {}
        }
    }

    None
}

fn visit_block(statements: &[Stmt], calls: &mut Vec<String>) {
    for stmt in statements {
        collect_calls_in_stmt(stmt, calls);
    }
}

fn collect_calls_in_stmt(stmt: &Stmt, calls: &mut Vec<String>) {
    match stmt {
        pyast::Stmt::FunctionDef(_)
        | pyast::Stmt::AsyncFunctionDef(_)
        | pyast::Stmt::ClassDef(_) => {}
        pyast::Stmt::Expr(expr_stmt) => collect_calls_in_expr(&expr_stmt.value, calls),
        pyast::Stmt::Return(ret) => {
            if let Some(value) = &ret.value {
                collect_calls_in_expr(value, calls);
            }
        }
        pyast::Stmt::Assign(assign) => {
            collect_calls_in_expr(&assign.value, calls);
            for target in &assign.targets {
                collect_calls_in_expr(target, calls);
            }
        }
        pyast::Stmt::AnnAssign(assign) => {
            collect_calls_in_expr(&assign.target, calls);
            if let Some(value) = &assign.value {
                collect_calls_in_expr(value, calls);
            }
        }
        pyast::Stmt::AugAssign(assign) => {
            collect_calls_in_expr(&assign.target, calls);
            collect_calls_in_expr(&assign.value, calls);
        }
        pyast::Stmt::If(stmt_if) => {
            collect_calls_in_expr(&stmt_if.test, calls);
            visit_block(&stmt_if.body, calls);
            visit_block(&stmt_if.orelse, calls);
        }
        pyast::Stmt::For(stmt_for) => {
            collect_calls_in_expr(&stmt_for.target, calls);
            collect_calls_in_expr(&stmt_for.iter, calls);
            visit_block(&stmt_for.body, calls);
            visit_block(&stmt_for.orelse, calls);
        }
        pyast::Stmt::AsyncFor(stmt_for) => {
            collect_calls_in_expr(&stmt_for.target, calls);
            collect_calls_in_expr(&stmt_for.iter, calls);
            visit_block(&stmt_for.body, calls);
            visit_block(&stmt_for.orelse, calls);
        }
        pyast::Stmt::While(stmt_while) => {
            collect_calls_in_expr(&stmt_while.test, calls);
            visit_block(&stmt_while.body, calls);
            visit_block(&stmt_while.orelse, calls);
        }
        pyast::Stmt::With(stmt_with) => {
            for item in &stmt_with.items {
                collect_calls_in_expr(&item.context_expr, calls);
                if let Some(vars) = &item.optional_vars {
                    collect_calls_in_expr(vars, calls);
                }
            }
            visit_block(&stmt_with.body, calls);
        }
        pyast::Stmt::AsyncWith(stmt_with) => {
            for item in &stmt_with.items {
                collect_calls_in_expr(&item.context_expr, calls);
                if let Some(vars) = &item.optional_vars {
                    collect_calls_in_expr(vars, calls);
                }
            }
            visit_block(&stmt_with.body, calls);
        }
        pyast::Stmt::Try(stmt_try) => {
            visit_block(&stmt_try.body, calls);
            for handler in &stmt_try.handlers {
                let pyast::ExceptHandler::ExceptHandler(except) = handler;
                if let Some(ty) = &except.type_ {
                    collect_calls_in_expr(ty, calls);
                }
                visit_block(&except.body, calls);
            }
            visit_block(&stmt_try.orelse, calls);
            visit_block(&stmt_try.finalbody, calls);
        }
        pyast::Stmt::Raise(stmt_raise) => {
            if let Some(exc) = &stmt_raise.exc {
                collect_calls_in_expr(exc, calls);
            }
            if let Some(cause) = &stmt_raise.cause {
                collect_calls_in_expr(cause, calls);
            }
        }
        pyast::Stmt::Assert(stmt_assert) => {
            collect_calls_in_expr(&stmt_assert.test, calls);
            if let Some(msg) = &stmt_assert.msg {
                collect_calls_in_expr(msg, calls);
            }
        }
        pyast::Stmt::Match(stmt_match) => {
            collect_calls_in_expr(&stmt_match.subject, calls);
            for case in &stmt_match.cases {
                if let Some(guard) = &case.guard {
                    collect_calls_in_expr(guard, calls);
                }
                visit_block(&case.body, calls);
            }
        }
        _ => {}
    }
}

fn collect_calls_in_expr(expr: &Expr, calls: &mut Vec<String>) {
    match expr {
        pyast::Expr::Call(call) => {
            if let Some(name) = extract_call_name(&call.func) {
                calls.push(name);
            }
            collect_calls_in_expr(&call.func, calls);
            for arg in &call.args {
                collect_calls_in_expr(arg, calls);
            }
            for keyword in &call.keywords {
                collect_calls_in_expr(&keyword.value, calls);
            }
        }
        pyast::Expr::Attribute(attr) => collect_calls_in_expr(&attr.value, calls),
        pyast::Expr::BoolOp(boolop) => {
            for value in &boolop.values {
                collect_calls_in_expr(value, calls);
            }
        }
        pyast::Expr::BinOp(binop) => {
            collect_calls_in_expr(&binop.left, calls);
            collect_calls_in_expr(&binop.right, calls);
        }
        pyast::Expr::UnaryOp(unary) => collect_calls_in_expr(&unary.operand, calls),
        pyast::Expr::Lambda(lambda) => collect_calls_in_expr(&lambda.body, calls),
        pyast::Expr::IfExp(ifexp) => {
            collect_calls_in_expr(&ifexp.test, calls);
            collect_calls_in_expr(&ifexp.body, calls);
            collect_calls_in_expr(&ifexp.orelse, calls);
        }
        pyast::Expr::Dict(dict) => {
            for key in &dict.keys {
                if let Some(k) = key {
                    collect_calls_in_expr(k, calls);
                }
            }
            for value in &dict.values {
                collect_calls_in_expr(value, calls);
            }
        }
        pyast::Expr::Set(set_expr) => {
            for elt in &set_expr.elts {
                collect_calls_in_expr(elt, calls);
            }
        }
        pyast::Expr::ListComp(comp) => {
            collect_calls_in_expr(&comp.elt, calls);
            for generator in &comp.generators {
                collect_comprehension(generator, calls);
            }
        }
        pyast::Expr::SetComp(comp) => {
            collect_calls_in_expr(&comp.elt, calls);
            for generator in &comp.generators {
                collect_comprehension(generator, calls);
            }
        }
        pyast::Expr::DictComp(comp) => {
            collect_calls_in_expr(&comp.key, calls);
            collect_calls_in_expr(&comp.value, calls);
            for generator in &comp.generators {
                collect_comprehension(generator, calls);
            }
        }
        pyast::Expr::GeneratorExp(comp) => {
            collect_calls_in_expr(&comp.elt, calls);
            for generator in &comp.generators {
                collect_comprehension(generator, calls);
            }
        }
        pyast::Expr::Await(await_expr) => collect_calls_in_expr(&await_expr.value, calls),
        pyast::Expr::Yield(yield_expr) => {
            if let Some(value) = &yield_expr.value {
                collect_calls_in_expr(value, calls);
            }
        }
        pyast::Expr::YieldFrom(yield_from) => collect_calls_in_expr(&yield_from.value, calls),
        pyast::Expr::Compare(compare) => {
            collect_calls_in_expr(&compare.left, calls);
            for comp in &compare.comparators {
                collect_calls_in_expr(comp, calls);
            }
        }
        pyast::Expr::FormattedValue(fvalue) => collect_calls_in_expr(&fvalue.value, calls),
        pyast::Expr::JoinedStr(joined) => {
            for value in &joined.values {
                collect_calls_in_expr(value, calls);
            }
        }
        pyast::Expr::Subscript(sub) => {
            collect_calls_in_expr(&sub.value, calls);
            collect_calls_in_expr(&sub.slice, calls);
        }
        pyast::Expr::Starred(starred) => collect_calls_in_expr(&starred.value, calls),
        pyast::Expr::List(list) => {
            for elt in &list.elts {
                collect_calls_in_expr(elt, calls);
            }
        }
        pyast::Expr::Tuple(tuple) => {
            for elt in &tuple.elts {
                collect_calls_in_expr(elt, calls);
            }
        }
        pyast::Expr::Slice(slice) => {
            if let Some(lower) = &slice.lower {
                collect_calls_in_expr(lower, calls);
            }
            if let Some(upper) = &slice.upper {
                collect_calls_in_expr(upper, calls);
            }
            if let Some(step) = &slice.step {
                collect_calls_in_expr(step, calls);
            }
        }
        pyast::Expr::NamedExpr(named) => {
            collect_calls_in_expr(&named.target, calls);
            collect_calls_in_expr(&named.value, calls);
        }
        pyast::Expr::Constant(_) | pyast::Expr::Name(_) => {}
    }
}

fn collect_comprehension(generator: &pyast::Comprehension, calls: &mut Vec<String>) {
    collect_calls_in_expr(&generator.target, calls);
    collect_calls_in_expr(&generator.iter, calls);
    for if_expr in &generator.ifs {
        collect_calls_in_expr(if_expr, calls);
    }
}

fn collect_decorator_calls(decorators: &[Expr], calls: &mut Vec<String>) {
    for decorator in decorators {
        if let Some(name) = extract_call_name(decorator) {
            calls.push(name);
        }
        collect_calls_in_expr(decorator, calls);
    }
}

fn collect_base_names(class_def: &pyast::StmtClassDef) -> Vec<String> {
    class_def
        .bases
        .iter()
        .filter_map(extract_name_from_expr)
        .collect()
}

fn collect_class_init_calls(class_def: &pyast::StmtClassDef) -> Vec<String> {
    for stmt in &class_def.body {
        match stmt {
            pyast::Stmt::FunctionDef(func) if func.name.as_str() == "__init__" => {
                let mut calls = Vec::new();
                visit_block(&func.body, &mut calls);
                collect_decorator_calls(&func.decorator_list, &mut calls);
                return calls;
            }
            pyast::Stmt::AsyncFunctionDef(func) if func.name.as_str() == "__init__" => {
                let mut calls = Vec::new();
                visit_block(&func.body, &mut calls);
                collect_decorator_calls(&func.decorator_list, &mut calls);
                return calls;
            }
            _ => {}
        }
    }

    Vec::new()
}

fn extract_call_name(expr: &Expr) -> Option<String> {
    match expr {
        pyast::Expr::Name(name) => Some(name.id.to_string()),
        pyast::Expr::Attribute(attr) => Some(attr.attr.to_string()),
        _ => None,
    }
}

fn extract_name_from_expr(expr: &Expr) -> Option<String> {
    match expr {
        pyast::Expr::Name(name) => Some(name.id.to_string()),
        pyast::Expr::Attribute(attr) => Some(attr.attr.to_string()),
        pyast::Expr::Call(call) => extract_call_name(&call.func),
        _ => None,
    }
}

fn is_python_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.eq_ignore_ascii_case("py"))
        .unwrap_or(false)
}

fn should_skip(entry: &DirEntry) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    let name = entry.file_name().to_string_lossy();
    SKIP_DIRS.iter().any(|skip| *skip == name)
}
