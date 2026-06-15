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

/// FHIR complex type inheritance: returns true if `declared` is a subtype of `target`.
/// E.g. Age is a subtype of Quantity.
fn fhir_is_subtype_of(declared: &str, target: &str) -> bool {
    // FHIR quantity profiles: Age, Distance, Duration, Count, Money all extend Quantity
    let quantity_profiles = ["Age", "Distance", "Duration", "Count", "Money"];
    let lc_target = target.to_ascii_lowercase();
    if lc_target == "quantity" {
        return quantity_profiles
            .iter()
            .any(|p| p.eq_ignore_ascii_case(declared));
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TypeNamespace {
    System,
    Fhir,
    Unqualified,
}

fn split_type_name(type_name: &str) -> (TypeNamespace, &str, String) {
    if let Some(name) = type_name.strip_prefix("System.") {
        return (TypeNamespace::System, name, name.to_ascii_lowercase());
    }
    if let Some(name) = type_name.strip_prefix("FHIR.") {
        return (TypeNamespace::Fhir, name, name.to_ascii_lowercase());
    }
    if let Some(name) = type_name.strip_prefix("system.") {
        return (TypeNamespace::System, name, name.to_ascii_lowercase());
    }
    if let Some(name) = type_name.strip_prefix("fhir.") {
        return (TypeNamespace::Fhir, name, name.to_ascii_lowercase());
    }
    (
        TypeNamespace::Unqualified,
        type_name,
        type_name.to_ascii_lowercase(),
    )
}

fn matches_system_type(value: &FhirPathValue, lower_name: &str, exact: bool) -> bool {
    match lower_name {
        "boolean" => matches!(value, FhirPathValue::Boolean(_)),
        "string" => match value {
            FhirPathValue::String(_) => true,
            FhirPathValue::TypedString { fhir_type, .. } => {
                !exact || *fhir_type == FhirPrimitiveType::String
            }
            _ => false,
        },
        "integer" => matches!(value, FhirPathValue::Integer(_) | FhirPathValue::Long(_)),
        "decimal" | "number" => matches!(value, FhirPathValue::Number(_)),
        "date" => matches!(value, FhirPathValue::Date(_)),
        "datetime" => matches!(value, FhirPathValue::DateTime(_)),
        "time" => matches!(value, FhirPathValue::Time(_)),
        "quantity" => matches!(value, FhirPathValue::Quantity { .. }),
        "collection" => matches!(
            value,
            FhirPathValue::Collection(_) | FhirPathValue::UnorderedCollection(_)
        ),
        "object" | "resource" => matches!(value, FhirPathValue::Object(_)),
        _ => false,
    }
}

fn matches_fhir_type(value: &FhirPathValue, lower_name: &str, exact: bool) -> bool {
    match lower_name {
        "boolean" => matches!(
            value,
            FhirPathValue::TypedBoolean {
                fhir_type: FhirPrimitiveType::Boolean,
                ..
            }
        ),
        "uuid" => matches!(
            value,
            FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Uri,
            } if value.starts_with("urn:uuid:")
        ),
        "integer" => value.fhir_primitive_type() == Some(FhirPrimitiveType::Integer),
        "unsignedint" => value.fhir_primitive_type() == Some(FhirPrimitiveType::UnsignedInt),
        "positiveint" => value.fhir_primitive_type() == Some(FhirPrimitiveType::PositiveInt),
        "decimal" => value.fhir_primitive_type() == Some(FhirPrimitiveType::Decimal),
        "string" | "code" | "id" | "uri" | "url" | "canonical" | "oid" | "markdown"
        | "base64binary" | "datetime" | "instant" | "date" | "time" => {
            if let Some(target_type) = fhir_type_from_name(lower_name) {
                if let Some(declared) = value.fhir_primitive_type() {
                    return if exact {
                        declared == target_type
                    } else {
                        is_subtype_of(declared, target_type)
                    };
                }
            }
            false
        }
        "quantity" => matches!(value, FhirPathValue::Quantity { .. }),
        _ => false,
    }
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
        // Per spec: if the input is empty, the result is empty.
        if matches!(value, FhirPathValue::Empty) {
            return Ok(FhirPathValue::Empty);
        }
        let type_name = Self::get_type_name(type_specifier);
        let matches = Self::value_is_type(value, &type_name)?;
        Ok(FhirPathValue::Boolean(matches))
    }

    /// Evaluate 'as' operation — exact declared-type match.
    /// Per spec: errors on multi-item collections; for single items, returns item if type matches or empty.
    fn evaluate_as(
        value: &FhirPathValue,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        let type_name = Self::get_type_name(type_specifier);

        match value {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.len() > 1 =>
            {
                Err(crate::error::FhirPathError::EvaluationError {
                    message: format!(
                        "as({type_name}) cannot be applied to a collection with {} items (expected 0 or 1)",
                        items.len()
                    ),
                })
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                match items.first() {
                    Some(item) if Self::value_is_exact_type(item, &type_name)? => Ok(item.clone()),
                    _ => Ok(FhirPathValue::Empty),
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
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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
        // FhirPrimitive delegates to inner value
        if let FhirPathValue::FhirPrimitive { inner, .. } = value {
            return Self::value_is_type(inner, type_name);
        }

        let (namespace, raw_name, lower_name) = split_type_name(type_name);
        match namespace {
            TypeNamespace::System => return Ok(matches_system_type(value, &lower_name, false)),
            TypeNamespace::Fhir => {
                if matches_fhir_type(value, &lower_name, false) {
                    return Ok(true);
                }
                match value {
                    FhirPathValue::Object(obj) | FhirPathValue::TypedObject { value: obj, .. } => {
                        if let Some(resource_type) =
                            obj.get("resourceType").and_then(|v| v.as_str())
                        {
                            return Ok(resource_type.eq_ignore_ascii_case(raw_name));
                        }
                        if let FhirPathValue::TypedObject { fhir_type, .. } = value {
                            return Ok(fhir_type.eq_ignore_ascii_case(raw_name)
                                || fhir_is_subtype_of(fhir_type, raw_name));
                        }
                    }
                    _ => {}
                }
                return Ok(false);
            }
            TypeNamespace::Unqualified => {}
        }

        match raw_name {
            // Lowercase primitives are FHIR primitives.
            name if name.chars().next().is_some_and(char::is_lowercase) => {
                if lower_name.ends_with("[]") {
                    let element_type = &raw_name[..raw_name.len() - 2];
                    return match value {
                        FhirPathValue::Collection(items)
                        | FhirPathValue::UnorderedCollection(items) => {
                            for item in items {
                                if !Self::value_is_type(item, element_type)? {
                                    return Ok(false);
                                }
                            }
                            Ok(true)
                        }
                        _ => Ok(false),
                    };
                }
                Ok(matches_fhir_type(value, &lower_name, false))
            }
            // Handle collections recursively.
            _ if lower_name.ends_with("[]") => {
                let element_type = &raw_name[..raw_name.len() - 2];
                match value {
                    FhirPathValue::Collection(items)
                    | FhirPathValue::UnorderedCollection(items) => {
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
            _ => {
                if matches_system_type(value, &lower_name, false) {
                    return Ok(true);
                }
                let resource_name = raw_name;
                match value {
                    FhirPathValue::Object(obj) => {
                        if let Some(resource_type) = obj.get("resourceType") {
                            if let Some(resource_type_str) = resource_type.as_str() {
                                return Ok(resource_type_str.eq_ignore_ascii_case(resource_name));
                            }
                        }
                    }
                    FhirPathValue::TypedObject {
                        value: obj,
                        fhir_type,
                    } => {
                        // Check against declared FHIR type (exact match or subtype)
                        if fhir_type.eq_ignore_ascii_case(resource_name)
                            || fhir_is_subtype_of(fhir_type, resource_name)
                        {
                            return Ok(true);
                        }
                        if let Some(resource_type) =
                            obj.get("resourceType").and_then(|v| v.as_str())
                        {
                            return Ok(resource_type.eq_ignore_ascii_case(resource_name));
                        }
                    }
                    _ => {}
                }
                Ok(false)
            }
        }
    }

    /// Check if a value matches a type by **exact declared type** (for `as`/`ofType`).
    /// Unlike `is`, this does NOT follow the type hierarchy: a `code` value
    /// does NOT match `string` — only `code` matches.
    fn value_is_exact_type(value: &FhirPathValue, type_name: &str) -> FhirPathResult<bool> {
        // FhirPrimitive delegates to inner value
        if let FhirPathValue::FhirPrimitive { inner, .. } = value {
            return Self::value_is_exact_type(inner, type_name);
        }

        let (namespace, raw_name, lower_name) = split_type_name(type_name);
        match namespace {
            TypeNamespace::System => return Ok(matches_system_type(value, &lower_name, true)),
            TypeNamespace::Fhir => {
                if matches_fhir_type(value, &lower_name, true) {
                    return Ok(true);
                }
                match value {
                    FhirPathValue::Object(obj) => {
                        if let Some(resource_type) =
                            obj.get("resourceType").and_then(|v| v.as_str())
                        {
                            return Ok(resource_type.eq_ignore_ascii_case(raw_name));
                        }
                    }
                    FhirPathValue::TypedObject {
                        value: obj,
                        fhir_type,
                    } => {
                        if fhir_type.eq_ignore_ascii_case(raw_name) {
                            return Ok(true);
                        }
                        if let Some(resource_type) =
                            obj.get("resourceType").and_then(|v| v.as_str())
                        {
                            return Ok(resource_type.eq_ignore_ascii_case(raw_name));
                        }
                    }
                    _ => {}
                }
                return Ok(false);
            }
            TypeNamespace::Unqualified => {}
        }

        match raw_name {
            name if name.chars().next().is_some_and(char::is_lowercase) => {
                if lower_name.ends_with("[]") {
                    let element_type = &raw_name[..raw_name.len() - 2];
                    return match value {
                        FhirPathValue::Collection(items)
                        | FhirPathValue::UnorderedCollection(items) => {
                            for item in items {
                                if !Self::value_is_exact_type(item, element_type)? {
                                    return Ok(false);
                                }
                            }
                            Ok(true)
                        }
                        _ => Ok(false),
                    };
                }
                Ok(matches_fhir_type(value, &lower_name, true))
            }
            _ if lower_name.ends_with("[]") => {
                let element_type = &raw_name[..raw_name.len() - 2];
                match value {
                    FhirPathValue::Collection(items)
                    | FhirPathValue::UnorderedCollection(items) => {
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
                if matches_system_type(value, &lower_name, true) {
                    return Ok(true);
                }
                let resource_name = raw_name;
                match value {
                    FhirPathValue::Object(obj) => {
                        if let Some(resource_type) =
                            obj.get("resourceType").and_then(|v| v.as_str())
                        {
                            return Ok(resource_type.eq_ignore_ascii_case(resource_name));
                        }
                    }
                    FhirPathValue::TypedObject {
                        value: obj,
                        fhir_type,
                    } => {
                        if fhir_type.eq_ignore_ascii_case(resource_name) {
                            return Ok(true);
                        }
                        if let Some(resource_type) =
                            obj.get("resourceType").and_then(|v| v.as_str())
                        {
                            return Ok(resource_type.eq_ignore_ascii_case(resource_name));
                        }
                    }
                    _ => {}
                }
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

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
            qualified_name: vec!["string".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&code_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // code IS code = true (exact match)
        let code_type = TypeSpecifier {
            qualified_name: vec!["code".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&code_val, &code_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // code IS id = false (no inheritance)
        let id_type = TypeSpecifier {
            qualified_name: vec!["id".to_string()],
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
            qualified_name: vec!["uri".to_string()],
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
            qualified_name: vec!["code".to_string()],
        };
        let result = TypeEvaluator::evaluate_as(&code_val, &code_type).unwrap();
        assert_eq!(result, code_val);

        // code.as(string) = empty (exact type mismatch)
        let string_type = TypeSpecifier {
            qualified_name: vec!["string".to_string()],
        };
        let result = TypeEvaluator::evaluate_as(&code_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);

        // code.as(id) = empty
        let id_type = TypeSpecifier {
            qualified_name: vec!["id".to_string()],
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
        let number_val = FhirPathValue::Number(
            Decimal::from_f64_retain(std::f64::consts::PI).unwrap_or(Decimal::ZERO),
        );

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
