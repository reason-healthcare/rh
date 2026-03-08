use crate::elm;
use crate::semantics::typed_ast::{TypedCase, TypedCaseItem, TypedExpression, TypedNode};

use super::ElmEmitter;

impl ElmEmitter {
    /// Emit an If-Then-Else expression.
    pub fn emit_if(
        &mut self,
        condition: &TypedNode<TypedExpression>,
        then_expr: &TypedNode<TypedExpression>,
        else_expr: &TypedNode<TypedExpression>,
        outer_node: &TypedNode<TypedExpression>,
    ) -> elm::Expression {
        let element = self.element_fields(outer_node);
        let cond = Box::new(self.emit_expression(condition));
        let then = Box::new(self.emit_expression(then_expr));
        let els = Box::new(self.emit_expression(else_expr));
        elm::Expression::If(elm::IfExpr {
            element,
            condition: Some(cond),
            then_expr: Some(then),
            else_expr: Some(els),
        })
    }

    /// Emit a Case expression (with or without comparand).
    pub fn emit_case(
        &mut self,
        typed_case: &TypedCase,
        outer_node: &TypedNode<TypedExpression>,
    ) -> elm::Expression {
        let element = self.element_fields(outer_node);

        let comparand = typed_case
            .comparand
            .as_ref()
            .map(|c| Box::new(self.emit_expression(c)));

        let case_items = typed_case
            .case_items
            .iter()
            .map(|item| self.emit_case_item(item))
            .collect();

        let else_expr = Some(Box::new(self.emit_expression(&typed_case.else_expr)));

        elm::Expression::Case(elm::Case {
            element,
            comparand,
            case_item: case_items,
            else_expr,
        })
    }

    fn emit_case_item(&mut self, item: &TypedCaseItem) -> elm::CaseItem {
        elm::CaseItem {
            when_expr: Some(Box::new(self.emit_expression(&item.when))),
            then_expr: Some(Box::new(self.emit_expression(&item.then))),
        }
    }
}
