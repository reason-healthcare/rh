use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::context::ApplyContext;
use crate::error::{CpgError, CpgResult};
use crate::expression::evaluate_expression;

const MEASURE_EVAL_ISSUE_EXTENSION_URL: &str =
    "http://reasonhealth.org/fhir/StructureDefinition/measure-eval-issue";

/// Evaluate a FHIR R4 Measure for the subject in `ctx`, producing an
/// individual-type MeasureReport with population membership counts and
/// stratifier assignments.
pub fn evaluate_measure(
    measure: &serde_json::Value,
    ctx: &ApplyContext,
) -> CpgResult<serde_json::Value> {
    if measure.get("resourceType").and_then(Value::as_str) != Some("Measure") {
        return Err(CpgError::InvalidResource(
            "expected a FHIR Measure object".to_string(),
        ));
    }

    let library_canonicals = measure_library_canonicals(measure);
    let groups = match measure.get("group").and_then(Value::as_array) {
        Some(groups) => groups
            .iter()
            .enumerate()
            .map(|(group_index, group)| {
                evaluate_group(group, group_index, measure, &library_canonicals, ctx)
            })
            .collect::<CpgResult<Vec<_>>>()?,
        None => Vec::new(),
    };
    let mut report = json!({
        "resourceType": "MeasureReport",
        "id": Uuid::new_v4().to_string(),
        "status": "complete",
        "type": "individual",
        // The evaluation clock is fixed to the same date as CQL expression
        // evaluation. This deterministic required period represents the
        // single-day point-in-time evaluation.
        "period": {"start": "2026-01-01", "end": "2026-01-01"},
        "subject": {"reference": ctx.subject},
    });

    if let Some(url) = measure.get("url").and_then(Value::as_str) {
        report["measure"] = json!(url);
    }

    let (report_groups, group_issues) = unzip_groups(groups);
    if !report_groups.is_empty() {
        report["group"] = Value::Array(report_groups);
    }

    add_evaluation_issues(&mut report, &group_issues);

    Ok(report)
}

/// Measure population and stratifier criteria (FHIRPath) are evaluated
/// against the subject resource, matching the patient-centered Measure
/// evaluation context. When the subject cannot be resolved, evaluate
/// against an empty resource: patient-literal expressions then yield
/// empty collections (no data means no membership) instead of leaking
/// Measure fields into population criteria.
fn subject_context_root(ctx: &ApplyContext) -> Value {
    ctx.resolve_context_resource(&ctx.subject)
        .unwrap_or_else(|| serde_json::Map::new().into())
}

fn measure_library_canonicals(measure: &Value) -> Vec<String> {
    measure
        .get("library")
        .and_then(Value::as_array)
        .map(|libraries| {
            libraries
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn evaluate_group(
    group: &Value,
    group_index: usize,
    measure: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> CpgResult<(Value, Vec<String>)> {
    let populations = match group.get("population").and_then(Value::as_array) {
        Some(populations) => populations
            .iter()
            .enumerate()
            .map(|(population_index, population)| {
                evaluate_population(
                    population,
                    group_index,
                    population_index,
                    measure,
                    library_canonicals,
                    ctx,
                )
            })
            .collect::<CpgResult<Vec<_>>>()?,
        None => Vec::new(),
    };

    let mut issues: Vec<String> = populations
        .iter()
        .flat_map(|(_, issue)| issue.iter().cloned())
        .collect();

    let stratifiers = match group.get("stratifier").and_then(Value::as_array) {
        Some(stratifiers) => stratifiers
            .iter()
            .map(|stratifier| {
                evaluate_stratifier(
                    stratifier,
                    group_index,
                    measure,
                    library_canonicals,
                    ctx,
                    &populations,
                )
            })
            .collect::<CpgResult<Vec<_>>>()?,
        None => Vec::new(),
    };

    issues.extend(
        stratifiers
            .iter()
            .flat_map(|(_, issue)| issue.iter().cloned()),
    );

    let mut report_group = Map::new();
    let report_populations = populations
        .into_iter()
        .map(|(population, _)| population)
        .collect::<Vec<_>>();
    if !report_populations.is_empty() {
        report_group.insert("population".to_string(), Value::Array(report_populations));
    }
    let report_stratifiers = stratifiers
        .into_iter()
        .map(|(stratifier, _)| stratifier)
        .collect::<Vec<_>>();
    if !report_stratifiers.is_empty() {
        report_group.insert("stratifier".to_string(), Value::Array(report_stratifiers));
    }

    Ok((Value::Object(report_group), issues))
}

#[allow(clippy::too_many_arguments)]
fn evaluate_population(
    population: &Value,
    group_index: usize,
    population_index: usize,
    measure: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> CpgResult<(Value, Vec<String>)> {
    let code = population.get("code").cloned().unwrap_or(Value::Null);
    let expression = population.get("criteria");
    let subject_root = subject_context_root(ctx);
    let (count, issue) = match expression {
        Some(expression) => {
            match evaluate_expression(expression, &subject_root, library_canonicals, ctx) {
                Ok(result) => (u64::from(is_member(&result)), Vec::new()),
                Err(error) => (
                    0,
                    vec![format!(
                        "group {group_index}, population {population_index}: {error}"
                    )],
                ),
            }
        }
        None => (0, Vec::new()),
    };

    let mut report_population = Map::new();
    if !code.is_null() {
        report_population.insert("code".to_string(), code);
    }
    report_population.insert("count".to_string(), json!(count));

    Ok((Value::Object(report_population), issue))
}

fn is_member(result: &Value) -> bool {
    match result {
        Value::Null => false,
        Value::Bool(value) => *value,
        // Existence semantics apply to CQL and FHIRPath collections.
        Value::Array(items) => !items.is_empty(),
        // Any other scalar or structured value represents successful
        // evaluation; its non-null existence establishes membership.
        _ => true,
    }
}

fn evaluate_stratifier(
    stratifier: &Value,
    group_index: usize,
    measure: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
    populations: &[(Value, Vec<String>)],
) -> CpgResult<(Value, Vec<String>)> {
    let mut stratifiers = Vec::new();
    let mut issues = Vec::new();
    let subject_root = subject_context_root(ctx);
    if let Some(criteria) = stratifier.get("criteria") {
        match evaluate_expression(criteria, &subject_root, library_canonicals, ctx) {
            Ok(result) => {
                if let Some(value) = stratifier_value(&result) {
                    stratifiers.push(stratum(value, populations));
                }
            }
            Err(error) => {
                issues.push(format!("group {group_index}, stratifier: {error}"));
            }
        }
    } else if stratifier
        .get("component")
        .and_then(Value::as_array)
        .is_some_and(|components| !components.is_empty())
    {
        // Component-based stratifiers are valid FHIR but not supported yet;
        // surface that instead of silently emitting an incomplete report.
        issues.push(format!(
            "group {group_index}, stratifier: component-based stratifiers are not supported yet"
        ));
    }

    let mut report_stratifier = Map::new();
    if let Some(code) = stratifier.get("code") {
        report_stratifier.insert("code".to_string(), code.clone());
    }
    if !stratifiers.is_empty() {
        report_stratifier.insert("stratum".to_string(), Value::Array(stratifiers));
    }

    Ok((Value::Object(report_stratifier), issues))
}

fn stratifier_value(result: &Value) -> Option<Value> {
    let value = match result {
        Value::Array(items) => items.first(),
        Value::Null => None,
        value => Some(value),
    };
    let value = value?;

    Some(match value {
        Value::Object(_) => value.clone(),
        Value::String(code) => json!({"coding": [{"code": code}]}),
        scalar => json!({"text": scalar.to_string()}),
    })
}

/// Build an individual-report stratum: the subject's stratifier value and
/// the group's full population list with the subject's 0/1 membership.
fn stratum(value: Value, populations: &[(Value, Vec<String>)]) -> Value {
    let mut stratum = Map::new();
    stratum.insert("valueCodeableConcept".to_string(), value);
    stratum.insert(
        "population".to_string(),
        Value::Array(
            populations
                .iter()
                .map(|(population, _)| population.clone())
                .collect(),
        ),
    );
    Value::Object(stratum)
}

fn unzip_groups(groups: Vec<(Value, Vec<String>)>) -> (Vec<Value>, Vec<String>) {
    let mut report_groups = Vec::with_capacity(groups.len());
    let mut issues = Vec::new();
    for (group, group_issues) in groups {
        report_groups.push(group);
        issues.extend(group_issues);
    }
    (report_groups, issues)
}

fn add_evaluation_issues(report: &mut Value, issues: &[String]) {
    if issues.is_empty() {
        return;
    }

    let extensions = issues
        .iter()
        .map(|issue| {
            json!({
                "url": MEASURE_EVAL_ISSUE_EXTENSION_URL,
                "valueString": issue,
            })
        })
        .collect::<Vec<_>>();

    report["extension"] = Value::Array(extensions);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::context::ApplyContext;
    use crate::error::CpgError;
    use crate::expression::ElmWrapper;
    use crate::resolver::BundleResolver;
    use base64::Engine;
    use rh_cql::compile;
    use rh_cql::elm::Library;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn evaluates_cql_populations_and_stratifier() {
        let library = compiled_test_library();
        let library_resource = json!({
            "resourceType": "Library",
            "url": "http://test/Library/TestLib",
            "name": "TestLib",
            "content": [{
                "contentType": "application/elm+json",
                "data": encode_elm(&library),
            }],
        });
        let measure = json!({
            "resourceType": "Measure",
            "url": "http://test/Measure/AdultPopulation",
            "library": ["http://test/Library/TestLib"],
            "group": [{
                "population": [
                    {"code": {"coding": [{"code": "initial-population"}]}, "criteria": criteria("InInitial")},
                    {"code": {"coding": [{"code": "numerator"}]}, "criteria": criteria("InNumerator")},
                ],
                "stratifier": [{"code": {"coding": [{"code": "age-group"}]}, "criteria": criteria("Stratifier")}],
            }],
        });
        let ctx = test_context(Some(json!({
            "resourceType": "Bundle",
            "entry": [{"resource": library_resource}],
        })));

        let report = evaluate_measure(&measure, &ctx).expect("measure should evaluate");

        assert_eq!(report["resourceType"], json!("MeasureReport"));
        assert_eq!(report["type"], json!("individual"));
        assert_eq!(report["status"], json!("complete"));
        assert_eq!(report["subject"], json!({"reference": "Patient/123"}));
        assert_eq!(
            report["measure"],
            json!("http://test/Measure/AdultPopulation")
        );
        assert_eq!(
            report["period"],
            json!({"start": "2026-01-01", "end": "2026-01-01"})
        );
        assert_eq!(
            report["group"][0]["population"][0],
            json!({"code": {"coding": [{"code": "initial-population"}]}, "count": 1})
        );
        assert_eq!(
            report["group"][0]["population"][1],
            json!({"code": {"coding": [{"code": "numerator"}]}, "count": 0})
        );
        assert_eq!(
            report["group"][0]["stratifier"][0]["stratum"][0]["valueCodeableConcept"],
            json!({"coding": [{"code": "adult"}]})
        );
        // The stratum carries the group's full population list with the
        // subject's 0/1 membership.
        assert_eq!(
            report["group"][0]["stratifier"][0]["stratum"][0]["population"][0]["code"],
            json!({"coding": [{"code": "initial-population"}]})
        );
        assert_eq!(
            report["group"][0]["stratifier"][0]["stratum"][0]["population"][0]["count"],
            json!(1)
        );
        assert_eq!(
            report["group"][0]["stratifier"][0]["stratum"][0]["population"][1]["code"],
            json!({"coding": [{"code": "numerator"}]})
        );
        assert_eq!(
            report["group"][0]["stratifier"][0]["stratum"][0]["population"][1]["count"],
            json!(0)
        );
        assert!(report.get("extension").is_none());
    }

    #[test]
    fn fhirpath_criteria_evaluate_against_the_subject() {
        let measure = json!({
            "resourceType": "Measure",
            "group": [{
                "population": [
                    {"code": {"coding": [{"code": "initial-population"}]}, "criteria": {"language": "text/fhirpath", "expression": "active = true"}},
                    {"code": {"coding": [{"code": "numerator"}]}, "criteria": {"language": "text/fhirpath", "expression": "gender = 'male'"}},
                ],
                "stratifier": [{"code": {"coding": [{"code": "gender"}]}, "criteria": {"language": "text/fhirpath", "expression": "gender"}}],
            }],
        });
        let ctx = test_context_with_data(json!({
            "resourceType": "Bundle",
            "entry": [{
                "resource": {"resourceType": "Patient", "id": "123", "active": true, "gender": "female"}
            }]
        }));

        let report = evaluate_measure(&measure, &ctx).expect("measure should evaluate");

        assert_eq!(report["group"][0]["population"][0]["count"], json!(1));
        assert_eq!(report["group"][0]["population"][1]["count"], json!(0));
        assert_eq!(
            report["group"][0]["stratifier"][0]["stratum"][0]["valueCodeableConcept"],
            json!({"coding": [{"code": "female"}]})
        );
    }

    #[test]
    fn component_stratifiers_report_an_issue() {
        let measure = json!({
            "resourceType": "Measure",
            "group": [{
                "population": [{"code": {"coding": [{"code": "initial-population"}]}}],
                "stratifier": [{"code": {"text": "combo"}, "component": [{"code": {"text": "part"}}]}],
            }],
        });
        let ctx = test_context(None);

        let report = evaluate_measure(&measure, &ctx).expect("measure should evaluate");

        assert!(report["group"][0]["stratifier"][0].get("stratum").is_none());
        let issues = report["extension"].as_array().expect("issues");
        assert!(issues[0]["valueString"]
            .as_str()
            .is_some_and(|issue| issue.contains("component-based")));
    }

    #[test]
    fn missing_criteria_reports_empty_population() {
        let population_code = json!({"coding": [{"code": "initial-population"}]});
        let measure = json!({
            "resourceType": "Measure",
            "group": [{"population": [{"code": population_code}]}],
        });
        let ctx = test_context(None);

        let report = evaluate_measure(&measure, &ctx).expect("measure should evaluate");

        assert_eq!(
            report["group"][0]["population"][0],
            json!({"code": population_code, "count": 0})
        );
        assert!(report.get("extension").is_none());
    }

    #[test]
    fn failed_criteria_and_stratifier_are_recovered_as_issues() {
        let measure = json!({
            "resourceType": "Measure",
            "group": [{
                "population": [
                    {"code": {"coding": [{"code": "initial-population"}]}, "criteria": {"language": "text/unsupported", "expression": "true"}},
                ],
                "stratifier": [{"code": {"coding": [{"code": "age-group"}]}, "criteria": {"language": "text/unsupported", "expression": "adult"}}],
            }],
        });
        let ctx = test_context(None);

        let report = evaluate_measure(&measure, &ctx).expect("errors should not abort evaluation");

        assert_eq!(report["group"][0]["population"][0]["count"], json!(0));
        assert!(report["group"][0]["stratifier"][0].get("stratum").is_none());
        assert_eq!(report["extension"].as_array().map(Vec::len), Some(2));
        for extension in report["extension"].as_array().expect("issues exist") {
            assert_eq!(
                extension["url"],
                json!("http://reasonhealth.org/fhir/StructureDefinition/measure-eval-issue")
            );
            assert!(extension["valueString"].as_str().is_some());
        }
    }

    #[test]
    fn rejects_non_measure_resource() {
        let plan_definition = json!({"resourceType": "PlanDefinition"});
        let ctx = test_context(None);

        assert!(matches!(
            evaluate_measure(&plan_definition, &ctx),
            Err(CpgError::InvalidResource(_))
        ));
    }

    fn criteria(expression: &str) -> Value {
        json!({
            "language": "text/cql-identifier",
            "expression": expression,
        })
    }

    fn test_context(bundle: Option<Value>) -> ApplyContext {
        let content_bundle = bundle.unwrap_or(json!({"resourceType": "Bundle"}));
        let resolver =
            Arc::new(BundleResolver::new(&content_bundle).expect("test bundle should be valid"));
        ApplyContext::new(resolver, "Patient/123")
    }

    fn test_context_with_data(data: Value) -> ApplyContext {
        let content_bundle = json!({"resourceType": "Bundle"});
        let resolver =
            Arc::new(BundleResolver::new(&content_bundle).expect("test bundle should be valid"));
        let mut ctx = ApplyContext::new(resolver, "Patient/123");
        ctx.data = Some(data);
        ctx
    }

    fn compiled_test_library() -> Library {
        let source = concat!(
            "library TestLib version '1.0' define \"InInitial\": true ",
            "define \"InNumerator\": false define \"Stratifier\": 'adult'"
        );
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
