use super::ElmEmitter;
use crate::elm;
use crate::semantics::typed_ast::{TypedExpression, TypedNode};

pub fn emit_code_system_ref(
    name: &str,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
) -> elm::Expression {
    elm::Expression::CodeSystemRef(elm::CodeSystemRef {
        element: ctx.element_fields(node),
        name: Some(name.to_string()),
        library_name: None, // usually mapped if cross-library
    })
}

pub fn emit_value_set_ref(
    name: &str,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
) -> elm::Expression {
    elm::Expression::ValueSetRef(elm::ValueSetRef {
        element: ctx.element_fields(node),
        name: Some(name.to_string()),
        library_name: None,
        preserve: None,
    })
}

pub fn emit_code_ref(
    name: &str,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
) -> elm::Expression {
    elm::Expression::CodeRef(elm::CodeRef {
        element: ctx.element_fields(node),
        name: Some(name.to_string()),
        library_name: None,
    })
}

pub fn emit_concept_ref(
    name: &str,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
) -> elm::Expression {
    elm::Expression::ConceptRef(elm::ConceptRef {
        element: ctx.element_fields(node),
        name: Some(name.to_string()),
        library_name: None,
    })
}
