//! Calendar duration literals use unquoted singular or plural units after a magnitude.

use rh_fhirpath::ast::{Expression, Literal, Term};
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use rust_decimal::Decimal;
use serde_json::json;

#[test]
fn calendar_units_bind_to_the_quantity_magnitude() {
    let parser = FhirPathParser::new();
    for unit in [
        "year",
        "month",
        "week",
        "day",
        "hour",
        "minute",
        "second",
        "millisecond",
    ] {
        for suffix in [unit.to_owned(), format!("{unit}s")] {
            let expression = format!("1.25 {suffix}");
            assert_eq!(
                parser.parse(&expression).unwrap().root,
                Expression::Term(Term::Literal(Literal::Quantity {
                    value: Decimal::new(125, 2),
                    unit: Some(unit.to_owned()),
                })),
                "{expression}",
            );
        }
    }
}

#[test]
fn equal_calendar_quantities_deduplicate() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));
    let expression = parser.parse("(1 day | 1 days | 1 month).count()").unwrap();
    assert_eq!(
        evaluator.evaluate(&expression, &context).unwrap(),
        FhirPathValue::Integer(2)
    );
}
