use super::extract_qualified_type_name;
use crate::ast::{Expression, Invocation, Term, TypeOperator};
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::metadata::{self, FhirModel, FhirNode};
use crate::evaluator::FhirPathValue;
use crate::json::{number_text, parse_decimal, JsonNumbers};
use serde_json::Value;
use std::sync::Arc;

pub(super) fn validate_cast_member(
    expression: &Expression,
    invocation: &Invocation,
    model: FhirModel,
) -> FhirPathResult<()> {
    let Invocation::Member(member) = invocation else {
        return Ok(());
    };
    let Some(ty) = declared_cast_type(expression) else {
        return Ok(());
    };
    let ty = ty.strip_prefix("FHIR.").unwrap_or(&ty);
    if model.is_type(ty)
        && model.field_type(ty, member).is_none()
        && model.field_type(ty, &format!("{member}[x]")).is_none()
    {
        return Err(FhirPathError::type_error(format!(
            "Unknown member '{member}' of FHIR.{ty}"
        )));
    }
    Ok(())
}

/// The result type of an explicit cast remains known when the cast has no matches.
fn declared_cast_type(expression: &Expression) -> Option<String> {
    match expression {
        Expression::Term(Term::Parenthesized(inner)) | Expression::Indexer { left: inner, .. } => {
            declared_cast_type(inner)
        }
        Expression::Type {
            operator: TypeOperator::As,
            type_specifier,
            ..
        } => Some(type_specifier.qualified_name.join(".")),
        Expression::Invocation {
            invocation: Invocation::Function { name, parameters },
            ..
        }
        | Expression::Term(Term::Invocation(Invocation::Function { name, parameters }))
            if matches!(name.as_str(), "as" | "ofType") =>
        {
            extract_qualified_type_name(parameters.first()?)
        }
        _ => None,
    }
}

pub(super) fn navigate_member(
    target: &FhirPathValue,
    member: &str,
    model: FhirModel,
    strict: bool,
) -> FhirPathResult<FhirPathValue> {
    match target {
        FhirPathValue::Node(node) if node.is_untyped_object() => navigate_json_member(
            target,
            &node.raw,
            node.numbers.as_deref(),
            member,
            model,
            strict,
        ),
        FhirPathValue::Node(node) => node.member(member, strict),
        FhirPathValue::TypedObject { value, fhir_type } if model.is_type(fhir_type) => {
            FhirNode::detached(value, fhir_type, model).member(member, strict)
        }
        FhirPathValue::Object(object) | FhirPathValue::TypedObject { value: object, .. } => {
            navigate_json_member(target, object, None, member, model, strict)
        }
        FhirPathValue::FhirPrimitive { inner, .. } => navigate_member(inner, member, model, strict),
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
            let mut result = Vec::new();
            for item in items {
                result.extend(navigate_member(item, member, model, strict)?.into_items());
            }
            Ok(FhirPathValue::from_items(result))
        }
        FhirPathValue::Quantity { value, unit } => match member {
            "value" => Ok(FhirPathValue::Number(*value)),
            "unit" => Ok(unit
                .clone()
                .map(FhirPathValue::String)
                .unwrap_or(FhirPathValue::Empty)),
            _ => Ok(FhirPathValue::Empty),
        },
        _ => Ok(FhirPathValue::Empty),
    }
}

fn navigate_json_member(
    target: &FhirPathValue,
    object: &Value,
    numbers: Option<&JsonNumbers>,
    member: &str,
    model: FhirModel,
    strict: bool,
) -> FhirPathResult<FhirPathValue> {
    let resource_type = object.get("resourceType").and_then(Value::as_str);
    if resource_type == Some(member) {
        return Ok(target.clone());
    }

    if let Some(value) = object.get(member) {
        if strict {
            if let Some(base) = choice_type_base(resource_type, member) {
                return Err(FhirPathError::evaluation_error(format!(
                    "Invalid choice-type access: use '{base}[x]' instead of '{member}'"
                )));
            }
        }
        let field_numbers = numbers.and_then(|numbers| numbers.field(member));
        let result = type_json_member(value, resource_type, member, field_numbers, model);
        if !matches!(
            result.system_value(),
            FhirPathValue::Object(_) | FhirPathValue::Collection(_)
        ) {
            if let Some(extensions) = object.get(format!("_{member}")).and_then(Value::as_object) {
                return Ok(FhirPathValue::FhirPrimitive {
                    inner: Box::new(result),
                    extensions: extensions.clone(),
                });
            }
        }
        return Ok(result);
    }

    let choice = object.as_object().and_then(|fields| {
        fields.iter().find(|(key, _)| {
            key.starts_with(member)
                && key.len() > member.len()
                && key
                    .chars()
                    .nth(member.len())
                    .is_some_and(|c| c.is_uppercase())
        })
    });
    match choice {
        Some((key, _)) if strict => Err(FhirPathError::evaluation_error(format!(
            "Invalid choice-type access: use '{member}[x]' instead of '{key}' in strict mode"
        ))),
        Some((key, value)) => {
            let field_numbers = numbers.and_then(|numbers| numbers.field(key));
            Ok(
                construct_typed_choice_value(&key[member.len()..], value, field_numbers)
                    .unwrap_or_else(|| {
                        type_json_member(value, resource_type, key, field_numbers, model)
                    }),
            )
        }
        None => Ok(FhirPathValue::Empty),
    }
}

fn type_json_member(
    value: &Value,
    resource_type: Option<&str>,
    member: &str,
    numbers: Option<&Arc<JsonNumbers>>,
    model: FhirModel,
) -> FhirPathValue {
    if numbers.is_some() {
        return FhirPathValue::from_json_with_numbers(value, numbers, model);
    }
    match resource_type {
        Some(resource_type) => metadata::apply_fhir_typing(value, resource_type, member),
        None => FhirPathValue::from_json(value),
    }
}

/// Check if `member` is a FHIR choice-type suffixed key (e.g. `valueQuantity`
/// is `value` + `Quantity`). Returns the base name (`value`) if so.
///
/// Uses the FHIR metadata registry to distinguish real element names like
/// `birthDate` (registered) from suffixed choice-type keys like
/// `valueQuantity` (not registered; the real element is `value[x]`).
fn choice_type_base<'a>(resource_type: Option<&'a str>, member: &'a str) -> Option<&'a str> {
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
    for suffix in FHIR_TYPE_SUFFIXES {
        if member.ends_with(suffix) && member.len() > suffix.len() {
            let split_at = member.len() - suffix.len();
            if member
                .as_bytes()
                .get(split_at)
                .is_some_and(|&c| c.is_ascii_uppercase())
            {
                let base = &member[..split_at];
                // Only flag as choice-type if the base + "[x]" element exists
                // in the FHIR metadata for the resource. This distinguishes
                // `valueQuantity` (from `value[x]`) from `linkId` (a regular
                // BackboneElement sub-field) and `birthDate` (a regular date).
                if let Some(rt) = resource_type {
                    let choice_key = format!("{}[x]", base);
                    if crate::evaluator::metadata::get_field_type(rt, &choice_key).is_some() {
                        return Some(base);
                    }
                }
                // If no resource type metadata is available, we can't
                // reliably detect choice-type suffixed keys, so we don't
                // flag them.
            }
        }
    }
    None
}

/// Build a typed `FhirPathValue` for a FHIR choice-type field based on the
/// type suffix of its JSON key (e.g. `valueQuantity` → suffix "Quantity").
///
/// Returns `Some(typed)` for suffixes that map onto a concrete FhirPathValue
/// variant and whose JSON shape matches; `None` to let the caller fall back
/// to metadata-driven typing.
fn construct_typed_choice_value(
    suffix: &str,
    value: &Value,
    numbers: Option<&Arc<JsonNumbers>>,
) -> Option<FhirPathValue> {
    match suffix {
        "Quantity" => {
            let obj = value.as_object()?;
            let amount_numbers = numbers
                .and_then(|numbers| numbers.field("value"))
                .map(Arc::as_ref);
            let literal = number_text(obj.get("value")?, amount_numbers)?;
            let amount = parse_decimal(&literal)?;
            let unit = obj
                .get("unit")
                .and_then(Value::as_str)
                .or_else(|| obj.get("code").and_then(Value::as_str))
                .map(str::to_string);
            Some(FhirPathValue::Quantity {
                value: amount,
                unit,
            })
        }
        // Quantity profiles: return as TypedObject so that `is Age`, `is Quantity`
        // (subtype), and member access (`.value`, `.unit`) all work correctly.
        "Age" | "Distance" | "Duration" | "Count" | "Money" => Some(FhirPathValue::TypedObject {
            value: value.clone(),
            fhir_type: suffix.to_string(),
        }),
        "Boolean" => value.as_bool().map(|b| FhirPathValue::TypedBoolean {
            value: b,
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Boolean,
        }),
        "Integer" | "UnsignedInt" | "PositiveInt" => value.as_i64().map(FhirPathValue::Integer),
        "Decimal" => {
            parse_decimal(&number_text(value, numbers.map(Arc::as_ref))?).map(FhirPathValue::Number)
        }
        "Date" => value.as_str().map(|s| FhirPathValue::Date(s.to_string())),
        "DateTime" | "Instant" => value.as_str().map(|s| FhirPathValue::TypedDateTime {
            value: s.to_string(),
            fhir_type: if suffix == "Instant" {
                rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Instant
            } else {
                rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::DateTime
            },
        }),
        "Time" => value
            .as_str()
            .map(|s| FhirPathValue::Time(s.strip_prefix('T').unwrap_or(s).to_string())),
        _ => {
            use rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType;
            let fhir_type = match suffix {
                "String" => FhirPrimitiveType::String,
                "Code" => FhirPrimitiveType::Code,
                "Uri" | "Uuid" => FhirPrimitiveType::Uri,
                "Url" => FhirPrimitiveType::Url,
                "Canonical" => FhirPrimitiveType::Canonical,
                "Oid" => FhirPrimitiveType::Oid,
                "Id" => FhirPrimitiveType::Id,
                "Markdown" => FhirPrimitiveType::Markdown,
                "Base64Binary" => FhirPrimitiveType::Base64Binary,
                _ => return None,
            };
            value.as_str().map(|text| FhirPathValue::TypedString {
                value: text.to_owned(),
                fhir_type,
            })
        }
    }
}
