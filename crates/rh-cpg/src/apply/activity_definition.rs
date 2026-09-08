use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::apply::dynamic_value::process_dynamic_value;
use crate::context::ApplyContext;
use crate::error::CpgResult;

/// Apply an ActivityDefinition to a subject, producing the target request
/// resource. Returns `None` when the definition has no supported request kind.
pub fn apply_activity_definition(
    activity_definition: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> CpgResult<Option<Value>> {
    let Some(kind) = non_null_field(activity_definition, "kind").and_then(Value::as_str) else {
        return Ok(None);
    };

    if !matches!(
        kind,
        "Appointment"
            | "AppointmentResponse"
            | "CarePlan"
            | "Claim"
            | "CommunicationRequest"
            | "Contract"
            | "DeviceRequest"
            | "EnrollmentRequest"
            | "ImmunizationRecommendation"
            | "MedicationRequest"
            | "NutritionOrder"
            | "ServiceRequest"
            | "SupplyRequest"
            | "Task"
            | "VisionPrescription"
    ) {
        return Ok(None);
    }

    let mut target = json!({
        "id": Uuid::new_v4().to_string(),
        "resourceType": kind,
        "status": "draft"
    });
    let canonical = canonicalize(activity_definition);

    if supports_intent(kind) {
        if let Some(intent) = non_null_field(activity_definition, "intent") {
            set_field(&mut target, "intent", intent.clone());
        }
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
        "AppointmentResponse" => {
            set_field(&mut target, "participantStatus", json!("needs-action"));
            if let Some(timing_period) = field(activity_definition, "timingPeriod") {
                if let Some(start) = timing_period.get("start") {
                    set_field(&mut target, "start", start.clone());
                }
                if let Some(end) = timing_period.get("end") {
                    set_field(&mut target, "end", end.clone());
                }
            }
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
        "Claim" => {
            if let Some(practitioner) = &ctx.practitioner {
                set_field(
                    &mut target,
                    "provider",
                    reference_from_string(practitioner, "Practitioner"),
                );
            }
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
        "Contract" => {
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
                    "authority",
                    reference_from_string(organization, "Organization"),
                );
            }
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
        "EnrollmentRequest" => {
            set_reference(&mut target, "candidate", subject, "Patient");
            set_context_reference(&mut target, &ctx.practitioner, "provider", "Practitioner");
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
            set_field(&mut target, "date", json!(now_iso8601()));
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
            push_or_create_array(&mut target, "dosageInstruction", Value::Object(new_dosage));
        }
        "NutritionOrder" => {
            push_canonical(&mut target, canonical.as_deref());
            set_reference(&mut target, "patient", subject, "Patient");
            set_context_reference(&mut target, &ctx.encounter, "encounter", "Encounter");
            set_context_reference(&mut target, &ctx.practitioner, "orderer", "Practitioner");
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
        "VisionPrescription" => {
            set_reference(&mut target, "patient", subject, "Patient");
            set_context_reference(&mut target, &ctx.encounter, "encounter", "Encounter");
            set_context_reference(&mut target, &ctx.practitioner, "prescriber", "Practitioner");
            set_field(&mut target, "created", json!(now_iso8601()));
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

fn supports_intent(kind: &str) -> bool {
    matches!(
        kind,
        "CarePlan" | "DeviceRequest" | "MedicationRequest" | "ServiceRequest" | "Task"
    )
}

fn now_iso8601() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after the Unix epoch");
    let seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();
    let days = (seconds / 86_400) as i64;
    let time_seconds = seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{milliseconds:03}Z",
        time_seconds / 3_600,
        (time_seconds % 3_600) / 60,
        time_seconds % 60
    )
}

fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = (z - era * 146_097) as u64;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era as i64 + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    (year, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::context::ApplyContext;
    use crate::resolver::BundleResolver;
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

    fn test_context(bundle: Value) -> ApplyContext {
        let resolver = Arc::new(BundleResolver::new(&bundle).expect("test bundle should be valid"));
        let mut context = ApplyContext::new(resolver, "Patient/123");
        context.encounter = Some("Encounter/789".to_string());
        context.practitioner = Some("Practitioner/456".to_string());
        context
    }
}
