//! BuilderState and core graph building state management
//!
//! This module contains the language-agnostic state and orchestration logic
//! for building dependency graphs. Language-specific operations are delegated
//! to the `python` module (and future language modules).

use super::python::ast_utils::collect_module_data_from_ast;
use crate::graph::{
    DependencyGraph, EdgeKind, GraphNode, GraphNodeIndex, ImportDirective, ModuleSpecifier,
    NodeKind, ParsedEntity, ParserError, PythonParser, SourceRange,
};
use rustpython_parser::ast::Suite;
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

#[derive(Debug, Default, Clone)]
pub(super) struct ModuleExports {
    pub(super) names: HashSet<String>,
    pub(super) sources: Vec<ExportSource>,
}

impl ModuleExports {
    pub(super) fn is_empty(&self) -> bool {
        self.names.is_empty() && self.sources.is_empty()
    }

    pub(super) fn merge(&mut self, other: &ModuleExports) {
        self.names.extend(other.names.iter().cloned());
        self.sources.extend(other.sources.iter().cloned());
    }

    pub(super) fn add_name(&mut self, name: String) {
        if !name.is_empty() {
            self.names.insert(name);
        }
    }

    pub(super) fn add_source(&mut self, source: ExportSource) {
        self.sources.push(source);
    }
}

#[derive(Debug, Clone)]
pub(super) enum ExportSource {
    Module(ModuleSpecifier),
    Alias(String),
}

#[derive(Debug, Default)]
pub(super) struct AstModuleData {
    pub(super) imports: Vec<ImportDirective>,
    pub(super) exports: ModuleExports,
}

impl AstModuleData {
    pub(super) fn into_parts(self) -> (Vec<ImportDirective>, ModuleExports) {
        (self.imports, self.exports)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphBuilderConfig {
    pub follow_symlinks: bool,
    pub max_python_files: Option<usize>,
    pub allowed_python_files: Option<HashSet<String>>,
    pub required_directories: Option<HashSet<String>>,
    pub allowed_edges: Option<HashMap<(String, String, EdgeKind), usize>>,
}

#[derive(Debug, Clone)]
pub(super) struct DeferredAttributeImport {
    pub(super) source_idx: GraphNodeIndex,
    pub(super) module_path: PathBuf,
    pub(super) name: String,
    pub(super) alias: Option<String>,
}

pub(super) struct PendingWildcardExport {
    pub(super) source_idx: GraphNodeIndex,
    pub(super) module_path: PathBuf,
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

        let mut processed_files = 0usize;
        let max_files = self.config.max_python_files;

        for entry in walker {
            if let Some(limit) = max_files {
                if processed_files >= limit {
                    break;
                }
            }
            let entry = entry?;
            let rel_path = relative_path(&self.repo_root, entry.path());

            if entry.file_type().is_dir() {
                continue;
            }

            if is_python_file(&entry) {
                if let Some(allowed) = &self.config.allowed_python_files {
                    let normalized = normalized_path(&rel_path);
                    if !allowed.contains(&normalized) {
                        continue;
                    }
                }
                state.process_python_file(&mut parser, &rel_path, entry.path())?;
                processed_files += 1;
            }
        }

        if let Some(required_dirs) = &self.config.required_directories {
            for dir in required_dirs {
                let rel_path = if dir.is_empty() {
                    Path::new("")
                } else {
                    Path::new(dir)
                };
                state.ensure_directory_node(rel_path);
            }
        }

        state.process_pending_imports();
        state.process_behavior_edges();

        if let Some(allowed_edges) = &self.config.allowed_edges {
            let mut remaining = allowed_edges.clone();
            let mut to_remove = Vec::new();
            for edge_idx in state.graph.graph().edge_indices() {
                if let Some(weight) = state.graph.graph().edge_weight(edge_idx) {
                    match weight.kind {
                        EdgeKind::Contain => {}
                        EdgeKind::Import | EdgeKind::Invoke | EdgeKind::Inherit => {
                            if let Some((source_idx, target_idx)) =
                                state.graph.graph().edge_endpoints(edge_idx)
                            {
                                if let (Some(source_node), Some(target_node)) =
                                    (state.graph.node(source_idx), state.graph.node(target_idx))
                                {
                                    let key = (
                                        normalize_allowed_id(&source_node.id),
                                        normalize_allowed_id(&target_node.id),
                                        weight.kind,
                                    );
                                    let remove_edge = match remaining.get_mut(&key) {
                                        Some(count) if *count > 0 => {
                                            *count -= 1;
                                            false
                                        }
                                        _ => true,
                                    };
                                    if remove_edge {
                                        to_remove.push(edge_idx);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !to_remove.is_empty() {
                let graph = state.graph.graph_mut();
                for edge_idx in to_remove {
                    graph.remove_edge(edge_idx);
                }
            }
        }

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

pub(super) struct BuilderState {
    pub(super) repo_root: PathBuf,
    pub(super) graph: DependencyGraph,
    pub(super) directory_nodes: HashMap<PathBuf, GraphNodeIndex>,
    pub(super) file_nodes: HashMap<PathBuf, GraphNodeIndex>,
    pub(super) file_index_lookup: HashMap<GraphNodeIndex, PathBuf>,
    pub(super) pending_imports: HashMap<PathBuf, Vec<ImportDirective>>,
    pub(super) file_sources: HashMap<PathBuf, String>,
    pub(super) file_symbols: HashMap<PathBuf, HashMap<String, Vec<GraphNodeIndex>>>,
    pub(super) file_entities: HashMap<PathBuf, Vec<GraphNodeIndex>>,
    pub(super) entity_segments: HashMap<GraphNodeIndex, Vec<String>>,
    pub(super) parsed_modules: HashMap<PathBuf, Rc<Suite>>,
    pub(super) behavior_edge_cache: HashSet<(GraphNodeIndex, GraphNodeIndex, EdgeKind)>,
    pub(super) deferred_attribute_imports: Vec<DeferredAttributeImport>,
    pub(super) pending_wildcard_exports: Vec<PendingWildcardExport>,
    pub(super) module_exports: HashMap<PathBuf, ModuleExports>,
    pub(super) module_aliases: HashMap<PathBuf, HashMap<String, PathBuf>>,
    pub(super) resolved_exports: HashMap<PathBuf, HashSet<String>>,
    pub(super) wildcard_imports: HashMap<PathBuf, Vec<PathBuf>>,
    pub(super) stats: GraphBuildStats,
}

impl BuilderState {
    pub(super) fn new(repo_root: PathBuf) -> Self {
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

    pub(super) fn finish(self) -> GraphBuilderResult {
        GraphBuilderResult {
            graph: self.graph,
            stats: self.stats,
        }
    }

    pub(super) fn resolve_scope_entity(
        &self,
        rel_path: &Path,
        scope: &[String],
    ) -> Option<GraphNodeIndex> {
        let entities = self.file_entities.get(rel_path)?;
        for idx in entities {
            if let Some(segments) = self.entity_segments.get(idx) {
                if segments == scope {
                    return Some(*idx);
                }
            }
        }
        None
    }

    pub(super) fn process_python_file(
        &mut self,
        parser: &mut PythonParser,
        rel_path: &Path,
        absolute_path: &Path,
    ) -> Result<(), GraphError> {
        let file_idx = self.ensure_file_node(rel_path);
        let source = fs::read_to_string(absolute_path)?;
        let tree = parser.parse(&source)?;
        let mut entities = PythonParser::collect_entities_from_tree(&tree, &source);
        let module_data = match Suite::parse(&source, rel_path.to_string_lossy().as_ref()) {
            Ok(ast) => {
                let rc = Rc::new(ast);
                let data = collect_module_data_from_ast(rc.as_ref());
                self.parsed_modules.insert(rel_path.to_path_buf(), rc);
                data
            }
            Err(err) => {
                if let Some(sanitized) = sanitize_f_string_prefixes(&source) {
                    match Suite::parse(&sanitized, rel_path.to_string_lossy().as_ref()) {
                        Ok(ast) => {
                            let rc = Rc::new(ast);
                            let data = collect_module_data_from_ast(rc.as_ref());
                            self.parsed_modules.insert(rel_path.to_path_buf(), rc);
                            data
                        }
                        Err(err2) => {
                            warn!(
                                "Failed to parse Python AST for {:?}: {err} (sanitized: {err2})",
                                rel_path
                            );
                            println!(
                                "[PARITY DEBUG] rustpython parse failed for {}: {}; sanitized err: {}",
                                rel_path.display(),
                                err,
                                err2
                            );
                            AstModuleData {
                                imports: PythonParser::collect_imports_from_tree(&tree, &source),
                                exports: ModuleExports::default(),
                            }
                        }
                    }
                } else {
                    warn!("Failed to parse Python AST for {:?}: {err}", rel_path);
                    println!(
                        "[PARITY DEBUG] rustpython parse failed for {}: {}",
                        rel_path.display(),
                        err
                    );
                    AstModuleData {
                        imports: PythonParser::collect_imports_from_tree(&tree, &source),
                        exports: ModuleExports::default(),
                    }
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
        if entities.is_empty() {
            entities = fallback_entities_from_source(&source);
        }
        let absolute = absolute_path.to_path_buf();
        self.add_entities(file_idx, rel_path, &absolute, entities);
        Ok(())
    }

    pub(super) fn ensure_directory_node(&mut self, rel_path: &Path) -> GraphNodeIndex {
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

    pub(super) fn ensure_file_node(&mut self, rel_path: &Path) -> GraphNodeIndex {
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

    pub(super) fn add_entities(
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
        let symbol_table = self.file_symbols.entry(rel_path.to_path_buf()).or_default();
        symbol_table.clear();
        let entity_list = self
            .file_entities
            .entry(rel_path.to_path_buf())
            .or_default();
        if let Some(source) = self.file_sources.get(rel_path) {
            if let Some(file_node) = self.graph.graph_mut().node_weight_mut(file_idx) {
                file_node
                    .attributes
                    .entry("source_snippet".to_string())
                    .or_insert_with(|| source.clone());
                if let Some(doc) = extract_docstring(source) {
                    file_node
                        .attributes
                        .entry("docstring".to_string())
                        .or_insert(doc);
                }
            }
        }

        for entity in entities {
            let suffix = entity.qualified_name("::");
            let node_id = format!("{}::{}", file_id, suffix);
            let display_name = entity
                .identifier()
                .map(|name| name.to_string())
                .unwrap_or_else(|| suffix.clone());
            let range = entity.range;
            let mut node = GraphNode::entity(
                node_id.clone(),
                entity.kind,
                display_name,
                absolute_path.to_path_buf(),
                range,
            );
            if let Some(range) = range {
                if let Some(source) = self.file_sources.get(rel_path) {
                    if let Some(snippet) = extract_source_snippet(source, range) {
                        if !snippet.trim().is_empty() {
                            if let Some(doc) = extract_docstring(&snippet) {
                                node.attributes.insert("docstring".to_string(), doc);
                            }
                            node.attributes
                                .insert("source_snippet".to_string(), snippet);
                        }
                    }
                }
            }
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
                let entry = symbol_table.entry(identifier.to_string()).or_default();
                if !entry.contains(&node_idx) {
                    entry.push(node_idx);
                }
            }
            let entry = symbol_table.entry(suffix.clone()).or_default();
            if !entry.contains(&node_idx) {
                entry.push(node_idx);
            }
        }
    }

    // Import processing methods will be moved to imports.rs
    pub(super) fn process_pending_imports(&mut self) {
        // Placeholder - will delegate to imports module
        crate::graph::builder::imports::process_pending_imports(self);
    }

    // Behavior edge processing will be moved to behaviors.rs
    pub(super) fn process_behavior_edges(&mut self) {
        // Placeholder - will delegate to behaviors module
        crate::graph::builder::behaviors::process_behavior_edges(self);
    }
}

// Helper functions
pub(super) fn normalize_allowed_id(id: &str) -> String {
    if let Some((file, rest)) = id.split_once("::") {
        let normalized_rest = rest.replace("::", ".");
        format!("{}:{}", file, normalized_rest)
    } else {
        id.to_string()
    }
}

fn extract_source_snippet(source: &str, range: SourceRange) -> Option<String> {
    let start = range.start_line.saturating_sub(1) as usize;
    let end = range.end_line.max(range.start_line) as usize;
    if start >= end {
        return None;
    }

    let lines: Vec<&str> = source.lines().collect();
    if lines.is_empty() || start >= lines.len() {
        return None;
    }
    let end = end.min(lines.len());
    Some(lines[start..end].join("\n"))
}

fn sanitize_f_string_prefixes(source: &str) -> Option<String> {
    if !source.contains(['f', 'F']) {
        return None;
    }

    let mut sanitized = source.to_string();
    const REPLACEMENTS: [(&str, &str); 16] = [
        ("rf\"\"\"", "r\"\"\""),
        ("fr\"\"\"", "r\"\"\""),
        ("RF\"\"\"", "R\"\"\""),
        ("FR\"\"\"", "R\"\"\""),
        ("rf'''", "r'''"),
        ("fr'''", "r'''"),
        ("RF'''", "R'''"),
        ("FR'''", "R'''"),
        ("rf\"", "r\""),
        ("fr\"", "r\""),
        ("RF\"", "R\""),
        ("FR\"", "R\""),
        ("rf'", "r'"),
        ("fr'", "r'"),
        ("RF'", "R'"),
        ("FR'", "R'"),
    ];
    for (pattern, replacement) in REPLACEMENTS {
        sanitized = sanitized.replace(pattern, replacement);
    }
    sanitized = sanitized.replace("f\"\"\"", "\"\"\"");
    sanitized = sanitized.replace("F\"\"\"", "\"\"\"");
    sanitized = sanitized.replace("f'''", "'''");
    sanitized = sanitized.replace("F'''", "'''");
    sanitized = sanitized.replace("f\"", "\"");
    sanitized = sanitized.replace("F\"", "\"");
    sanitized = sanitized.replace("f'", "'");
    sanitized = sanitized.replace("F'", "'");

    Some(strip_format_specs(&sanitized))
}

fn strip_format_specs(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut brace_depth = 0usize;
    let mut skipping_format = false;

    for ch in input.chars() {
        if skipping_format {
            match ch {
                '{' => {
                    brace_depth += 1;
                }
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    skipping_format = false;
                    output.push('}');
                }
                _ => {}
            }
            continue;
        }

        match ch {
            '{' => {
                brace_depth += 1;
                output.push(ch);
            }
            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
                output.push(ch);
            }
            ':' if brace_depth > 0 => {
                skipping_format = true;
            }
            _ => output.push(ch),
        }
    }

    output
}

fn fallback_entities_from_source(source: &str) -> Vec<ParsedEntity> {
    let lines: Vec<&str> = source.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    let mut entities = Vec::new();
    let mut scope_stack: Vec<(usize, String)> = Vec::new();
    let mut line_idx = 0usize;

    while line_idx < lines.len() {
        let line = lines[line_idx];
        let trimmed = line.trim_start();
        if trimmed.starts_with('@') || trimmed.is_empty() {
            line_idx += 1;
            continue;
        }

        let indent = line.len() - trimmed.len();
        while let Some((level, _)) = scope_stack.last() {
            if indent <= *level {
                scope_stack.pop();
            } else {
                break;
            }
        }

        let mut tokens = trimmed;
        let mut is_async = false;
        if tokens.starts_with("async def ") {
            is_async = true;
            tokens = &tokens[6..];
        }

        if tokens.starts_with("def ") {
            if let Some(name) = parse_identifier(tokens, 4) {
                let segments = collect_segments(&scope_stack, &name);
                let end_line = compute_block_extent(&lines, line_idx, indent);
                entities.push(ParsedEntity {
                    segments,
                    kind: NodeKind::Function,
                    range: Some(SourceRange::new((line_idx + 1) as u32, end_line)),
                    is_async,
                });
                scope_stack.push((indent, name));
            }
        } else if tokens.starts_with("class ") {
            if let Some(name) = parse_identifier(tokens, 6) {
                let segments = collect_segments(&scope_stack, &name);
                let end_line = compute_block_extent(&lines, line_idx, indent);
                entities.push(ParsedEntity {
                    segments: segments.clone(),
                    kind: NodeKind::Class,
                    range: Some(SourceRange::new((line_idx + 1) as u32, end_line)),
                    is_async: false,
                });
                scope_stack.push((indent, name));
            }
        }

        line_idx += 1;
    }

    entities
}

fn parse_identifier(tokens: &str, offset: usize) -> Option<String> {
    tokens[offset..]
        .split(|c: char| c == '(' || c == ':' || c.is_whitespace())
        .next()
        .map(str::to_string)
        .filter(|name| !name.is_empty())
}

fn collect_segments(scope_stack: &[(usize, String)], name: &str) -> Vec<String> {
    let mut segments: Vec<String> = scope_stack
        .iter()
        .map(|(_, segment)| segment.clone())
        .collect();
    segments.push(name.to_string());
    segments
}

fn compute_block_extent(lines: &[&str], start_idx: usize, base_indent: usize) -> u32 {
    let mut end_idx = start_idx;
    let mut idx = start_idx + 1;
    let mut saw_body = false;

    while idx < lines.len() {
        let line = lines[idx];
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            if saw_body {
                end_idx = idx;
            }
            idx += 1;
            continue;
        }

        let indent = line.len() - trimmed.len();
        if indent <= base_indent && !trimmed.starts_with('#') {
            break;
        }

        saw_body = true;
        end_idx = idx;

        if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
            let doc_end = find_docstring_end(lines, idx, trimmed);
            end_idx = doc_end;
            idx = doc_end + 1;
            continue;
        }

        idx += 1;
    }

    (end_idx + 1) as u32
}

fn find_docstring_end(lines: &[&str], start_idx: usize, trimmed: &str) -> usize {
    let delimiter = if trimmed.starts_with("\"\"\"") {
        "\"\"\""
    } else {
        "'''"
    };

    if trimmed[delimiter.len()..].contains(delimiter) {
        return start_idx;
    }

    let mut idx = start_idx + 1;
    while idx < lines.len() {
        if lines[idx].contains(delimiter) {
            return idx;
        }
        idx += 1;
    }

    lines.len().saturating_sub(1)
}

fn extract_docstring(text: &str) -> Option<String> {
    for delim in ["\"\"\"", "'''"] {
        let mut start_idx = 0usize;
        while let Some(pos) = text[start_idx..].find(delim) {
            let absolute_start = start_idx + pos;
            let prefix_slice = &text[..absolute_start];
            let has_invalid_prefix = prefix_slice
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_alphabetic())
                .any(|c| !matches!(c.to_ascii_lowercase(), 'r' | 'u' | 'b' | 'f'));
            if has_invalid_prefix {
                start_idx = absolute_start + delim.len();
                continue;
            }
            let after = &text[absolute_start + delim.len()..];
            if let Some(end_pos) = after.find(delim) {
                let doc = after[..end_pos].trim();
                if !doc.is_empty() {
                    return Some(doc.to_string());
                }
                start_idx = absolute_start + delim.len() + end_pos + delim.len();
            } else {
                break;
            }
        }
    }
    None
}

pub(super) fn normalized_path(path: &Path) -> String {
    let value = path.to_string_lossy().replace('\\', "/");
    if value.is_empty() {
        ".".to_string()
    } else {
        value
    }
}

fn relative_path(root: &Path, path: &Path) -> PathBuf {
    match path.strip_prefix(root) {
        Ok(rel) => rel.to_path_buf(),
        Err(_) => path.to_path_buf(),
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
