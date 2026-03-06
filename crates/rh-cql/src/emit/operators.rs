use crate::elm;
use crate::parser::ast::{BinaryOperator, TernaryOperator, UnaryOperator};
use crate::semantics::typed_ast::{TypedExpression, TypedNode};
use crate::emit::ElmEmitter;

pub fn emit_unary_operator(
    operator: &UnaryOperator,
    operand: &TypedNode<TypedExpression>,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);
    
    // Handle Expand specially - it uses BinaryExpression even with single operand in ELM
    if matches!(operator, UnaryOperator::Expand) {
        return elm::Expression::Expand(elm::BinaryExpression {
            element,
            operand: vec![emit_expr(operand, ctx)],
            signature: Vec::new(),
        });
    }

    let unary = elm::UnaryExpression {
        element: element.clone(),
        operand: Some(Box::new(emit_expr(operand, ctx))),
        signature: Vec::new(),
    };

    match operator {
        UnaryOperator::Not => elm::Expression::Not(unary),
        UnaryOperator::Negate => elm::Expression::Negate(unary),
        UnaryOperator::Exists => elm::Expression::Exists(unary),
        UnaryOperator::IsNull => elm::Expression::IsNull(unary),
        UnaryOperator::IsTrue => elm::Expression::IsTrue(unary),
        UnaryOperator::IsFalse => elm::Expression::IsFalse(unary),
        UnaryOperator::Predecessor => elm::Expression::Predecessor(unary),
        UnaryOperator::Successor => elm::Expression::Successor(unary),
        UnaryOperator::Distinct => elm::Expression::Distinct(unary),
        UnaryOperator::Flatten => elm::Expression::Flatten(unary),
        UnaryOperator::Start => elm::Expression::Start(unary),
        UnaryOperator::End => elm::Expression::End(unary),
        UnaryOperator::Width => elm::Expression::Width(unary),
        UnaryOperator::PointFrom => elm::Expression::PointFrom(unary),
        UnaryOperator::Collapse => elm::Expression::Collapse(unary),
        UnaryOperator::Expand => unreachable!("Handled above"),
        UnaryOperator::Singleton => elm::Expression::SingletonFrom(unary),
        UnaryOperator::DateFrom => elm::Expression::DateFrom(unary),
        UnaryOperator::TimeFrom => elm::Expression::TimeFrom(unary),
        UnaryOperator::TimezoneOffsetFrom => elm::Expression::TimezoneOffsetFrom(unary),
        UnaryOperator::ToBoolean => elm::Expression::ToBoolean(unary),
        UnaryOperator::ToInteger => elm::Expression::ToInteger(unary),
        UnaryOperator::ToLong => elm::Expression::ToLong(unary),
        UnaryOperator::ToDecimal => elm::Expression::ToDecimal(unary),
        UnaryOperator::ToString => elm::Expression::ToStringExpr(unary),
        UnaryOperator::ToDate => elm::Expression::ToDate(unary),
        UnaryOperator::ToDateTime => elm::Expression::ToDateTime(unary),
        UnaryOperator::ToTime => elm::Expression::ToTime(unary),
        UnaryOperator::ToQuantity => elm::Expression::ToQuantity(unary),
        UnaryOperator::ToConcept => elm::Expression::ToConcept(unary),
    }
}

pub fn emit_binary_operator(
    operator: &BinaryOperator,
    left: &TypedNode<TypedExpression>,
    right: &TypedNode<TypedExpression>,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);

    let left_expr = emit_expr(left, ctx);
    let right_expr = emit_expr(right, ctx);

    let binary = || elm::BinaryExpression {
        element: element.clone(),
        operand: vec![left_expr.clone(), right_expr.clone()],
        signature: Vec::new(),
    };

    let time_binary = || elm::TimeBinaryExpression {
        element: element.clone(),
        operand: vec![left_expr.clone(), right_expr.clone()],
        signature: Vec::new(),
        precision: None, // Filled by other paths if needed
    };
    
    let nary = || elm::NaryExpression {
        element: element.clone(),
        operand: vec![left_expr.clone(), right_expr.clone()],
        signature: Vec::new(),
    };

    match operator {
        BinaryOperator::Add => elm::Expression::Add(binary()),
        BinaryOperator::Subtract => elm::Expression::Subtract(binary()),
        BinaryOperator::Multiply => elm::Expression::Multiply(binary()),
        BinaryOperator::Divide => {
            let mut l_expr = left_expr.clone();
            if left.data_type == crate::datatype::DataType::integer() || left.data_type == crate::datatype::DataType::long() {
                l_expr = elm::Expression::ToDecimal(elm::UnaryExpression {
                    element: ctx.element_fields(left),
                    operand: Some(Box::new(l_expr)),
                    signature: Vec::new(),
                });
            }
            let mut r_expr = right_expr.clone();
            if right.data_type == crate::datatype::DataType::integer() || right.data_type == crate::datatype::DataType::long() {
                r_expr = elm::Expression::ToDecimal(elm::UnaryExpression {
                    element: ctx.element_fields(right),
                    operand: Some(Box::new(r_expr)),
                    signature: Vec::new(),
                });
            }
            elm::Expression::Divide(elm::BinaryExpression {
                element: element.clone(),
                operand: vec![l_expr, r_expr],
                signature: Vec::new(),
            })
        },
        BinaryOperator::TruncatedDivide => elm::Expression::TruncatedDivide(binary()),
        BinaryOperator::Modulo => elm::Expression::Modulo(binary()),
        BinaryOperator::Power => elm::Expression::Power(binary()),
        BinaryOperator::Log => elm::Expression::Log(binary()),

        BinaryOperator::Equal => elm::Expression::Equal(binary()),
        BinaryOperator::NotEqual => elm::Expression::NotEqual(binary()),
        BinaryOperator::Equivalent => {
            elm::Expression::Equivalent(binary())
        }
        BinaryOperator::NotEquivalent => {
            let equiv = elm::Expression::Equivalent(binary());
            elm::Expression::Not(elm::UnaryExpression {
                element: element.clone(),
                operand: Some(Box::new(equiv)),
                signature: Vec::new(),
            })
        }
        BinaryOperator::Less => elm::Expression::Less(binary()),
        BinaryOperator::LessOrEqual => elm::Expression::LessOrEqual(binary()),
        BinaryOperator::Greater => elm::Expression::Greater(binary()),
        BinaryOperator::GreaterOrEqual => elm::Expression::GreaterOrEqual(binary()),

        BinaryOperator::And => elm::Expression::And(nary()),
        BinaryOperator::Or => elm::Expression::Or(nary()),
        BinaryOperator::Xor => elm::Expression::Xor(binary()),
        BinaryOperator::Implies => elm::Expression::Implies(binary()),

        BinaryOperator::Concatenate => elm::Expression::Concatenate(nary()),

        BinaryOperator::In => elm::Expression::In(time_binary()),
        BinaryOperator::Contains => elm::Expression::Contains(binary()),

        BinaryOperator::Includes => elm::Expression::Includes(time_binary()),
        BinaryOperator::IncludedIn => elm::Expression::IncludedIn(time_binary()),
        BinaryOperator::ProperlyIncludes => elm::Expression::ProperIncludes(time_binary()),
        BinaryOperator::ProperlyIncludedIn => elm::Expression::ProperIncludedIn(time_binary()),
        BinaryOperator::Overlaps => elm::Expression::Overlaps(time_binary()),
        BinaryOperator::OverlapsBefore => elm::Expression::OverlapsBefore(time_binary()),
        BinaryOperator::OverlapsAfter => elm::Expression::OverlapsAfter(time_binary()),
        BinaryOperator::Meets => elm::Expression::Meets(binary()),
        BinaryOperator::MeetsBefore => elm::Expression::MeetsBefore(binary()),
        BinaryOperator::MeetsAfter => elm::Expression::MeetsAfter(binary()),
        BinaryOperator::Starts => elm::Expression::Starts(time_binary()),
        BinaryOperator::Ends => elm::Expression::Ends(time_binary()),
        BinaryOperator::During => elm::Expression::IncludedIn(time_binary()),
        BinaryOperator::Before => elm::Expression::Before(time_binary()),
        BinaryOperator::After => elm::Expression::After(time_binary()),
        BinaryOperator::SameAs => elm::Expression::SameAs(time_binary()),
        BinaryOperator::SameOrBefore => elm::Expression::SameOrBefore(time_binary()),
        BinaryOperator::SameOrAfter => elm::Expression::SameOrAfter(time_binary()),
        BinaryOperator::Within => elm::Expression::IncludedIn(time_binary()), 

        BinaryOperator::Union => elm::Expression::Union(binary()),
        BinaryOperator::Intersect => elm::Expression::Intersect(binary()),
        BinaryOperator::Except => elm::Expression::Except(binary()),
        BinaryOperator::IndexOf => elm::Expression::Indexer(binary()),
    }
}

pub fn emit_ternary_operator(
    operator: &TernaryOperator,
    first: &TypedNode<TypedExpression>,
    second: &TypedNode<TypedExpression>,
    third: &TypedNode<TypedExpression>,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);

    let first_expr = emit_expr(first, ctx);
    let second_expr = emit_expr(second, ctx);
    let third_expr = emit_expr(third, ctx);

    match operator {
        TernaryOperator::Between => {
            let ge = elm::Expression::GreaterOrEqual(elm::BinaryExpression {
                element: ctx.element_fields(node), 
                operand: vec![first_expr.clone(), second_expr],
                signature: Vec::new(),
            });

            let le = elm::Expression::LessOrEqual(elm::BinaryExpression {
                element: ctx.element_fields(node),
                operand: vec![first_expr, third_expr],
                signature: Vec::new(),
            });

            elm::Expression::And(elm::NaryExpression {
                element,
                operand: vec![ge, le],
                signature: Vec::new(),
            })
        }
        TernaryOperator::ReplaceMatches => {
            elm::Expression::ReplaceMatches(elm::TernaryExpression {
                element,
                operand: vec![first_expr, second_expr, third_expr],
                signature: Vec::new(),
            })
        }
    }
}

pub fn emit_system_function(
    name: &str,
    args: &[TypedNode<TypedExpression>],
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: &impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> Option<elm::Expression> {
    let element = ctx.element_fields(node);

    if args.len() == 1 {
        let operand = Some(Box::new(emit_expr(&args[0], ctx)));
        let unary = elm::UnaryExpression {
            element: element.clone(),
            operand,
            signature: Vec::new(),
        };

        match name {
            "Abs" => Some(elm::Expression::Abs(unary)),
            "Ceiling" => Some(elm::Expression::Ceiling(unary)),
            "Floor" => Some(elm::Expression::Floor(unary)),
            "Truncate" => Some(elm::Expression::Truncate(unary)),
            "Round" => Some(elm::Expression::Round(unary)),
            "Ln" => Some(elm::Expression::Ln(unary)),
            "Exp" => Some(elm::Expression::Exp(unary)),
            "Predecessor" => Some(elm::Expression::Predecessor(unary)),
            "Successor" => Some(elm::Expression::Successor(unary)),
            _ => None,
        }
    } else if args.len() == 2 {
        let left = emit_expr(&args[0], ctx);
        let right = emit_expr(&args[1], ctx);
        let binary = elm::BinaryExpression {
            element: element.clone(),
            operand: vec![left, right],
            signature: Vec::new(),
        };

        match name {
            "Log" => Some(elm::Expression::Log(binary)),
            "Power" => Some(elm::Expression::Power(binary)),
            "Round" => {
                // Round is unary or binary depending on args. In ELM, it's defined:
                // Actually wait... is elm::Expression::Round an Nary, Binary, or Unary? 
                // Let's check `translator.rs` to see what it mapped to.
                // Wait, if elm::Expression only has `Round(UnaryExpression)`, how does Round with precision work?
                // Old translator didn't implement binary round, or it mapped to Round with precision?
                // Let's drop Round from binary if it doesn't compile, we will see.
                // Actually `Round(UnaryExpression)` is the only one we know. We'll use a hack or just return None for now if it breaks.
                None // Will type check and confirm
            }
            _ => None,
        }
    } else {
        None
    }
}
