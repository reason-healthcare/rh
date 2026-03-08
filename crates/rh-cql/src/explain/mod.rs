use crate::elm::Library;
use crate::eval::context::{EvalContext, EvalError};
use crate::eval::engine::{evaluate_elm_with_trace, TraceEvent};
use crate::parser::ast;
use crate::parser::span::SourceLocation;
use crate::semantics::typed_ast::{TypedExpression, TypedLibrary, TypedNode, TypedStatement};
use std::fmt::Write;

pub fn explain_parse(library: &ast::Library) -> String {
    let mut out = String::new();
    out.push_str("AST Explanation:\n");
    for stmt in &library.statements {
        match stmt {
            ast::Statement::ExpressionDef(ed) => {
                let loc = format_ast_location(ed.location.as_ref());
                writeln!(out, "ExpressionDef({}){}", ed.name, loc).unwrap();
                explain_expression(&ed.expression, 1, &mut out);
            }
            ast::Statement::FunctionDef(fd) => {
                let loc = format_ast_location(fd.location.as_ref());
                writeln!(out, "FunctionDef({}){}", fd.name, loc).unwrap();
                if let Some(body) = &fd.body {
                    explain_expression(body, 1, &mut out);
                }
            }
        }
    }
    out
}

fn format_ast_location(loc: Option<&SourceLocation>) -> String {
    match loc {
        Some(l) => format!(" @{}:{}", l.line, l.column),
        None => String::new(),
    }
}

/// Recursively walks an `ast::Expression` and appends an indented tree line
/// for each node, showing its type name, any literal value, and source location.
fn explain_expression(expr: &ast::Expression, depth: usize, out: &mut String) {
    let indent = "  ".repeat(depth);
    match expr {
        ast::Expression::Literal(lit) => {
            let desc = match lit {
                ast::Literal::Null => "Literal(Null)".to_string(),
                ast::Literal::Boolean(b) => format!("Literal(Boolean: {b})"),
                ast::Literal::Integer(n) => format!("Literal(Integer: {n})"),
                ast::Literal::Long(n) => format!("Literal(Long: {n})"),
                ast::Literal::Decimal(d) => format!("Literal(Decimal: {d})"),
                ast::Literal::String(s) => format!("Literal(String: {s:?})"),
                ast::Literal::Date(s) => format!("Literal(Date: {s})"),
                ast::Literal::DateTime(s) => format!("Literal(DateTime: {s})"),
                ast::Literal::Time(s) => format!("Literal(Time: {s})"),
                ast::Literal::Quantity { value, unit } => {
                    format!("Literal(Quantity: {value} '{unit}')")
                }
                ast::Literal::Ratio { .. } => "Literal(Ratio)".to_string(),
                ast::Literal::Code { code, .. } => format!("Literal(Code: {code})"),
            };
            writeln!(out, "{indent}{desc}").unwrap();
        }
        ast::Expression::IdentifierRef(r) => {
            let loc = format_ast_location(r.location.as_ref());
            writeln!(out, "{indent}IdentifierRef({}){loc}", r.name).unwrap();
        }
        ast::Expression::QualifiedIdentifierRef(r) => {
            let loc = format_ast_location(r.location.as_ref());
            writeln!(out, "{indent}QualifiedIdentifierRef({}.{}){loc}", r.qualifier, r.name)
                .unwrap();
        }
        ast::Expression::UnaryExpression(u) => {
            let loc = format_ast_location(u.location.as_ref());
            writeln!(out, "{indent}UnaryExpression({:?}){loc}", u.operator).unwrap();
            explain_expression(&u.operand, depth + 1, out);
        }
        ast::Expression::BinaryExpression(b) => {
            let loc = format_ast_location(b.location.as_ref());
            writeln!(out, "{indent}BinaryExpression({:?}){loc}", b.operator).unwrap();
            explain_expression(&b.left, depth + 1, out);
            explain_expression(&b.right, depth + 1, out);
        }
        ast::Expression::TernaryExpression(t) => {
            let loc = format_ast_location(t.location.as_ref());
            writeln!(out, "{indent}TernaryExpression({:?}){loc}", t.operator).unwrap();
            explain_expression(&t.first, depth + 1, out);
            explain_expression(&t.second, depth + 1, out);
            explain_expression(&t.third, depth + 1, out);
        }
        ast::Expression::DateTimeComponentFrom(d) => {
            let loc = format_ast_location(d.location.as_ref());
            writeln!(out, "{indent}DateTimeComponentFrom({:?}){loc}", d.precision).unwrap();
            explain_expression(&d.operand, depth + 1, out);
        }
        ast::Expression::TypeExpression(t) => {
            let loc = format_ast_location(t.location.as_ref());
            writeln!(out, "{indent}TypeExpression({:?}){loc}", t.operator).unwrap();
            explain_expression(&t.operand, depth + 1, out);
        }
        ast::Expression::TimingExpression(t) => {
            let loc = format_ast_location(t.location.as_ref());
            writeln!(out, "{indent}TimingExpression({:?}){loc}", t.timing).unwrap();
            explain_expression(&t.left, depth + 1, out);
            explain_expression(&t.right, depth + 1, out);
        }
        ast::Expression::FunctionInvocation(f) => {
            let loc = format_ast_location(f.location.as_ref());
            writeln!(out, "{indent}FunctionInvocation({}){loc}", f.name).unwrap();
            for arg in &f.arguments {
                explain_expression(arg, depth + 1, out);
            }
        }
        ast::Expression::MemberInvocation(m) => {
            let loc = format_ast_location(m.location.as_ref());
            writeln!(out, "{indent}MemberInvocation({}){loc}", m.name).unwrap();
            explain_expression(&m.source, depth + 1, out);
        }
        ast::Expression::IndexInvocation(i) => {
            let loc = format_ast_location(i.location.as_ref());
            writeln!(out, "{indent}IndexInvocation{loc}").unwrap();
            explain_expression(&i.source, depth + 1, out);
            explain_expression(&i.index, depth + 1, out);
        }
        ast::Expression::Query(q) => {
            let loc = format_ast_location(q.location.as_ref());
            writeln!(out, "{indent}Query{loc}").unwrap();
            for src in &q.sources {
                writeln!(out, "{}  Source({})", indent, src.alias).unwrap();
                explain_expression(&src.expression, depth + 2, out);
            }
            if let Some(w) = &q.where_clause {
                writeln!(out, "{}  Where", indent).unwrap();
                explain_expression(w, depth + 2, out);
            }
            if let Some(r) = &q.return_clause {
                writeln!(out, "{}  Return", indent).unwrap();
                explain_expression(&r.expression, depth + 2, out);
            }
        }
        ast::Expression::Retrieve(r) => {
            let loc = format_ast_location(r.location.as_ref());
            let type_name = match &r.data_type {
                ast::TypeSpecifier::Named(n) => n.name.clone(),
                other => format!("{other:?}"),
            };
            writeln!(out, "{indent}Retrieve({type_name}){loc}").unwrap();
            if let Some(codes) = &r.codes {
                writeln!(out, "{}  Codes", indent).unwrap();
                explain_expression(codes, depth + 2, out);
            }
        }
        ast::Expression::IfThenElse(i) => {
            let loc = format_ast_location(i.location.as_ref());
            writeln!(out, "{indent}IfThenElse{loc}").unwrap();
            explain_expression(&i.condition, depth + 1, out);
            explain_expression(&i.then_expr, depth + 1, out);
            explain_expression(&i.else_expr, depth + 1, out);
        }
        ast::Expression::Case(c) => {
            let loc = format_ast_location(c.location.as_ref());
            writeln!(out, "{indent}Case{loc}").unwrap();
            if let Some(comp) = &c.comparand {
                explain_expression(comp, depth + 1, out);
            }
            for item in &c.items {
                writeln!(out, "{}  CaseItem", indent).unwrap();
                explain_expression(&item.when, depth + 2, out);
                explain_expression(&item.then, depth + 2, out);
            }
            writeln!(out, "{}  Else", indent).unwrap();
            explain_expression(&c.else_expr, depth + 2, out);
        }
        ast::Expression::IntervalExpression(i) => {
            let loc = format_ast_location(i.location.as_ref());
            writeln!(out, "{indent}IntervalExpression{loc}").unwrap();
            if let Some(low) = &i.low {
                explain_expression(low, depth + 1, out);
            }
            if let Some(high) = &i.high {
                explain_expression(high, depth + 1, out);
            }
        }
        ast::Expression::ListExpression(l) => {
            let loc = format_ast_location(l.location.as_ref());
            writeln!(out, "{indent}ListExpression{loc}").unwrap();
            for elem in &l.elements {
                explain_expression(elem, depth + 1, out);
            }
        }
        ast::Expression::TupleExpression(t) => {
            let loc = format_ast_location(t.location.as_ref());
            writeln!(out, "{indent}TupleExpression{loc}").unwrap();
            for elem in &t.elements {
                writeln!(out, "{}  TupleElement({})", indent, elem.name).unwrap();
                explain_expression(&elem.value, depth + 2, out);
            }
        }
        ast::Expression::Instance(inst) => {
            let loc = format_ast_location(inst.location.as_ref());
            let type_name = match &inst.class_type {
                ast::TypeSpecifier::Named(n) => n.name.clone(),
                other => format!("{other:?}"),
            };
            writeln!(out, "{indent}Instance({type_name}){loc}").unwrap();
            for elem in &inst.elements {
                writeln!(out, "{}  InstanceElement({})", indent, elem.name).unwrap();
                explain_expression(&elem.value, depth + 2, out);
            }
        }
        ast::Expression::Let(l) => {
            let loc = format_ast_location(l.location.as_ref());
            writeln!(out, "{indent}Let({}){loc}", l.identifier).unwrap();
            explain_expression(&l.expression, depth + 1, out);
        }
        ast::Expression::Parenthesized(inner) => {
            writeln!(out, "{indent}Parenthesized").unwrap();
            explain_expression(inner, depth + 1, out);
        }
        ast::Expression::MinValue(ts) => {
            writeln!(out, "{indent}MinValue({ts:?})").unwrap();
        }
        ast::Expression::MaxValue(ts) => {
            writeln!(out, "{indent}MaxValue({ts:?})").unwrap();
        }
    }
}

pub fn explain_compile(lib: &TypedLibrary) -> String {
    let mut out = String::new();
    out.push_str("Compile Explanation:\n");
    for stmt_node in &lib.statements {
        match &stmt_node.inner {
            TypedStatement::ExpressionDef { name, body } => {
                writeln!(out, "ExpressionDef({}) → {:?}", name, stmt_node.data_type).unwrap();
                let mut events = Vec::new();
                collect_semantic_events(body, &mut events);
                for event in events {
                    writeln!(out, "{}", event).unwrap();
                }
            }
            TypedStatement::FunctionDef { name, body, .. } => {
                writeln!(out, "FunctionDef({}) → {:?}", name, stmt_node.data_type).unwrap();
                if let Some(body) = body {
                    let mut events = Vec::new();
                    collect_semantic_events(body, &mut events);
                    for event in events {
                        writeln!(out, "{}", event).unwrap();
                    }
                }
            }
        }
    }
    out
}

/// Recursively collects semantic decisions recorded in `SemanticMeta` for each
/// node in the typed expression tree: overload resolution, implicit conversions,
/// and symbol resolution. Each event is prefixed with the source `[line:col]`.
fn collect_semantic_events(node: &TypedNode<TypedExpression>, events: &mut Vec<String>) {
    let line = node.span.start.line;
    let col = node.span.start.column;
    let prefix = format!("  [{line}:{col}]");

    let meta = &node.meta;
    if let Some(ref overload) = meta.resolved_overload {
        events.push(format!(
            "{prefix} overload resolved: {} → {:?}",
            overload, node.data_type
        ));
    }
    if !meta.implicit_conversions.is_empty() {
        events.push(format!(
            "{prefix} implicit conversions: {}",
            meta.implicit_conversions.join(", ")
        ));
    }
    if let Some(ref sym) = meta.resolved_symbol {
        events.push(format!(
            "{prefix} symbol resolved: {} → {:?}",
            sym, node.data_type
        ));
    }

    // Recurse into child nodes
    match &node.inner {
        TypedExpression::UnaryExpression(_, child) => {
            collect_semantic_events(child, events);
        }
        TypedExpression::BinaryExpression(_, l, r) => {
            collect_semantic_events(l, events);
            collect_semantic_events(r, events);
        }
        TypedExpression::TernaryExpression(_, a, b, c) => {
            collect_semantic_events(a, events);
            collect_semantic_events(b, events);
            collect_semantic_events(c, events);
        }
        TypedExpression::FunctionInvocation(f) => {
            for arg in &f.arguments {
                collect_semantic_events(arg, events);
            }
        }
        TypedExpression::IfThenElse(cond, then, else_) => {
            collect_semantic_events(cond, events);
            collect_semantic_events(then, events);
            collect_semantic_events(else_, events);
        }
        TypedExpression::MemberInvocation(m) => {
            collect_semantic_events(&m.expression, events);
        }
        TypedExpression::IndexInvocation(idx) => {
            collect_semantic_events(&idx.expression, events);
            collect_semantic_events(&idx.index, events);
        }
        TypedExpression::Parenthesized(inner) => {
            collect_semantic_events(inner, events);
        }
        TypedExpression::DateTimeComponentFrom(d) => {
            collect_semantic_events(&d.operand, events);
        }
        TypedExpression::TypeExpression(t) => {
            collect_semantic_events(&t.operand, events);
        }
        TypedExpression::TimingExpression(t) => {
            collect_semantic_events(&t.left, events);
            collect_semantic_events(&t.right, events);
        }
        TypedExpression::Query(q) => {
            for src in &q.sources {
                collect_semantic_events(&src.expression, events);
            }
            for lc in &q.let_clauses {
                collect_semantic_events(&lc.expression, events);
            }
            for rel in &q.relationships {
                collect_semantic_events(&rel.source.expression, events);
                if let Some(st) = &rel.such_that {
                    collect_semantic_events(st, events);
                }
            }
            if let Some(w) = &q.where_clause {
                collect_semantic_events(w, events);
            }
            if let Some(r) = &q.return_clause {
                collect_semantic_events(&r.expression, events);
            }
            if let Some(agg) = &q.aggregate_clause {
                collect_semantic_events(&agg.expression, events);
            }
        }
        TypedExpression::Retrieve(r) => {
            if let Some(codes) = &r.codes {
                collect_semantic_events(codes, events);
            }
            if let Some(date_range) = &r.date_range {
                collect_semantic_events(date_range, events);
            }
        }
        TypedExpression::Case(c) => {
            if let Some(comp) = &c.comparand {
                collect_semantic_events(comp, events);
            }
            for item in &c.case_items {
                collect_semantic_events(&item.when, events);
                collect_semantic_events(&item.then, events);
            }
            collect_semantic_events(&c.else_expr, events);
        }
        TypedExpression::IntervalExpression(i) => {
            if let Some(low) = &i.low {
                collect_semantic_events(low, events);
            }
            if let Some(high) = &i.high {
                collect_semantic_events(high, events);
            }
        }
        TypedExpression::ListExpression(elems) => {
            for elem in elems {
                collect_semantic_events(elem, events);
            }
        }
        TypedExpression::TupleExpression(elems) => {
            for elem in elems {
                collect_semantic_events(&elem.value, events);
            }
        }
        TypedExpression::Instance(inst) => {
            for elem in &inst.elements {
                collect_semantic_events(&elem.value, events);
            }
        }
        TypedExpression::LetClause(_, expr) => {
            collect_semantic_events(expr, events);
        }
        _ => {}
    }
}

/// Evaluate a named expression from a compiled ELM [`Library`] and return a
/// human-readable trace of every sub-expression evaluated.
///
/// Uses `evaluate_elm_with_trace` internally once the eval engine is
/// functional.
///
/// # Arguments
///
/// - `library`  — the compiled ELM library
/// - `name`     — the expression definition name to evaluate
/// - `ctx`      — runtime evaluation context
///
/// # Returns
///
/// A formatted trace string on success, or an `EvalError` on failure.
pub fn explain_eval(
    library: &Library,
    name: &str,
    ctx: &EvalContext,
) -> Result<String, EvalError> {
    let (result, trace) = evaluate_elm_with_trace(library, name, ctx)?;
    let mut out = String::new();
    writeln!(out, "Eval Explanation for expression '{name}':").unwrap();
    writeln!(out, "Result: {result:?}").unwrap();
    if trace.is_empty() {
        writeln!(out, "\n(No trace events recorded)").unwrap();
    } else {
        writeln!(out, "\nTrace ({} events):", trace.len()).unwrap();
        for event in &trace {
            format_trace_event(event, &mut out);
        }
    }
    Ok(out)
}

fn format_trace_event(event: &TraceEvent, out: &mut String) {
    let node_id = event.elm_node_id.as_deref().unwrap_or("-");
    let inputs_str: Vec<String> = event.inputs.iter().map(|v| format!("{v:?}")).collect();
    let children_part = if event.children.is_empty() {
        String::new()
    } else {
        format!(" (children: {:?})", event.children)
    };
    writeln!(
        out,
        "  [{}] {} [node:{}]: ({}) \u{2192} {:?}{children_part}",
        event.event_id,
        event.op,
        node_id,
        inputs_str.join(", "),
        event.output,
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elm::{BinaryExpression, Expression, ExpressionDef, ExpressionDefs, Library, Literal, StatementDef};
    use crate::eval::context::{EvalContextBuilder, FixedClock};
    use crate::eval::value::{CqlDateTime, Value};

    fn make_library(name: &str, expr: Expression) -> Library {
        let mut lib = Library::default();
        lib.statements = Some(ExpressionDefs {
            defs: vec![StatementDef::Expression(ExpressionDef {
                name: Some(name.to_string()),
                expression: Some(Box::new(expr)),
                ..Default::default()
            })],
        });
        lib
    }

    fn fixed_ctx() -> EvalContext {
        let clock = FixedClock::new(CqlDateTime {
            year: 2024,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: None,
        });
        EvalContextBuilder::new(clock).build()
    }

    fn int_literal(v: i64) -> Expression {
        Expression::Literal(Literal {
            value: Some(v.to_string()),
            value_type: Some("Integer".to_string()),
            ..Default::default()
        })
    }

    #[test]
    fn test_explain_eval_literal() {
        let lib = make_library("X", int_literal(42));
        let result = explain_eval(&lib, "X", &fixed_ctx()).unwrap();
        assert!(result.contains("Eval Explanation for expression 'X':"));
        assert!(result.contains("Result:"));
        assert!(result.contains("Literal"));
    }

    #[test]
    fn test_explain_eval_add() {
        let expr = Expression::Add(BinaryExpression {
            operand: vec![int_literal(2), int_literal(3)],
            ..Default::default()
        });
        let lib = make_library("Sum", expr);
        let result = explain_eval(&lib, "Sum", &fixed_ctx()).unwrap();
        assert!(result.contains("Eval Explanation for expression 'Sum':"));
        assert!(result.contains("Result: Integer(5)"));
        assert!(result.contains("Add"));
        assert!(result.contains("Trace"));
    }

    #[test]
    fn test_explain_eval_unknown_expression() {
        let lib = Library::default();
        let result = explain_eval(&lib, "Missing", &fixed_ctx());
        assert!(result.is_err());
    }

    #[test]
    fn test_explain_eval_result_contains_value() {
        let lib = make_library("Flag", Expression::Literal(Literal {
            value: Some("true".to_string()),
            value_type: Some("Boolean".to_string()),
            ..Default::default()
        }));
        let result = explain_eval(&lib, "Flag", &fixed_ctx()).unwrap();
        assert!(result.contains("Boolean(true)"));
    }
}
