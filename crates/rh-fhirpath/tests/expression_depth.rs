use rh_fhirpath::{
    ast::{Expression, FhirPathExpression, Invocation, Literal, Term},
    EvaluationContext, FhirPathError, FhirPathEvaluator, FhirPathParser, FhirPathValue,
};
use serde_json::Value;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn guards_delimiters_before_recursive_parsing() {
    let parser = FhirPathParser::new();
    let boundary = format!("{}1{}", "(".repeat(12), ")".repeat(12));
    assert!(parser.parse(&boundary).is_ok());
    let expression = format!("({boundary})");
    assert!(
        matches!(parser.parse(&expression), Err(FhirPathError::SyntaxError { message, .. }) if message.contains("nesting"))
    );
    let literal = "'((((((((((((((((((((((((((((((((((('";
    assert!(parser.parse(literal).is_ok());
    assert!(parser.parse(r#"`x\`((((((((((((((((((((((`"#).is_ok());
    assert!(parser.parse("1 /* ((((((((((((((((((( */ + 2").is_ok());

    let ordinary = parser.parse("iif((1 + 1) = 2, (4 + 5), 0)").unwrap();
    let result = FhirPathEvaluator::new()
        .evaluate(&ordinary, &EvaluationContext::new(Value::Null))
        .unwrap();
    assert_eq!(result, FhirPathValue::Integer(9));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn guards_ast_depth_across_precedence_levels() {
    let parser = FhirPathParser::new();
    let boundary = ["1"; 64].join(" + ");
    assert!(parser.parse(&boundary).is_ok());
    for expression in [
        format!("{boundary} + 1"),
        format!("({boundary})"),
        format!("{}1", "-".repeat(64)),
        format!("1.{}", ["toString()"; 64].join(".")),
    ] {
        assert!(
            matches!(parser.parse(&expression), Err(FhirPathError::SyntaxError { message, .. }) if message.contains("64 levels")),
            "{expression}"
        );
    }
    let wide = format!("example({})", ["1"; 80].join(","));
    assert!(parser.parse(&wide).is_ok());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rejects_over_depth_caller_supplied_parameter() {
    let root = (0..64).fold(
        Expression::Term(Term::Literal(Literal::Integer(1))),
        |child, _| {
            Expression::Term(Term::Invocation(Invocation::Function {
                name: "abs".into(),
                parameters: vec![child],
            }))
        },
    );
    let result = FhirPathEvaluator::new().evaluate(
        &FhirPathExpression { root },
        &EvaluationContext::new(Value::Null),
    );
    assert!(
        matches!(result, Err(FhirPathError::SyntaxError { message, .. }) if message.contains("64 levels"))
    );
}
