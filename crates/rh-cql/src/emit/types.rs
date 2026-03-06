use super::ElmEmitter;
use crate::elm;
use crate::parser::ast;
use crate::semantics::typed_ast::{
    TypedNode, TypedExpression, TypedTypeExpression, TypedIntervalExpression, TypedTupleElement, TypedInstanceElement, TypedInstance
};

pub fn emit_type_specifier(ts: &ast::TypeSpecifier) -> elm::TypeSpecifier {
    match ts {
        ast::TypeSpecifier::Named(named) => {
            let ns = named.namespace.as_deref().unwrap_or("urn:hl7-org:elm-types:r1");
            let qname = format!("{{{}}}{}", ns, named.name);
            elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
                local_id: None,
                locator: None,
                name: qname,
            })
        }
        ast::TypeSpecifier::List(list) => {
            elm::TypeSpecifier::List(elm::ListTypeSpecifier {
                local_id: None,
                locator: None,
                element_type: Some(Box::new(emit_type_specifier(&list.element_type))),
            })
        }
        ast::TypeSpecifier::Interval(interval) => {
            elm::TypeSpecifier::Interval(elm::IntervalTypeSpecifier {
                local_id: None,
                locator: None,
                point_type: Some(Box::new(emit_type_specifier(&interval.point_type))),
            })
        }
        ast::TypeSpecifier::Tuple(tuple) => {
            let elements = tuple.elements.iter().map(|e| {
                elm::TupleElementDefinition {
                    name: e.name.clone(),
                    element_type: None,
                    type_specifier: Some(Box::new(emit_type_specifier(&e.element_type))),
                }
            }).collect();
            elm::TypeSpecifier::Tuple(elm::TupleTypeSpecifier {
                local_id: None,
                locator: None,
                element: elements,
            })
        }
        ast::TypeSpecifier::Choice(choice) => {
            let choices = choice.types.iter().map(|c| emit_type_specifier(c)).collect();
            elm::TypeSpecifier::Choice(elm::ChoiceTypeSpecifier {
                local_id: None,
                locator: None,
                choice: choices,
            })
        }
    }
}

pub fn type_specifier_to_qname(ts: &ast::TypeSpecifier) -> Result<elm::QName, String> {
    match ts {
        ast::TypeSpecifier::Named(named) => {
            let ns = named.namespace.as_deref().unwrap_or("urn:hl7-org:elm-types:r1");
            Ok(format!("{{{}}}{}", ns, named.name))
        }
        ast::TypeSpecifier::List(list) => {
            let elem = type_specifier_to_qname(&list.element_type)?;
            Ok(format!("{{urn:hl7-org:elm-types:r1}}List<{}>", elem))
        }
        ast::TypeSpecifier::Interval(interval) => {
            let point = type_specifier_to_qname(&interval.point_type)?;
            Ok(format!("{{urn:hl7-org:elm-types:r1}}Interval<{}>", point))
        }
        _ => Err("Could not convert complex type specifier to QName".to_string()),
    }
}

pub fn emit_type_expression(
    type_expr: &TypedTypeExpression,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    let operand_elm = Box::new(emit_expr(&type_expr.operand, ctx));
    let qname = type_specifier_to_qname(&type_expr.type_specifier).ok();
    
    // In strict ELM, we try to use isType / asType / toType QNames first, 
    // and fallback to asTypeSpecifier if necessary. For simplicity, we just use the specifier directly.
    let ts = emit_type_specifier(&type_expr.type_specifier);
    
    match type_expr.operator {
        ast::TypeOperator::Is => {
            elm::Expression::Is(elm::IsExpr {
                element,
                operand: Some(operand_elm),
                is_type: qname,
                is_type_specifier: Some(ts),
            })
        }
        ast::TypeOperator::As => {
            elm::Expression::As(elm::AsExpr {
                element,
                operand: Some(operand_elm),
                as_type: qname,
                as_type_specifier: Some(ts),
                strict: Some(false),
            })
        }
        ast::TypeOperator::Cast => {
            elm::Expression::As(elm::AsExpr {
                element,
                operand: Some(operand_elm),
                as_type: qname.clone(),
                as_type_specifier: Some(ts.clone()),
                strict: Some(true),
            })
        }
        ast::TypeOperator::Convert => {
            elm::Expression::Convert(elm::ConvertExpr {
                element,
                operand: Some(operand_elm),
                to_type: qname,
                to_type_specifier: Some(ts),
            })
        }
    }
}

pub fn emit_interval_expression(
    interval: &TypedIntervalExpression,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    
    let low = interval.low.as_ref().map(|l| Box::new(emit_expr(l, ctx)));
    let high = interval.high.as_ref().map(|h| Box::new(emit_expr(h, ctx)));
    
    elm::Expression::Interval(elm::IntervalExpr {
        element,
        low,
        high,
        low_closed_expression: None,
        high_closed_expression: None,
        low_closed: Some(interval.low_closed),
        high_closed: Some(interval.high_closed),
    })
}

pub fn emit_list_expression(
    elements: &Vec<TypedNode<TypedExpression>>,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    
    let elms = elements.iter().map(|e| emit_expr(e, ctx)).collect();
    
    elm::Expression::List(elm::ListExpr {
        element,
        type_specifier: None, // Infer by runtime or add if available from TypedAST data_type
        elements: elms,
    })
}

pub fn emit_tuple_expression(
    elements: &Vec<TypedTupleElement>,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    
    let tuple_elements = elements.iter().map(|e| elm::TupleElement {
        name: Some(e.name.clone()),
        value: Some(Box::new(emit_expr(&e.value, ctx))),
    }).collect();
    
    elm::Expression::Tuple(elm::TupleExpr {
        element,
        elements: tuple_elements,
    })
}

pub fn emit_instance(
    instance: &TypedInstance,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    
    let class_type = type_specifier_to_qname(&instance.class_type).ok();
    
    let elms = instance.elements.iter().map(|e| elm::InstanceElement {
        name: Some(e.name.clone()),
        value: Some(Box::new(emit_expr(&e.value, ctx))),
    }).collect();
    
    elm::Expression::Instance(elm::Instance {
        element,
        class_type,
        elements: elms,
    })
}
