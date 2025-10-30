//! Python AST utilities and visitors
//!
//! This module contains Python-specific AST visiting and parsing logic
//! using rustpython_parser.

use super::super::state::{AstModuleData, ExportSource, ModuleExports};
use crate::graph::{ImportDirective, ImportEntity, ModuleSpecifier};
use rustpython_parser::ast::{self as pyast, Constant, Expr, Operator, Stmt, Suite};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Collects module-level imports and exports from a Python AST
///
/// This is the main entry point for extracting module data from rustpython AST.
/// It visits all statements at module level and collects:
/// - Import directives (import X, from X import Y)
/// - Export declarations (__all__ assignments)
pub(in crate::graph::builder) fn collect_module_data_from_ast(suite: &Suite) -> AstModuleData {
    let mut data = AstModuleData::default();
    let mut scope_stack = Vec::new();
    visit_ast_statements(suite, &mut data, true, &mut scope_stack);
    data
}

fn visit_ast_statements(
    statements: &[Stmt],
    data: &mut AstModuleData,
    module_level: bool,
    scope_stack: &mut Vec<String>,
) {
    for stmt in statements {
        match stmt {
            pyast::Stmt::Import(import_stmt) => {
                for alias in &import_stmt.names {
                    if let Some(directive) = convert_module_import(alias, scope_stack) {
                        data.imports.push(directive);
                    }
                }
            }
            pyast::Stmt::ImportFrom(import_from) => {
                if let Some(directive) = convert_from_import(import_from, scope_stack) {
                    if module_level {
                        if let ImportDirective::FromModule {
                            module, entities, ..
                        } = &directive
                        {
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
            pyast::Stmt::FunctionDef(func) => {
                scope_stack.push(func.name.to_string());
                visit_ast_statements(&func.body, data, false, scope_stack);
                scope_stack.pop();
            }
            pyast::Stmt::AsyncFunctionDef(func) => {
                scope_stack.push(func.name.to_string());
                visit_ast_statements(&func.body, data, false, scope_stack);
                scope_stack.pop();
            }
            pyast::Stmt::ClassDef(class_def) => {
                scope_stack.push(class_def.name.to_string());
                visit_ast_statements(&class_def.body, data, false, scope_stack);
                scope_stack.pop();
            }
            pyast::Stmt::If(stmt_if) => {
                visit_ast_statements(&stmt_if.body, data, false, scope_stack);
                visit_ast_statements(&stmt_if.orelse, data, false, scope_stack);
            }
            pyast::Stmt::For(stmt_for) => {
                visit_ast_statements(&stmt_for.body, data, false, scope_stack);
                visit_ast_statements(&stmt_for.orelse, data, false, scope_stack);
            }
            pyast::Stmt::AsyncFor(stmt_for) => {
                visit_ast_statements(&stmt_for.body, data, false, scope_stack);
                visit_ast_statements(&stmt_for.orelse, data, false, scope_stack);
            }
            pyast::Stmt::While(stmt_while) => {
                visit_ast_statements(&stmt_while.body, data, false, scope_stack);
                visit_ast_statements(&stmt_while.orelse, data, false, scope_stack);
            }
            pyast::Stmt::With(stmt_with) => {
                visit_ast_statements(&stmt_with.body, data, false, scope_stack);
            }
            pyast::Stmt::AsyncWith(stmt_with) => {
                visit_ast_statements(&stmt_with.body, data, false, scope_stack);
            }
            pyast::Stmt::Try(stmt_try) => {
                visit_ast_statements(&stmt_try.body, data, false, scope_stack);
                visit_ast_statements(&stmt_try.orelse, data, false, scope_stack);
                visit_ast_statements(&stmt_try.finalbody, data, false, scope_stack);
                for handler in &stmt_try.handlers {
                    let pyast::ExceptHandler::ExceptHandler(except) = handler;
                    visit_ast_statements(&except.body, data, false, scope_stack);
                }
            }
            pyast::Stmt::Match(stmt_match) => {
                for case in &stmt_match.cases {
                    visit_ast_statements(&case.body, data, false, scope_stack);
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

fn convert_module_import(alias: &pyast::Alias, scope_stack: &[String]) -> Option<ImportDirective> {
    let module_name = alias.name.to_string();
    if module_name.is_empty() {
        return None;
    }
    Some(ImportDirective::Module {
        module: ModuleSpecifier::new(0, split_entity_segments(&module_name)),
        alias: alias.asname.as_ref().map(|value| value.to_string()),
        scope: scope_from_stack(scope_stack),
    })
}

fn convert_from_import(
    import_from: &pyast::StmtImportFrom,
    scope_stack: &[String],
) -> Option<ImportDirective> {
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

    Some(ImportDirective::FromModule {
        module,
        entities,
        scope: scope_from_stack(scope_stack),
    })
}

fn scope_from_stack(scope_stack: &[String]) -> Option<Vec<String>> {
    if scope_stack.is_empty() {
        None
    } else {
        Some(scope_stack.to_vec())
    }
}

fn split_entity_segments(value: &str) -> Vec<String> {
    value
        .split('.')
        .map(|segment| segment.trim())
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_string())
        .collect()
}

/// Helper functions for module path resolution (used by import resolver)
pub(in crate::graph::builder) fn module_components(rel_path: &Path) -> Vec<String> {
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

pub(in crate::graph::builder) fn finalize_module_path(
    components: &[String],
    file_nodes: &HashMap<PathBuf, crate::graph::GraphNodeIndex>,
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

/// AST entity lookup helpers (used by behavior edge builder)
pub(in crate::graph::builder) enum EntityAstRef<'a> {
    Function(&'a pyast::StmtFunctionDef),
    AsyncFunction(&'a pyast::StmtAsyncFunctionDef),
    Class(&'a pyast::StmtClassDef),
}

pub(in crate::graph::builder) fn find_entity_ast<'a>(
    suite: &'a Suite,
    segments: &[String],
) -> Option<EntityAstRef<'a>> {
    find_in_block(suite, segments)
}

fn find_in_block<'a>(block: &'a [Stmt], segments: &[String]) -> Option<EntityAstRef<'a>> {
    let (first, rest) = segments.split_first()?;
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
            pyast::Stmt::If(stmt_if) => {
                if let Some(result) = find_in_block(&stmt_if.body, segments) {
                    return Some(result);
                }
                if let Some(result) = find_in_block(&stmt_if.orelse, segments) {
                    return Some(result);
                }
            }
            _ => {}
        }
    }

    None
}

/// Call extraction helpers (used by behavior edge builder)
pub(in crate::graph::builder) fn visit_block(statements: &[Stmt], calls: &mut Vec<String>) {
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
            for key in dict.keys.iter().flatten() {
                collect_calls_in_expr(key, calls);
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

pub(in crate::graph::builder) fn collect_decorator_calls(
    decorators: &[Expr],
    calls: &mut Vec<String>,
) {
    for decorator in decorators {
        if let Some(name) = extract_call_name(decorator) {
            calls.push(name);
        }
        collect_calls_in_expr(decorator, calls);
    }
}

pub(in crate::graph::builder) fn collect_base_names(
    class_def: &pyast::StmtClassDef,
) -> Vec<String> {
    class_def
        .bases
        .iter()
        .filter_map(extract_name_from_expr)
        .collect()
}

pub(in crate::graph::builder) fn collect_class_init_calls(
    class_def: &pyast::StmtClassDef,
) -> Vec<String> {
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
