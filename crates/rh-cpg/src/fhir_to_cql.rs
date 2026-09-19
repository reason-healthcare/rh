use std::collections::HashMap;

use rh_cql::eval::Value as CqlValue;
use rh_cql::eval::{CqlQuantity, EvalError};
use serde_json::Value;

pub fn fhir_to_cql_value(json: &Value) -> CqlValue {
    match json {
        Value::Null => CqlValue::Null,
        Value::Bool(boolean) => CqlValue::Boolean(*boolean),
        Value::Number(number) => match number.as_i64() {
            Some(integer) => CqlValue::Integer(integer),
            None => number
                .as_f64()
                .map(CqlValue::Decimal)
                // Preserve unrepresentable values rather than corrupting
                // them to 0.0 (for example, very large integers).
                .unwrap_or_else(|| CqlValue::String(number.to_string())),
        },
        Value::String(string) => CqlValue::String(string.clone()),
        Value::Array(items) => CqlValue::List(items.iter().map(fhir_to_cql_value).collect()),
        Value::Object(object) => CqlValue::Tuple(
            object
                .iter()
                .map(|(key, value)| (key.to_string(), fhir_to_cql_value(value)))
                .collect(),
        ),
    }
}

pub fn cql_value_to_json(value: &CqlValue) -> Value {
    match value {
        CqlValue::Null => Value::Null,
        CqlValue::Boolean(boolean) => Value::Bool(*boolean),
        CqlValue::Integer(integer) => Value::from(*integer),
        CqlValue::Long(integer) => {
            i64::try_from(*integer).map_or_else(|_| Value::String(integer.to_string()), Value::from)
        }
        CqlValue::Decimal(decimal) => {
            serde_json::Number::from_f64(*decimal).map_or(Value::Null, Value::Number)
        }
        CqlValue::String(string) => Value::String(string.clone()),
        CqlValue::Date(date) => Value::String(date.to_string()),
        CqlValue::DateTime(date_time) => Value::String(date_time.to_string()),
        CqlValue::Time(time) => Value::String(time.to_string()),
        CqlValue::Quantity(quantity) => {
            let mut object = serde_json::Map::new();
            object.insert("value".to_string(), cql_scalar_to_json(quantity.value));
            object.insert("unit".to_string(), Value::String(quantity.unit.clone()));
            Value::Object(object)
        }
        CqlValue::Ratio {
            numerator,
            denominator,
        } => {
            let mut object = serde_json::Map::new();
            object.insert("numerator".to_string(), cql_quantity_to_json(numerator));
            object.insert("denominator".to_string(), cql_quantity_to_json(denominator));
            Value::Object(object)
        }
        CqlValue::Code(code) => Value::String(code.code.clone()),
        CqlValue::Concept(concept) => {
            let mut object = serde_json::Map::new();
            object.insert(
                "codes".to_string(),
                Value::Array(
                    concept
                        .codes
                        .iter()
                        .map(|code| Value::String(code.code.clone()))
                        .collect(),
                ),
            );
            if let Some(display) = concept.display.as_ref() {
                object.insert("display".to_string(), Value::String(display.clone()));
            }
            Value::Object(object)
        }
        CqlValue::List(items) => Value::Array(items.iter().map(cql_value_to_json).collect()),
        CqlValue::Tuple(fields) => {
            let mut object = serde_json::Map::new();
            for (key, field) in fields {
                object.insert(key.clone(), cql_value_to_json(field));
            }
            Value::Object(object)
        }
        CqlValue::Interval { .. } => Value::Null,
    }
}

fn cql_quantity_to_json(quantity: &CqlQuantity) -> Value {
    let mut object = serde_json::Map::new();
    object.insert("value".to_string(), cql_scalar_to_json(quantity.value));
    object.insert("unit".to_string(), Value::String(quantity.unit.clone()));
    Value::Object(object)
}

fn cql_scalar_to_json(value: f64) -> Value {
    serde_json::Number::from_f64(value).map_or(Value::Null, Value::Number)
}

pub struct FhirDataProvider {
    resources: HashMap<String, Vec<CqlValue>>,
    subject: String,
}

impl FhirDataProvider {
    pub fn from_bundle(bundle: &Value, subject: impl Into<String>) -> Self {
        Self {
            resources: bundle
                .get("entry")
                .and_then(Value::as_array)
                .map(|entries| {
                    let mut resources: HashMap<String, Vec<CqlValue>> = HashMap::new();
                    for resource in entries
                        .iter()
                        .filter_map(|entry| entry.get("resource"))
                        .filter(|resource| resource.is_object())
                    {
                        if let Some(resource_type) =
                            resource.get("resourceType").and_then(Value::as_str)
                        {
                            resources
                                .entry(resource_type.to_string())
                                .or_default()
                                .push(fhir_to_cql_value(resource));
                        }
                    }
                    resources
                })
                .unwrap_or_default(),
            subject: subject.into(),
        }
    }
}

impl rh_cql::eval::DataProvider for FhirDataProvider {
    fn retrieve(
        &self,
        _context: Option<&str>,
        data_type: &str,
        _code_path: Option<&str>,
        _codes: Option<&CqlValue>,
        _date_path: Option<&str>,
        _date_range: Option<&CqlValue>,
    ) -> Result<Vec<CqlValue>, EvalError> {
        let resource_type = normalize_data_type(data_type);
        let candidates = self
            .resources
            .get(&resource_type)
            .cloned()
            .unwrap_or_default();
        filter_resources_for_subject(candidates, &resource_type, &self.subject)
    }
}

/// Apply the supported FHIR Patient-compartment relationships to CQL retrieves.
///
/// CPG expression evaluation is always invoked for one subject. Returning an
/// unscoped list would allow another patient's resource to make an applicability
/// condition true, so unsupported relationships fail closed.
fn filter_resources_for_subject(
    candidates: Vec<CqlValue>,
    resource_type: &str,
    subject: &str,
) -> Result<Vec<CqlValue>, EvalError> {
    let patient_id = subject
        .strip_prefix("Patient/")
        .filter(|id| !id.is_empty() && !id.contains('/'))
        .ok_or_else(|| {
            EvalError::RetrieveError(format!(
                "CPG CQL subject must be a relative Patient/<id> reference, got '{subject}'"
            ))
        })?;

    match resource_type {
        "Patient" => Ok(candidates
            .into_iter()
            .filter(|candidate| {
                tuple_scalar(candidate, "id")
                    .and_then(reference_value)
                    .is_some_and(|id| id == patient_id)
            })
            .collect()),
        "Encounter" | "Observation" | "QuestionnaireResponse" => candidates
            .into_iter()
            .map(|candidate| {
                let matches = tuple_scalar(&candidate, "subject")
                    .and_then(reference_value)
                    .map(|reference| matches_patient_reference(reference, subject))
                    .transpose()?
                    .unwrap_or(false);
                Ok((candidate, matches))
            })
            .filter_map(|candidate| match candidate {
                Ok((candidate, true)) => Some(Ok(candidate)),
                Ok((_, false)) => None,
                Err(error) => Some(Err(error)),
            })
            .collect(),
        _ => Err(EvalError::RetrieveError(format!(
            "Patient-context retrieve is unsupported for FHIR {resource_type}; a Patient-compartment relationship is required"
        ))),
    }
}

fn tuple_scalar<'a>(value: &'a CqlValue, field: &str) -> Option<&'a CqlValue> {
    match value {
        CqlValue::Tuple(fields) => fields.get(field),
        _ => None,
    }
}

fn reference_value(value: &CqlValue) -> Option<&str> {
    match value {
        CqlValue::String(value) => Some(value),
        CqlValue::Tuple(fields) => match fields.get("reference").or_else(|| fields.get("value")) {
            Some(CqlValue::String(value)) => Some(value),
            _ => None,
        },
        _ => None,
    }
}

fn matches_patient_reference(reference: &str, subject: &str) -> Result<bool, EvalError> {
    if reference == subject {
        return Ok(true);
    }
    if let Some(id) = reference.strip_prefix("Patient/") {
        if !id.is_empty() && !id.contains('/') {
            return Ok(false);
        }
        return Err(EvalError::RetrieveError(format!(
            "unsupported Patient reference form '{reference}'; use relative Patient/<id>"
        )));
    }
    if reference.contains("/Patient/") {
        return Err(EvalError::RetrieveError(format!(
            "unsupported Patient reference form '{reference}'; use relative Patient/<id>"
        )));
    }
    Ok(false)
}

fn normalize_data_type(data_type: &str) -> String {
    let data_type = data_type
        .strip_prefix('{')
        .and_then(|prefixed| prefixed.split_once('}'))
        .map(|(_, name)| name)
        .unwrap_or(data_type);

    data_type
        .split("FHIR.")
        .last()
        .unwrap_or(data_type)
        .to_string()
}

#[cfg(test)]
mod tests {
    use rh_cql::eval::{CqlDateTime, DataProvider};

    use super::*;
    use serde_json::json;

    #[test]
    fn converts_scalars_objects_and_arrays_round_trip() {
        let json = json!({
            "boolean": true,
            "integer": 42,
            "decimal": 4.5,
            "string": "value",
            "array": [1, "two"],
            "object": { "nested": true }
        });

        assert_eq!(cql_value_to_json(&fhir_to_cql_value(&json)), json);
    }

    #[test]
    fn converts_cql_datetime_to_iso_string() {
        let date_time = CqlDateTime {
            year: 2026,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: Some(0),
        };

        assert_eq!(
            cql_value_to_json(&CqlValue::DateTime(date_time)),
            json!("2026-01-01T00:00:00Z")
        );
    }

    #[test]
    fn retrieves_resources_by_qualified_and_plain_type() {
        let bundle = json!({
            "resourceType": "Bundle",
            "entry": [{
                "resource": {
                    "resourceType": "Observation",
                    "id": "observation-1",
                    "subject": {"reference": "Patient/123"},
                    "status": "final"
                }
            }]
        });
        let provider = FhirDataProvider::from_bundle(&bundle, "Patient/123");

        for data_type in ["{http://hl7.org/fhir}Observation", "Observation"] {
            let resources = provider
                .retrieve(None, data_type, None, None, None, None)
                .expect("retrieve should succeed");
            assert_eq!(resources.len(), 1);
        }
    }

    #[test]
    fn retrieves_only_resources_for_the_requested_patient() {
        let bundle = json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {
                    "resourceType": "Observation",
                    "id": "matching",
                    "subject": {"reference": "Patient/123"}
                }},
                {"resource": {
                    "resourceType": "Observation",
                    "id": "other",
                    "subject": {"reference": "Patient/999"}
                }}
            ]
        });
        let provider = FhirDataProvider::from_bundle(&bundle, "Patient/123");

        let resources = provider
            .retrieve(None, "Observation", None, None, None, None)
            .expect("patient-scoped retrieve should succeed");

        assert_eq!(resources.len(), 1);
        assert_eq!(
            tuple_scalar(&resources[0], "id").and_then(reference_value),
            Some("matching")
        );
    }

    #[test]
    fn unsupported_patient_relationship_fails_closed() {
        let bundle = json!({
            "resourceType": "Bundle",
            "entry": [{"resource": {
                "resourceType": "Condition",
                "id": "condition-1",
                "subject": {"reference": "Patient/123"}
            }}]
        });
        let provider = FhirDataProvider::from_bundle(&bundle, "Patient/123");

        let error = provider
            .retrieve(None, "Condition", None, None, None, None)
            .unwrap_err();

        assert!(
            matches!(error, EvalError::RetrieveError(message) if message.contains("unsupported for FHIR Condition"))
        );
    }
}
