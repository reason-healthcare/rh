use crate::elm;
use crate::semantics::typed_ast::{TypedExpression, TypedNode};
use crate::options::CompilerOptions;

pub fn emit_clinical(
    expr: &TypedNode<TypedExpression>,
    ctx: &mut super::ElmEmitter
) -> Option<elm::Expression> {
    // This looks like it needs to map typed clinical expressions
    // Let's first check what typed expressions map to clinical
    None
}
