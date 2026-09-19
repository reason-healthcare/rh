//! Constrained SDC Boolean independent-Observation extraction.
//!
//! This module is deliberately smaller than the SDC QuestionnaireResponse
//! extraction operation. It supports source-profiled Boolean question items and
//! returns a standard FHIR transaction Bundle for callers that invoke it.

use chrono::DateTime;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::error::{CpgError, CpgResult};

const SDC_EXTRACTION_PROFILE: &str =
    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-extr-obsn";
const OBSERVATION_EXTRACT: &str =
    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-observationExtract";
const OBSERVATION_CATEGORY: &str =
    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-observation-extract-category";

const PERFORMER_REFERENCE_TYPES: &[&str] = &[
    "CareTeam",
    "Organization",
    "Patient",
    "Practitioner",
    "PractitionerRole",
    "RelatedPerson",
];
const BASED_ON_REFERENCE_TYPES: &[&str] = &[
    "CarePlan",
    "DeviceRequest",
    "ImmunizationRecommendation",
    "MedicationRequest",
    "NutritionOrder",
    "ServiceRequest",
];
const PART_OF_REFERENCE_TYPES: &[&str] = &[
    "Immunization",
    "MedicationAdministration",
    "MedicationDispense",
    "MedicationStatement",
    "Observation",
    "Procedure",
];

/// Workflow-gated result. Non-invocation is normal only when the response has
/// not reached the completed/all-required-answer workflow state.
#[derive(Debug, PartialEq)]
pub enum SdcObservationExtraction {
    NotInvoked {
        reason: String,
    },
    Extracted {
        transaction: Value,
        observations: Vec<Value>,
    },
}

/// Extract usable Boolean answers and, only when every required Boolean answer
/// is present, calculated integer score Observations without imposing workflow
/// status gating.
///
/// This function allows direct SDC-focused partial-response checks. Consumers
/// evaluating clinical logic should call extract_completed_sdc_boolean_observations.
pub fn extract_sdc_boolean_observations(
    questionnaire: &Value,
    response: &Value,
    subject: &str,
    encounter: &str,
) -> CpgResult<Vec<Value>> {
    let specification = extraction_specification(questionnaire)?;
    validate_response_provenance(
        response,
        &specification.questionnaire_canonical,
        subject,
        encounter,
    )?;
    let answers = response_answers(response, &specification.boolean_items)?;
    let submitted_scores = response_scores(response, &specification.calculated_score_items)?;

    let response_reference = format!(
        "QuestionnaireResponse/{}",
        required_string(response, "id", "QuestionnaireResponse")?
    );
    let authored = required_string(response, "authored", "QuestionnaireResponse")?;
    let author = required_reference(response, "author", "QuestionnaireResponse")?;
    validate_reference_type(
        author,
        PERFORMER_REFERENCE_TYPES,
        "QuestionnaireResponse.author",
    )?;
    let based_on = optional_reference_array(
        response,
        "basedOn",
        BASED_ON_REFERENCE_TYPES,
        "QuestionnaireResponse.basedOn",
    )?;
    let part_of = optional_reference_array(
        response,
        "partOf",
        PART_OF_REFERENCE_TYPES,
        "QuestionnaireResponse.partOf",
    )?;
    let security = optional_security_labels(response)?;
    let observation_provenance = ObservationProvenance {
        subject,
        encounter,
        authored,
        author,
        response_reference: &response_reference,
        category: specification.category.as_ref(),
        security: security.as_ref(),
        based_on: based_on.as_deref(),
        part_of: part_of.as_deref(),
    };

    let mut observations = specification
        .boolean_items
        .iter()
        .filter_map(|item| answers.get(&item.link_id).map(|answer| (item, *answer)))
        .map(|(item, value_boolean)| {
            let mut observation = json!({
                "resourceType": "Observation",
                "status": "final",
                "code": { "coding": [item.coding.clone()] },
                "subject": { "reference": subject },
                "encounter": { "reference": encounter },
                "effectiveDateTime": authored,
                "issued": authored,
                "performer": [{ "reference": author }],
                "valueBoolean": value_boolean,
                "derivedFrom": [{ "reference": response_reference }],
            });
            if let Some(category) = &specification.category {
                observation["category"] = Value::Array(vec![category.clone()]);
            }
            if let Some(security) = &security {
                observation["meta"] = json!({ "security": security });
            }
            if let Some(based_on) = &based_on {
                observation["basedOn"] = Value::Array(based_on.clone());
            }
            if let Some(part_of) = &part_of {
                observation["partOf"] = Value::Array(part_of.clone());
            }
            Ok(observation)
        })
        .collect::<CpgResult<Vec<_>>>()?;

    // A direct caller may inspect a partial response, but a final calculated
    // score is never inferred from partial input. This does not substitute a
    // false/default value for an unanswered Boolean.
    let required_answers_complete = specification
        .boolean_items
        .iter()
        .all(|item| !item.required || answers.contains_key(&item.link_id));
    if required_answers_complete {
        let response_without_scores =
            response_without_score_answers(response.clone(), &specification.calculated_score_items);
        for item in &specification.calculated_score_items {
            let calculated = evaluate_score(item, questionnaire, &response_without_scores)?;
            if let Some(supplied) = submitted_scores.get(&item.link_id) {
                if *supplied != calculated {
                    return Err(invalid(format!(
                        "QuestionnaireResponse calculated score '{}' is {supplied}, but its FHIRPath expression recomputes {calculated}",
                        item.link_id
                    )));
                }
            }
            observations.push(numeric_observation(
                item,
                calculated,
                &observation_provenance,
            ));
        }
    }

    Ok(observations)
}

/// Apply the preview workflow's completed/all-required-answer gate before
/// producing a transaction Bundle with one POST entry per Observation.
pub fn extract_completed_sdc_boolean_observations(
    questionnaire: &Value,
    response: &Value,
    subject: &str,
    encounter: &str,
) -> CpgResult<SdcObservationExtraction> {
    let specification = extraction_specification(questionnaire)?;
    validate_response_provenance(
        response,
        &specification.questionnaire_canonical,
        subject,
        encounter,
    )?;

    let answers = response_answers(response, &specification.boolean_items)?;

    if response.get("status").and_then(Value::as_str) != Some("completed") {
        return Ok(SdcObservationExtraction::NotInvoked {
            reason: "QuestionnaireResponse.status must be completed before SDC extraction"
                .to_string(),
        });
    }
    let missing = specification
        .boolean_items
        .iter()
        .filter(|item| item.required && !answers.contains_key(&item.link_id))
        .map(|item| item.link_id.as_str())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Ok(SdcObservationExtraction::NotInvoked {
            reason: format!(
                "QuestionnaireResponse is missing required Boolean answers: {}",
                missing.join(", ")
            ),
        });
    }

    let observations =
        extract_sdc_boolean_observations(questionnaire, response, subject, encounter)?;
    Ok(SdcObservationExtraction::Extracted {
        transaction: transaction_bundle(&observations),
        observations,
    })
}

#[derive(Debug)]
struct ExtractionSpecification {
    questionnaire_canonical: String,
    category: Option<Value>,
    boolean_items: Vec<BooleanItem>,
    calculated_score_items: Vec<CalculatedScoreItem>,
}

#[derive(Debug)]
struct BooleanItem {
    link_id: String,
    required: bool,
    coding: Value,
}

#[derive(Debug)]
struct CalculatedScoreItem {
    link_id: String,
    coding: Value,
    expression: String,
    questionnaire_item: Value,
}

fn extraction_specification(questionnaire: &Value) -> CpgResult<ExtractionSpecification> {
    require_resource_type(questionnaire, "Questionnaire")?;
    reject_modifier_extensions(questionnaire, "Questionnaire")?;
    if !has_sdc_extraction_profile(questionnaire) {
        return Err(invalid(format!(
            "Questionnaire.meta.profile must include {SDC_EXTRACTION_PROFILE}"
        )));
    }
    if !has_true_extension(questionnaire, OBSERVATION_EXTRACT) {
        return Err(invalid(format!(
            "Questionnaire must enable {OBSERVATION_EXTRACT} with valueBoolean=true"
        )));
    }

    let (boolean_items, calculated_score_items) = collect_extractable_items(questionnaire)?;
    if boolean_items.is_empty() {
        return Err(invalid(
            "Questionnaire has no Boolean items supported by the SDC extraction subset",
        ));
    }
    Ok(ExtractionSpecification {
        questionnaire_canonical: canonical_identity(questionnaire, "Questionnaire")?,
        category: extraction_category(questionnaire)?,
        boolean_items,
        calculated_score_items,
    })
}

fn has_sdc_extraction_profile(questionnaire: &Value) -> bool {
    questionnaire
        .pointer("/meta/profile")
        .and_then(Value::as_array)
        .is_some_and(|profiles| {
            profiles.iter().any(|profile| {
                profile
                    .as_str()
                    .is_some_and(|value| value.split('|').next() == Some(SDC_EXTRACTION_PROFILE))
            })
        })
}

fn has_true_extension(questionnaire: &Value, url: &str) -> bool {
    let matching = extensions(questionnaire)
        .into_iter()
        .filter(|extension| extension.get("url").and_then(Value::as_str) == Some(url))
        .collect::<Vec<_>>();
    matching.len() == 1 && matching[0].get("valueBoolean").and_then(Value::as_bool) == Some(true)
}

fn extraction_category(questionnaire: &Value) -> CpgResult<Option<Value>> {
    let categories = extensions(questionnaire)
        .into_iter()
        .filter(|extension| {
            extension.get("url").and_then(Value::as_str) == Some(OBSERVATION_CATEGORY)
        })
        .collect::<Vec<_>>();
    if categories.len() > 1 {
        return Err(invalid(format!(
            "Questionnaire may declare {OBSERVATION_CATEGORY} at most once"
        )));
    }
    let Some(category) = categories.first() else {
        return Ok(None);
    };
    let Some(value) = category.get("valueCodeableConcept") else {
        return Err(invalid(format!(
            "Questionnaire extension {OBSERVATION_CATEGORY} requires valueCodeableConcept"
        )));
    };
    let Some(codings) = value.get("coding").and_then(Value::as_array) else {
        return Err(invalid(format!(
            "Questionnaire extension {OBSERVATION_CATEGORY} requires coding"
        )));
    };
    if codings.is_empty() || codings.iter().any(|coding| !has_system_and_code(coding)) {
        return Err(invalid(format!(
            "Questionnaire extension {OBSERVATION_CATEGORY} has an invalid coding"
        )));
    }
    Ok(Some(value.clone()))
}

fn collect_extractable_items(
    questionnaire: &Value,
) -> CpgResult<(Vec<BooleanItem>, Vec<CalculatedScoreItem>)> {
    let Some(items) = questionnaire.get("item").and_then(Value::as_array) else {
        return Ok((Vec::new(), Vec::new()));
    };
    let mut boolean_items = Vec::new();
    let mut calculated_score_items = Vec::new();
    let mut link_ids = HashSet::new();

    for item in items {
        reject_unsupported_item_structure(item)?;
        let link_id = required_string(item, "linkId", "Questionnaire.item")?.to_string();
        if !link_ids.insert(link_id.clone()) {
            return Err(invalid(format!(
                "Questionnaire has duplicate extractable item linkId '{link_id}'"
            )));
        }
        match item.get("type").and_then(Value::as_str) {
            Some("boolean") => {
                reject_item_extensions(item, &link_id)?;
                boolean_items.push(BooleanItem {
                    link_id,
                    required: item.get("required").and_then(Value::as_bool) == Some(true),
                    coding: exactly_one_coding(item, "Questionnaire Boolean item")?,
                });
            }
            Some("integer") => {
                let expression = calculated_expression(item, &link_id)?;
                require_item_observation_extract(item, &link_id)?;
                if item.get("readOnly").and_then(Value::as_bool) != Some(true) {
                    return Err(invalid(format!(
                        "calculated integer score item '{link_id}' must declare readOnly=true"
                    )));
                }
                if item.get("initial").is_some() {
                    return Err(invalid(format!(
                        "calculated integer score item '{link_id}' must not declare initial"
                    )));
                }
                calculated_score_items.push(CalculatedScoreItem {
                    link_id,
                    coding: exactly_one_coding(item, "Questionnaire calculated integer score item")?,
                    expression,
                    questionnaire_item: item.clone(),
                });
            }
            other => return Err(invalid(format!(
                "Questionnaire item '{link_id}' has unsupported type '{}' for the constrained SDC extraction subset",
                other.unwrap_or("unknown")
            ))),
        }
    }
    Ok((boolean_items, calculated_score_items))
}

fn reject_unsupported_item_structure(item: &Value) -> CpgResult<()> {
    let link_id = item
        .get("linkId")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    reject_modifier_extensions(item, &format!("Questionnaire.item '{link_id}'"))?;
    if item.get("item").is_some() {
        return Err(invalid(format!(
            "Questionnaire item '{link_id}' has nested items, unsupported by the SDC Boolean extraction subset"
        )));
    }
    if item.get("enableWhen").is_some() || item.get("enableBehavior").is_some() {
        return Err(invalid(format!(
            "Questionnaire item '{link_id}' has conditional enablement, unsupported by the SDC Boolean extraction subset"
        )));
    }
    if item.get("repeats").and_then(Value::as_bool) == Some(true) {
        return Err(invalid(format!(
            "Questionnaire item '{link_id}' repeats, unsupported by the SDC Boolean extraction subset"
        )));
    }
    Ok(())
}

fn reject_item_extensions(item: &Value, link_id: &str) -> CpgResult<()> {
    if item
        .get("extension")
        .and_then(Value::as_array)
        .is_some_and(|extensions| !extensions.is_empty())
    {
        return Err(invalid(format!(
            "Questionnaire Boolean item '{link_id}' has extensions, unsupported by the constrained SDC extraction subset"
        )));
    }
    Ok(())
}

fn response_answers(response: &Value, items: &[BooleanItem]) -> CpgResult<HashMap<String, bool>> {
    require_resource_type(response, "QuestionnaireResponse")?;
    reject_modifier_extensions(response, "QuestionnaireResponse")?;
    let supported = items
        .iter()
        .map(|item| item.link_id.as_str())
        .collect::<HashSet<_>>();
    let mut answers = HashMap::new();
    let mut seen_items = HashSet::new();

    for item in flatten_response_items(response.get("item")) {
        reject_modifier_extensions(item, "QuestionnaireResponse.item")?;
        let Some(link_id) = item.get("linkId").and_then(Value::as_str) else {
            continue;
        };
        if !supported.contains(link_id) {
            continue;
        }
        if !seen_items.insert(link_id) {
            return Err(invalid(format!(
                "QuestionnaireResponse has duplicate item '{link_id}' for the non-repeating SDC Boolean extraction subset"
            )));
        }
        let Some(answer_array) = item.get("answer").and_then(Value::as_array) else {
            continue;
        };
        if answer_array.len() != 1 {
            return Err(invalid(format!(
                "QuestionnaireResponse item '{link_id}' must contain exactly one Boolean answer"
            )));
        }
        let answer = &answer_array[0];
        reject_modifier_extensions(answer, "QuestionnaireResponse.item.answer")?;
        let value_fields = answer
            .as_object()
            .map(|fields| {
                fields
                    .keys()
                    .filter(|field| field.starts_with("value"))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if value_fields.len() != 1 || value_fields[0] != "valueBoolean" {
            return Err(invalid(format!(
                "QuestionnaireResponse item '{link_id}' must contain exactly one value[x], valueBoolean"
            )));
        }
        let Some(value) = answer.get("valueBoolean").and_then(Value::as_bool) else {
            return Err(invalid(format!(
                "QuestionnaireResponse item '{link_id}' must contain valueBoolean"
            )));
        };
        if answers.insert(link_id.to_string(), value).is_some() {
            return Err(invalid(format!(
                "QuestionnaireResponse has duplicate answers for Boolean item '{link_id}'"
            )));
        }
    }
    Ok(answers)
}

/// Validate optional materialized calculated answers without using them as the
/// calculation input. SDC renderers may keep calculated answers in a
/// QuestionnaireResponse; a stale, wrong-choice, or duplicate value is an
/// invalid response rather than a reason to trust it.
fn response_scores(
    response: &Value,
    items: &[CalculatedScoreItem],
) -> CpgResult<HashMap<String, i32>> {
    let supported = items
        .iter()
        .map(|item| item.link_id.as_str())
        .collect::<HashSet<_>>();
    let mut scores = HashMap::new();
    let mut seen_items = HashSet::new();

    for item in flatten_response_items(response.get("item")) {
        let Some(link_id) = item.get("linkId").and_then(Value::as_str) else {
            continue;
        };
        if !supported.contains(link_id) {
            continue;
        }
        if !seen_items.insert(link_id) {
            return Err(invalid(format!(
                "QuestionnaireResponse has duplicate calculated score item '{link_id}'"
            )));
        }
        let Some(answer_array) = item.get("answer").and_then(Value::as_array) else {
            continue;
        };
        if answer_array.len() != 1 {
            return Err(invalid(format!(
                "QuestionnaireResponse calculated score item '{link_id}' must contain exactly one valueInteger answer when materialized"
            )));
        }
        let answer = &answer_array[0];
        reject_modifier_extensions(answer, "QuestionnaireResponse.item.answer")?;
        let value_fields = answer
            .as_object()
            .map(|fields| {
                fields
                    .keys()
                    .filter(|field| field.starts_with("value"))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if value_fields.len() != 1 || value_fields[0] != "valueInteger" {
            return Err(invalid(format!(
                "QuestionnaireResponse calculated score item '{link_id}' must contain exactly one value[x], valueInteger"
            )));
        }
        let value = answer
            .get("valueInteger")
            .and_then(Value::as_i64)
            .and_then(|value| i32::try_from(value).ok())
            .ok_or_else(|| {
                invalid(format!(
                    "QuestionnaireResponse calculated score item '{link_id}' must contain a 32-bit valueInteger"
                ))
            })?;
        scores.insert(link_id.to_string(), value);
    }
    Ok(scores)
}

fn calculated_expression(item: &Value, link_id: &str) -> CpgResult<String> {
    let matching = extensions(item)
        .into_iter()
        .filter(|extension| {
            extension.get("url").and_then(Value::as_str)
                == Some("http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-calculatedExpression")
        })
        .collect::<Vec<_>>();
    if matching.len() != 1 {
        return Err(invalid(format!(
            "calculated integer score item '{link_id}' requires exactly one sdc-questionnaire-calculatedExpression extension"
        )));
    }
    let expression = matching[0].get("valueExpression").ok_or_else(|| {
        invalid(format!(
            "calculated integer score item '{link_id}' requires valueExpression"
        ))
    })?;
    if expression.get("language").and_then(Value::as_str) != Some("text/fhirpath") {
        return Err(invalid(format!(
            "calculated integer score item '{link_id}' requires valueExpression.language text/fhirpath"
        )));
    }
    required_string(
        expression,
        "expression",
        "calculated integer score valueExpression",
    )
    .map(str::to_string)
}

fn require_item_observation_extract(item: &Value, link_id: &str) -> CpgResult<()> {
    if !has_true_extension(item, OBSERVATION_EXTRACT) {
        return Err(invalid(format!(
            "calculated integer score item '{link_id}' must enable {OBSERVATION_EXTRACT} with valueBoolean=true"
        )));
    }
    // Apart from the two explicitly supported SDC extensions, accepting an
    // item extension would claim semantics this small extractor does not have.
    if extensions(item).into_iter().any(|extension| {
        !matches!(
            extension.get("url").and_then(Value::as_str),
            Some("http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-calculatedExpression")
                | Some(OBSERVATION_EXTRACT)
        )
    }) {
        return Err(invalid(format!(
            "calculated integer score item '{link_id}' has unsupported extensions"
        )));
    }
    Ok(())
}

fn response_without_score_answers(
    mut response: Value,
    score_items: &[CalculatedScoreItem],
) -> Value {
    let score_link_ids = score_items
        .iter()
        .map(|item| item.link_id.as_str())
        .collect::<HashSet<_>>();
    strip_score_answers(response.get_mut("item"), &score_link_ids);
    response
}

fn strip_score_answers(items: Option<&mut Value>, score_link_ids: &HashSet<&str>) {
    let Some(items) = items.and_then(Value::as_array_mut) else {
        return;
    };
    for item in items {
        if item
            .get("linkId")
            .and_then(Value::as_str)
            .is_some_and(|link_id| score_link_ids.contains(link_id))
        {
            item.as_object_mut()
                .expect("FHIR JSON item object")
                .remove("answer");
        }
        strip_score_answers(item.get_mut("item"), score_link_ids);
        if let Some(answers) = item.get_mut("answer").and_then(Value::as_array_mut) {
            for answer in answers {
                strip_score_answers(answer.get_mut("item"), score_link_ids);
            }
        }
    }
}

fn evaluate_score(
    item: &CalculatedScoreItem,
    questionnaire: &Value,
    response_without_scores: &Value,
) -> CpgResult<i32> {
    let parsed = FhirPathParser::new()
        .parse(&item.expression)
        .map_err(|error| {
            invalid(format!(
                "calculated integer score item '{}' has invalid FHIRPath expression: {error}",
                item.link_id
            ))
        })?;
    let current_item = response_without_scores
        .get("item")
        .and_then(Value::as_array)
        .and_then(|items| {
            items.iter().find(|response_item| {
                response_item.get("linkId").and_then(Value::as_str) == Some(&item.link_id)
            })
        })
        .cloned()
        .unwrap_or_else(|| json!({ "linkId": item.link_id }));
    let mut context =
        EvaluationContext::new(response_without_scores.clone()).with_current(current_item);
    context.add_constant(
        "questionnaire".to_string(),
        FhirPathValue::Object(questionnaire.clone()),
    );
    context.add_constant(
        "qitem".to_string(),
        FhirPathValue::Object(item.questionnaire_item.clone()),
    );
    let value = FhirPathEvaluator::new()
        .evaluate(&parsed, &context)
        .map_err(|error| {
            invalid(format!(
                "calculated integer score item '{}' FHIRPath evaluation failed: {error}",
                item.link_id
            ))
        })?;
    single_integral_score(value, &item.link_id)
}

fn single_integral_score(value: FhirPathValue, link_id: &str) -> CpgResult<i32> {
    let single = match value {
        FhirPathValue::Collection(mut values) | FhirPathValue::UnorderedCollection(mut values)
            if values.len() == 1 => values.remove(0),
        FhirPathValue::Collection(_) | FhirPathValue::UnorderedCollection(_) | FhirPathValue::Empty => {
            return Err(invalid(format!(
                "calculated integer score item '{link_id}' FHIRPath expression must return exactly one integral numeric result"
            )))
        }
        value => value,
    };
    let integral = match single {
        FhirPathValue::Integer(value) | FhirPathValue::Long(value) => value,
        FhirPathValue::Number(value) if value.fract().is_zero() => value.to_string().parse::<i64>().map_err(|_| invalid(format!(
            "calculated integer score item '{link_id}' result is outside integer range"
        )))?,
        _ => return Err(invalid(format!(
            "calculated integer score item '{link_id}' FHIRPath expression must return exactly one integral numeric result"
        ))),
    };
    i32::try_from(integral).map_err(|_| {
        invalid(format!(
            "calculated integer score item '{link_id}' result is outside 32-bit valueInteger range"
        ))
    })
}

struct ObservationProvenance<'a> {
    subject: &'a str,
    encounter: &'a str,
    authored: &'a str,
    author: &'a str,
    response_reference: &'a str,
    category: Option<&'a Value>,
    security: Option<&'a Value>,
    based_on: Option<&'a [Value]>,
    part_of: Option<&'a [Value]>,
}

fn numeric_observation(
    item: &CalculatedScoreItem,
    score: i32,
    provenance: &ObservationProvenance<'_>,
) -> Value {
    let mut observation = json!({
        "resourceType": "Observation", "status": "final",
        "code": { "coding": [item.coding.clone()] },
        "subject": { "reference": provenance.subject }, "encounter": { "reference": provenance.encounter },
        "effectiveDateTime": provenance.authored, "issued": provenance.authored,
        "performer": [{ "reference": provenance.author }], "valueInteger": score,
        "derivedFrom": [{ "reference": provenance.response_reference }],
    });
    if let Some(category) = provenance.category {
        observation["category"] = Value::Array(vec![category.clone()]);
    }
    if let Some(security) = provenance.security {
        observation["meta"] = json!({ "security": security });
    }
    if let Some(based_on) = provenance.based_on {
        observation["basedOn"] = Value::Array(based_on.to_vec());
    }
    if let Some(part_of) = provenance.part_of {
        observation["partOf"] = Value::Array(part_of.to_vec());
    }
    observation
}

fn validate_response_provenance(
    response: &Value,
    questionnaire_canonical: &str,
    subject: &str,
    encounter: &str,
) -> CpgResult<()> {
    require_resource_type(response, "QuestionnaireResponse")?;
    let response_questionnaire =
        required_string(response, "questionnaire", "QuestionnaireResponse")?;
    if response_questionnaire != questionnaire_canonical {
        return Err(invalid(format!(
            "QuestionnaireResponse.questionnaire '{response_questionnaire}' does not match Questionnaire canonical '{questionnaire_canonical}'"
        )));
    }
    let response_subject = required_reference(response, "subject", "QuestionnaireResponse")?;
    if response_subject != subject {
        return Err(invalid(format!(
            "QuestionnaireResponse.subject '{response_subject}' does not match selected subject '{subject}'"
        )));
    }
    let response_encounter = required_reference(response, "encounter", "QuestionnaireResponse")?;
    if response_encounter != encounter {
        return Err(invalid(format!(
            "QuestionnaireResponse.encounter '{response_encounter}' does not match selected encounter '{encounter}'"
        )));
    }
    required_string(response, "id", "QuestionnaireResponse")?;
    let authored = required_string(response, "authored", "QuestionnaireResponse")?;
    DateTime::parse_from_rfc3339(authored).map_err(|_| {
        invalid("QuestionnaireResponse.authored must be an RFC 3339 date-time with timezone for SDC Observation extraction")
    })?;
    required_reference(response, "author", "QuestionnaireResponse")?;
    Ok(())
}

fn transaction_bundle(observations: &[Value]) -> Value {
    json!({
        "resourceType": "Bundle",
        "type": "transaction",
        "entry": observations.iter().map(|observation| json!({
            "fullUrl": format!("urn:uuid:{}", Uuid::new_v4()),
            "resource": observation,
            "request": { "method": "POST", "url": "Observation" }
        })).collect::<Vec<_>>()
    })
}

fn canonical_identity(resource: &Value, resource_name: &str) -> CpgResult<String> {
    Ok(format!(
        "{}|{}",
        required_string(resource, "url", resource_name)?,
        required_string(resource, "version", resource_name)?
    ))
}

fn extensions(resource: &Value) -> Vec<&Value> {
    resource
        .get("extension")
        .and_then(Value::as_array)
        .map(|extensions| extensions.iter().collect())
        .unwrap_or_default()
}

fn exactly_one_coding(item: &Value, owner: &str) -> CpgResult<Value> {
    let codings = item
        .get("code")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid(format!("{owner} requires a Coding in code")))?;
    if codings.len() != 1 || !is_complete_coding(&codings[0]) {
        return Err(invalid(format!(
            "{owner} requires exactly one coding with system, version, and code"
        )));
    }
    Ok(codings[0].clone())
}

fn is_complete_coding(value: &Value) -> bool {
    has_system_and_code(value)
        && value
            .get("version")
            .and_then(Value::as_str)
            .is_some_and(|value| !value.is_empty())
}

fn has_system_and_code(value: &Value) -> bool {
    ["system", "code"].iter().all(|field| {
        value
            .get(*field)
            .and_then(Value::as_str)
            .is_some_and(|value| !value.is_empty())
    })
}

fn flatten_response_items(items: Option<&Value>) -> Vec<&Value> {
    let Some(items) = items.and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut flattened = Vec::new();
    for item in items {
        flattened.push(item);
        flattened.extend(flatten_response_items(item.get("item")));
        if let Some(answers) = item.get("answer").and_then(Value::as_array) {
            for answer in answers {
                flattened.extend(flatten_response_items(answer.get("item")));
            }
        }
    }
    flattened
}

fn reject_modifier_extensions(resource: &Value, path: &str) -> CpgResult<()> {
    if resource
        .get("modifierExtension")
        .and_then(Value::as_array)
        .is_some_and(|extensions| !extensions.is_empty())
    {
        return Err(invalid(format!(
            "{path}.modifierExtension is unsupported by the SDC Boolean extraction subset"
        )));
    }
    Ok(())
}

fn optional_security_labels(response: &Value) -> CpgResult<Option<Value>> {
    let Some(security) = response.pointer("/meta/security") else {
        return Ok(None);
    };
    let Some(security) = security.as_array() else {
        return Err(invalid(
            "QuestionnaireResponse.meta.security must be an array",
        ));
    };
    if security.iter().any(|coding| !has_system_and_code(coding)) {
        return Err(invalid(
            "QuestionnaireResponse.meta.security contains an invalid coding",
        ));
    }
    Ok(Some(Value::Array(security.clone())))
}

fn optional_reference_array(
    response: &Value,
    field: &str,
    allowed_types: &[&str],
    path: &str,
) -> CpgResult<Option<Vec<Value>>> {
    let Some(references) = response.get(field) else {
        return Ok(None);
    };
    let Some(references) = references.as_array() else {
        return Err(invalid(format!("{path} must be an array of references")));
    };
    for reference in references {
        let value = reference
            .get("reference")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid(format!("{path} requires reference")))?;
        validate_reference_type(value, allowed_types, path)?;
    }
    Ok(Some(references.clone()))
}

fn validate_reference_type(reference: &str, allowed_types: &[&str], path: &str) -> CpgResult<()> {
    let Some((resource_type, id)) = reference.split_once('/') else {
        return Err(invalid(format!(
            "{path} reference '{reference}' must be a relative ResourceType/id reference"
        )));
    };
    if id.is_empty() || !allowed_types.contains(&resource_type) {
        return Err(invalid(format!(
            "{path} reference '{reference}' is not permitted for Observation"
        )));
    }
    Ok(())
}

fn required_reference<'a>(
    resource: &'a Value,
    field: &str,
    resource_name: &str,
) -> CpgResult<&'a str> {
    resource
        .get(field)
        .and_then(|value| value.get("reference"))
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            invalid(format!(
                "{resource_name}.{field}.reference is required for SDC Observation extraction"
            ))
        })
}

fn required_string<'a>(
    resource: &'a Value,
    field: &str,
    resource_name: &str,
) -> CpgResult<&'a str> {
    resource
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            invalid(format!(
                "{resource_name}.{field} is required for SDC Observation extraction"
            ))
        })
}

fn require_resource_type(resource: &Value, resource_type: &str) -> CpgResult<()> {
    if resource.get("resourceType").and_then(Value::as_str) != Some(resource_type) {
        return Err(invalid(format!("expected resourceType {resource_type}")));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> CpgError {
    CpgError::InvalidResource(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUESTIONNAIRE_URL: &str = "https://example.org/fhir/Questionnaire/three-question-screen";
    const SUBJECT: &str = "Patient/patient-1";
    const ENCOUNTER: &str = "Encounter/encounter-1";

    fn questionnaire() -> Value {
        json!({
            "resourceType": "Questionnaire",
            "url": QUESTIONNAIRE_URL,
            "version": "0.2.0",
            "meta": { "profile": [format!("{SDC_EXTRACTION_PROFILE}|4.0.0")] },
            "extension": [
                { "url": OBSERVATION_EXTRACT, "valueBoolean": true },
                { "url": OBSERVATION_CATEGORY, "valueCodeableConcept": { "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/observation-category",
                    "code": "survey", "display": "Survey"
                }]}}
            ],
            "item": [
                {"linkId":"unsteady","type":"boolean","required":true,"code":[{
                    "system":"http://loinc.org","version":"2.81","code":"100257-5","display":"Unsteady"
                }]},
                {"linkId":"worried","type":"boolean","required":true,"code":[{
                    "system":"http://loinc.org","version":"2.81","code":"97878-3","display":"Worried"
                }]},
                {"linkId":"fallen","type":"boolean","required":true,"code":[{
                    "system":"http://loinc.org","version":"2.81","code":"52552-7","display":"Fallen"
                }]}
            ]
        })
    }

    fn response(status: &str) -> Value {
        json!({
            "resourceType":"QuestionnaireResponse", "id":"response-1",
            "questionnaire":format!("{QUESTIONNAIRE_URL}|0.2.0"), "status":status,
            "subject":{"reference":SUBJECT}, "encounter":{"reference":ENCOUNTER},
            "authored":"2026-06-15T09:20:00Z", "author":{"reference":SUBJECT},
            "meta":{"security":[{"system":"http://terminology.hl7.org/CodeSystem/v3-ActReason","code":"HTEST","display":"test health data"}]},
            "item":[
                {"linkId":"unsteady","answer":[{"valueBoolean":true}]},
                {"linkId":"worried","answer":[{"valueBoolean":false}]},
                {"linkId":"fallen","answer":[{"valueBoolean":false}]}
            ]
        })
    }

    fn scored_questionnaire() -> Value {
        let mut value = questionnaire();
        value["item"].as_array_mut().expect("items").push(json!({
            "linkId":"arbitrary-final-score", "type":"integer", "readOnly":true,
            "code":[{"system":"https://example.org/codes","version":"2026","code":"screen-score"}],
            "extension":[
                {"url":OBSERVATION_EXTRACT,"valueBoolean":true},
                {"url":"http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-calculatedExpression",
                 "valueExpression":{"language":"text/fhirpath","expression":"iif(%resource.item.where(linkId = 'unsteady').answer.valueBoolean, 1, 0) + iif(%resource.item.where(linkId = 'worried').answer.valueBoolean, 1, 0) + iif(%resource.item.where(linkId = 'fallen').answer.valueBoolean, 1, 0)"}}
            ]
        }));
        value
    }

    fn scored_response(status: &str, supplied_score: Option<i32>) -> Value {
        let mut value = response(status);
        if let Some(score) = supplied_score {
            value["item"].as_array_mut().expect("items").push(json!({
                "linkId":"arbitrary-final-score", "answer":[{"valueInteger":score}]
            }));
        }
        value
    }

    #[test]
    fn emits_transaction_with_complete_source_provenance() {
        let output = extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &response("completed"),
            SUBJECT,
            ENCOUNTER,
        )
        .expect("extraction succeeds");
        let SdcObservationExtraction::Extracted {
            transaction,
            observations,
        } = output
        else {
            panic!("expected extraction");
        };
        assert_eq!(transaction["resourceType"], "Bundle");
        assert_eq!(transaction["type"], "transaction");
        assert_eq!(transaction["entry"].as_array().map(Vec::len), Some(3));
        assert!(transaction["entry"]
            .as_array()
            .expect("entries")
            .iter()
            .all(|entry| entry["fullUrl"]
                .as_str()
                .is_some_and(|value| value.starts_with("urn:uuid:"))
                && entry["request"] == json!({"method":"POST","url":"Observation"})));
        assert_eq!(observations[0]["status"], "final");
        assert_eq!(observations[0]["code"]["coding"][0]["version"], "2.81");
        assert_eq!(observations[0]["valueBoolean"], true);
        assert_eq!(observations[0]["subject"]["reference"], SUBJECT);
        assert_eq!(observations[0]["encounter"]["reference"], ENCOUNTER);
        assert_eq!(observations[0]["effectiveDateTime"], "2026-06-15T09:20:00Z");
        assert_eq!(observations[0]["issued"], "2026-06-15T09:20:00Z");
        assert_eq!(observations[0]["performer"][0]["reference"], SUBJECT);
        assert_eq!(
            observations[0]["derivedFrom"][0]["reference"],
            "QuestionnaireResponse/response-1"
        );
        assert_eq!(
            observations[0]["category"][0]["coding"][0]["code"],
            "survey"
        );
        assert_eq!(observations[0]["meta"]["security"][0]["code"], "HTEST");
    }

    #[test]
    fn direct_extraction_allows_partial_but_workflow_does_not_invoke() {
        let mut partial = response("in-progress");
        partial["item"].as_array_mut().expect("items").truncate(1);
        let direct =
            extract_sdc_boolean_observations(&questionnaire(), &partial, SUBJECT, ENCOUNTER)
                .expect("direct subset permits partial response");
        assert_eq!(direct.len(), 1);
        let workflow = extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &partial,
            SUBJECT,
            ENCOUNTER,
        )
        .expect("in-progress is a normal non-invocation");
        assert!(matches!(
            workflow,
            SdcObservationExtraction::NotInvoked { .. }
        ));
    }

    #[test]
    fn completed_missing_required_answer_is_not_invoked() {
        let mut incomplete = response("completed");
        incomplete["item"].as_array_mut().expect("items").pop();
        let output = extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &incomplete,
            SUBJECT,
            ENCOUNTER,
        )
        .expect("incomplete is not a malformed extraction");
        assert!(matches!(
            output,
            SdcObservationExtraction::NotInvoked { .. }
        ));
    }

    #[test]
    fn extracts_metadata_driven_recomputed_integer_score_and_preserves_zero() {
        let extracted = extract_completed_sdc_boolean_observations(
            &scored_questionnaire(),
            &scored_response("completed", Some(1)),
            SUBJECT,
            ENCOUNTER,
        )
        .expect("matching calculated score may be materialized by an SDC renderer");
        let SdcObservationExtraction::Extracted { observations, .. } = extracted else {
            panic!("complete response must extract");
        };
        assert_eq!(observations.len(), 4);
        let score = observations.last().expect("score observation");
        assert_eq!(score["valueInteger"], 1);
        assert_eq!(score["code"]["coding"][0]["code"], "screen-score");

        let mut no_yes = scored_response("completed", Some(0));
        for item in no_yes["item"]
            .as_array_mut()
            .expect("items")
            .iter_mut()
            .take(3)
        {
            item["answer"][0]["valueBoolean"] = Value::Bool(false);
        }
        let zero = extract_completed_sdc_boolean_observations(
            &scored_questionnaire(),
            &no_yes,
            SUBJECT,
            ENCOUNTER,
        )
        .expect("zero is a valid complete score");
        let SdcObservationExtraction::Extracted { observations, .. } = zero else {
            panic!("complete response must extract");
        };
        assert_eq!(observations.last().expect("score")["valueInteger"], 0);

        let mut two_yes = scored_response("completed", Some(2));
        two_yes["item"][1]["answer"][0]["valueBoolean"] = Value::Bool(true);
        let two = extract_completed_sdc_boolean_observations(
            &scored_questionnaire(),
            &two_yes,
            SUBJECT,
            ENCOUNTER,
        )
        .expect("two affirmative answers are calculated from metadata");
        let SdcObservationExtraction::Extracted { observations, .. } = two else {
            panic!("complete response must extract");
        };
        assert_eq!(observations.last().expect("score")["valueInteger"], 2);

        let mut three_yes = scored_response("completed", Some(3));
        three_yes["item"][1]["answer"][0]["valueBoolean"] = Value::Bool(true);
        three_yes["item"][2]["answer"][0]["valueBoolean"] = Value::Bool(true);
        let three = extract_completed_sdc_boolean_observations(
            &scored_questionnaire(),
            &three_yes,
            SUBJECT,
            ENCOUNTER,
        )
        .expect("three affirmative answers are calculated from metadata");
        let SdcObservationExtraction::Extracted { observations, .. } = three else {
            panic!("complete response must extract");
        };
        assert_eq!(observations.last().expect("score")["valueInteger"], 3);
    }

    #[test]
    fn score_is_recomputed_not_trusted_and_partial_never_defaults() {
        let error = extract_completed_sdc_boolean_observations(
            &scored_questionnaire(),
            &scored_response("completed", Some(3)),
            SUBJECT,
            ENCOUNTER,
        )
        .expect_err("stale calculated value must be visible");
        assert!(error.to_string().contains("recomputes 1"));

        let mut partial = scored_response("in-progress", None);
        partial["item"].as_array_mut().expect("items").truncate(1);
        let direct =
            extract_sdc_boolean_observations(&scored_questionnaire(), &partial, SUBJECT, ENCOUNTER)
                .expect("direct partial extraction remains available");
        assert_eq!(
            direct.len(),
            1,
            "partial inputs do not receive a default score"
        );

        let mut wrong_choice = scored_response("completed", None);
        wrong_choice["item"]
            .as_array_mut()
            .expect("items")
            .push(json!({
                "linkId":"arbitrary-final-score", "answer":[{"valueQuantity":{"value":1}}]
            }));
        assert!(extract_completed_sdc_boolean_observations(
            &scored_questionnaire(),
            &wrong_choice,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());
    }

    #[test]
    fn evaluates_sdc_count_of_type_boolean_expression() {
        let mut questionnaire = scored_questionnaire();
        questionnaire["item"][3]["extension"][1]["valueExpression"]["expression"] = json!(
            "iif(%resource.item.answer.value.ofType(boolean).where($this = true).count() >= 1, 1, 0)"
        );
        let output = extract_completed_sdc_boolean_observations(
            &questionnaire,
            &scored_response("completed", Some(1)),
            SUBJECT,
            ENCOUNTER,
        )
        .expect("standard FHIRPath Boolean count expression is supported");
        let SdcObservationExtraction::Extracted { observations, .. } = output else {
            panic!("complete response must extract");
        };
        assert_eq!(observations.last().expect("score")["valueInteger"], 1);
    }

    #[test]
    fn rejects_ambiguous_or_unsupported_calculated_score_metadata() {
        let mut missing_item_flag = scored_questionnaire();
        missing_item_flag["item"][3]["extension"] = json!([{
            "url":"http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-calculatedExpression",
            "valueExpression":{"language":"text/fhirpath","expression":"1"}
        }]);
        assert!(extract_completed_sdc_boolean_observations(
            &missing_item_flag,
            &scored_response("completed", None),
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut duplicate_score = scored_response("completed", Some(1));
        duplicate_score["item"]
            .as_array_mut()
            .expect("items")
            .push(json!({
                "linkId":"arbitrary-final-score", "answer":[{"valueInteger":1}]
            }));
        assert!(extract_completed_sdc_boolean_observations(
            &scored_questionnaire(),
            &duplicate_score,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut non_integral = scored_questionnaire();
        non_integral["item"][3]["extension"][1]["valueExpression"]["expression"] = json!("1.5");
        assert!(extract_completed_sdc_boolean_observations(
            &non_integral,
            &scored_response("completed", None),
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut self_referential = scored_questionnaire();
        self_referential["item"][3]["extension"][1]["valueExpression"]["expression"] =
            json!("%resource.item.where(linkId = 'arbitrary-final-score').answer.valueInteger + 1");
        let error = extract_completed_sdc_boolean_observations(
            &self_referential,
            &scored_response("completed", Some(1)),
            SUBJECT,
            ENCOUNTER,
        )
        .expect_err("a supplied calculated answer cannot become calculation input");
        assert!(error
            .to_string()
            .contains("exactly one integral numeric result"));
    }

    #[test]
    fn rejects_mismatched_or_unsupported_input_without_guessing() {
        let mut wrong_encounter = response("completed");
        wrong_encounter["encounter"]["reference"] = Value::String("Encounter/other".to_string());
        assert!(extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &wrong_encounter,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut duplicate = response("completed");
        duplicate["item"]
            .as_array_mut()
            .expect("items")
            .push(json!({"linkId":"unsteady"}));
        assert!(extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &duplicate,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut unsupported = questionnaire();
        unsupported["extension"] = Value::Array(vec![]);
        assert!(extract_completed_sdc_boolean_observations(
            &unsupported,
            &response("completed"),
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut device_author = response("completed");
        device_author["author"]["reference"] = Value::String("Device/device-1".to_string());
        assert!(extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &device_author,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut duplicate_root_flag = questionnaire();
        duplicate_root_flag["extension"]
            .as_array_mut()
            .expect("extensions")
            .push(json!({
                "url": OBSERVATION_EXTRACT,
                "valueBoolean": false
            }));
        assert!(extract_completed_sdc_boolean_observations(
            &duplicate_root_flag,
            &response("completed"),
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut unsupported_relationship = questionnaire();
        unsupported_relationship["item"][0]["enableWhen"] = json!([]);
        assert!(extract_completed_sdc_boolean_observations(
            &unsupported_relationship,
            &response("completed"),
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut invalid_authored = response("completed");
        invalid_authored["authored"] = Value::String("2026-06-15".to_string());
        assert!(extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &invalid_authored,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut ambiguous_choice = response("completed");
        ambiguous_choice["item"][0]["answer"][0]["valueString"] = Value::String("true".to_string());
        assert!(extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &ambiguous_choice,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut modifier_extension = response("completed");
        modifier_extension["modifierExtension"] = json!([{"url":"https://example.org/modifier"}]);
        assert!(extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &modifier_extension,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());

        let mut malformed_in_progress = response("in-progress");
        malformed_in_progress["item"][0]["answer"][0] = json!({"valueString":"true"});
        assert!(extract_completed_sdc_boolean_observations(
            &questionnaire(),
            &malformed_in_progress,
            SUBJECT,
            ENCOUNTER
        )
        .is_err());
    }
}
