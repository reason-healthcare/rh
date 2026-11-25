use rh_fhirpath::{
    evaluator::metadata::FhirModel, extensions::fhir::host::FhirHost, EvaluationContext,
    FhirPathEvaluator, FhirPathParser, FhirPathResult, FhirPathValue as V,
};
use serde_json::{json, Value};
use std::sync::Arc;

fn eval(expression: &str, context: &EvaluationContext) -> FhirPathResult<V> {
    FhirPathEvaluator::new().evaluate(&FhirPathParser::new().parse(expression)?, context)
}

fn eval_raw(expression: &str, context: &EvaluationContext) -> Box<serde_json::value::RawValue> {
    let parsed = FhirPathParser::new().parse(expression).unwrap();
    FhirPathEvaluator::new()
        .evaluate_nodes(&parsed, context)
        .unwrap()
        .to_raw_json()
        .unwrap()
}

#[test]
fn aligned_primitives_choices_paths_and_inheritance() {
    for model in [FhirModel::R4, FhirModel::R5] {
        let mut c = EvaluationContext::new(
            json!({"resourceType":"Patient","_birthDate":{"extension":[{"url":"u","valueString":"missing"}]},"name":[{"given":[null,"Ana","Ana"],"_given":[{"id":"absent"},null,{"id":"last"}]}],"deceasedBoolean":false}),
        );
        c.model = model;
        c.strict_mode = true;
        for (expression, result) in [
            ("birthDate.exists()", json!(true)),
            (
                "birthDate.hasValue()",
                if model == FhirModel::R4 {
                    json!(false)
                } else {
                    Value::Null
                },
            ),
            ("birthDate.getValue().empty()", json!(true)),
            ("birthDate.extension('u').value", json!("missing")),
            ("name.given.count()", json!(3)),
            (
                "name.given.select(hasValue())",
                if model == FhirModel::R4 {
                    json!([false, true, true])
                } else {
                    json!([true, true])
                },
            ),
            (
                "name.given.where(id='last').pathname()",
                json!("Patient.name[0].given[2]"),
            ),
            ("name.given[0].id", json!("absent")),
            ("name.given[1].upper().pathname().empty()", json!(true)),
            (
                "name.given[2].value.pathname()",
                json!("Patient.name[0].given[2].value[0]"),
            ),
            (
                "name.given[2].children().ofType(System.String).first().pathname(true)",
                json!("Patient.name[0].given[2].value"),
            ),
            ("name.given[0].value.pathname().empty()", json!(true)),
            ("deceased.value.type().namespace", json!("System")),
            ("deceased.value.type().name", json!("Boolean")),
            ("deceased.value is FHIR.boolean", json!(false)),
            ("deceased.value is System.Boolean", json!(true)),
            ("deceased.value = false", json!(true)),
            (
                "deceased.value.ofType(System.Boolean).pathname()",
                json!("Patient.deceased[0].value[0]"),
            ),
            ("deceased.type().name", json!("boolean")),
            ("name.type() is System.ClassInfo", json!(true)),
            ("1.type() is SimpleTypeInfo", json!(true)),
            ("1.type().ofType(TypeInfo).exists()", json!(true)),
            ("deceased.pathname(true)", json!("Patient.deceased")),
            ("Patient is DomainResource", json!(true)),
            ("Patient.ofType(Resource).exists()", json!(true)),
            ("name is HumanName", json!(true)),
            (
                "name.given.type().name",
                json!(["string", "string", "string"]),
            ),
            ("birthDate.children().ofType(Extension).count()", json!(1)),
        ] {
            assert_eq!(
                eval(expression, &c).unwrap().to_json(),
                result,
                "{model:?}: {expression}"
            );
        }
        assert!(eval("deceasedBoolean", &c).is_err());
        assert!(eval("1 is System.MissingType", &c).is_err());
    }
}

#[test]
fn release_specific_field_and_long() {
    let mut c = EvaluationContext::new(
        json!({"resourceType":"Parameters","parameter":[{"name":"large","valueInteger64":"9223372036854775807"}]}),
    );
    assert_eq!(eval("parameter.value", &c).unwrap(), V::Empty);
    c.model = FhirModel::R5;
    assert_eq!(
        eval("parameter.value.getValue()", &c).unwrap(),
        V::Long(i64::MAX)
    );
    assert_eq!(
        eval("parameter.value is FHIR.integer64", &c).unwrap(),
        V::Boolean(true)
    );
    assert_eq!(
        eval("parameter.value.getValue() is System.Long", &c).unwrap(),
        V::Boolean(true)
    );
}

#[test]
fn local_resolution_preserves_origin_and_starting_resource() {
    let c = EvaluationContext::new(
        json!({"resourceType":"Bundle","entry":[{"fullUrl":"urn:uuid:obs","resource":{"resourceType":"Observation","id":"o","subject":{"reference":"Patient/p"}}},{"fullUrl":"urn:uuid:pat","resource":{"resourceType":"Patient","id":"p","contained":[{"resourceType":"Organization","id":"org"}],"managingOrganization":{"reference":"#org"}}}]}),
    );
    assert_eq!(
        eval(
            "entry[0].resource.subject.resolve().managingOrganization.resolve().pathname()",
            &c
        )
        .unwrap()
        .to_json(),
        json!("Bundle.entry[1].resource[0].contained[0]")
    );
    assert_eq!(
        eval("'urn:uuid:pat'.resolve().id", &c).unwrap().to_json(),
        json!("p")
    );
    assert_eq!(eval("'Missing/1'.resolve()", &c).unwrap(), V::Empty);
    let focus = FhirPathEvaluator::new()
        .evaluate_nodes(
            &FhirPathParser::new()
                .parse("entry[1].resource.managingOrganization")
                .unwrap(),
            &c,
        )
        .unwrap();
    let c = c.with_initial_focus(focus);
    assert_eq!(eval("%resource.id", &c).unwrap().to_json(), json!("p"));
    assert_eq!(
        eval("%rootResource is Bundle", &c).unwrap(),
        V::Boolean(true)
    );
}

#[derive(Debug)]
struct Host;
impl FhirHost for Host {
    fn conforms_to(
        &self,
        value: &Value,
        _ty: Option<&str>,
        profile: &str,
    ) -> FhirPathResult<Option<bool>> {
        Ok((profile == "urn:only-male").then(|| value["gender"] == "male"))
    }
}

#[test]
fn profile_service_and_missing_release_results() {
    for gender in ["male", "other"] {
        let mut c = EvaluationContext::new(json!({"resourceType":"Patient","gender":gender}));
        c.host = Some(Arc::new(Host));
        assert_eq!(
            eval("conformsTo('urn:only-male')", &c).unwrap(),
            V::Boolean(gender == "male")
        );
        assert!(eval("conformsTo('urn:missing')", &c).is_err());
        c.model = FhirModel::R5;
        assert_eq!(eval("conformsTo('urn:missing')", &c).unwrap(), V::Empty);
        assert_eq!(
            eval("gender.memberOf('urn:missing')", &c).unwrap(),
            V::Empty
        );
    }
}

#[test]
fn narrative_and_modifier_checks() {
    let mut c = EvaluationContext::new(
        json!({"resourceType":"Patient","text":{"div":"<div xmlns='http://www.w3.org/1999/xhtml'><p>Okay</p></div>"},"modifierExtension":[{"url":"urn:modifier","valueBoolean":true}]}),
    );
    c.model = FhirModel::R5;
    assert_eq!(eval("text.div.htmlChecks()", &c).unwrap(), V::Boolean(true));
    assert_eq!(eval("Patient.htmlChecks()", &c).unwrap(), V::Empty);
    assert!(eval("Patient.checkModifiers('urn:other')", &c).is_err());
    assert_eq!(
        eval("Patient.checkModifiers('urn:modifier').exists()", &c).unwrap(),
        V::Boolean(true)
    );
    c.root["text"]["div"] =
        json!("<div xmlns='http://www.w3.org/1999/xhtml'><script>bad</script></div>");
    c.current = c.root.clone();
    assert_eq!(
        eval("text.div.htmlChecks()", &c).unwrap(),
        V::Boolean(false)
    );
}

#[test]
fn narrative_checks_normalize_attribute_whitespace() {
    for (markup, expected) in [
        ("<a href='JaVa ScRiPt:'>Link</a>", false),
        ("<a href='vb&#32;script:'>Link</a>", false),
        ("<a href='&#127;vbscript:'>Link</a>", false),
        ("<a href='da&#x9;ta:text/plain,example'>Link</a>", false),
        (
            "<img alt='Preview' src='da ta:image/png;base64,AA=='/>",
            false,
        ),
        ("<p style='behavior\t: none'>Text</p>", false),
        ("<p style='background-image: URL (none)'>Text</p>", false),
        ("<p style='width: expression ()'>Text</p>", false),
        ("<p style='behavior/**/: none'>Text</p>", false),
        (r"<p style='\62 ehavior: none'>Text</p>", false),
        ("<a href='/care plan#notes'>Link</a>", true),
        ("<p style='color : navy; margin: 0 1em'>Text</p>", true),
        (
            r"<p style='color: \006e avy; /* note */ margin: 0'>Text</p>",
            true,
        ),
        (
            r#"<p style='font-family: "/*"; color: navy'>Text</p>"#,
            true,
        ),
        (
            "<img alt='Preview' src='DaTa:image/png;base64,AA=='/>",
            true,
        ),
    ] {
        let mut context = EvaluationContext::new(json!({
            "resourceType": "Patient",
            "text": {
                "div": format!("<div xmlns='http://www.w3.org/1999/xhtml'>{markup}</div>")
            }
        }));
        context.model = FhirModel::R5;
        assert_eq!(
            eval("text.div.htmlChecks()", &context).unwrap(),
            V::Boolean(expected),
            "{markup}"
        );
    }
}

#[test]
fn scientific_values_preserve_magnitude_without_expanding_the_exponent() {
    let mut c = EvaluationContext::from_json_str(r#"{
        "resourceType":"Parameters","parameter":[
          {"name":"large","valueQuantity":{"value":1e999999999,"system":"http://unitsofmeasure.org","code":"kg"}},
          {"name":"small","valueDecimal":1e-999999999}
        ]}"#)
    .unwrap();
    c.model = FhirModel::R5;
    for (expression, result) in [
        ("parameter[0].value > 1 'kg'", V::Boolean(true)),
        ("parameter[0].value = 1 'g'", V::Empty),
        (
            "parameter[0].value.toString()",
            V::String("1E+999999999 'kg'".into()),
        ),
        ("parameter[0].value.value.hasValue()", V::Boolean(true)),
        ("parameter[0].value * 2", V::Empty),
        ("parameter[1].value.hasValue()", V::Boolean(true)),
        ("parameter[1].value > 0", V::Boolean(true)),
        ("parameter[1].value ~ 0", V::Boolean(true)),
        (
            "parameter[1].value.toString()",
            V::String("1E-999999999".into()),
        ),
    ] {
        assert_eq!(eval(expression, &c).unwrap(), result, "{expression}");
    }
    let raw = eval_raw("parameter[1].value", &c);
    assert_eq!(raw.get(), "1e-999999999");
    let quantity = eval_raw("parameter[0].value", &c);
    let fields: std::collections::HashMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(quantity.get()).unwrap();
    assert_eq!(fields["value"].get(), "1e999999999");
    assert_eq!(fields["code"].get(), r#""kg""#);
    assert_eq!(
        eval_raw(
            "Quantity { value: parameter[1].value, unit: 'kg' }.value",
            &c
        )
        .get(),
        "1e-999999999"
    );
}

#[test]
fn exact_decimal_lexemes_follow_nested_choice_and_primitive_metadata() {
    let c = EvaluationContext::from_json_str(
        r#"{
      "resourceType": "Observation",
      "component": [
        {"valueQuantity": {"value": 1.23456789012345670, "unit": "kg"}},
        {
          "valueQuantity": {
            "value": 1.23456789012345671,
            "_value": {
              "id": "second",
              "extension": [{"url": "urn:precision", "valueDecimal": 0.0000000000000000100}]
            },
            "unit": "kg"
          }
        }
      ]
    }"#,
    )
    .unwrap();
    assert_eq!(
        eval("component[0].value.value < component[1].value.value", &c).unwrap(),
        V::Boolean(true)
    );
    let expression = "component.value.value.where(id = 'second')";
    assert_eq!(eval_raw(expression, &c).get(), "1.23456789012345671");
    assert_eq!(
        eval_raw(
            "component.value.value.where(id = 'second').extension('urn:precision').value",
            &c
        )
        .get(),
        "0.0000000000000000100"
    );
    assert_eq!(
        eval_raw("component.value.value.select(getValue())", &c).get(),
        "[1.23456789012345670,1.23456789012345671]"
    );
}

#[test]
fn constructed_decimal_fields_keep_exact_values() {
    let c = EvaluationContext::from_json_str("null").unwrap();
    let expression = "Quantity { value: 1.23456789012345671, unit: 'kg' }.value";
    assert_eq!(eval_raw(expression, &c).get(), "1.23456789012345671");
}

#[test]
fn quantity_boundaries_use_the_source_decimal_exponent() {
    let c = EvaluationContext::from_json_str(
        r#"{
          "resourceType": "Observation",
          "valueQuantity": {"value": 9.5e2, "system": "http://unitsofmeasure.org", "code": "kg"}
        }"#,
    )
    .unwrap();
    assert_eq!(eval_raw("value.lowBoundary(0).value", &c).get(), "945");
    assert_eq!(eval_raw("value.highBoundary(0).value", &c).get(), "955");
}

#[test]
fn untyped_json_objects_compare_nested_exact_decimals() {
    let mut c = EvaluationContext::from_json_str(
        r#"{
            "a": {"nested": [[{"_amount": 1.23456789012345671}]]},
            "b": {"nested": [[{"_amount": 1.23456789012345672}]]},
            "exact": {"nested": [{"amount": 1.250}]},
            "huge": {"amount": 1e9999},
            "hugeEquivalent": {"amount": 10e9998},
            "absent": {"amount": null}
        }"#,
    )
    .unwrap();
    assert_eq!(c.root["a"], c.root["b"]);
    c.add_constant(
        "ordinary".into(),
        V::from_json(&json!({"nested": [{"amount": 1.25}]})),
    );
    for (expression, expected) in [
        ("a = b", false),
        ("a ~ b", false),
        ("a = a", true),
        ("a ~ a", true),
        ("exact = %ordinary", true),
        ("%ordinary ~ exact", true),
        ("huge = hugeEquivalent", true),
        ("huge ~ hugeEquivalent", true),
        ("huge = absent", false),
        ("huge ~ absent", false),
    ] {
        assert_eq!(
            eval(expression, &c).unwrap(),
            V::Boolean(expected),
            "{expression}"
        );
    }
}

#[test]
fn repeat_distinguishes_exact_objects_and_deduplicates_ordinary_matches() {
    let c = EvaluationContext::from_json_str(
        r#"{
            "items": [
                {"nested": {"amount": 1.23456789012345670}},
                {"nested": {"amount": 1.23456789012345671}},
                {"nested": {"amount": 1.250}},
                {"nested": {"amount": 1.25}}
            ]
        }"#,
    )
    .unwrap();
    assert_eq!(eval("repeat(items).count()", &c).unwrap(), V::Integer(3));
}

#[test]
fn contained_references_remain_in_the_owning_bundle_entry() {
    let c = EvaluationContext::new(
        json!({"resourceType":"Bundle","entry":[{"resource":{"resourceType":"Patient","id":"p","contained":[
        {"resourceType":"Organization","id":"a","partOf":{"reference":"#b"}},
        {"resourceType":"Organization","id":"b","partOf":{"reference":"#"}}]}}]}),
    );
    assert_eq!(
        eval("entry.resource.contained[0].partOf.resolve().id", &c)
            .unwrap()
            .as_str(),
        Some("b")
    );
    assert_eq!(
        eval("entry.resource.contained[1].partOf.resolve().id", &c)
            .unwrap()
            .as_str(),
        Some("p")
    );
}
