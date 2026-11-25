use super::FhirPathEvaluator;
use crate::ast::{Expression, TypeSpecifier};
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::metadata::{FhirModel, FhirNode};
use crate::evaluator::{EvaluationContext, FhirPathValue};
use crate::json::JsonData;
use serde_json::Value;
use std::collections::BTreeMap;

impl FhirPathEvaluator {
    pub(super) fn evaluate_instance_selector(
        &self,
        type_specifier: &TypeSpecifier,
        elements: &[(String, Expression)],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let focus = context.current_focus();
        focus.singleton("Instance selector input")?;
        if focus.is_empty() && context.this_value.is_some() {
            return Ok(FhirPathValue::Empty);
        }

        let model = context.model;
        let type_name = match type_specifier.qualified_name.as_slice() {
            [name] => Some(name),
            [namespace, name] if namespace == "FHIR" => Some(name),
            _ => None,
        }
        .filter(|name| model.is_type(name))
        .ok_or_else(|| {
            FhirPathError::type_error(format!("Unknown constructor type {type_specifier}"))
        })?;

        let is_primitive = type_name.starts_with(|c: char| c.is_ascii_lowercase());
        let mut fields = BTreeMap::new();
        let mut assigned = std::collections::HashSet::new();
        for (name, expression) in elements {
            if !assigned.insert(name) {
                return Err(FhirPathError::type_error(format!(
                    "Duplicate constructor field {type_specifier}.{name}"
                )));
            }
            let value = self.evaluate_expression(expression, context)?;
            let direct_type = model.field_type(type_name, name);
            let choice = model.field_type(type_name, &format!("{name}[x]")).is_some();
            if value.is_empty() && (direct_type.is_some() || choice) {
                continue;
            }

            let key = if direct_type.is_none() && choice {
                let item = value
                    .singleton("Choice constructor element")?
                    .ok_or_else(|| {
                        FhirPathError::type_error(
                            "Cannot infer constructor choice from an empty value",
                        )
                    })?;
                let item_type = constructor_value_type(item).ok_or_else(|| {
                    FhirPathError::type_error(format!(
                        "Cannot infer the type of {type_specifier}.{name}"
                    ))
                })?;
                model
                    .choice_key(type_name, name, item_type)
                    .ok_or_else(|| {
                        FhirPathError::type_error(format!(
                            "{item_type} cannot populate {type_specifier}.{name}"
                        ))
                    })?
            } else {
                name.clone()
            };
            let field_type = model.field_type(type_name, &key).ok_or_else(|| {
                FhirPathError::type_error(format!(
                    "Unknown constructor field {type_specifier}.{name}"
                ))
            })?;
            if fields.contains_key(&key) {
                return Err(FhirPathError::type_error(format!(
                    "Duplicate constructor field {type_specifier}.{key}"
                )));
            }

            let built = if model.field_is_repeating(type_name, &key) {
                let items = value
                    .items()
                    .iter()
                    .map(|item| constructor_json(item, field_type, model))
                    .collect::<FhirPathResult<Vec<_>>>()?;
                JsonData::array(items)
            } else {
                let Some(item) = value.singleton("Constructor element")? else {
                    continue;
                };
                constructor_json(item, field_type, model)?
            };
            fields.insert(key, built);
        }

        let node = if is_primitive {
            let value = fields
                .remove("value")
                .unwrap_or_else(|| JsonData::from_value(Value::Null));
            let metadata = (!fields.is_empty()).then(|| JsonData::object(fields));
            FhirNode::primitive_json(type_name, &value, metadata, model)
        } else {
            if model.is_subtype(type_name, "Resource") {
                fields.insert(
                    "resourceType".into(),
                    JsonData::from_value(Value::String(type_name.clone())),
                );
            }
            FhirNode::detached_json(&JsonData::object(fields), type_name, model)
        };
        Ok(FhirPathValue::Node(Box::new(node)))
    }
}

fn constructor_value_type(value: &FhirPathValue) -> Option<&str> {
    match value {
        FhirPathValue::Node(node) if node.fhir_type == "System.Decimal" => Some("decimal"),
        FhirPathValue::Node(node) if node.is_system_projection() => {
            constructor_value_type(&node.inner)
        }
        FhirPathValue::Node(node) => Some(&node.fhir_type),
        FhirPathValue::TypedObject { fhir_type, .. } => Some(fhir_type),
        FhirPathValue::Object(value) => value.get("resourceType").and_then(Value::as_str),
        FhirPathValue::String(_) | FhirPathValue::TypedString { .. } => Some("string"),
        FhirPathValue::Boolean(_) | FhirPathValue::TypedBoolean { .. } => Some("boolean"),
        FhirPathValue::Integer(_) => Some("integer"),
        FhirPathValue::Long(_) => Some("integer64"),
        FhirPathValue::Number(_) => Some("decimal"),
        FhirPathValue::Date(_) => Some("date"),
        FhirPathValue::DateTime(_) | FhirPathValue::TypedDateTime { .. } => Some("dateTime"),
        FhirPathValue::Time(_) => Some("time"),
        FhirPathValue::Quantity { .. } => Some("Quantity"),
        _ => None,
    }
}

fn constructor_json(
    item: &FhirPathValue,
    expected_type: &str,
    model: FhirModel,
) -> FhirPathResult<JsonData> {
    let data = item.json_data()?;
    let json = &data.value;
    let accepted = match expected_type {
        "integer64" if json.is_i64() => {
            return Ok(JsonData::from_value(Value::String(json.to_string())))
        }
        "integer64" => json
            .as_str()
            .is_some_and(|value| value.parse::<i64>().is_ok()),
        "boolean" => json.is_boolean(),
        "integer" => json.as_i64().is_some_and(|i| i32::try_from(i).is_ok()),
        "positiveInt" => json
            .as_i64()
            .is_some_and(|i| (1..=i64::from(i32::MAX)).contains(&i)),
        "unsignedInt" => json
            .as_i64()
            .is_some_and(|i| (0..=i64::from(i32::MAX)).contains(&i)),
        "decimal" => {
            json.is_number() || data.numbers.as_ref().is_some_and(|n| n.number().is_some())
        }
        name if name.starts_with(|c: char| c.is_ascii_lowercase()) => json.is_string(),
        name => constructor_value_type(item).is_some_and(|actual| model.is_subtype(actual, name)),
    };
    if !accepted {
        return Err(FhirPathError::type_error(format!(
            "Cannot populate {expected_type} with {item:?}"
        )));
    }
    Ok(data)
}
