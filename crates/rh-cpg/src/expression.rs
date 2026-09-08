use std::collections::HashMap;

use base64::Engine;
use rh_cql::compile;
use rh_cql::elm::Library;
use rh_cql::eval::engine::evaluate_elm_with_libraries;
use rh_cql::eval::{CqlDateTime, EvalContextBuilder, FixedClock, InMemoryTerminologyProvider};
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::ApplyContext;
use crate::error::{CpgError, CpgResult};
use crate::fhir_to_cql::{cql_value_to_json, fhir_to_cql_value, FhirDataProvider};

pub fn evaluate_expression(
    expression: &Value,
    definitional_resource: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> CpgResult<Value> {
    match language(expression)? {
        "text/fhirpath" => evaluate_fhirpath(expression, definitional_resource, ctx),
        "text/cql-identifier" => evaluate_cql_identifier(expression, library_canonicals, ctx),
        unsupported => Err(CpgError::UnsupportedExpressionLanguage(
            unsupported.to_string(),
        )),
    }
}

#[derive(Serialize, Deserialize)]
struct ElmWrapper {
    library: Library,
}

fn language(expression: &Value) -> CpgResult<&str> {
    expression
        .get("language")
        .and_then(Value::as_str)
        .ok_or_else(|| CpgError::ExpressionError("expression language is required".to_string()))
}

fn evaluate_fhirpath(
    expression: &Value,
    definitional_resource: &Value,
    ctx: &ApplyContext,
) -> CpgResult<Value> {
    let source = expression_source(expression)?;
    let parsed = FhirPathParser::new()
        .parse(source)
        .map_err(|error| CpgError::ExpressionError(error.to_string()))?;

    let mut context = EvaluationContext::new(definitional_resource.clone());
    for constant_name in ["subject", "encounter", "practitioner", "organization"] {
        if let Some(reference) = context_reference(ctx, constant_name) {
            if let Some(resource) = ctx.resolve_context_resource(reference) {
                context.add_constant(constant_name.to_string(), FhirPathValue::Object(resource));
            }
        }
    }

    let result = FhirPathEvaluator::new()
        .evaluate(&parsed, &context)
        .map_err(|error| CpgError::ExpressionError(error.to_string()))?;

    Ok(fhir_path_result_to_json(result))
}

fn context_reference<'ctx>(ctx: &'ctx ApplyContext, constant_name: &str) -> Option<&'ctx str> {
    match constant_name {
        "subject" => Some(&ctx.subject),
        "encounter" => ctx.encounter.as_deref(),
        "practitioner" => ctx.practitioner.as_deref(),
        "organization" => ctx.organization.as_deref(),
        _ => None,
    }
}

fn fhir_path_result_to_json(result: FhirPathValue) -> Value {
    let items = match result {
        FhirPathValue::Collection(items) => items,
        item => vec![item],
    };

    match items.as_slice() {
        [] => Value::Null,
        [item] => item.to_json(),
        items => Value::Array(items.iter().map(FhirPathValue::to_json).collect()),
    }
}

fn evaluate_cql_identifier(
    expression: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> CpgResult<Value> {
    let canonical = expression
        .get("reference")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| library_canonicals.first().cloned())
        .ok_or_else(|| {
            CpgError::ExpressionError(
                "CQL identifier expression has no library reference".to_string(),
            )
        })?;
    let library = ctx
        .content_resolver
        .resolve_canonical(&canonical)
        .map_err(|error| {
            CpgError::ExpressionError(format!("failed to resolve library {canonical}: {error}"))
        })?
        .ok_or_else(|| CpgError::CanonicalNotFound(canonical.clone()))?;
    let elm = extract_elm(&library)?;
    let included_libraries = included_libraries(ctx, &canonical)?;

    let patient = ctx.resolve_context_resource(&ctx.subject);
    let mut builder = EvalContextBuilder::new(FixedClock::new(fixed_now()))
        .data_provider(FhirDataProvider::from_bundle(
            ctx.data.as_ref().unwrap_or(&Value::Null),
        ))
        .terminology_provider(InMemoryTerminologyProvider::new());
    if let Some(patient) = patient {
        builder = builder.context_value(fhir_to_cql_value(&patient));
    }
    let eval_context = builder.build();

    let definition_name = expression_source(expression)?;
    let result = evaluate_elm_with_libraries(
        elm.as_ref()
            .expect("library must contain ELM or CQL content"),
        &included_libraries,
        definition_name,
        &eval_context,
    )
    .map_err(|error| CpgError::CqlEval(error.to_string()))?;

    Ok(cql_value_to_json(&result))
}

fn included_libraries(
    ctx: &ApplyContext,
    target_canonical: &str,
) -> CpgResult<HashMap<String, Library>> {
    let mut libraries = HashMap::new();
    for library in ctx.content_resolver.all_by_type("Library")? {
        let is_target = library
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|url| {
                url == target_canonical || target_canonical.starts_with(&format!("{url}|"))
            });
        if !is_target {
            if let Some(name) = library.get("name").and_then(Value::as_str) {
                if let Some(elm) = extract_elm(&library)? {
                    libraries.insert(name.to_string(), elm);
                }
            }
        }
    }

    Ok(libraries)
}

fn extract_elm(library: &Value) -> CpgResult<Option<Library>> {
    let Some(elm_attachment) = content_by_type(library, "application/elm+json") else {
        return extract_elm_from_cql(library);
    };

    let data = elm_attachment
        .get("data")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            CpgError::ExpressionError("ELM attachment has no base64 data".to_string())
        })?;
    let decoded = decode_base64(data)?;
    let wrapper: ElmWrapper = serde_json::from_str(&decoded)
        .map_err(|error| CpgError::ExpressionError(error.to_string()))?;

    Ok(Some(wrapper.library))
}

fn extract_elm_from_cql(library: &Value) -> CpgResult<Option<Library>> {
    let Some(cql_attachment) = content_by_type(library, "text/cql") else {
        return Ok(None);
    };
    let data = cql_attachment
        .get("data")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            CpgError::ExpressionError("CQL attachment has no base64 data".to_string())
        })?;
    let source = decode_base64(data)?;
    let compiled = compile(&source, None)
        .map_err(|error| CpgError::ExpressionError(format!("failed to compile CQL: {error}")))?;

    Ok(Some(compiled.library))
}

fn content_by_type<'value>(library: &'value Value, content_type: &str) -> Option<&'value Value> {
    library
        .get("content")
        .and_then(Value::as_array)?
        .iter()
        .find(|attachment| {
            attachment.get("contentType").and_then(Value::as_str) == Some(content_type)
        })
}

fn decode_base64(data: &str) -> CpgResult<String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|error| CpgError::ExpressionError(error.to_string()))?;
    String::from_utf8(bytes).map_err(|error| CpgError::ExpressionError(error.to_string()))
}

fn expression_source(expression: &Value) -> CpgResult<&str> {
    expression
        .get("expression")
        .and_then(Value::as_str)
        .ok_or_else(|| CpgError::ExpressionError("expression value is required".to_string()))
}

fn fixed_now() -> CqlDateTime {
    CqlDateTime {
        year: 2026,
        month: Some(1),
        day: Some(1),
        hour: Some(0),
        minute: Some(0),
        second: Some(0),
        millisecond: None,
        offset_seconds: Some(0),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::resolver::BundleResolver;
    use serde_json::json;

    use super::*;
    use crate::context::ApplyContext;

    #[test]
    fn evaluates_fhirpath_against_definitional_resource() {
        let plan_definition = json!({
            "resourceType": "PlanDefinition",
            "title": "Diabetes Screening"
        });
        let expression = json!({
            "language": "text/fhirpath",
            "expression": "title"
        });
        let ctx = test_context(None);

        let result = evaluate_expression(&expression, &plan_definition, &[], &ctx)
            .expect("FHIRPath evaluation should succeed");

        assert_eq!(result, json!("Diabetes Screening"));
    }

    #[test]
    fn evaluates_cql_identifier_from_elm_library() {
        let library = compiled_test_library();
        let library_resource = json!({
            "resourceType": "Library",
            "url": "http://test/Library/TestLib",
            "name": "TestLib",
            "content": [{
                "contentType": "application/elm+json",
                "data": encode_elm(&library)
            }]
        });
        let expression = json!({
            "language": "text/cql-identifier",
            "expression": "IsAdult",
            "reference": "http://test/Library/TestLib"
        });
        let ctx = test_context(Some(json!({
            "resourceType": "Bundle",
            "entry": [{ "resource": library_resource }]
        })));

        let result = evaluate_expression(&expression, &json!({}), &[], &ctx)
            .expect("CQL evaluation should succeed");

        assert_eq!(result, json!(true));
    }

    fn test_context(bundle: Option<Value>) -> ApplyContext {
        let content_bundle = bundle.unwrap_or(json!({ "resourceType": "Bundle" }));
        let resolver =
            Arc::new(BundleResolver::new(&content_bundle).expect("test bundle should be valid"));
        ApplyContext::new(resolver, "Patient/123")
    }

    fn compiled_test_library() -> Library {
        let source = "library TestLib version '1.0' define \"IsAdult\": true";
        compile(source, None)
            .expect("test CQL should compile")
            .library
    }

    fn encode_elm(library: &Library) -> String {
        let wrapper = ElmWrapper {
            library: library.clone(),
        };
        let encoded = serde_json::to_string(&wrapper).expect("ELM library should serialize");
        base64::engine::general_purpose::STANDARD.encode(encoded)
    }
}
