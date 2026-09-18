use std::collections::{HashMap, HashSet};

use base64::Engine;
use rh_cql::compile;
use rh_cql::elm::Library;
use rh_cql::eval::engine::evaluate_elm_with_libraries;
use rh_cql::eval::{CqlCode, EvalContextBuilder, FixedClock, InMemoryTerminologyProvider};
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
pub(crate) struct ElmWrapper {
    pub(crate) library: Library,
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
    let mut builder = EvalContextBuilder::new(FixedClock::new(ctx.cql_evaluation_date()?))
        .data_provider(FhirDataProvider::from_bundle(
            ctx.data.as_ref().unwrap_or(&Value::Null),
            &ctx.subject,
        ))
        .terminology_provider(terminology_provider(ctx)?);
    for (name, value) in &ctx.parameters {
        builder = builder.parameter(name, fhir_to_cql_value(value));
    }
    if let Some(measurement_period) = &ctx.measurement_period {
        builder = builder.parameter("Measurement Period", measurement_period.as_cql_interval()?);
    }
    if let Some(patient) = patient {
        builder = builder.context_value(fhir_to_cql_value(&patient));
    }
    let eval_context = builder.build();

    let definition_name = expression_source(expression)?;
    let elm = elm.ok_or_else(|| {
        CpgError::ExpressionError("library contains neither ELM nor CQL content".to_string())
    })?;
    let result =
        evaluate_elm_with_libraries(&elm, &included_libraries, definition_name, &eval_context)
            .map_err(|error| CpgError::CqlEval(error.to_string()))?;

    Ok(cql_value_to_json(&result))
}

fn terminology_provider(ctx: &ApplyContext) -> CpgResult<InMemoryTerminologyProvider> {
    let mut provider = InMemoryTerminologyProvider::new();
    let mut valuesets: HashMap<String, Vec<(Option<String>, Vec<CqlCode>)>> = HashMap::new();
    for value_set in ctx.content_resolver.all_by_type("ValueSet")? {
        let Some(url) = value_set.get("url").and_then(Value::as_str) else {
            continue;
        };
        let codes = value_set
            .pointer("/expansion/contains")
            .and_then(Value::as_array)
            .map_or_else(Vec::new, |contains| collect_expansion_codes(contains));
        if codes.is_empty() {
            continue;
        }
        valuesets.entry(url.to_string()).or_default().push((
            value_set
                .get("version")
                .and_then(Value::as_str)
                .map(str::to_string),
            codes,
        ));
    }

    for (url, versions) in valuesets {
        let mut seen_versions = HashSet::new();
        for (version, codes) in &versions {
            if let Some(version) = version {
                if !seen_versions.insert(version) {
                    return Err(CpgError::EvaluationError(format!(
                        "duplicate ValueSet canonical '{url}|{version}' in execution content"
                    )));
                }
                provider.register_valueset(format!("{url}|{version}"), codes.clone());
            }
        }
        if versions.len() == 1 {
            provider.register_valueset(url, versions[0].1.clone());
        }
    }
    Ok(provider)
}

fn collect_expansion_codes(contains: &[Value]) -> Vec<CqlCode> {
    let mut codes = Vec::new();
    for concept in contains {
        if let (Some(code), Some(system)) = (
            concept.get("code").and_then(Value::as_str),
            concept.get("system").and_then(Value::as_str),
        ) {
            codes.push(CqlCode {
                code: code.to_string(),
                system: system.to_string(),
                display: concept
                    .get("display")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                version: concept
                    .get("version")
                    .and_then(Value::as_str)
                    .map(str::to_string),
            });
        }
        if let Some(children) = concept.get("contains").and_then(Value::as_array) {
            codes.extend(collect_expansion_codes(children));
        }
    }
    codes
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::resolver::BundleResolver;
    use rh_cql::eval::TerminologyProvider;
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

    #[test]
    fn evaluates_patient_questionnaire_period_clock_and_parameters_from_context() {
        let source = r#"library ConnectathonContext version '1.0.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
parameter "Marker" Boolean
context Patient
define "Completed Three Question Screen":
  exists ([QuestionnaireResponse] Response
    where Response.status = 'completed'
      and Count(Response.item) = 3
      and exists (Response.item Item
        where exists (Item.answer Answer where Answer.valueBoolean = true)))
define "Three Question Risk":
  if exists ([QuestionnaireResponse]) then
    if "Completed Three Question Screen" then true else null
  else null
define "Measurement Period Is 2027":
  start of "Measurement Period" = @2027-01-01T00:00:00.000Z
define "Clock Is 2027": year from Now() = 2027
define "Marker Is True": "Marker"
"#;
        let library = compile(source, None)
            .expect("context library should compile")
            .library;
        let content = json!({
            "resourceType": "Bundle",
            "entry": [{"resource": {
                "resourceType": "Library",
                "url": "http://test/Library/ConnectathonContext",
                "name": "ConnectathonContext",
                "content": [{"contentType": "application/elm+json", "data": encode_elm(&library)}]
            }}]
        });
        let resolver = Arc::new(BundleResolver::new(&content).expect("content should resolve"));
        let mut ctx = ApplyContext::new(resolver, "Patient/fixture");
        ctx.data = Some(json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {"resourceType": "Patient", "id": "fixture"}},
                {"resource": {
                    "resourceType": "QuestionnaireResponse",
                    "subject": {"reference": "Patient/fixture"},
                    "status": "completed",
                    "item": [
                        {"linkId": "unsteady", "answer": [{"valueBoolean": true}]},
                        {"linkId": "worries", "answer": [{"valueBoolean": false}]},
                        {"linkId": "prior", "answer": [{"valueBoolean": false}]}
                    ]
                }}
            ]
        }));
        ctx.evaluation_date = Some("2027-06-15T09:20:00Z".to_string());
        ctx.measurement_period = Some(crate::context::MeasurementPeriod {
            start: "2027-01-01T00:00:00Z".to_string(),
            end: "2027-12-31T23:59:59Z".to_string(),
            start_inclusive: true,
            end_inclusive: true,
        });
        ctx.parameters.insert("Marker".to_string(), json!(true));

        for definition in [
            "Completed Three Question Screen",
            "Three Question Risk",
            "Measurement Period Is 2027",
            "Clock Is 2027",
            "Marker Is True",
        ] {
            let expression = json!({
                "language": "text/cql-identifier",
                "expression": definition,
                "reference": "http://test/Library/ConnectathonContext"
            });
            assert_eq!(
                evaluate_expression(&expression, &json!({}), &[], &ctx)
                    .expect("contextual CQL should evaluate"),
                json!(true),
                "{definition}"
            );
        }

        let incomplete_response = ctx
            .data
            .as_mut()
            .and_then(|bundle| bundle.pointer_mut("/entry/1/resource"))
            .expect("response fixture")
            .as_object_mut()
            .expect("response object");
        incomplete_response.insert("status".to_string(), json!("in-progress"));
        let expression = json!({
            "language": "text/cql-identifier",
            "expression": "Three Question Risk",
            "reference": "http://test/Library/ConnectathonContext"
        });
        assert_eq!(
            evaluate_expression(&expression, &json!({}), &[], &ctx)
                .expect("incomplete response should evaluate"),
            Value::Null
        );
    }

    #[test]
    fn evaluates_valueset_membership_from_bundled_expansion() {
        let source = r#"library ValueSetContext version '1.0.0'
codesystem "Test System": 'http://example.org/system'
code "Allowed": 'allowed' from "Test System"
code "Denied": 'denied' from "Test System"
valueset "Test ValueSet": 'http://example.org/ValueSet/test'
define "Allowed Is Member": "Allowed" in "Test ValueSet"
define "Denied Is Member": "Denied" in "Test ValueSet"
"#;
        let library = compile(source, None)
            .expect("valueset library should compile")
            .library;
        let content = json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {
                    "resourceType": "Library",
                    "url": "http://test/Library/ValueSetContext",
                    "name": "ValueSetContext",
                    "content": [{"contentType": "application/elm+json", "data": encode_elm(&library)}]
                }},
                {"resource": {
                    "resourceType": "ValueSet",
                    "url": "http://example.org/ValueSet/test",
                    "version": "1.0.0",
                    "expansion": {"contains": [{
                        "system": "http://example.org/system",
                        "code": "allowed"
                    }]}
                }}
            ]
        });
        let ctx = ApplyContext::new(
            Arc::new(BundleResolver::new(&content).expect("content should resolve")),
            "Patient/example",
        );

        for (definition, expected) in [
            ("Allowed Is Member", json!(true)),
            ("Denied Is Member", json!(false)),
        ] {
            let expression = json!({
                "language": "text/cql-identifier",
                "expression": definition,
                "reference": "http://test/Library/ValueSetContext"
            });
            assert_eq!(
                evaluate_expression(&expression, &json!({}), &[], &ctx)
                    .expect("membership should evaluate"),
                expected,
                "{definition}"
            );
        }
    }

    #[test]
    fn rejects_duplicate_valueset_canonical_versions() {
        let content = json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {
                    "resourceType": "ValueSet",
                    "url": "http://example.org/ValueSet/test",
                    "version": "1.0.0",
                    "expansion": {"contains": [{"system": "http://example.org/system", "code": "one"}]}
                }},
                {"resource": {
                    "resourceType": "ValueSet",
                    "url": "http://example.org/ValueSet/test",
                    "version": "1.0.0",
                    "expansion": {"contains": [{"system": "http://example.org/system", "code": "two"}]}
                }}
            ]
        });
        assert!(matches!(
            BundleResolver::new(&content),
            Err(CpgError::InvalidResource(message)) if message.contains("duplicate canonical")
        ));
    }

    #[test]
    fn valueset_membership_fails_when_a_bundled_valueset_has_no_expansion() {
        let source = r#"library MissingExpansion version '1.0.0'
codesystem "Test System": 'http://example.org/system'
code "Allowed": 'allowed' from "Test System"
valueset "Test ValueSet": 'http://example.org/ValueSet/test'
define "Allowed Is Member": "Allowed" in "Test ValueSet"
"#;
        let library = compile(source, None)
            .expect("valueset library should compile")
            .library;
        let content = json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {
                    "resourceType": "Library",
                    "url": "http://test/Library/MissingExpansion",
                    "name": "MissingExpansion",
                    "content": [{"contentType": "application/elm+json", "data": encode_elm(&library)}]
                }},
                {"resource": {
                    "resourceType": "ValueSet",
                    "url": "http://example.org/ValueSet/test"
                }}
            ]
        });
        let ctx = ApplyContext::new(
            Arc::new(BundleResolver::new(&content).expect("content should resolve")),
            "Patient/example",
        );
        let expression = json!({
            "language": "text/cql-identifier",
            "expression": "Allowed Is Member",
            "reference": "http://test/Library/MissingExpansion"
        });

        assert!(matches!(
            evaluate_expression(&expression, &json!({}), &[], &ctx),
            Err(CpgError::CqlEval(message)) if message.contains("ValueSet")
        ));
    }

    #[test]
    fn requires_an_explicit_version_when_bundled_valueset_versions_differ() {
        let content = json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {
                    "resourceType": "ValueSet",
                    "url": "http://example.org/ValueSet/test",
                    "version": "1.0.0",
                    "expansion": {"contains": [{"system": "http://example.org/system", "code": "one"}]}
                }},
                {"resource": {
                    "resourceType": "ValueSet",
                    "url": "http://example.org/ValueSet/test",
                    "version": "2.0.0",
                    "expansion": {"contains": [{"system": "http://example.org/system", "code": "two"}]}
                }}
            ]
        });
        let ctx = ApplyContext::new(
            Arc::new(BundleResolver::new(&content).expect("content should resolve")),
            "Patient/example",
        );
        let provider = terminology_provider(&ctx).expect("versioned valuesets should load");

        assert!(provider
            .expand_valueset("http://example.org/ValueSet/test")
            .is_err());
        assert_eq!(
            provider
                .expand_valueset("http://example.org/ValueSet/test|2.0.0")
                .expect("versioned valueset should resolve")[0]
                .code,
            "two"
        );
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
