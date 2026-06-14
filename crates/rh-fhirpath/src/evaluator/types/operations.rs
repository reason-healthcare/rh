//! Type operations for FHIRPath expressions (is, as, ofType)

use crate::ast::*;
use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType;

/// Type operations handler
pub struct TypeEvaluator;

/// FHIR primitive type hierarchy for `is` (inheritance-aware) checks.
///
/// ```text
/// string <- code, id, markdown, base64Binary
/// uri    <- url, oid, canonical
/// integer <- unsignedInt <- positiveInt
/// ```
fn fhir_supertypes(t: FhirPrimitiveType) -> &'static [FhirPrimitiveType] {
    use FhirPrimitiveType::*;
    match t {
        // Direct subtypes of String
        Code => &[String],
        Id => &[String],
        Markdown => &[String],
        Base64Binary => &[String],
        // Direct subtypes of Uri
        Url => &[Uri, String],
        Oid => &[Uri, String],
        Canonical => &[Uri, String],
        // Uri is a subtype of String
        Uri => &[String],
        // Direct subtypes of Integer
        UnsignedInt => &[Integer],
        PositiveInt => &[UnsignedInt, Integer],
        // Root types have no supertypes in the FHIR hierarchy
        Boolean | String | Integer | Decimal | Date | DateTime | Instant | Time => &[],
    }
}

/// Convert a type name string to FhirPrimitiveType.
fn fhir_type_from_name(name: &str) -> Option<FhirPrimitiveType> {
    use FhirPrimitiveType::*;
    match name {
        "boolean" => Some(Boolean),
        "integer" => Some(Integer),
        "decimal" => Some(Decimal),
        "string" => Some(String),
        "uri" => Some(Uri),
        "url" => Some(Url),
        "canonical" => Some(Canonical),
        "oid" => Some(Oid),
        "id" => Some(Id),
        "code" => Some(Code),
        "markdown" => Some(Markdown),
        "base64binary" => Some(Base64Binary),
        "unsignedint" => Some(UnsignedInt),
        "positiveint" => Some(PositiveInt),
        "date" => Some(Date),
        "datetime" => Some(DateTime),
        "instant" => Some(Instant),
        "time" => Some(Time),
        _ => None,
    }
}

/// Check whether `declared` is the same as or a subtype of `target`, following
/// the FHIR primitive type hierarchy. E.g. `Code` IS-A `String`, but `String`
/// is NOT `Code`.
fn is_subtype_of(declared: FhirPrimitiveType, target: FhirPrimitiveType) -> bool {
    if declared == target {
        return true;
    }
    for &supertype in fhir_supertypes(declared) {
        if is_subtype_of(supertype, target) {
            return true;
        }
    }
    false
}

impl TypeEvaluator {
    /// Evaluate type operation (is, as)
    pub fn evaluate_type_operation(
        left: &FhirPathValue,
        operator: &TypeOperator,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        match operator {
            TypeOperator::Is => Self::evaluate_is(left, type_specifier),
            TypeOperator::As => Self::evaluate_as(left, type_specifier),
        }
    }

    /// Evaluate 'is' operation — inheritance-aware type testing.
    fn evaluate_is(
        value: &FhirPathValue,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        let type_name = Self::get_type_name(type_specifier);
        let matches = Self::value_is_type(value, &type_name)?;
        Ok(FhirPathValue::Boolean(matches))
    }

    /// Evaluate 'as' operation — exact declared-type match.
    fn evaluate_as(
        value: &FhirPathValue,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        let type_name = Self::get_type_name(type_specifier);

        match value {
            FhirPathValue::Collection(items) => {
                let mut filtered_items = Vec::new();
                for item in items {
                    if Self::value_is_exact_type(item, &type_name)? {
                        filtered_items.push(item.clone());
                    }
                }

                if filtered_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Collection(filtered_items))
                }
            }
            _ => {
                if Self::value_is_exact_type(value, &type_name)? {
                    Ok(value.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
    }

    /// Convert TypeSpecifier to a simple type name string
    fn get_type_name(type_specifier: &TypeSpecifier) -> String {
        type_specifier.qualified_name.join(".")
    }

    /// Evaluate 'ofType' operation — exact declared-type filter.
    pub fn evaluate_of_type(
        value: &FhirPathValue,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        let type_name = Self::get_type_name(type_specifier);

        match value {
            FhirPathValue::Collection(items) => {
                let mut filtered_items = Vec::new();
                for item in items {
                    if Self::value_is_exact_type(item, &type_name)? {
                        filtered_items.push(item.clone());
                    }
                }

                if filtered_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Collection(filtered_items))
                }
            }
            _ => {
                if Self::value_is_exact_type(value, &type_name)? {
                    Ok(value.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
    }

    /// Check if a value's type `is` the specified type, following FHIR
    /// type hierarchy (inheritance-aware). E.g. `code IS-A string` = true.
    fn value_is_type(value: &FhirPathValue, type_name: &str) -> FhirPathResult<bool> {
        let lookup = type_name.to_lowercase();
        match lookup.as_str() {
            "boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "string" => {
                // TypedString(Code|Id|Uri|...) IS-A String, plain String IS String
                Ok(value.is_string_like())
            }
            "integer" => Ok(matches!(value, FhirPathValue::Integer(_) | FhirPathValue::Long(_))),
            "number" | "decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "datetime" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            // FHIR primitive string subtypes — must have matching provenance
            "code" | "id" | "uri" | "url" | "canonical" | "oid" | "markdown" | "base64binary" => {
                if let Some(target_type) = fhir_type_from_name(&lookup) {
                    if let Some(declared) = value.fhir_primitive_type() {
                        return Ok(is_subtype_of(declared, target_type));
                    }
                    // Untyped string value: only matches 'string', not subtypes
                    return Ok(false);
                }
                Ok(false)
            }

            "collection" => Ok(matches!(value, FhirPathValue::Collection(_))),
            "object" | "resource" => Ok(matches!(value, FhirPathValue::Object(_))),

            // System types
            "system.boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "system.string" => Ok(value.is_string_like()),
            "system.integer" => Ok(matches!(value, FhirPathValue::Integer(_) | FhirPathValue::Long(_))),
            "system.decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "system.date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "system.datetime" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "system.time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "system.quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            // FHIR-qualified types
            "fhir.boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "fhir.integer" | "fhir.unsignedint" | "fhir.positiveint" => {
                Ok(matches!(value, FhirPathValue::Integer(_) | FhirPathValue::Long(_)))
            }
            "fhir.decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "fhir.string" | "fhir.code" | "fhir.uri" | "fhir.url" | "fhir.canonical"
            | "fhir.oid" | "fhir.uuid" | "fhir.id" | "fhir.markdown" | "fhir.base64binary" => {
                Ok(value.is_string_like())
            }
            "fhir.date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "fhir.datetime" | "fhir.instant" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "fhir.time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "fhir.quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            // Handle collections recursively
            lookup if lookup.ends_with("[]") => {
                let element_type = &lookup[..lookup.len() - 2];
                match value {
                    FhirPathValue::Collection(items) => {
                        for item in items {
                            if !Self::value_is_type(item, element_type)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }

            // Fallback: FHIR resource types
            _ => {
                let resource_name = lookup.strip_prefix("fhir.").unwrap_or(&lookup);
                if let FhirPathValue::Object(obj) = value {
                    if let Some(resource_type) = obj.get("resourceType") {
                        if let Some(resource_type_str) = resource_type.as_str() {
                            return Ok(resource_type_str.eq_ignore_ascii_case(resource_name));
                        }
                    }
                }
                Ok(false)
            }
        }
    }

    /// Check if a value matches a type by **exact declared type** (for `as`/`ofType`).
    /// Unlike `is`, this does NOT follow the type hierarchy: a `code` value
    /// does NOT match `string` — only `code` matches.
    fn value_is_exact_type(value: &FhirPathValue, type_name: &str) -> FhirPathResult<bool> {
        let lookup = type_name.to_lowercase();
        match lookup.as_str() {
            "boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "string" => {
                // Exact match: only plain String or TypedString with fhir_type=String
                match value {
                    FhirPathValue::String(_) => Ok(true),
                    FhirPathValue::TypedString { fhir_type, .. } => Ok(*fhir_type == FhirPrimitiveType::String),
                    _ => Ok(false),
                }
            }
            "integer" => Ok(matches!(value, FhirPathValue::Integer(_) | FhirPathValue::Long(_))),
            "number" | "decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "datetime" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            // FHIR primitive string subtypes — exact provenance match
            "code" | "id" | "uri" | "url" | "canonical" | "oid" | "markdown" | "base64binary" => {
                if let Some(target_type) = fhir_type_from_name(&lookup) {
                    return Ok(value.fhir_primitive_type() == Some(target_type));
                }
                Ok(false)
            }

            "collection" => Ok(matches!(value, FhirPathValue::Collection(_))),
            "object" | "resource" => Ok(matches!(value, FhirPathValue::Object(_))),

            "system.boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "system.string" => Ok(value.is_string_like()),
            "system.integer" => Ok(matches!(value, FhirPathValue::Integer(_) | FhirPathValue::Long(_))),
            "system.decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "system.date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "system.datetime" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "system.time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "system.quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            "fhir.boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "fhir.integer" | "fhir.unsignedint" | "fhir.positiveint" => {
                Ok(matches!(value, FhirPathValue::Integer(_) | FhirPathValue::Long(_)))
            }
            "fhir.decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "fhir.string" | "fhir.code" | "fhir.uri" | "fhir.url" | "fhir.canonical"
            | "fhir.oid" | "fhir.uuid" | "fhir.id" | "fhir.markdown" | "fhir.base64binary" => {
                Ok(value.is_string_like())
            }
            "fhir.date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "fhir.datetime" | "fhir.instant" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "fhir.time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "fhir.quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            lookup if lookup.ends_with("[]") => {
                let element_type = &lookup[..lookup.len() - 2];
                match value {
                    FhirPathValue::Collection(items) => {
                        for item in items {
                            if !Self::value_is_exact_type(item, element_type)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }

            _ => {
                let resource_name = lookup.strip_prefix("fhir.").unwrap_or(&lookup);
                if let FhirPathValue::Object(obj) = value {
                    if let Some(resource_type) = obj.get("resourceType") {
                        if let Some(resource_type_str) = resource_type.as_str() {
                            return Ok(resource_type_str.eq_ignore_ascii_case(resource_name));
                        }
                    }
                }
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_operator_basic_types() {
        let boolean_val = FhirPathValue::Boolean(true);
        let boolean_type = TypeSpecifier {
            qualified_name: vec!["Boolean".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&boolean_val, &boolean_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let string_val = FhirPathValue::String("hello".to_string());
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&string_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let result = TypeEvaluator::evaluate_is(&boolean_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_is_operator_system_types() {
        let boolean_val = FhirPathValue::Boolean(true);
        let system_boolean_type = TypeSpecifier {
            qualified_name: vec!["System".to_string(), "Boolean".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&boolean_val, &system_boolean_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_is_fhir_type_hierarchy() {
        // code IS-A string (inheritance) = true
        let code_val = FhirPathValue::TypedString {
            value: "male".to_string(),
            fhir_type: FhirPrimitiveType::Code,
        };
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&code_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // code IS code = true (exact match)
        let code_type = TypeSpecifier {
            qualified_name: vec!["Code".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&code_val, &code_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // code IS id = false (no inheritance)
        let id_type = TypeSpecifier {
            qualified_name: vec!["Id".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&code_val, &id_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        // plain string IS code = false (plain string has no provenance)
        let plain_string_val = FhirPathValue::String("hello".to_string());
        let result = TypeEvaluator::evaluate_is(&plain_string_val, &code_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        // url IS-A uri (direct subtype)
        let url_val = FhirPathValue::TypedString {
            value: "http://example.com".to_string(),
            fhir_type: FhirPrimitiveType::Url,
        };
        let uri_type = TypeSpecifier {
            qualified_name: vec!["Uri".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&url_val, &uri_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // url IS-A string (transitive: url → uri → string)
        let result = TypeEvaluator::evaluate_is(&url_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_as_exact_type_match() {
        // code.as(code) = value
        let code_val = FhirPathValue::TypedString {
            value: "male".to_string(),
            fhir_type: FhirPrimitiveType::Code,
        };
        let code_type = TypeSpecifier {
            qualified_name: vec!["Code".to_string()],
        };
        let result = TypeEvaluator::evaluate_as(&code_val, &code_type).unwrap();
        assert_eq!(result, code_val);

        // code.as(string) = empty (exact type mismatch)
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let result = TypeEvaluator::evaluate_as(&code_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);

        // code.as(id) = empty
        let id_type = TypeSpecifier {
            qualified_name: vec!["Id".to_string()],
        };
        let result = TypeEvaluator::evaluate_as(&code_val, &id_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }

    #[test]
    fn test_as_operator() {
        let string_val = FhirPathValue::String("hello".to_string());
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let result = TypeEvaluator::evaluate_as(&string_val, &string_type).unwrap();
        assert_eq!(result, string_val);

        let boolean_val = FhirPathValue::Boolean(true);
        let result = TypeEvaluator::evaluate_as(&boolean_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }

    #[test]
    fn test_numeric_types() {
        let integer_val = FhirPathValue::Integer(42);
        let number_val = FhirPathValue::Number(std::f64::consts::PI);

        let integer_type = TypeSpecifier {
            qualified_name: vec!["Integer".to_string()],
        };
        let decimal_type = TypeSpecifier {
            qualified_name: vec!["Decimal".to_string()],
        };

        let result = TypeEvaluator::evaluate_is(&integer_val, &integer_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let result = TypeEvaluator::evaluate_is(&integer_val, &decimal_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        let result = TypeEvaluator::evaluate_is(&number_val, &decimal_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_of_type_basic_filtering() {
        let mixed_collection = FhirPathValue::Collection(vec![
            FhirPathValue::String("hello".to_string()),
            FhirPathValue::Integer(42),
            FhirPathValue::Boolean(true),
            FhirPathValue::String("world".to_string()),
        ]);

        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };

        let result = TypeEvaluator::evaluate_of_type(&mixed_collection, &string_type).unwrap();

        if let FhirPathValue::Collection(items) = result {
            assert_eq!(items.len(), 2);
            assert!(matches!(items[0], FhirPathValue::String(_)));
            assert!(matches!(items[1], FhirPathValue::String(_)));
        } else {
            panic!("Expected collection result");
        }
    }

    #[test]
    fn test_of_type_resource_filtering() {
        use serde_json::json;

        let resources = FhirPathValue::Collection(vec![
            FhirPathValue::Object(json!({
                "resourceType": "Patient",
                "id": "patient1"
            })),
            FhirPathValue::Object(json!({
                "resourceType": "Observation",
                "id": "obs1"
            })),
            FhirPathValue::Object(json!({
                "resourceType": "Patient",
                "id": "patient2"
            })),
        ]);

        let patient_type = TypeSpecifier {
            qualified_name: vec!["Patient".to_string()],
        };

        let result = TypeEvaluator::evaluate_of_type(&resources, &patient_type).unwrap();

        if let FhirPathValue::Collection(items) = result {
            assert_eq!(items.len(), 2);
            for item in items {
                if let FhirPathValue::Object(obj) = item {
                    assert_eq!(
                        obj.get("resourceType").unwrap().as_str().unwrap(),
                        "Patient"
                    );
                } else {
                    panic!("Expected object result");
                }
            }
        } else {
            panic!("Expected collection result");
        }
    }

    #[test]
    fn test_of_type_single_item() {
        let string_val = FhirPathValue::String("hello".to_string());
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let integer_type = TypeSpecifier {
            qualified_name: vec!["Integer".to_string()],
        };

        let result = TypeEvaluator::evaluate_of_type(&string_val, &string_type).unwrap();
        assert_eq!(result, string_val);

        let result = TypeEvaluator::evaluate_of_type(&string_val, &integer_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }

    #[test]
    fn test_of_type_empty_result() {
        let integer_collection = FhirPathValue::Collection(vec![
            FhirPathValue::Integer(1),
            FhirPathValue::Integer(2),
            FhirPathValue::Integer(3),
        ]);

        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };

        let result = TypeEvaluator::evaluate_of_type(&integer_collection, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }
}