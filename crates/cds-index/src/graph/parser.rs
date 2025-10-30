//! AST parsing with tree-sitter for Python repositories.

use crate::graph::{NodeKind, SourceRange};
use std::fmt;
use thiserror::Error;
use tree_sitter::{Language, Node, Parser, Point, Tree};

/// Lightweight representation of a parsed entity extracted from the AST.
#[derive(Debug, Clone)]
pub struct ParsedEntity {
    pub segments: Vec<String>,
    pub kind: NodeKind,
    pub range: Option<SourceRange>,
    pub is_async: bool,
}

impl ParsedEntity {
    pub fn qualified_name(&self, separator: &str) -> String {
        self.segments.join(separator)
    }

    pub fn identifier(&self) -> Option<&str> {
        self.segments.last().map(|segment| segment.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct ModuleSpecifier {
    pub level: usize,
    pub segments: Vec<String>,
}

impl ModuleSpecifier {
    pub fn new(level: usize, segments: Vec<String>) -> Self {
        Self { level, segments }
    }
}

#[derive(Debug, Clone)]
pub struct ImportEntity {
    pub name: String,
    pub alias: Option<String>,
    pub is_wildcard: bool,
}

#[derive(Debug, Clone)]
pub enum ImportDirective {
    Module {
        module: ModuleSpecifier,
        alias: Option<String>,
        scope: Option<Vec<String>>,
    },
    FromModule {
        module: ModuleSpecifier,
        entities: Vec<ImportEntity>,
        scope: Option<Vec<String>>,
    },
}

impl ImportDirective {
    pub fn scope(&self) -> Option<&[String]> {
        match self {
            ImportDirective::Module { scope, .. } | ImportDirective::FromModule { scope, .. } => {
                scope.as_deref()
            }
        }
    }
}

/// Error returned by the parsing helpers.
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("failed to initialize tree-sitter python grammar: {0}")]
    Language(String),
    #[error("tree-sitter failed to parse source")]
    Parse,
}

impl From<tree_sitter::LanguageError> for ParserError {
    fn from(err: tree_sitter::LanguageError) -> Self {
        Self::Language(err.to_string())
    }
}

/// Stateful tree-sitter parser wrapper.
pub struct PythonParser {
    parser: Parser,
}

impl fmt::Debug for PythonParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PythonParser").finish()
    }
}

impl PythonParser {
    pub fn new() -> Result<Self, ParserError> {
        let mut parser = Parser::new();
        let language: Language = tree_sitter_python::LANGUAGE.into();
        parser.set_language(&language)?;
        Ok(Self { parser })
    }

    pub fn parse(&mut self, source: &str) -> Result<Tree, ParserError> {
        self.parser.parse(source, None).ok_or(ParserError::Parse)
    }

    /// Parses the source text and extracts class/function entities following LocAgent semantics.
    pub fn parse_entities(&mut self, source: &str) -> Result<Vec<ParsedEntity>, ParserError> {
        let tree = self.parse(source)?;
        Ok(collect_entities(&tree, source.as_bytes()))
    }

    pub fn collect_entities_from_tree(tree: &Tree, source: &str) -> Vec<ParsedEntity> {
        collect_entities(tree, source.as_bytes())
    }

    pub fn collect_imports_from_tree(tree: &Tree, source: &str) -> Vec<ImportDirective> {
        collect_imports(tree, source.as_bytes())
    }
}

fn collect_entities(tree: &Tree, source: &[u8]) -> Vec<ParsedEntity> {
    let mut entities = Vec::new();
    let mut name_stack: Vec<String> = Vec::new();
    let mut kind_stack: Vec<NodeKind> = Vec::new();
    visit_node(
        tree.root_node(),
        source,
        &mut name_stack,
        &mut kind_stack,
        &mut entities,
    );
    entities
}

fn visit_node(
    node: Node,
    source: &[u8],
    name_stack: &mut Vec<String>,
    kind_stack: &mut Vec<NodeKind>,
    entities: &mut Vec<ParsedEntity>,
) {
    match node.kind() {
        "class_definition" => {
            handle_class(node, source, name_stack, kind_stack, entities);
            return;
        }
        "function_definition" | "async_function_definition" => {
            handle_function(
                node,
                source,
                name_stack,
                kind_stack,
                entities,
                node.kind() == "async_function_definition",
            );
            return;
        }
        "decorated_definition" => {
            // The decorated node wraps the actual class/function definition.
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                loop {
                    visit_node(cursor.node(), source, name_stack, kind_stack, entities);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
            return;
        }
        _ => {}
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, name_stack, kind_stack, entities);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn handle_class(
    node: Node,
    source: &[u8],
    name_stack: &mut Vec<String>,
    kind_stack: &mut Vec<NodeKind>,
    entities: &mut Vec<ParsedEntity>,
) {
    let Some(name_node) = node.child_by_field_name("name") else {
        return;
    };
    let Ok(name) = name_node.utf8_text(source) else {
        return;
    };

    name_stack.push(name.to_string());
    kind_stack.push(NodeKind::Class);
    entities.push(ParsedEntity {
        segments: name_stack.clone(),
        kind: NodeKind::Class,
        range: Some(range_from_node(&node)),
        is_async: false,
    });

    if let Some(body) = node.child_by_field_name("body") {
        visit_node(body, source, name_stack, kind_stack, entities);
    }

    kind_stack.pop();
    name_stack.pop();
}

fn handle_function(
    node: Node,
    source: &[u8],
    name_stack: &mut Vec<String>,
    kind_stack: &mut Vec<NodeKind>,
    entities: &mut Vec<ParsedEntity>,
    is_async: bool,
) {
    let Some(name_node) = node.child_by_field_name("name") else {
        return;
    };
    let Ok(name) = name_node.utf8_text(source) else {
        return;
    };

    // Skip __init__ to stay aligned with LocAgent's CodeAnalyzer.
    if matches!(kind_stack.last(), Some(NodeKind::Class)) && name == "__init__" {
        if let Some(body) = node.child_by_field_name("body") {
            visit_node(body, source, name_stack, kind_stack, entities);
        }
        return;
    }

    name_stack.push(name.to_string());
    kind_stack.push(NodeKind::Function);
    entities.push(ParsedEntity {
        segments: name_stack.clone(),
        kind: NodeKind::Function,
        range: Some(range_from_node(&node)),
        is_async,
    });

    if let Some(body) = node.child_by_field_name("body") {
        visit_node(body, source, name_stack, kind_stack, entities);
    }

    kind_stack.pop();
    name_stack.pop();
}

fn range_from_node(node: &Node) -> SourceRange {
    let start: Point = node.start_position();
    let end: Point = node.end_position();
    SourceRange::new(start.row as u32 + 1, end.row as u32 + 1)
}

fn collect_imports(tree: &Tree, source: &[u8]) -> Vec<ImportDirective> {
    let mut directives = Vec::new();
    let mut scope_stack: Vec<String> = Vec::new();
    visit_import_nodes(tree.root_node(), source, &mut directives, &mut scope_stack);
    directives
}

fn visit_import_nodes(
    node: Node,
    source: &[u8],
    directives: &mut Vec<ImportDirective>,
    scope_stack: &mut Vec<String>,
) {
    match node.kind() {
        "class_definition" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                if let Ok(name) = name_node.utf8_text(source) {
                    scope_stack.push(name.to_string());
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        visit_import_nodes(child, source, directives, scope_stack);
                    }
                    scope_stack.pop();
                    return;
                }
            }
        }
        "function_definition" | "async_function_definition" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                if let Ok(name) = name_node.utf8_text(source) {
                    scope_stack.push(name.to_string());
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        visit_import_nodes(child, source, directives, scope_stack);
                    }
                    scope_stack.pop();
                    return;
                }
            }
        }
        "decorated_definition" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                visit_import_nodes(child, source, directives, scope_stack);
            }
            return;
        }
        "import_statement" => {
            directives.extend(parse_import_statement_node(node, source, scope_stack));
            return;
        }
        "import_from_statement" => {
            if let Some(stmt) = parse_from_statement_node(node, source, scope_stack) {
                directives.push(stmt);
            }
            return;
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_import_nodes(child, source, directives, scope_stack);
    }
}

fn parse_import_statement_node(
    node: Node,
    source: &[u8],
    scope_stack: &[String],
) -> Vec<ImportDirective> {
    match node.utf8_text(source) {
        Ok(text) => parse_import_statement_text(text, scope_stack),
        Err(_) => Vec::new(),
    }
}

fn parse_from_statement_node(
    node: Node,
    source: &[u8],
    scope_stack: &[String],
) -> Option<ImportDirective> {
    let text = node.utf8_text(source).ok()?;
    parse_from_statement_text(text, scope_stack)
}

fn parse_import_statement_text(text: &str, scope_stack: &[String]) -> Vec<ImportDirective> {
    let content = text.trim();
    if !content.starts_with("import") {
        return Vec::new();
    }

    let remainder = content.trim_start_matches("import").trim();
    remainder
        .split(',')
        .filter_map(|segment| parse_plain_import(segment.trim(), scope_stack))
        .collect()
}

fn parse_plain_import(segment: &str, scope_stack: &[String]) -> Option<ImportDirective> {
    if segment.is_empty() {
        return None;
    }

    let mut parts = segment.split_whitespace();
    let module_name = parts.next()?.trim();
    if module_name.is_empty() {
        return None;
    }

    let mut alias: Option<String> = None;
    if let Some(token) = parts.next() {
        if token == "as" {
            alias = parts.next().map(|value| value.trim().to_string());
        }
    }

    let scope = scope_from_stack(scope_stack);
    Some(ImportDirective::Module {
        module: ModuleSpecifier::new(0, split_segments(module_name)),
        alias,
        scope,
    })
}

fn parse_from_statement_text(text: &str, scope_stack: &[String]) -> Option<ImportDirective> {
    let content = text.trim();
    if !content.starts_with("from") {
        return None;
    }

    let remainder = content.trim_start_matches("from").trim();
    let (module_part, names_part) = remainder.split_once("import")?;
    let module_spec = parse_module_spec(module_part.trim());
    let entities = parse_import_entities(names_part.trim());
    if entities.is_empty() {
        return None;
    }

    let scope = scope_from_stack(scope_stack);
    Some(ImportDirective::FromModule {
        module: module_spec,
        entities,
        scope,
    })
}

fn scope_from_stack(scope_stack: &[String]) -> Option<Vec<String>> {
    if scope_stack.is_empty() {
        None
    } else {
        Some(scope_stack.to_vec())
    }
}

fn parse_module_spec(raw: &str) -> ModuleSpecifier {
    let mut level = 0;
    let mut index = 0;
    for ch in raw.chars() {
        if ch == '.' {
            level += 1;
            index += 1;
        } else {
            break;
        }
    }
    let remainder = raw[index..].trim();
    ModuleSpecifier::new(level, split_segments(remainder))
}

fn parse_import_entities(raw: &str) -> Vec<ImportEntity> {
    if raw.is_empty() {
        return Vec::new();
    }

    let mut cleaned = raw.replace('\n', " ");
    cleaned = cleaned
        .trim()
        .trim_matches(|c| c == '(' || c == ')')
        .to_string();
    cleaned
        .split(',')
        .filter_map(parse_import_entity)
        .collect()
}

fn parse_import_entity(entry: &str) -> Option<ImportEntity> {
    let mut token = entry.trim();
    if token.is_empty() {
        return None;
    }

    if let Some(idx) = token.find('#') {
        token = &token[..idx];
    }
    let token = token.trim();
    if token.is_empty() {
        return None;
    }

    if token == "*" {
        return Some(ImportEntity {
            name: "*".to_string(),
            alias: None,
            is_wildcard: true,
        });
    }

    let mut parts = token.split_whitespace();
    let name = parts.next()?.trim().to_string();
    if name.is_empty() {
        return None;
    }

    let mut alias: Option<String> = None;
    if let Some(next) = parts.next() {
        if next == "as" {
            alias = parts.next().map(|value| value.trim().to_string());
        }
    }

    Some(ImportEntity {
        name,
        alias,
        is_wildcard: false,
    })
}

fn split_segments(value: &str) -> Vec<String> {
    value
        .split('.')
        .map(|segment| segment.trim())
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_string())
        .collect()
}
