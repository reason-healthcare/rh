//! Constrained SDC Boolean independent-Observation extraction.
//!
//! This module is deliberately smaller than the SDC QuestionnaireResponse
//! extraction operation. It supports source-profiled Boolean question items and
//! returns a standard FHIR transaction Bundle for callers that invoke it.

use chrono::DateTime;
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

/// Extract usable Boolean answers without imposing clinical completion gating.
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

    let response_reference = format!(
        "QuestionnaireResponse/{}",
        required_string(response, "id", "QuestionnaireResponse")?
    );
    let authored = required_string(response, "authored", "QuestionnaireResponse")?;
    let author = required_reference(response, "author", "QuestionnaireResponse")?;
    validate_reference_type(
        &author,
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

    specification
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
        .collect()
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
}

#[derive(Debug)]
struct BooleanItem {
    link_id: String,
    required: bool,
    coding: Value,
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

    let boolean_items = collect_boolean_items(questionnaire)?;
    if boolean_items.is_empty() {
        return Err(invalid(
            "Questionnaire has no Boolean items supported by the SDC extraction subset",
        ));
    }
    Ok(ExtractionSpecification {
        questionnaire_canonical: canonical_identity(questionnaire, "Questionnaire")?,
        category: extraction_category(questionnaire)?,
        boolean_items,
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

fn collect_boolean_items(questionnaire: &Value) -> CpgResult<Vec<BooleanItem>> {
    let Some(items) = questionnaire.get("item").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };
    let mut output = Vec::new();
    let mut link_ids = HashSet::new();

    for item in items {
        reject_unsupported_item_metadata(item)?;
        if item.get("type").and_then(Value::as_str) != Some("boolean") {
            return Err(invalid(format!(
                "Questionnaire item '{}' has unsupported type '{}' for the SDC Boolean extraction subset",
                item.get("linkId").and_then(Value::as_str).unwrap_or("unknown"),
                item.get("type").and_then(Value::as_str).unwrap_or("unknown")
            )));
        }
        let link_id = required_string(item, "linkId", "Questionnaire.item")?.to_string();
        if !link_ids.insert(link_id.clone()) {
            return Err(invalid(format!(
                "Questionnaire has duplicate Boolean item linkId '{link_id}'"
            )));
        }
        output.push(BooleanItem {
            link_id,
            required: item.get("required").and_then(Value::as_bool) == Some(true),
            coding: exactly_one_coding(item, "Questionnaire Boolean item")?,
        });
    }
    Ok(output)
}

fn reject_unsupported_item_metadata(item: &Value) -> CpgResult<()> {
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
    if item
        .get("extension")
        .and_then(Value::as_array)
        .is_some_and(|extensions| !extensions.is_empty())
    {
        return Err(invalid(format!(
            "Questionnaire item '{link_id}' has extraction or relationship extensions, unsupported by the SDC Boolean extraction subset"
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

    for item in flatten_response_items(response.get("item")) {
        reject_modifier_extensions(item, "QuestionnaireResponse.item")?;
        let Some(link_id) = item.get("linkId").and_then(Value::as_str) else {
            continue;
        };
        if !supported.contains(link_id) {
            continue;
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
            .push(json!({"linkId":"unsteady","answer":[{"valueBoolean":true}]}));
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
