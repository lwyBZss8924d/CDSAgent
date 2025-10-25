//! Graph construction from parsed AST entities.
//!
//! The builder mirrors LocAgent's `build_graph.py` pipeline:
//! 1. Walk the repository while honoring the SKIP_DIRS rules.
//! 2. Parse every Python file with tree-sitter to extract class/function nodes.
//! 3. Assemble the heterogeneous graph (directories, files, entities) with contain edges.

use crate::graph::{
    DependencyGraph, EdgeKind, GraphNode, GraphNodeIndex, ImportDirective, ImportEntity,
    ModuleSpecifier, ParsedEntity, ParserError, PythonParser,
};
use petgraph::visit::EdgeRef;
use rustpython_parser::ast::{self as pyast, Expr, Stmt, Suite};
use rustpython_parser::Parse;
use std::{
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

#[derive(Debug, Clone)]
pub struct GraphBuilderConfig {
    pub follow_symlinks: bool,
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
                state.ensure_directory_node(&rel_path);
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
    pending_imports: HashMap<PathBuf, Vec<ImportDirective>>,
    file_sources: HashMap<PathBuf, String>,
    file_symbols: HashMap<PathBuf, HashMap<String, GraphNodeIndex>>,
    file_entities: HashMap<PathBuf, Vec<GraphNodeIndex>>,
    entity_segments: HashMap<GraphNodeIndex, Vec<String>>,
    parsed_modules: HashMap<PathBuf, Rc<Suite>>,
    behavior_edge_cache: HashSet<(GraphNodeIndex, GraphNodeIndex, EdgeKind)>,
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
            pending_imports: HashMap::new(),
            file_sources: HashMap::new(),
            file_symbols: HashMap::new(),
            file_entities: HashMap::new(),
            entity_segments: HashMap::new(),
            parsed_modules: HashMap::new(),
            behavior_edge_cache: HashSet::new(),
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
        let imports = PythonParser::collect_imports_from_tree(&tree, &source);
        self.pending_imports
            .entry(rel_path.to_path_buf())
            .or_default()
            .extend(imports);
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
                        }
                    }
                    ImportDirective::FromModule { module, entities } => {
                        self.process_from_import(file_idx, &rel_path, &module, &entities);
                    }
                }
            }
        }
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
                continue;
            }

            if let Some(ref module_path) = base_module_path {
                self.add_attribute_import_edge(
                    file_idx,
                    module_path,
                    &entity.name,
                    entity.alias.as_deref(),
                );
            }
        }
    }

    fn add_file_import_edge(
        &mut self,
        source_idx: GraphNodeIndex,
        target_path: &Path,
        alias: Option<&str>,
    ) {
        if let Some(&target_idx) = self.file_nodes.get(target_path) {
            self.graph.add_edge_with_alias(
                source_idx,
                target_idx,
                EdgeKind::Import,
                alias.map(|value| value.to_string()),
            );
        }
    }

    fn add_attribute_import_edge(
        &mut self,
        source_idx: GraphNodeIndex,
        module_path: &Path,
        name: &str,
        alias: Option<&str>,
    ) {
        let module_id = normalized_path(module_path);
        let suffix = split_entity_segments(name).join("::");
        if suffix.is_empty() {
            return;
        }

        let candidate_id = format!("{}::{}", module_id, suffix);
        if let Some(target_idx) = self.graph.get_index(&candidate_id) {
            self.graph.add_edge_with_alias(
                source_idx,
                target_idx,
                EdgeKind::Import,
                alias.map(|value| value.to_string()),
            );
        }
    }

    fn resolve_module_spec(&self, current_file: &Path, spec: &ModuleSpecifier) -> Option<PathBuf> {
        let mut components = module_components(current_file);
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

    fn build_alias_map(&self, file_idx: GraphNodeIndex) -> HashMap<String, GraphNodeIndex> {
        let mut aliases = HashMap::new();
        for edge in self.graph.graph().edges(file_idx) {
            if edge.weight().kind != EdgeKind::Import {
                continue;
            }
            let target = edge.target();
            if let Some(alias) = &edge.weight().alias {
                aliases.entry(alias.clone()).or_insert(target);
            } else if let Some(node) = self.graph.node(target) {
                aliases.entry(node.display_name.clone()).or_insert(target);
            }
        }
        aliases
    }

    fn connect_behavior_edges(
        &mut self,
        caller_idx: GraphNodeIndex,
        rel_path: &Path,
        alias_map: &HashMap<String, GraphNodeIndex>,
        names: &[String],
        kind: EdgeKind,
    ) {
        let mut seen_targets = HashSet::new();
        for name in names {
            if let Some(target_idx) = self.resolve_name(rel_path, alias_map, name) {
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

    fn resolve_name(
        &self,
        rel_path: &Path,
        alias_map: &HashMap<String, GraphNodeIndex>,
        name: &str,
    ) -> Option<GraphNodeIndex> {
        if let Some(&idx) = alias_map.get(name) {
            return Some(idx);
        }
        if let Some(symbols) = self.file_symbols.get(rel_path) {
            if let Some(&idx) = symbols.get(name) {
                return Some(idx);
            }
        }
        self.lookup_global(name)
    }

    fn lookup_global(&self, name: &str) -> Option<GraphNodeIndex> {
        self.graph
            .graph()
            .node_indices()
            .find(|idx| match self.graph.node(*idx) {
                Some(node) if node.display_name == name => true,
                _ => false,
            })
    }
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
