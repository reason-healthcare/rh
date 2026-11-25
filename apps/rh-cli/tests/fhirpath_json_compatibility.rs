use rh_cql::elm::{Expression, Library, StatementDef};
use rh_hl7_fhir_r4_core::datatypes::extension::Extension;

#[test]
fn nested_r4_extensions_deserialize_numeric_values() {
    let extension: Extension = serde_json::from_str(
        r#"{
            "url": "urn:parent",
            "extension": [
                {"url": "urn:decimal", "valueDecimal": 1.25},
                {"url": "urn:integer", "valueInteger": 12}
            ]
        }"#,
    )
    .unwrap();

    let children = extension.base.extension;
    assert_eq!(children.len(), 2);
    assert_eq!(children[0].value_decimal, Some(1.25));
    assert_eq!(children[1].value_integer, Some(12));
}

#[test]
fn cql_elm_library_deserializes_quantity_value() {
    let library: Library = serde_json::from_str(
        r#"{
            "identifier": {"id": "NumericExample"},
            "statements": {
                "def": [{
                    "type": "ExpressionDef",
                    "name": "Weight",
                    "expression": {"type": "Quantity", "value": 1.250, "unit": "kg"}
                }]
            }
        }"#,
    )
    .unwrap();

    let definitions = library.statements.unwrap().defs;
    let [StatementDef::Expression(definition)] = definitions.as_slice() else {
        panic!("expected one ELM expression definition");
    };
    let Some(Expression::Quantity(quantity)) = definition.expression.as_deref() else {
        panic!("expected an ELM Quantity");
    };
    assert_eq!(quantity.value, Some(1.25));
    assert_eq!(quantity.unit.as_deref(), Some("kg"));
}
