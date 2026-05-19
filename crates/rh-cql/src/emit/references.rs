//! Emit helpers for identifier, member, index, function-ref, and let
//! expressions — the "reference and access" expression family.

use crate::elm;
use crate::emit::ElmEmitter;
use crate::parser::ast::{IdentifierRef, QualifiedIdentifierRef};
use crate::semantics::typed_ast::{
    TypedExpression, TypedFunctionInvocation, TypedIndexInvocation, TypedMemberInvocation,
    TypedNode,
};

pub fn emit_identifier_ref(
    id_ref: &IdentifierRef,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
) -> elm::Expression {
    use crate::semantics::scope::SymbolKind;
    let element = ctx.element_fields(node);
    // Emit the correct ELM ref node based on the resolved symbol kind so that
    // evaluators (e.g. cql-execution) can look the name up in the right table
    // (library.codes vs library.expressions).
    match &node.meta.symbol_kind {
        Some(SymbolKind::Code) => elm::Expression::CodeRef(elm::CodeRef {
            element,
            name: Some(id_ref.name.clone()),
            library_name: None,
        }),
        Some(SymbolKind::Concept) => elm::Expression::ConceptRef(elm::ConceptRef {
            element,
            name: Some(id_ref.name.clone()),
            library_name: None,
        }),
        Some(SymbolKind::ValueSet) => elm::Expression::ValueSetRef(elm::ValueSetRef {
            element,
            name: Some(id_ref.name.clone()),
            library_name: None,
            preserve: None,
        }),
        Some(SymbolKind::CodeSystem) => elm::Expression::CodeSystemRef(elm::CodeSystemRef {
            element,
            name: Some(id_ref.name.clone()),
            library_name: None,
        }),
        _ => elm::Expression::ExpressionRef(elm::ExpressionRef {
            element,
            name: Some(id_ref.name.clone()),
            library_name: None,
        }),
    }
}

pub fn emit_qualified_identifier_ref(
    qid: &QualifiedIdentifierRef,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    elm::Expression::ExpressionRef(elm::ExpressionRef {
        element,
        name: Some(qid.name.clone()),
        library_name: Some(qid.qualifier.clone()),
    })
}

pub fn emit_function_invocation(
    fi: &TypedFunctionInvocation,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    // Try built-in system functions first; fall through to FunctionRef
    if let Some(expr) = crate::emit::operators::emit_system_function(
        &fi.function,
        &fi.arguments,
        node,
        ctx,
        &emit_expr,
    ) {
        return expr;
    }
    let element = ctx.element_fields(node);
    let operand = fi.arguments.iter().map(|a| emit_expr(a, ctx)).collect();
    elm::Expression::FunctionRef(elm::FunctionRef {
        element,
        name: Some(fi.function.clone()),
        library_name: None,
        operand,
        signature: Vec::new(),
    })
}

pub fn emit_member_invocation(
    mi: &TypedMemberInvocation,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    let source = Box::new(emit_expr(&mi.expression, ctx));
    elm::Expression::Property(elm::Property {
        element,
        path: Some(mi.member.clone()),
        source: Some(source),
        scope: None,
    })
}

pub fn emit_index_invocation(
    ii: &TypedIndexInvocation,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    let operand = vec![emit_expr(&ii.expression, ctx), emit_expr(&ii.index, ctx)];
    elm::Expression::Indexer(elm::BinaryExpression {
        element,
        operand,
        signature: Vec::new(),
    })
}

/// `let` clauses encountered outside a query context are emitted as alias refs.
pub fn emit_let_clause_standalone(
    name: &str,
    value: &TypedNode<TypedExpression>,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    let _ = emit_expr(value, ctx); // ensure value subtree is visited
    elm::Expression::AliasRef(elm::AliasRef {
        element,
        name: Some(name.to_string()),
    })
}
