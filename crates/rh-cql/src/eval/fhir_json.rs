//! FHIR JSON conversion helpers for the CQL runtime.
//!
//! FHIR JSON represents choice elements with suffixed property names such as
//! `valueBoolean` and `performedDateTime`. The FHIR CQL model exposes those
//! elements through the base choice property (`value`, `performed`) and relies
//! on casts like `O.value as FHIR.boolean` to select the concrete type.

use std::collections::BTreeMap;

use rh_hl7_fhir_r4_core::metadata::{self, FhirFieldType};
use serde_json::Value as JsonValue;

use super::value::Value;

const FHIR_TYPE_SUFFIXES: &[&str] = &[
    "Quantity",
    "Age",
    "Distance",
    "Duration",
    "Count",
    "Money",
    "Boolean",
    "Integer",
    "UnsignedInt",
    "PositiveInt",
    "Decimal",
    "String",
    "Code",
    "Uri",
    "Uuid",
    "Url",
    "Canonical",
    "Oid",
    "Id",
    "Markdown",
    "Base64Binary",
    "Date",
    "DateTime",
    "Instant",
    "Time",
    "Reference",
    "Identifier",
    "Annotation",
    "Attachment",
    "CodeableConcept",
    "Coding",
    "Ratio",
    "SampledData",
    "HumanName",
    "Address",
    "ContactPoint",
    "Period",
    "Meta",
    "Narrative",
    "Dosage",
];

/// Convert arbitrary JSON into the CQL runtime value model.
pub fn json_to_cql_value(value: JsonValue) -> Value {
    json_to_cql_value_for_type(value, None)
}

/// Convert a FHIR JSON resource into the CQL runtime value model.
///
/// Choice fields are exposed both by their original JSON property name and by
/// their FHIR model base property. For example, an `Observation` containing
/// `valueBoolean` will produce tuple fields named `valueBoolean` and `value`.
pub fn fhir_resource_json_to_cql_value(resource: JsonValue) -> Value {
    let resource_type = resource
        .get("resourceType")
        .and_then(JsonValue::as_str)
        .map(str::to_string);
    json_to_cql_value_for_type(resource, resource_type.as_deref())
}

fn json_to_cql_value_for_type(value: JsonValue, fhir_type: Option<&str>) -> Value {
    match value {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(value) => Value::Boolean(value),
        JsonValue::Number(value) => {
            if let Some(integer) = value.as_i64() {
                Value::Integer(integer)
            } else {
                Value::Decimal(value.as_f64().unwrap_or_default())
            }
        }
        JsonValue::String(value) => Value::String(value),
        JsonValue::Array(items) => Value::List(
            items
                .into_iter()
                .map(|item| json_to_cql_value_for_type(item, fhir_type))
                .collect(),
        ),
        JsonValue::Object(map) => {
            let object_type = map
                .get("resourceType")
                .and_then(JsonValue::as_str)
                .or(fhir_type)
                .map(str::to_string);
            let mut fields = BTreeMap::new();
            let mut choice_aliases = Vec::new();

            for (key, raw_value) in map {
                let choice = object_type
                    .as_deref()
                    .and_then(|type_name| choice_base_and_suffix(type_name, &key));
                let child_type = choice
                    .and_then(|(_, suffix)| fhir_type_for_choice_suffix(suffix))
                    .or_else(|| {
                        object_type
                            .as_deref()
                            .and_then(|type_name| child_type_for_field(type_name, &key))
                    });
                let converted = json_to_cql_value_for_type(raw_value, child_type);

                if let Some((base, _)) = choice {
                    choice_aliases.push((base.to_string(), converted.clone()));
                }

                fields.insert(key, converted);
            }

            for (base, value) in choice_aliases {
                fields.entry(base).or_insert(value);
            }

            Value::Tuple(fields)
        }
    }
}

fn choice_base_and_suffix<'a>(
    type_name: &str,
    field_name: &'a str,
) -> Option<(&'a str, &'static str)> {
    for suffix in FHIR_TYPE_SUFFIXES {
        if field_name.ends_with(suffix) && field_name.len() > suffix.len() {
            let split_at = field_name.len() - suffix.len();
            if field_name
                .as_bytes()
                .get(split_at)
                .is_some_and(|&c| c.is_ascii_uppercase())
            {
                let base = &field_name[..split_at];
                let choice_key = format!("{base}[x]");
                if metadata::get_field_info(type_name, &choice_key)
                    .is_some_and(|info| info.is_choice_type)
                {
                    return Some((base, suffix));
                }
            }
        }
    }
    None
}

fn child_type_for_field(type_name: &str, field_name: &str) -> Option<&'static str> {
    let field_info = metadata::get_field_info(type_name, field_name)?;
    match field_info.field_type {
        FhirFieldType::Complex(type_name) | FhirFieldType::BackboneElement(type_name) => {
            Some(type_name)
        }
        FhirFieldType::Reference => Some("Reference"),
        FhirFieldType::Primitive(_) => None,
    }
}

fn fhir_type_for_choice_suffix(suffix: &'static str) -> Option<&'static str> {
    match suffix {
        "Boolean" | "Integer" | "UnsignedInt" | "PositiveInt" | "Decimal" | "String" | "Code"
        | "Uri" | "Uuid" | "Url" | "Canonical" | "Oid" | "Id" | "Markdown" | "Base64Binary"
        | "Date" | "DateTime" | "Instant" | "Time" => None,
        other => Some(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn observation_value_boolean_is_exposed_as_choice_base() {
        let value = fhir_resource_json_to_cql_value(json!({
            "resourceType": "Observation",
            "id": "obs-1",
            "valueBoolean": true
        }));

        let Value::Tuple(fields) = value else {
            panic!("expected tuple");
        };
        assert_eq!(fields.get("valueBoolean"), Some(&Value::Boolean(true)));
        assert_eq!(fields.get("value"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn procedure_and_condition_choice_date_times_use_base_properties() {
        let procedure = fhir_resource_json_to_cql_value(json!({
            "resourceType": "Procedure",
            "performedDateTime": "2025-01-01T00:00:00Z"
        }));
        let condition = fhir_resource_json_to_cql_value(json!({
            "resourceType": "Condition",
            "onsetDateTime": "2025-01-01T00:00:00Z"
        }));

        let Value::Tuple(procedure_fields) = procedure else {
            panic!("expected Procedure tuple");
        };
        let Value::Tuple(condition_fields) = condition else {
            panic!("expected Condition tuple");
        };
        assert_eq!(
            procedure_fields.get("performed"),
            Some(&Value::String("2025-01-01T00:00:00Z".to_string()))
        );
        assert_eq!(
            condition_fields.get("onset"),
            Some(&Value::String("2025-01-01T00:00:00Z".to_string()))
        );
    }

    #[test]
    fn regular_date_suffix_fields_are_not_treated_as_choices() {
        let value = fhir_resource_json_to_cql_value(json!({
            "resourceType": "Patient",
            "birthDate": "1970-01-01"
        }));

        let Value::Tuple(fields) = value else {
            panic!("expected tuple");
        };
        assert_eq!(
            fields.get("birthDate"),
            Some(&Value::String("1970-01-01".to_string()))
        );
        assert!(!fields.contains_key("birth"));
    }
}
