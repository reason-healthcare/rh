use crate::elm;
use crate::parser::ast::Literal;
use crate::semantics::typed_ast::{TypedExpression, TypedNode};
use crate::emit::ElmEmitter;

pub fn emit_literal(
    lit: &Literal,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
) -> elm::Expression {
    match lit {
        Literal::Null => elm::Expression::Null(elm::Null {
            element: ctx.element_fields(node),
            value_type: None,
        }),
        Literal::Boolean(b) => elm::Expression::Literal(elm::Literal {
            element: ctx.element_fields(node),
            value_type: Some("{urn:hl7-org:elm-types:r1}Boolean".to_string()),
            value: Some(b.to_string()),
        }),
        Literal::Integer(i) => emit_numeric_literal(*i, "Integer", ctx, node, |v| v.to_string()),
        Literal::Long(l) => emit_numeric_literal(*l, "Long", ctx, node, |v| format!("{}L", v)),
        Literal::Decimal(d) => emit_numeric_literal(*d, "Decimal", ctx, node, |v| format_decimal(*v)),
        Literal::String(s) => elm::Expression::Literal(elm::Literal {
            element: ctx.element_fields(node),
            value_type: Some("{urn:hl7-org:elm-types:r1}String".to_string()),
            value: Some(s.clone()),
        }),
        Literal::Date(d) => elm::Expression::Literal(elm::Literal {
            element: ctx.element_fields(node),
            value_type: Some("{urn:hl7-org:elm-types:r1}Date".to_string()),
            value: Some(d.clone()),
        }),
        Literal::DateTime(dt) => elm::Expression::Literal(elm::Literal {
            element: ctx.element_fields(node),
            value_type: Some("{urn:hl7-org:elm-types:r1}DateTime".to_string()),
            value: Some(dt.clone()),
        }),
        Literal::Time(t) => elm::Expression::Literal(elm::Literal {
            element: ctx.element_fields(node),
            value_type: Some("{urn:hl7-org:elm-types:r1}Time".to_string()),
            value: Some(t.clone()),
        }),
        Literal::Quantity { value, unit } => emit_quantity(*value, unit, ctx, node),
        Literal::Ratio { numerator, denominator } => emit_ratio(numerator, denominator, ctx, node),
        Literal::Code { code, system, display } => elm::Expression::Code(elm::CodeExpr {
            element: ctx.element_fields(node),
            code: Some(code.clone()),
            system: system.as_ref().map(|s| elm::CodeSystemRef {
                element: elm::ElementFields::default(),
                name: Some(s.clone()),
                library_name: None,
            }),
            display: display.clone(),
        }),
    }
}

fn format_decimal(d: f64) -> String {
    let s = format!("{d}");
    if s.contains('.') {
        s
    } else {
        format!("{s}.0")
    }
}

trait NumericLiteral: Copy + PartialOrd + std::ops::Neg<Output = Self> + std::fmt::Display {
    fn is_negative(&self) -> bool;
    fn zero() -> Self;
}

impl NumericLiteral for i64 {
    fn is_negative(&self) -> bool { *self < 0 }
    fn zero() -> Self { 0 }
}

impl NumericLiteral for f64 {
    fn is_negative(&self) -> bool { *self < 0.0 }
    fn zero() -> Self { 0.0 }
}

fn emit_numeric_literal<T: NumericLiteral>(
    value: T,
    type_name: &str,
    ctx: &mut ElmEmitter,
    node: &TypedNode<TypedExpression>,
    formatter: impl Fn(&T) -> String,
) -> elm::Expression {
    let qname = format!("{{urn:hl7-org:elm-types:r1}}{type_name}");
    
    if value.is_negative() {
        let abs_val = -value;
        let lit_expr = elm::Expression::Literal(elm::Literal {
            element: elm::ElementFields::default(),
            value_type: Some(qname),
            value: Some(formatter(&abs_val)),
        });
        
        elm::Expression::Negate(elm::UnaryExpression {
            element: ctx.element_fields(node),
            operand: Some(Box::new(lit_expr)),
            signature: vec![],
        })
    } else {
        elm::Expression::Literal(elm::Literal {
            element: ctx.element_fields(node),
            value_type: Some(qname),
            value: Some(formatter(&value)),
        })
    }
}

fn emit_quantity(value: f64, unit: &str, ctx: &mut ElmEmitter, node: &TypedNode<TypedExpression>) -> elm::Expression {
    if value < 0.0 {
        let abs_val = -value;
        let q_expr = elm::Expression::Quantity(elm::QuantityExpr {
            element: elm::ElementFields::default(),
            value: Some(abs_val),
            unit: Some(unit.to_string()),
        });
        
        elm::Expression::Negate(elm::UnaryExpression {
            element: ctx.element_fields(node),
            operand: Some(Box::new(q_expr)),
            signature: vec![],
        })
    } else {
        elm::Expression::Quantity(elm::QuantityExpr {
            element: ctx.element_fields(node),
            value: Some(value),
            unit: Some(unit.to_string()),
        })
    }
}

fn emit_ratio(
    numerator: &Literal,
    denominator: &Literal,
    ctx: &mut ElmEmitter,
    node: &TypedNode<TypedExpression>,
) -> elm::Expression {
    let dummy_node = TypedNode {
        node_id: node.node_id,
        span: node.span.clone(),
        meta: node.meta.clone(),
        data_type: node.data_type.clone(),
        inner: TypedExpression::Literal(Literal::Null),
    };
    
    let num_expr = emit_literal(numerator, &dummy_node, ctx);
    let den_expr = emit_literal(denominator, &dummy_node, ctx);
    
    elm::Expression::Ratio(elm::RatioExpr {
        element: ctx.element_fields(node),
        numerator: Some(Box::new(num_expr)),
        denominator: Some(Box::new(den_expr)),
    })
}
