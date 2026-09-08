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
            None => CqlValue::Decimal(number.as_f64().unwrap_or_default()),
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
        CqlValue::Long(integer) => i64::try_from(*integer).map_or_else(
            |_| Value::String(integer.to_string()),
            |integer| Value::from(integer),
        ),
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
}

impl FhirDataProvider {
    pub fn from_bundle(bundle: &Value) -> Self {
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
        Ok(self
            .resources
            .get(&normalize_data_type(data_type))
            .cloned()
            .unwrap_or_default())
    }
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
                    "status": "final"
                }
            }]
        });
        let provider = FhirDataProvider::from_bundle(&bundle);

        for data_type in ["{http://hl7.org/fhir}Observation", "Observation"] {
            let resources = provider
                .retrieve(None, data_type, None, None, None, None)
                .expect("retrieve should succeed");
            assert_eq!(resources.len(), 1);
        }
    }
}
