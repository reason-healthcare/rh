//! Regression tests for FHIRPath STU3 evaluation and collection semantics.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::{json, Value};

fn evaluate(expression: &str, context: &EvaluationContext) -> FhirPathValue {
    FhirPathEvaluator::new()
        .evaluate(&FhirPathParser::new().parse(expression).unwrap(), context)
        .unwrap_or_else(|error| panic!("{expression}: {error}"))
}

fn assert_result(expression: &str, context: &EvaluationContext, expected: Value) {
    assert_eq!(
        evaluate(expression, context).to_json(),
        expected,
        "{expression}"
    );
}

fn assert_error(expression: &str, context: &EvaluationContext, message: &str) {
    let parsed = FhirPathParser::new().parse(expression).unwrap();
    let error = FhirPathEvaluator::new()
        .evaluate(&parsed, context)
        .expect_err(expression)
        .to_string();
    assert!(error.contains(message), "{expression}: {error}");
}

#[test]
fn scoped_focus_keeps_initial_context_and_resets_index() {
    let context = EvaluationContext::new(json!({
        "resourceType": "Patient",
        "id": "root",
        "name": [{"family": "First"}, {"family": "Second"}]
    }));
    assert_result("$index", &context, json!(0));
    assert_result("(1 | 2).select(1.select($index))", &context, json!([0, 0]));
    assert_result(
        "(1 | 2).select(1.where($index = 0))",
        &context,
        json!([1, 1]),
    );
    assert_result(
        "(1 | 2).select(1.iif(true, $index))",
        &context,
        json!([0, 1]),
    );
    assert_result(
        "name.select(%context.id)",
        &context,
        json!(["root", "root"]),
    );

    let focus = context.with_current(json!({"code": "start", "numbers": [1, 2]}));
    assert_result(
        "numbers.select(%context.code)",
        &focus,
        json!(["start", "start"]),
    );
    assert_result("%rootResource.id", &focus, json!("root"));
}

#[test]
fn each_evaluation_prepares_focus_from_the_current_input() {
    let mut context = EvaluationContext::new(json!({
        "resourceType": "Patient",
        "id": "root",
        "name": [{"family": "First"}]
    }));
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let parsed = parser
        .parse("name.select(family & '|' & $this.family & '|' & %context.id & '|' & %resource.id)")
        .unwrap();
    for (current, expected) in [
        (context.root.clone(), "First|First|root|root"),
        (
            json!({"resourceType": "Patient", "id": "other", "name": [{"family": "Second"}]}),
            "Second|Second|other|other",
        ),
    ] {
        context.current = current;
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        assert_eq!(result.to_json(), json!(expected));
    }
    assert_result("%rootResource.id", &context, json!("root"));
}

#[test]
fn lexical_variables_follow_the_receiver_chain() {
    let context = EvaluationContext::new(json!({ "items": [{"value": 2}, {"value": 3}] }));
    assert_result(
        "items.defineVariable('first', first().value).select(%first + value)",
        &context,
        json!([4, 5]),
    );
    assert_result(
        "items.select(defineVariable('v', value).select(%v + $index))",
        &context,
        json!([2, 3]),
    );
    assert_result(
        "(1.defineVariable('v').select(%v) | 2.defineVariable('v').select(%v))",
        &context,
        json!([1, 2]),
    );
    assert_result(
        "items.defineVariable('v', first().value)[0].select(%v)",
        &context,
        json!(2),
    );
    assert_result(
        "(1.defineVariable('v') as Integer).select(%v)",
        &context,
        json!(1),
    );
    for expression in [
        "1.defineVariable('v').defineVariable('v')",
        "1.defineVariable('context')",
    ] {
        assert_error(expression, &context, "already defined");
    }
    for expression in [
        "%missing",
        "1.defineVariable('v') | %v",
        "1.select(defineVariable('v')).select(%v)",
    ] {
        assert_error(expression, &context, "Undefined environment variable");
    }
}

#[test]
fn singleton_boolean_consumers_check_cardinality() {
    let context = EvaluationContext::new(json!({}));
    for expression in [
        "1.where(true | false)",
        "1.all(true | false)",
        "1.exists(true | false)",
        "iif(true | false, 1, 2)",
        "(true | false) and true",
        "(true | false).not()",
        "1.combine(2, true | false)",
        "1[(0 | 1)]",
    ] {
        assert_error(expression, &context, "at most one item");
    }
    assert_result("iif(0, 'yes', 'no')", &context, json!("yes"));
    assert_result("1.where({})", &context, Value::Null);
    assert_result("false and {}", &context, json!(false));
    assert_result("false or {}", &context, Value::Null);
    assert_result(
        "1.combine(1 | 2, 'yes'.exists())",
        &context,
        json!([1, 1, 2]),
    );
    assert_result("1['0'.split(',').toInteger()]", &context, json!(1));
    for function in ["allTrue", "allFalse", "anyTrue", "anyFalse"] {
        assert_error(
            &format!("(true | 'text').{function}()"),
            &context,
            "requires Boolean items",
        );
    }
}

#[test]
fn comparison_operators_and_collections_share_equality() {
    let context = EvaluationContext::new(json!({}));
    for expression in [
        "'a'.split(',') in 'a'",
        "'a' contains 'a'.split(',')",
        "23 ~ 23 '1'",
        "1 'm'.comparable(1 'cm')",
        "'a  b' ~ 'A\t\tB'",
        "'Straße' ~ 'STRASSE'",
        "(@2024-01-01T23:00:00Z | @2024-01-02T01:00:00+02:00).count() = 1",
        "@2024-01-01T23:00:00Z.subsetOf(@2024-01-02T01:00:00+02:00)",
        "@2020.combine(@2020-01-01).isDistinct()",
        "@2020.combine(@2020-01-01).distinct().count() = 2",
    ] {
        assert_result(expression, &context, json!(true));
    }
    for expression in [
        "'a  b' ~ 'a b'",
        "'a ' ~ 'a'",
        "1 year.comparable(1 'a')",
        "(@2012-01 | 1) = (@2012 | 2)",
        "@2012-01 in @2012",
    ] {
        assert_result(expression, &context, json!(false));
    }
    for expression in [
        "(@2012-01 | 1) = (@2012 | 1)",
        "(@2012-01 | 1) != (@2012 | 1)",
        "1 'cm' < 1 's'",
    ] {
        assert_result(expression, &context, Value::Null);
    }
}

#[test]
fn comparison_preserves_units_from_fhir_quantity_nodes() {
    let context = EvaluationContext::new(json!({
        "resourceType": "Observation",
        "component": [
            {"valueQuantity": {"value": 2, "unit": "cups"}},
            {"valueQuantity": {"value": 3, "unit": "cups"}},
            {"valueQuantity": {"value": 2, "unit": "containers"}}
        ]
    }));
    assert_result(
        "component[0].value < component[1].value",
        &context,
        json!(true),
    );
    assert_result(
        "component[0].value.comparable(component[1].value)",
        &context,
        json!(true),
    );
    assert_result(
        "component[0].value = component[2].value",
        &context,
        Value::Null,
    );
    assert_result(
        "component[0].value.comparable(component[2].value)",
        &context,
        json!(false),
    );
}

#[test]
fn equality_recurses_through_typed_fhir_children() {
    let context = EvaluationContext::new(json!({
        "resourceType": "Patient",
        "name": [
            {"period": {"start": "2024-01-01T23:00:00Z"}},
            {"period": {"start": "2024-01-02T01:00:00+02:00"}},
            {"period": {"start": "2024-01-01"}}
        ]
    }));
    assert_result("name[0].period = name[1].period", &context, json!(true));
    assert_result("name[0].period = name[2].period", &context, Value::Null);
    assert_result("name[0].period ~ name[2].period", &context, json!(false));
    assert_result("name.period.distinct().count()", &context, json!(2));
}

#[test]
fn subsetting_handles_empty_arguments_and_optional_order_checks() {
    let mut context = EvaluationContext::new(json!({
        "resourceType": "Patient",
        "id": "example",
        "active": true
    }));
    assert_result("(1 | 2).skip({})", &context, Value::Null);
    assert_result("(1 | 2).take({})", &context, Value::Null);
    assert_result(
        "(1 | 2).skip('1'.split(',').toInteger())",
        &context,
        json!(2),
    );
    assert_error("(1 | 2).take(1 | 2)", &context, "at most one item");
    context.check_ordered_functions = true;
    assert_error(
        "Patient.children().skip(1)",
        &context,
        "unordered collection",
    );
}

#[test]
fn coalesce_is_lazy_and_repeat_all_keeps_duplicates() {
    let context = EvaluationContext::new(json!({
        "tree": {"item": [{"value": 1}, {"value": 1}]},
        "object": {"values": [7, 8]}
    }));
    assert_result(
        "object.coalesce(missing, values, %unused)",
        &context,
        json!([7, 8]),
    );
    assert_result("coalesce({}, false, %unused)", &context, json!(false));
    assert_result("coalesce({}, {})", &context, Value::Null);
    assert_error("coalesce()", &context, "at least one argument");
    assert_result("1.repeat($this)", &context, json!(1));
    assert_result("tree.repeat(item).value", &context, json!(1));
    assert_result("tree.repeatAll(item).value", &context, json!([1, 1]));
    assert_error("1.repeatAll($this)", &context, "recursive projection limit");
}

#[test]
fn repeat_compares_numeric_and_quantity_values_in_either_order() {
    let context = EvaluationContext::new(json!({}));
    for expression in [
        "1.combine(1 '1').combine(2.00).combine(2L).combine(100 'cm').combine(1 'm')",
        "1 '1'.combine(1).combine(2L).combine(2.00).combine(1 'm').combine(100 'cm')",
    ] {
        assert_result(
            &format!("({expression}).repeat($this).count()"),
            &context,
            json!(3),
        );
    }
}

#[test]
fn sort_only_evaluates_later_selectors_to_break_ties() {
    let context = EvaluationContext::new(json!({
        "items": [
            {"id": "first", "key": 2, "rank": 1},
            {"id": "empty"},
            {"id": "second", "key": 2, "rank": 2},
            {"id": "third", "key": 1, "rank": 2}
        ]
    }));
    assert_result("(1 | 2).sort($this desc, %unused)", &context, json!([2, 1]));
    assert_result(
        "items.sort(key desc, rank desc).id",
        &context,
        json!(["second", "first", "third", "empty"]),
    );
    assert_result(
        "items.sort(key asc).id",
        &context,
        json!(["empty", "third", "first", "second"]),
    );
    assert_error(
        "items.sort(key, %used)",
        &context,
        "Undefined environment variable",
    );
    assert_error("1.sort(1 | 2)", &context, "at most one item");
    assert_error("(1 | 's').sort()", &context, "Cannot compare");
}
