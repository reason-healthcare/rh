use crate::parser::ast;
use crate::semantics::typed_ast::{TypedExpression, TypedLibrary, TypedNode, TypedStatement};
use std::fmt::Write;

pub fn explain_parse(library: &ast::Library) -> String {
    let mut out = String::new();
    out.push_str("AST Explanation:\n");
    for stmt in &library.statements {
        match stmt {
            ast::Statement::ExpressionDef(ed) => {
                writeln!(out, "ExpressionDef({})", ed.name).unwrap();
            }
            ast::Statement::FunctionDef(fd) => {
                writeln!(out, "FunctionDef({})", fd.name).unwrap();
            }
        }
    }
    out
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
/// and symbol resolution.
fn collect_semantic_events(node: &TypedNode<TypedExpression>, events: &mut Vec<String>) {
    let meta = &node.meta;
    if let Some(ref overload) = meta.resolved_overload {
        events.push(format!(
            "  overload resolved: {} → {:?}",
            overload, node.data_type
        ));
    }
    if !meta.implicit_conversions.is_empty() {
        events.push(format!(
            "  implicit conversions: {}",
            meta.implicit_conversions.join(", ")
        ));
    }
    if let Some(ref sym) = meta.resolved_symbol {
        events.push(format!("  symbol resolved: {} → {:?}", sym, node.data_type));
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
        _ => {}
    }
}

pub fn explain_eval() -> Result<String, String> {
    Err("Evaluation engine not yet available".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explain_eval_stub() {
        assert_eq!(
            explain_eval(),
            Err("Evaluation engine not yet available".to_string())
        );
    }
}
