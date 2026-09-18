use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::apply::dynamic_value::process_dynamic_value;
use crate::context::ApplyContext;
use crate::error::{CpgError, CpgResult};

/// Apply an ActivityDefinition to a subject, producing the target request
/// resource. Returns `None` when the definition has no supported request kind.
pub fn apply_activity_definition(
    activity_definition: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> CpgResult<Option<Value>> {
    ctx.validate_execution_context()?;
    let Some(kind) = non_null_field(activity_definition, "kind").and_then(Value::as_str) else {
        return Ok(None);
    };

    if !matches!(
        kind,
        "Appointment"
            | "CarePlan"
            | "CommunicationRequest"
            | "DeviceRequest"
            | "ImmunizationRecommendation"
            | "MedicationRequest"
            | "NutritionOrder"
            | "ServiceRequest"
            | "SupplyRequest"
            | "Task"
    ) {
        return Ok(None);
    }

    let mut target = json!({
        "id": Uuid::new_v4().to_string(),
        "resourceType": kind
    });
    let canonical = canonicalize(activity_definition);

    if let Some(status) = default_status(kind) {
        set_field(&mut target, "status", json!(status));
    }

    if supports_intent(kind) {
        set_field(
            &mut target,
            "intent",
            validated_intent(kind, activity_definition)?,
        );
    }

    let subject = &ctx.subject;
    match kind {
        "Appointment" => {
            if let Some(timing_period) = field(activity_definition, "timingPeriod") {
                push_or_create_array(&mut target, "requestedPeriod", timing_period.clone());
            }
            set_field(
                &mut target,
                "participant",
                json!([{
                    "actor": reference_from_string(subject, "Patient"),
                    "status": "needs-action"
                }]),
            );
        }
        "CarePlan" => {
            push_canonical(&mut target, canonical.as_deref());
            copy_field(&mut target, activity_definition, "timingPeriod", "period");
            if let Some(encounter) = &ctx.encounter {
                set_field(
                    &mut target,
                    "encounter",
                    reference_from_string(encounter, "Encounter"),
                );
            }
            if let Some(practitioner) = &ctx.practitioner {
                set_field(
                    &mut target,
                    "author",
                    reference_from_string(practitioner, "Practitioner"),
                );
            }
            if let Some(organization) = &ctx.organization {
                push_or_create_array(
                    &mut target,
                    "contributor",
                    reference_from_string(organization, "Organization"),
                );
            }
            set_reference(&mut target, "subject", subject, "Patient");
        }
        "CommunicationRequest" => {
            copy_field(
                &mut target,
                activity_definition,
                "timingDateTime",
                "occurrenceDateTime",
            );
            copy_field(
                &mut target,
                activity_definition,
                "timingPeriod",
                "occurrencePeriod",
            );
            if let Some(practitioner) = &ctx.practitioner {
                set_field(
                    &mut target,
                    "requester",
                    reference_from_string(practitioner, "Practitioner"),
                );
            }
            if let Some(encounter) = &ctx.encounter {
                set_field(
                    &mut target,
                    "encounter",
                    reference_from_string(encounter, "Encounter"),
                );
            }
            set_reference(&mut target, "subject", subject, "Patient");
            copy_field(
                &mut target,
                activity_definition,
                "doNotPerform",
                "doNotPerform",
            );
        }
        "DeviceRequest" => {
            push_canonical(&mut target, canonical.as_deref());
            copy_field(
                &mut target,
                activity_definition,
                "code",
                "codeCodeableConcept",
            );
            copy_field(
                &mut target,
                activity_definition,
                "productCodeableConcept",
                "codeCodeableConcept",
            );
            copy_field(
                &mut target,
                activity_definition,
                "productReference",
                "codeReference",
            );
            set_context_reference(&mut target, &ctx.encounter, "encounter", "Encounter");
            set_context_reference(&mut target, &ctx.practitioner, "requester", "Practitioner");
            set_reference(&mut target, "subject", subject, "Patient");
        }
        "ImmunizationRecommendation" => {
            set_reference(&mut target, "patient", subject, "Patient");
            if let Some(product) = field(activity_definition, "productCodeableConcept") {
                set_field(
                    &mut target,
                    "recommendation",
                    json!([{
                        "vaccineCode": [product],
                        "forecastStatus": {
                            "coding": [{
                                "system": "http://terminology.hl7.org/CodeSystem/immunization-recommendation-status",
                                "code": "due",
                                "display": "Due"
                            }]
                        }
                    }]),
                );
            }
            if let Some(evaluation_date) = deterministic_datetime(ctx)? {
                set_field(&mut target, "date", json!(evaluation_date));
            }
        }
        "MedicationRequest" => {
            push_canonical(&mut target, canonical.as_deref());
            set_reference(&mut target, "subject", subject, "Patient");
            set_context_reference(&mut target, &ctx.practitioner, "requester", "Practitioner");
            set_context_reference(&mut target, &ctx.encounter, "encounter", "Encounter");
            copy_field(
                &mut target,
                activity_definition,
                "doNotPerform",
                "doNotPerform",
            );
            copy_field(
                &mut target,
                activity_definition,
                "productCodeableConcept",
                "medicationCodeableConcept",
            );
            copy_field(
                &mut target,
                activity_definition,
                "productReference",
                "medicationReference",
            );
            if let Some(Value::Array(dosage)) = field(activity_definition, "dosage") {
                for dosage_entry in dosage {
                    push_or_create_array(&mut target, "dosageInstruction", dosage_entry.clone());
                }
            }

            let mut new_dosage = Map::new();
            if let Some(timing_date_time) = field(activity_definition, "timingDateTime") {
                let timing = new_dosage
                    .entry("timing".to_string())
                    .or_insert_with(|| json!({}));
                push_or_create_array(timing, "event", timing_date_time.clone());
            }
            let timing_targets = [
                ("timingDuration", "boundsDuration"),
                ("timingPeriod", "boundsPeriod"),
                ("timingRange", "boundsRange"),
            ];
            for (source_field, target_field) in timing_targets {
                if let Some(timing_value) = field(activity_definition, source_field) {
                    let timing = new_dosage
                        .entry("timing".to_string())
                        .or_insert_with(|| json!({}));
                    let repeat = timing
                        .as_object_mut()
                        .unwrap()
                        .entry("repeat".to_string())
                        .or_insert_with(|| json!({}));
                    set_field(repeat, target_field, timing_value.clone());
                }
            }
            if let Some(timing_timing) = field(activity_definition, "timingTiming") {
                new_dosage.insert("timing".to_string(), timing_timing.clone());
            }
            if let Some(Value::Array(body_site)) = field(activity_definition, "bodySite") {
                if let Some(site) = body_site.first() {
                    new_dosage.insert("site".to_string(), site.clone());
                }
            }
            if let Some(quantity) = field(activity_definition, "quantity") {
                new_dosage.insert(
                    "doseAndRate".to_string(),
                    json!([{"doseQuantity": quantity}]),
                );
            }
            if !new_dosage.is_empty() {
                push_or_create_array(&mut target, "dosageInstruction", Value::Object(new_dosage));
            }
        }
        "NutritionOrder" => {
            push_canonical(&mut target, canonical.as_deref());
            set_reference(&mut target, "patient", subject, "Patient");
            set_context_reference(&mut target, &ctx.encounter, "encounter", "Encounter");
            set_context_reference(&mut target, &ctx.practitioner, "orderer", "Practitioner");
            if let Some(evaluation_date) = deterministic_datetime(ctx)? {
                set_field(&mut target, "dateTime", json!(evaluation_date));
            }
        }
        "ServiceRequest" => {
            push_canonical(&mut target, canonical.as_deref());
            set_reference(&mut target, "subject", subject, "Patient");
            set_context_reference(&mut target, &ctx.encounter, "encounter", "Encounter");
            set_context_reference(&mut target, &ctx.practitioner, "requester", "Practitioner");
            copy_field(
                &mut target,
                activity_definition,
                "doNotPerform",
                "doNotPerform",
            );
            copy_field(&mut target, activity_definition, "code", "code");
            copy_field(
                &mut target,
                activity_definition,
                "quantity",
                "quantityQuantity",
            );
            copy_field(&mut target, activity_definition, "bodySite", "bodySite");
            copy_field(
                &mut target,
                activity_definition,
                "productCodeableConcept",
                "code",
            );
            copy_field(
                &mut target,
                activity_definition,
                "timingTiming",
                "occurrenceTiming",
            );
            copy_field(
                &mut target,
                activity_definition,
                "timingDateTime",
                "occurrenceDateTime",
            );
            copy_field(
                &mut target,
                activity_definition,
                "timingPeriod",
                "occurrencePeriod",
            );
        }
        "SupplyRequest" => {
            set_context_reference(&mut target, &ctx.practitioner, "requester", "Practitioner");
            copy_field(&mut target, activity_definition, "quantity", "quantity");
            copy_field(
                &mut target,
                activity_definition,
                "productCodeableConcept",
                "itemCodeableConcept",
            );
            copy_field(
                &mut target,
                activity_definition,
                "productReference",
                "itemReference",
            );
            copy_field(
                &mut target,
                activity_definition,
                "timingTiming",
                "occurrenceTiming",
            );
            copy_field(
                &mut target,
                activity_definition,
                "timingDateTime",
                "occurrenceDateTime",
            );
            copy_field(
                &mut target,
                activity_definition,
                "timingPeriod",
                "occurrencePeriod",
            );
        }
        "Task" => {
            if let Some(canonical) = canonical.as_deref() {
                set_field(&mut target, "instantiatesCanonical", json!(canonical));
            }
            set_reference(&mut target, "for", subject, "Patient");
            set_context_reference(&mut target, &ctx.encounter, "encounter", "Encounter");
            set_context_reference(&mut target, &ctx.practitioner, "requester", "Practitioner");
            copy_field(&mut target, activity_definition, "code", "code");
            copy_field(&mut target, activity_definition, "priority", "priority");
            if let Some(intent) = field(activity_definition, "intent").and_then(Value::as_str) {
                set_field(
                    &mut target,
                    "intent",
                    json!(if intent == "directive" {
                        "unknown"
                    } else {
                        intent
                    }),
                );
            }
            copy_field(
                &mut target,
                activity_definition,
                "timingPeriod",
                "executionPeriod",
            );
            if field(activity_definition, "doNotPerform") == Some(&json!(true)) {
                set_field(
                    &mut target,
                    "modifierExtension",
                    json!([{
                        "url": "http://hl7.org/fhir/StructureDefinition/request-doNotPerform",
                        "valueBoolean": true
                    }]),
                );
            }
        }
        _ => {}
    }

    let dynamic_values = activity_definition
        .get("dynamicValue")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for dynamic_value in dynamic_values {
        process_dynamic_value(
            &dynamic_value,
            activity_definition,
            &mut target,
            library_canonicals,
            ctx,
        )?;
    }

    ensure_required_fields(&target)?;

    // ActivityDefinition.transform is intentionally unsupported.
    Ok(Some(target))
}

fn canonicalize(activity_definition: &Value) -> Option<String> {
    let url = activity_definition.get("url").and_then(Value::as_str)?;
    let canonical = match activity_definition.get("version").and_then(Value::as_str) {
        Some(version) => format!("{url}|{version}"),
        None => url.to_string(),
    };
    Some(canonical)
}

fn deterministic_datetime(ctx: &ApplyContext) -> CpgResult<Option<&str>> {
    let Some(value) = ctx.evaluation_date.as_deref() else {
        return Ok(None);
    };
    chrono::DateTime::parse_from_rfc3339(value).map_err(|error| {
        CpgError::InvalidResource(format!(
            "evaluation date must be an RFC 3339 date-time, got '{value}': {error}"
        ))
    })?;
    Ok(Some(value))
}

fn non_null_field<'a>(resource: &'a Value, field: &str) -> Option<&'a Value> {
    resource.get(field).filter(|value| !value.is_null())
}

fn field<'a>(resource: &'a Value, field: &str) -> Option<&'a Value> {
    non_null_field(resource, field)
}

fn copy_field(target: &mut Value, source: &Value, source_field: &str, target_field: &str) {
    if let Some(value) = field(source, source_field) {
        set_field(target, target_field, value.clone());
    }
}

fn set_field(target: &mut Value, field: &str, value: Value) {
    target
        .as_object_mut()
        .expect("target resource must be an object")
        .insert(field.to_string(), value);
}

fn push_or_create_array(target: &mut Value, field: &str, value: Value) {
    let object = target.as_object_mut().expect("target must be an object");
    let array = object
        .entry(field.to_string())
        .or_insert_with(|| Value::Array(Vec::new()));
    if !array.is_array() {
        *array = Value::Array(Vec::new());
    }
    array
        .as_array_mut()
        .expect("array was just normalized")
        .push(value);
}

fn push_canonical(target: &mut Value, canonical: Option<&str>) {
    if let Some(canonical) = canonical {
        push_or_create_array(target, "instantiatesCanonical", json!(canonical));
    }
}

fn set_reference(target: &mut Value, field: &str, reference: &str, resource_type: &str) {
    set_field(
        target,
        field,
        reference_from_string(reference, resource_type),
    );
}

fn set_context_reference(
    target: &mut Value,
    reference: &Option<String>,
    field: &str,
    resource_type: &str,
) {
    if let Some(reference) = reference {
        set_reference(target, field, reference, resource_type);
    }
}

fn reference_from_string(reference: &str, resource_type: &str) -> Value {
    if reference.contains('/') {
        json!({ "reference": reference })
    } else {
        json!({
            "reference": format!("{resource_type}/{reference}"),
            "type": resource_type
        })
    }
}

fn default_status(kind: &str) -> Option<&'static str> {
    match kind {
        "Appointment" => Some("proposed"),
        "CarePlan"
        | "CommunicationRequest"
        | "DeviceRequest"
        | "MedicationRequest"
        | "NutritionOrder"
        | "ServiceRequest"
        | "SupplyRequest"
        | "Task" => Some("draft"),
        "ImmunizationRecommendation" => None,
        _ => None,
    }
}

fn supports_intent(kind: &str) -> bool {
    matches!(
        kind,
        "CarePlan"
            | "DeviceRequest"
            | "MedicationRequest"
            | "NutritionOrder"
            | "ServiceRequest"
            | "Task"
    )
}

fn validated_intent(kind: &str, activity_definition: &Value) -> CpgResult<Value> {
    let intent = non_null_field(activity_definition, "intent")
        .and_then(Value::as_str)
        .unwrap_or("proposal");
    if kind == "Task" && intent == "directive" {
        return Ok(json!("unknown"));
    }
    if intent_is_valid(kind, intent) {
        Ok(json!(intent))
    } else {
        Err(CpgError::InvalidResource(format!(
            "ActivityDefinition intent '{intent}' is invalid for {kind}"
        )))
    }
}

fn intent_is_valid(kind: &str, intent: &str) -> bool {
    match kind {
        "CarePlan" => matches!(intent, "proposal" | "plan" | "order" | "option"),
        "MedicationRequest" => matches!(
            intent,
            "proposal"
                | "plan"
                | "order"
                | "original-order"
                | "reflex-order"
                | "filler-order"
                | "instance-order"
                | "option"
        ),
        "Task" => matches!(
            intent,
            "unknown"
                | "proposal"
                | "plan"
                | "order"
                | "original-order"
                | "reflex-order"
                | "filler-order"
                | "instance-order"
                | "option"
        ),
        "DeviceRequest" | "NutritionOrder" | "ServiceRequest" => matches!(
            intent,
            "proposal"
                | "plan"
                | "directive"
                | "order"
                | "original-order"
                | "reflex-order"
                | "filler-order"
                | "instance-order"
                | "option"
        ),
        _ => true,
    }
}

fn ensure_required_fields(resource: &Value) -> CpgResult<()> {
    let kind = resource
        .get("resourceType")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let required: &[&[&str]] = match kind {
        "Appointment" => &[&["status"], &["participant"]],
        "CarePlan" => &[&["status"], &["intent"], &["subject"]],
        "CommunicationRequest" => &[&["status"]],
        "DeviceRequest" => &[
            &["intent"],
            &["codeCodeableConcept", "codeReference"],
            &["subject"],
        ],
        "ImmunizationRecommendation" => &[&["patient"], &["date"], &["recommendation"]],
        "MedicationRequest" => &[
            &["status"],
            &["intent"],
            &["medicationCodeableConcept", "medicationReference"],
            &["subject"],
        ],
        "NutritionOrder" => &[&["status"], &["intent"], &["patient"], &["dateTime"]],
        "ServiceRequest" => &[&["status"], &["intent"], &["subject"]],
        "SupplyRequest" => &[&["itemCodeableConcept", "itemReference"], &["quantity"]],
        "Task" => &[&["status"], &["intent"]],
        _ => &[],
    };

    let missing = required
        .iter()
        .filter(|alternatives| {
            !alternatives.iter().any(|field| {
                resource.get(*field).is_some_and(|value| match value {
                    Value::Null => false,
                    Value::Array(values) => !values.is_empty(),
                    Value::Object(values) => !values.is_empty(),
                    Value::String(value) => !value.is_empty(),
                    _ => true,
                })
            })
        })
        .map(|alternatives| alternatives.join(" or "))
        .collect::<Vec<_>>();

    if !missing.is_empty() {
        return Err(CpgError::InvalidResource(format!(
            "applied {kind} is missing required field(s): {}",
            missing.join(", ")
        )));
    }

    if let Some(intent) = resource.get("intent").and_then(Value::as_str) {
        if !intent_is_valid(kind, intent) {
            return Err(CpgError::InvalidResource(format!(
                "applied {kind} has invalid intent '{intent}'"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::context::ApplyContext;
    use crate::resolver::BundleResolver;
    use rh_validator::{FhirValidator, FhirVersion};
    use serde_json::json;

    use super::*;

    #[test]
    fn applies_service_request_with_timing_and_dynamic_value() {
        let activity_definition = json!({
            "resourceType": "ActivityDefinition",
            "url": "http://example.org/ad",
            "title": "Screening",
            "kind": "ServiceRequest",
            "intent": "proposal",
            "code": {"text": "screening"},
            "timingTiming": {"event": ["2026-01-01"]},
            "dynamicValue": [{
                "path": "authoredOn",
                "expression": {
                    "language": "text/fhirpath",
                    "expression": "title"
                }
            }]
        });
        let ctx = test_context(json!({
            "resourceType": "Bundle",
            "entry": []
        }));

        let applied = apply_activity_definition(&activity_definition, &[], &ctx)
            .expect("apply should succeed")
            .expect("ServiceRequest should be supported");

        assert_eq!(applied["resourceType"], "ServiceRequest");
        assert_eq!(applied["intent"], "proposal");
        assert_eq!(applied["subject"], json!({"reference": "Patient/123"}));
        assert_eq!(
            applied["requester"],
            json!({"reference": "Practitioner/456"})
        );
        assert_eq!(applied["encounter"], json!({"reference": "Encounter/789"}));
        assert_eq!(applied["code"], json!({"text": "screening"}));
        assert_eq!(
            applied["occurrenceTiming"],
            json!({"event": ["2026-01-01"]})
        );
        assert_eq!(
            applied["instantiatesCanonical"],
            json!(["http://example.org/ad"])
        );
        assert_eq!(applied["authoredOn"], "Screening");
    }

    #[test]
    fn applies_medication_request_structural_fields() {
        let activity_definition = json!({
            "resourceType": "ActivityDefinition",
            "kind": "MedicationRequest",
            "productCodeableConcept": {"text": "aspirin"},
            "dosage": [{"text": "existing"}],
            "timingDateTime": "2026-01-01T10:00:00Z",
            "quantity": {"value": 2}
        });
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));

        let applied = apply_activity_definition(&activity_definition, &[], &ctx)
            .expect("apply should succeed")
            .expect("MedicationRequest should be supported");

        assert_eq!(applied["resourceType"], "MedicationRequest");
        assert_eq!(
            applied["medicationCodeableConcept"],
            json!({"text": "aspirin"})
        );
        assert_eq!(applied["subject"], json!({"reference": "Patient/123"}));
        assert_eq!(
            applied["requester"],
            json!({"reference": "Practitioner/456"})
        );
        assert_eq!(applied["dosageInstruction"].as_array().unwrap().len(), 2);
        assert_eq!(applied["dosageInstruction"][0], json!({"text": "existing"}));
        assert_eq!(
            applied["dosageInstruction"][1]["timing"]["event"],
            json!(["2026-01-01T10:00:00Z"])
        );
        assert_eq!(
            applied["dosageInstruction"][1]["doseAndRate"],
            json!([{"doseQuantity": {"value": 2}}])
        );
    }

    #[test]
    fn accepts_dynamic_medication_without_emitting_empty_dosage() {
        let activity_definition = json!({
            "resourceType": "ActivityDefinition",
            "kind": "MedicationRequest",
            "code": {"text": "dynamically selected medication"},
            "dynamicValue": [{
                "path": "medicationCodeableConcept",
                "expression": {
                    "language": "text/fhirpath",
                    "expression": "code"
                }
            }]
        });
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));

        let applied = apply_activity_definition(&activity_definition, &[], &ctx)
            .expect("dynamic medication should satisfy required fields")
            .expect("MedicationRequest should be supported");

        assert_eq!(
            applied["medicationCodeableConcept"],
            json!({"text": "dynamically selected medication"})
        );
        assert!(applied.get("dosageInstruction").is_none());
    }

    #[test]
    fn applies_task_with_reference_and_intent_mapping() {
        let activity_definition = json!({
            "resourceType": "ActivityDefinition",
            "url": "http://example.org/task-ad",
            "kind": "Task",
            "intent": "directive",
            "code": {"text": "follow-up"},
            "priority": "routine"
        });
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));

        let applied = apply_activity_definition(&activity_definition, &[], &ctx)
            .expect("apply should succeed")
            .expect("Task should be supported");

        assert_eq!(applied["for"], json!({"reference": "Patient/123"}));
        assert_eq!(applied["encounter"], json!({"reference": "Encounter/789"}));
        assert_eq!(
            applied["requester"],
            json!({"reference": "Practitioner/456"})
        );
        assert_eq!(applied["code"], json!({"text": "follow-up"}));
        assert_eq!(applied["priority"], "routine");
        assert_eq!(applied["intent"], "unknown");
        assert_eq!(
            applied["instantiatesCanonical"],
            "http://example.org/task-ad"
        );
    }

    #[test]
    fn applies_collect_information_task_dynamic_inputs_with_fhirpath() {
        let activity_definition = json!({
            "resourceType": "ActivityDefinition",
            "url": "http://example.org/ActivityDefinition/collect",
            "version": "1.0.0",
            "kind": "Task",
            "intent": "proposal",
            "code": {"text": "collect-information"},
            "extension": [{
                "url": "http://hl7.org/fhir/uv/cpg/StructureDefinition/cpg-collectWith",
                "valueCanonical": "http://example.org/Questionnaire/screen|1.0.0"
            }],
            "dynamicValue": [{
                "path": "input[0].type",
                "expression": {"language": "text/fhirpath", "expression": "code"}
            }, {
                "path": "input[0].valueCanonical",
                "expression": {"language": "text/fhirpath", "expression": "extension.where(url = 'http://hl7.org/fhir/uv/cpg/StructureDefinition/cpg-collectWith').value"}
            }]
        });
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));
        let applied = apply_activity_definition(&activity_definition, &[], &ctx)
            .expect("apply should succeed")
            .expect("Task should be supported");
        assert_eq!(applied["input"][0]["type"], json!({"text": "collect-information"}));
        assert_eq!(applied["input"][0]["valueCanonical"], "http://example.org/Questionnaire/screen|1.0.0");
    }

    #[test]
    fn returns_none_for_unknown_kind() {
        let activity_definition = json!({"kind": "NotAResource"});
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));

        assert_eq!(
            apply_activity_definition(&activity_definition, &[], &ctx)
                .expect("unknown kind should not fail"),
            None
        );
    }

    #[test]
    fn applies_do_not_perform_to_communication_request() {
        let activity_definition = json!({
            "resourceType": "ActivityDefinition",
            "kind": "CommunicationRequest",
            "doNotPerform": true
        });
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));

        let applied = apply_activity_definition(&activity_definition, &[], &ctx)
            .expect("apply should succeed")
            .expect("CommunicationRequest should be supported");

        assert_eq!(applied["doNotPerform"], true);
    }

    #[test]
    fn returns_none_when_kind_is_missing() {
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));

        assert_eq!(
            apply_activity_definition(&json!({}), &[], &ctx)
                .expect("empty definition should not fail"),
            None
        );
    }

    #[test]
    fn every_supported_kind_produces_core_r4_valid_output() {
        let validator =
            FhirValidator::new(FhirVersion::R4, None).expect("R4 validator should initialize");
        let definitions = [
            json!({"resourceType": "ActivityDefinition", "kind": "Appointment"}),
            json!({"resourceType": "ActivityDefinition", "kind": "CarePlan"}),
            json!({"resourceType": "ActivityDefinition", "kind": "CommunicationRequest"}),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "DeviceRequest",
                "productCodeableConcept": {"text": "Mobility aid"}
            }),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "ImmunizationRecommendation",
                "productCodeableConcept": {"text": "Seasonal vaccine"}
            }),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "MedicationRequest",
                "productCodeableConcept": {"text": "Aspirin"}
            }),
            json!({"resourceType": "ActivityDefinition", "kind": "NutritionOrder"}),
            json!({"resourceType": "ActivityDefinition", "kind": "ServiceRequest"}),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "SupplyRequest",
                "productCodeableConcept": {"text": "Wound dressing"},
                "quantity": {"value": 1}
            }),
            json!({"resourceType": "ActivityDefinition", "kind": "Task"}),
        ];
        let mut ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));
        ctx.evaluation_date = Some("2026-09-19T12:00:00Z".to_string());

        for definition in definitions {
            let kind = definition["kind"].as_str().expect("kind");
            let applied = apply_activity_definition(&definition, &[], &ctx)
                .unwrap_or_else(|error| panic!("{kind} application failed: {error}"))
                .unwrap_or_else(|| panic!("{kind} should be supported"));
            let result = validator
                .validate(&applied)
                .unwrap_or_else(|error| panic!("{kind} validation failed to run: {error}"));
            let errors = result
                .issues
                .iter()
                .filter(|issue| issue.severity == rh_validator::Severity::Error)
                .map(|issue| {
                    format!(
                        "{}: {}",
                        issue.path.as_deref().unwrap_or("?"),
                        issue.message
                    )
                })
                .collect::<Vec<_>>();
            assert!(
                errors.is_empty(),
                "{kind} is not valid R4: {errors:#?}\n{applied:#}"
            );
        }
    }

    #[test]
    fn missing_required_fields_fail_instead_of_emitting_invalid_resources() {
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));
        for definition in [
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "MedicationRequest"
            }),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "NutritionOrder"
            }),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "DeviceRequest"
            }),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "SupplyRequest"
            }),
            json!({
                "resourceType": "ActivityDefinition",
                "kind": "ImmunizationRecommendation"
            }),
        ] {
            let error = apply_activity_definition(&definition, &[], &ctx).unwrap_err();
            assert!(
                matches!(error, CpgError::InvalidResource(message) if message.contains("missing required field"))
            );
        }
    }

    #[test]
    fn unsupported_skeleton_kinds_return_none() {
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));
        for kind in [
            "AppointmentResponse",
            "Claim",
            "Contract",
            "EnrollmentRequest",
            "VisionPrescription",
        ] {
            let result = apply_activity_definition(
                &json!({"resourceType": "ActivityDefinition", "kind": kind}),
                &[],
                &ctx,
            )
            .expect("unsupported kind should not fail");
            assert!(result.is_none(), "{kind} must not emit an invalid skeleton");
        }
    }

    #[test]
    fn rejects_target_incompatible_intent() {
        let ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));
        for kind in ["CarePlan", "MedicationRequest"] {
            let error = apply_activity_definition(
                &json!({
                    "resourceType": "ActivityDefinition",
                    "kind": kind,
                    "intent": "directive",
                    "productCodeableConcept": {"text": "Example"}
                }),
                &[],
                &ctx,
            )
            .unwrap_err();
            assert!(
                matches!(error, CpgError::InvalidResource(message) if message.contains("intent 'directive'"))
            );
        }
    }

    #[test]
    fn rejects_invalid_evaluation_date() {
        let mut ctx = test_context(json!({"resourceType": "Bundle", "entry": []}));
        ctx.evaluation_date = Some("not-a-date-time".to_string());

        let error = apply_activity_definition(
            &json!({"resourceType": "ActivityDefinition", "kind": "NutritionOrder"}),
            &[],
            &ctx,
        )
        .unwrap_err();

        assert!(
            matches!(error, CpgError::InvalidResource(message) if message.contains("RFC 3339"))
        );
    }

    fn test_context(bundle: Value) -> ApplyContext {
        let resolver = Arc::new(BundleResolver::new(&bundle).expect("test bundle should be valid"));
        let mut context = ApplyContext::new(resolver, "Patient/123");
        context.encounter = Some("Encounter/789".to_string());
        context.practitioner = Some("Practitioner/456".to_string());
        context
    }
}
