use rh_fhirpath::{FhirPathEvaluator, FhirPathParser, FhirPathValue as V};
use rh_validator::{FhirValidator, FhirVersion};
use serde_json::json;

#[test]
fn registered_profiles_and_value_sets_drive_fhirpath() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    let url = "urn:only-female";
    validator.register_profile(&json!({
        "resourceType":"StructureDefinition","url":url,"name":"OnlyFemale","type":"Patient",
        "snapshot":{"element":[{"id":"Patient","path":"Patient"},{"id":"Patient.id","path":"Patient.id","type":[{"code":"id"}]},{"id":"Patient.gender","path":"Patient.gender","min":1,"max":"1","type":[{"code":"code"}],"fixedCode":"female"}]}
    }));
    validator.register_valueset(&json!({"resourceType":"ValueSet","url":"urn:genders","compose":{"include":[{"system":"urn:g","concept":[{"code":"female"}]}]}}));
    let evaluator = FhirPathEvaluator::new();
    let parser = FhirPathParser::new();
    for gender in ["female", "male"] {
        let c =
            validator.fhirpath_context(json!({"resourceType":"Patient","id":"p","gender":gender}));
        if gender == "female" {
            let result = validator.validate_with_profile(&c.root, url).unwrap();
            assert!(result.valid, "{:#?}", result.issues);
        }
        for expression in [
            "conformsTo('urn:only-female')",
            "gender.memberOf('urn:genders')",
        ] {
            assert_eq!(
                evaluator
                    .evaluate(&parser.parse(expression).unwrap(), &c)
                    .unwrap(),
                V::Boolean(gender == "female"),
                "{expression}, {gender}"
            );
        }
    }
}

#[test]
fn invariants_receive_the_same_host_and_propagate_operational_failures() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    validator.register_valueset(&json!({"resourceType":"ValueSet","url":"urn:genders","compose":{"include":[{"system":"urn:g","concept":[{"code":"female"}]}]}}));
    for (profile, expression) in [
        ("urn:patient-binding", "memberOf('urn:genders')"),
        ("urn:patient-missing", "memberOf('urn:unresolvable')"),
    ] {
        validator.register_profile(&json!({
            "resourceType":"StructureDefinition","url":profile,"name":"GenderBinding","type":"Patient",
            "snapshot":{"element":[{"id":"Patient","path":"Patient"},{"id":"Patient.gender","path":"Patient.gender","type":[{"code":"code"}],"constraint":[{"key":"g-1","human":"Use a member of the specified value set","severity":"error","expression":expression}]}]}
        }));
    }
    let eval = FhirPathEvaluator::new();
    let parser = FhirPathParser::new();
    for gender in ["female", "male"] {
        let c = validator.fhirpath_context(json!({"resourceType":"Patient","gender":gender}));
        assert_eq!(
            eval.evaluate(
                &parser.parse("conformsTo('urn:patient-binding')").unwrap(),
                &c
            )
            .unwrap(),
            V::Boolean(gender == "female")
        );
        assert!(eval
            .evaluate(
                &parser.parse("conformsTo('urn:patient-missing')").unwrap(),
                &c
            )
            .is_err());
    }
}

#[test]
fn boolean_primitive_invariant_reports_false() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    validator.register_profile(&json!({
        "resourceType":"StructureDefinition","url":"urn:active-patient","name":"ActivePatient","type":"Patient",
        "snapshot":{"element":[{"id":"Patient","path":"Patient","constraint":[{"key":"a-1","human":"The patient must be active","severity":"error","expression":"active"}]},{"id":"Patient.active","path":"Patient.active","type":[{"code":"boolean"}]}]}
    }));
    let expression = FhirPathParser::new()
        .parse("conformsTo('urn:active-patient')")
        .unwrap();
    for active in [false, true] {
        let context = validator.fhirpath_context(json!({"resourceType":"Patient","active":active}));
        assert_eq!(
            FhirPathEvaluator::new()
                .evaluate(&expression, &context)
                .unwrap(),
            V::Boolean(active)
        );
    }
}

#[test]
fn slice_matches_the_source_path_when_names_repeat() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    let url = "urn:sliced-telecom";
    let mut elements = vec![json!({"id":"Patient","path":"Patient"})];
    for (path, system) in [
        ("Patient.telecom", "phone"),
        ("Patient.contact.telecom", "email"),
    ] {
        elements.extend([
            json!({"id":path,"path":path,"max":"*","slicing":{"discriminator":[{"type":"value","path":"system"}],"rules":"open"}}),
            json!({"id":format!("{path}:work"),"path":path,"sliceName":"work","min":0,"max":"*"}),
            json!({"id":format!("{path}:work.system"),"path":format!("{path}.system"),"fixedCode":system}),
        ]);
    }
    validator.register_profile(&json!({
        "resourceType":"StructureDefinition","url":url,"name":"SlicedTelecom","type":"Patient",
        "snapshot":{"element":elements},
    }));
    let both_paths = ["Patient.telecom", "Patient.contact.telecom"];
    assert_eq!(
        rh_fhirpath::extensions::fhir::host::FhirHost::matches_slice(
            &validator,
            &json!({"system":"phone"}),
            &both_paths,
            url,
            "work"
        )
        .unwrap(),
        None
    );
    let phones = json!([{"system":"phone","value":"p"},{"system":"email","value":"e"}]);
    let context = validator.fhirpath_context(json!({"resourceType":"Bundle","entry":[{"resource":{
        "resourceType":"Patient","telecom":phones,"contact":[{"telecom":phones}]
    }}]}));
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    for (expr, expected) in [
        (
            "telecom.slice('urn:sliced-telecom','work').value",
            json!("p"),
        ),
        (
            "contact.telecom.slice('urn:sliced-telecom','work').value",
            json!("e"),
        ),
        (
            "telecom.slice('urn:sliced-telecom','unknown').empty()",
            json!(true),
        ),
        ("telecom.slice('urn:missing','work').empty()", json!(true)),
        (
            "contact.slice('urn:sliced-telecom','work').empty()",
            json!(true),
        ),
    ] {
        let expression = parser.parse(&format!("entry.resource.{expr}")).unwrap();
        assert_eq!(
            evaluator.evaluate(&expression, &context).unwrap().to_json(),
            expected,
            "{expr}"
        );
    }
}

#[test]
fn slice_matches_a_qualified_reslice() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    validator.register_profile(&json!({
        "resourceType":"StructureDefinition","url":"urn:telecom-reslices","name":"TelecomReslices","type":"Patient",
        "snapshot":{"element":[
            {"id":"Patient","path":"Patient"},
            {"id":"Patient.telecom","path":"Patient.telecom","slicing":{"discriminator":[{"type":"value","path":"system"}],"rules":"open"}},
            {"id":"Patient.telecom:phone","path":"Patient.telecom","sliceName":"phone","slicing":{"discriminator":[{"type":"value","path":"system"},{"type":"value","path":"use"}],"rules":"open"}},
            {"id":"Patient.telecom:phone.system","path":"Patient.telecom.system","fixedCode":"phone"},
            {"id":"Patient.telecom:phone/work","path":"Patient.telecom","sliceName":"phone/work"},
            {"id":"Patient.telecom:phone/work.system","path":"Patient.telecom.system","fixedCode":"phone"},
            {"id":"Patient.telecom:phone/work.use","path":"Patient.telecom.use","fixedCode":"work"},
            {"id":"Patient.telecom:email","path":"Patient.telecom","sliceName":"email"},
            {"id":"Patient.telecom:email.system","path":"Patient.telecom.system","fixedCode":"email"}
        ]}
    }));
    let context = validator.fhirpath_context(json!({"resourceType":"Patient","telecom":[
        {"system":"phone","use":"work","value":"work-phone"},
        {"system":"phone","use":"home","value":"home-phone"},
        {"system":"email","use":"work","value":"work-email"}
    ]}));
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let expression = parser
        .parse("telecom.slice('urn:telecom-reslices','phone/work').value")
        .unwrap();
    assert_eq!(
        evaluator.evaluate(&expression, &context).unwrap().to_json(),
        json!("work-phone")
    );
}
