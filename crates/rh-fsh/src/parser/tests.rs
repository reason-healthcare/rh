//! Parser unit tests — one focused test per entity kind, rule type, value
//! type, and path form (refactor plan task 2.8).

use super::*;
use crate::parser::ast::*;

fn parse(src: &str) -> FshDocument {
    FshParser::parse(src, "test.fsh").expect("parse should succeed")
}

fn single_entity(src: &str) -> FshEntity {
    let doc = parse(src);
    assert_eq!(doc.entities.len(), 1, "expected one entity in {src:?}");
    doc.entities.into_iter().next().unwrap().value
}

fn profile(src: &str) -> Profile {
    match single_entity(src) {
        FshEntity::Profile(p) => p,
        other => panic!("expected Profile, got {other:?}"),
    }
}

fn profile_rule(rule_src: &str) -> SdRule {
    let src = format!("Profile: P\nParent: Patient\n{rule_src}\n");
    let p = profile(&src);
    assert_eq!(p.rules.len(), 1, "expected one rule from {rule_src:?}");
    p.rules.into_iter().next().unwrap().value
}

// ============================================================================
// Entities
// ============================================================================

#[test]
fn parses_profile_with_full_metadata() {
    let p = profile(
        "Profile: MyPatient\nParent: Patient\nId: my-patient\nTitle: \"My Patient\"\nDescription: \"A profile.\"\n",
    );
    assert_eq!(p.metadata.name, "MyPatient");
    assert_eq!(p.metadata.parent.as_deref(), Some("Patient"));
    assert_eq!(p.metadata.id.as_deref(), Some("my-patient"));
    assert_eq!(p.metadata.title.as_deref(), Some("My Patient"));
    assert_eq!(p.metadata.description.as_deref(), Some("A profile."));
}

#[test]
fn parses_extension_entity() {
    let e = match single_entity("Extension: MyExt\nId: my-ext\nTitle: \"E\"\n") {
        FshEntity::Extension(e) => e,
        other => panic!("expected Extension, got {other:?}"),
    };
    assert_eq!(e.metadata.name, "MyExt");
}

#[test]
fn parses_logical_entity() {
    let l = match single_entity("Logical: MyModel\nTitle: \"Model\"\n") {
        FshEntity::Logical(l) => l,
        other => panic!("expected Logical, got {other:?}"),
    };
    assert_eq!(l.metadata.name, "MyModel");
}

#[test]
fn parses_resource_entity() {
    let r = match single_entity("Resource: MyResource\nTitle: \"R\"\n") {
        FshEntity::Resource(r) => r,
        other => panic!("expected Resource, got {other:?}"),
    };
    assert_eq!(r.metadata.name, "MyResource");
}

#[test]
fn parses_instance_entity() {
    let i = match single_entity(
        "Instance: example\nInstanceOf: Patient\nUsage: #example\n* active = true\n",
    ) {
        FshEntity::Instance(i) => i,
        other => panic!("expected Instance, got {other:?}"),
    };
    assert_eq!(i.metadata.name, "example");
    assert_eq!(i.metadata.instance_of, "Patient");
    assert_eq!(i.rules.len(), 1);
}

#[test]
fn parses_value_set_entity() {
    let vs = match single_entity(
        "ValueSet: MyVS\nId: my-vs\nTitle: \"VS\"\n* include codes from system http://loinc.org\n",
    ) {
        FshEntity::ValueSet(vs) => vs,
        other => panic!("expected ValueSet, got {other:?}"),
    };
    assert_eq!(vs.metadata.name, "MyVS");
    assert_eq!(vs.components.len(), 1);
}

#[test]
fn parses_code_system_entity() {
    let cs = match single_entity(
        "CodeSystem: MyCS\nId: my-cs\n* #active \"Active\" \"Is active.\"\n* #inactive \"Inactive\"\n",
    ) {
        FshEntity::CodeSystem(cs) => cs,
        other => panic!("expected CodeSystem, got {other:?}"),
    };
    assert_eq!(cs.metadata.name, "MyCS");
    assert_eq!(cs.concepts.len(), 2);
    assert_eq!(cs.concepts[0].value.code, "active");
    assert_eq!(cs.concepts[0].value.display.as_deref(), Some("Active"));
    assert_eq!(
        cs.concepts[0].value.definition.as_deref(),
        Some("Is active.")
    );
}

#[test]
fn parses_invariant_entity() {
    let inv = match single_entity(
        "Invariant: inv-1\nDescription: \"Must have a name\"\nExpression: \"name.exists()\"\nSeverity: #error\n",
    ) {
        FshEntity::Invariant(inv) => inv,
        other => panic!("expected Invariant, got {other:?}"),
    };
    assert_eq!(inv.name, "inv-1");
    assert_eq!(inv.severity.as_deref(), Some("#error"));
    assert_eq!(inv.expression.as_deref(), Some("name.exists()"));
}

#[test]
fn parses_mapping_entity() {
    let m = match single_entity(
        "Mapping: MyMap\nSource: Patient\nTarget: \"http://hl7.org/v2\"\nId: v2-map\n* identifier -> \"PID-3\"\n",
    ) {
        FshEntity::Mapping(m) => m,
        other => panic!("expected Mapping, got {other:?}"),
    };
    assert_eq!(m.metadata.name, "MyMap");
    assert_eq!(m.rules.len(), 1);
}

#[test]
fn parses_rule_set_entity() {
    let rs = match single_entity("RuleSet: CommonRules\n* active 1..1\n* name MS\n") {
        FshEntity::RuleSet(rs) => rs,
        other => panic!("expected RuleSet, got {other:?}"),
    };
    assert_eq!(rs.name, "CommonRules");
    assert_eq!(rs.rules.len(), 2);
}

#[test]
fn parses_parameterized_rule_set_entity() {
    let prs = match single_entity("RuleSet: CardRule(path, min, max)\n* {path} {min}..{max}\n") {
        FshEntity::ParamRuleSet(p) => p,
        other => panic!("expected ParamRuleSet, got {other:?}"),
    };
    assert_eq!(prs.name, "CardRule");
    assert_eq!(prs.params, vec!["path", "min", "max"]);
    assert!(prs.raw_rules.iter().any(|r| r.contains("{path}")));
}

#[test]
fn parses_alias_entity() {
    let a = match single_entity("Alias: $sct = http://snomed.info/sct\n") {
        FshEntity::Alias(a) => a,
        other => panic!("expected Alias, got {other:?}"),
    };
    assert_eq!(a.name, "$sct");
    assert_eq!(a.value, "http://snomed.info/sct");
}

#[test]
fn parses_multiple_entities_in_one_document() {
    let doc = parse(
        "Alias: $sct = http://snomed.info/sct\n\nProfile: P\nParent: Patient\n* active 1..1\n\nValueSet: V\n* include codes from system $sct\n",
    );
    assert_eq!(doc.entities.len(), 3);
}

// ============================================================================
// SD rules — one test per SdRule variant
// ============================================================================

#[test]
fn parses_card_rule() {
    let rule = profile_rule("* identifier 1..*");
    match rule {
        SdRule::Card(c) => {
            assert_eq!(c.path.to_dot_string(), "identifier");
            assert_eq!(c.min, Some(1));
            // By design: `*` (unbounded) is represented as `None`; the
            // exporter renders it back as "*".
            assert_eq!(c.max, None);
        }
        other => panic!("expected Card, got {other:?}"),
    }
}

#[test]
fn parses_card_rule_with_trailing_flags() {
    let rule = profile_rule("* identifier 1..1 MS SU");
    match rule {
        SdRule::Card(c) => {
            assert_eq!(c.min, Some(1));
            assert_eq!(c.max.as_deref(), Some("1"));
            assert_eq!(c.flags.len(), 2);
        }
        other => panic!("expected Card, got {other:?}"),
    }
}

#[test]
fn parses_flag_rule_multiple_paths() {
    let rule = profile_rule("* name and birthDate MS");
    match rule {
        SdRule::Flag(f) => {
            assert_eq!(f.paths.len(), 2);
            assert_eq!(f.paths[0].to_dot_string(), "name");
            assert_eq!(f.paths[1].to_dot_string(), "birthDate");
            assert_eq!(f.flags.len(), 1);
        }
        other => panic!("expected Flag, got {other:?}"),
    }
}

#[test]
fn parses_binding_rule_with_strength() {
    let rule =
        profile_rule("* gender from http://hl7.org/fhir/ValueSet/administrative-gender (required)");
    match rule {
        SdRule::Binding(b) => {
            assert_eq!(b.path.to_dot_string(), "gender");
            assert_eq!(
                b.value_set,
                "http://hl7.org/fhir/ValueSet/administrative-gender"
            );
            assert_eq!(b.strength.as_deref(), Some("required"));
        }
        other => panic!("expected Binding, got {other:?}"),
    }
}

#[test]
fn parses_assignment_rule_string() {
    let rule = profile_rule("* name.family = \"Doe\"");
    match rule {
        SdRule::Assignment(a) => {
            assert_eq!(a.path.to_dot_string(), "name.family");
            assert!(matches!(a.value, FshValue::Str(ref s) if s == "Doe"));
            assert!(!a.exactly);
        }
        other => panic!("expected Assignment, got {other:?}"),
    }
}

#[test]
fn parses_assignment_rule_exactly() {
    let rule = profile_rule("* active = true (exactly)");
    match rule {
        SdRule::Assignment(a) => {
            assert!(matches!(a.value, FshValue::Bool(true)));
            assert!(a.exactly);
        }
        other => panic!("expected Assignment, got {other:?}"),
    }
}

#[test]
fn parses_contains_rule_with_cardinality() {
    let rule = profile_rule("* identifier contains MRN 1..1 and SSN 0..1");
    match rule {
        SdRule::Contains(c) => {
            assert_eq!(c.path.to_dot_string(), "identifier");
            assert_eq!(c.items.len(), 2);
            assert_eq!(c.items[0].name, "MRN");
            assert_eq!(c.items[0].min, Some(1));
            assert_eq!(c.items[1].name, "SSN");
            assert_eq!(c.items[1].max.as_deref(), Some("1"));
        }
        other => panic!("expected Contains, got {other:?}"),
    }
}

#[test]
fn parses_contains_rule_with_named_alias() {
    let rule = profile_rule(
        "* extension contains http://example.org/StructureDefinition/my-ext named myExt 0..1",
    );
    match rule {
        SdRule::Contains(c) => {
            assert_eq!(c.items.len(), 1);
            assert_eq!(
                c.items[0].name,
                "http://example.org/StructureDefinition/my-ext"
            );
            assert_eq!(c.items[0].alias.as_deref(), Some("myExt"));
        }
        other => panic!("expected Contains, got {other:?}"),
    }
}

#[test]
fn parses_only_rule() {
    let rule = profile_rule("* value[x] only Quantity or CodeableConcept");
    match rule {
        SdRule::Only(o) => {
            assert_eq!(o.types, vec!["Quantity", "CodeableConcept"]);
        }
        other => panic!("expected Only, got {other:?}"),
    }
}

#[test]
fn parses_only_rule_with_reference() {
    let rule = profile_rule("* subject only Reference(Patient or Group)");
    match rule {
        SdRule::Only(o) => {
            assert!(!o.types.is_empty());
        }
        other => panic!("expected Only, got {other:?}"),
    }
}

#[test]
fn parses_obeys_rule_with_path() {
    let rule = profile_rule("* name obeys inv-1 and inv-2");
    match rule {
        SdRule::Obeys(o) => {
            assert_eq!(
                o.path.as_ref().map(|p| p.to_dot_string()).as_deref(),
                Some("name")
            );
            assert_eq!(o.invariants, vec!["inv-1", "inv-2"]);
        }
        other => panic!("expected Obeys, got {other:?}"),
    }
}

#[test]
fn parses_obeys_rule_without_path() {
    let rule = profile_rule("* obeys inv-1");
    match rule {
        SdRule::Obeys(o) => {
            assert!(o.path.is_none());
            assert_eq!(o.invariants, vec!["inv-1"]);
        }
        other => panic!("expected Obeys, got {other:?}"),
    }
}

#[test]
fn parses_caret_value_rule_standalone() {
    let rule = profile_rule("* ^url = \"http://example.org/sd\"");
    match rule {
        SdRule::CaretValue(c) => {
            assert!(c.path.is_none());
            assert_eq!(c.caret_path, "url");
            assert!(matches!(c.value, FshValue::Str(ref s) if s == "http://example.org/sd"));
        }
        other => panic!("expected CaretValue, got {other:?}"),
    }
}

#[test]
fn parses_caret_value_rule_with_element_path() {
    let rule = profile_rule("* identifier ^short = \"An identifier\"");
    match rule {
        SdRule::CaretValue(c) => {
            assert_eq!(
                c.path.as_ref().map(|p| p.to_dot_string()).as_deref(),
                Some("identifier")
            );
            assert_eq!(c.caret_path, "short");
        }
        other => panic!("expected CaretValue, got {other:?}"),
    }
}

#[test]
fn parses_caret_value_rule_nested_caret_path() {
    let rule = profile_rule("* ^status = #draft");
    match rule {
        SdRule::CaretValue(c) => {
            assert_eq!(c.caret_path, "status");
            assert!(matches!(c.value, FshValue::Code { ref code, .. } if code == "draft"));
        }
        other => panic!("expected CaretValue, got {other:?}"),
    }
}

#[test]
fn parses_insert_rule() {
    let rule = profile_rule("* insert CommonRules");
    match rule {
        SdRule::Insert(i) => {
            assert_eq!(i.rule_set, "CommonRules");
            assert!(i.params.is_empty());
        }
        other => panic!("expected Insert, got {other:?}"),
    }
}

#[test]
fn parses_insert_rule_with_params() {
    let rule = profile_rule("* insert CardRule(identifier, 1, *)");
    match rule {
        SdRule::Insert(i) => {
            assert_eq!(i.rule_set, "CardRule");
            assert_eq!(i.params, vec!["identifier", "1", "*"]);
        }
        other => panic!("expected Insert, got {other:?}"),
    }
}

#[test]
fn parses_path_rule() {
    let rule = profile_rule("* identifier.system");
    match rule {
        SdRule::Path(p) => {
            assert_eq!(p.path.to_dot_string(), "identifier.system");
        }
        other => panic!("expected Path, got {other:?}"),
    }
}

// ============================================================================
// Values — one test per FshValue variant
// ============================================================================

fn assignment_value(rule_src: &str) -> FshValue {
    match profile_rule(rule_src) {
        SdRule::Assignment(a) => a.value,
        other => panic!("expected Assignment from {rule_src:?}, got {other:?}"),
    }
}

#[test]
fn parses_boolean_value() {
    assert!(matches!(
        assignment_value("* active = false"),
        FshValue::Bool(false)
    ));
}

#[test]
fn parses_integer_value() {
    assert!(matches!(
        assignment_value("* multipleBirthInteger = 3"),
        FshValue::Integer(3)
    ));
}

#[test]
fn parses_decimal_value() {
    match assignment_value("* valueDecimal = 3.5") {
        FshValue::Decimal(d) => assert!((d - 3.5).abs() < f64::EPSILON),
        other => panic!("expected Decimal, got {other:?}"),
    }
}

#[test]
fn parses_code_value_with_system_and_display() {
    match assignment_value("* code = http://loinc.org#1234-5 \"My Code\"") {
        FshValue::Code {
            system,
            code,
            display,
        } => {
            assert_eq!(system.as_deref(), Some("http://loinc.org"));
            assert_eq!(code, "1234-5");
            assert_eq!(display.as_deref(), Some("My Code"));
        }
        other => panic!("expected Code, got {other:?}"),
    }
}

#[test]
fn parses_bare_code_value() {
    match assignment_value("* status = #final") {
        FshValue::Code { system, code, .. } => {
            assert!(system.is_none());
            assert_eq!(code, "final");
        }
        other => panic!("expected Code, got {other:?}"),
    }
}

#[test]
fn parses_quantity_value() {
    match assignment_value("* valueQuantity = 85 'kg'") {
        FshValue::Quantity { value, unit } => {
            assert!((value - 85.0).abs() < f64::EPSILON);
            assert_eq!(unit, "kg");
        }
        other => panic!("expected Quantity, got {other:?}"),
    }
}

#[test]
fn parses_reference_value() {
    match assignment_value("* subject = Reference(patient-1)") {
        FshValue::Reference(r) => assert_eq!(r, "patient-1"),
        other => panic!("expected Reference, got {other:?}"),
    }
}

#[test]
fn parses_canonical_value() {
    match assignment_value("* instantiatesCanonical = Canonical(my-definition)") {
        FshValue::Canonical(c) => assert!(c.contains("my-definition")),
        other => panic!("expected Canonical, got {other:?}"),
    }
}

#[test]
fn parses_date_value() {
    match assignment_value("* birthDate = 1960-04-25") {
        FshValue::Date(d) => assert_eq!(d, "1960-04-25"),
        other => panic!("expected Date, got {other:?}"),
    }
}

#[test]
fn parses_datetime_value() {
    match assignment_value("* effectiveDateTime = 2020-01-01T10:00:00Z") {
        FshValue::DateTime(d) => assert_eq!(d, "2020-01-01T10:00:00Z"),
        other => panic!("expected DateTime, got {other:?}"),
    }
}

#[test]
fn parses_string_with_escaped_quotes() {
    match assignment_value(r#"* name.text = "He said \"hi\"""#) {
        FshValue::Str(s) => assert_eq!(s, r#"He said "hi""#),
        other => panic!("expected Str, got {other:?}"),
    }
}

// ============================================================================
// Paths: indexes, soft indexing, slices, choice types, extensions
// ============================================================================

fn assignment_path(rule_src: &str) -> FshPath {
    match profile_rule(rule_src) {
        SdRule::Assignment(a) => a.path,
        other => panic!("expected Assignment from {rule_src:?}, got {other:?}"),
    }
}

#[test]
fn parses_numeric_index_path() {
    let path = assignment_path("* name[0].given = \"John\"");
    // Numeric indexes are currently represented as Slice segments with a
    // numeric slice name; `to_fhir_path` strips them correctly either way.
    assert!(path
        .segments
        .iter()
        .any(|s| matches!(s, FshPathSegment::Index(0) | FshPathSegment::Slice { .. })));
    assert_eq!(path.to_fhir_path("Patient"), "Patient.name.given");
}

#[test]
fn parses_soft_index_path() {
    // Soft indexing `[+]` appends a new element.
    let src = "Instance: ex\nInstanceOf: Patient\n* name[+].given = \"John\"\n* name[=].family = \"Doe\"\n";
    let doc = parse(src);
    let FshEntity::Instance(instance) = &doc.entities[0].value else {
        panic!("expected instance");
    };
    assert_eq!(instance.rules.len(), 2);
}

#[test]
fn parses_slice_path() {
    let path = assignment_path("* identifier[MRN].system = \"http://example.org/mrn\"");
    assert!(path.segments.iter().any(|s| matches!(
        s,
        FshPathSegment::Slice { element, slice }
            if element == "identifier" && slice == "MRN"
    )));
}

#[test]
fn parses_choice_type_path() {
    let path = assignment_path("* valueQuantity = 1 'mg'");
    assert!(!path.segments.is_empty());
}

#[test]
fn parses_deeply_nested_path() {
    let path = assignment_path("* contact.name.family = \"Smith\"");
    assert_eq!(path.to_dot_string(), "contact.name.family");
    assert_eq!(path.segments.len(), 3);
}

#[test]
fn fhir_path_strips_slice_names() {
    let path = assignment_path("* identifier[MRN].system = \"x\"");
    assert_eq!(path.to_fhir_path("Patient"), "Patient.identifier.system");
}

// ============================================================================
// ValueSet component rules
// ============================================================================

fn value_set(src: &str) -> ValueSet {
    match single_entity(src) {
        FshEntity::ValueSet(vs) => vs,
        other => panic!("expected ValueSet, got {other:?}"),
    }
}

#[test]
fn parses_vs_include_from_system() {
    let vs = value_set("ValueSet: V\n* include codes from system http://loinc.org\n");
    let rule = &vs.components[0].value;
    assert!(rule.inclusion);
    assert_eq!(rule.system.as_deref(), Some("http://loinc.org"));
}

#[test]
fn parses_vs_exclude_rule() {
    let vs = value_set(
        "ValueSet: V\n* include codes from system http://loinc.org\n* exclude http://loinc.org#1234-5\n",
    );
    assert_eq!(vs.components.len(), 2);
    assert!(!vs.components[1].value.inclusion);
}

#[test]
fn parses_vs_include_from_valueset() {
    let vs = value_set("ValueSet: V\n* include codes from valueset http://example.org/vs/other\n");
    let rule = &vs.components[0].value;
    assert!(rule
        .from_vs
        .iter()
        .any(|v| v == "http://example.org/vs/other"));
}

#[test]
fn parses_vs_filter_is_a() {
    let vs = value_set(
        "ValueSet: V\n* include codes from system http://snomed.info/sct where concept is-a #404684003\n",
    );
    let rule = &vs.components[0].value;
    assert!(!rule.filters.is_empty());
}

#[test]
fn parses_vs_direct_concept_includes() {
    let vs = value_set("ValueSet: V\n* http://loinc.org#1234-5 \"Display\"\n");
    let rule = &vs.components[0].value;
    assert!(!rule.concepts.is_empty());
    assert_eq!(rule.concepts[0].code, "1234-5");
}

// ============================================================================
// Instances
// ============================================================================

#[test]
fn parses_instance_with_insert_rule() {
    let src = "Instance: ex\nInstanceOf: Patient\n* insert CommonInstanceRules\n";
    let FshEntity::Instance(i) = single_entity(src) else {
        panic!("expected instance");
    };
    assert!(matches!(
        i.rules[0].value,
        InstanceRule::Insert(ref ins) if ins.rule_set == "CommonInstanceRules"
    ));
}

#[test]
fn parses_instance_inline_instance_reference() {
    let src = "Instance: ex\nInstanceOf: Bundle\n* contained[+] = myInstance\n";
    let FshEntity::Instance(i) = single_entity(src) else {
        panic!("expected instance");
    };
    assert!(matches!(
        i.rules[0].value,
        InstanceRule::Assignment(ref a) if matches!(a.value, FshValue::InstanceRef(ref r) if r == "myInstance")
    ));
}

// ============================================================================
// Comments and trivia
// ============================================================================

#[test]
fn skips_line_comments() {
    let p = profile("// leading comment\nProfile: P\nParent: Patient\n// between\n* active 1..1\n");
    assert_eq!(p.rules.len(), 1);
}

#[test]
fn skips_block_comments() {
    let p = profile("/* block\ncomment */\nProfile: P\nParent: Patient\n* active 1..1\n");
    assert_eq!(p.rules.len(), 1);
}

#[test]
fn parse_error_reports_location() {
    let err = FshParser::parse("NotAnEntity: nope\n", "bad.fsh");
    assert!(err.is_err());
}
