//! Smoke tests for rh-hl7-fhir-r5-core.
//!
//! Covers:
//! - Struct construction and Default for core resources
//! - Serde round-trip (JSON serialize/deserialize)
//! - Builder trait method chaining
//! - R5-new types: Transport, CodeableReference, Integer64Type,
//!   BiologicallyDerivedProductDispense
//! - SubscriptionStatus naming-collision fix (status: Option<StringType>)
//! - Metadata path resolution
//! - Binding enums (required-strength ValueSet bindings)
//! - Validation invariants and cardinalities present

use rh_hl7_fhir_r5_core::{
    bindings::administrative_gender::AdministrativeGender,
    bindings::subscription_notification_type::SubscriptionNotificationType,
    datatypes::{
        codeable_concept::CodeableConcept, codeable_reference::CodeableReference,
        human_name::HumanName, identifier::Identifier, reference::Reference,
    },
    metadata::{resolve_path, FhirFieldType, FhirPrimitiveType},
    primitives::integer64::Integer64Type,
    resources::{patient::Patient, subscription_status::SubscriptionStatus, transport::Transport},
    traits::patient::{PatientExistence, PatientMutators},
    traits::resource::{ResourceExistence, ResourceMutators},
    traits::subscription_status::SubscriptionStatusAccessors,
    validation::ValidatableResource,
};

// ── Struct construction ───────────────────────────────────────────────────────

#[test]
fn test_patient_default_construction() {
    let patient = Patient::default();
    assert!(patient.base.base.id.is_none());
    assert!(patient.name.is_empty());
    assert!(patient.gender.is_none());
}

#[test]
fn test_patient_builder_chaining() {
    let patient = <Patient as PatientMutators>::new()
        .set_id("p-001".to_string())
        .set_active(true)
        .add_name(HumanName {
            family: Some("Smith".to_string()),
            given: vec!["Alice".to_string()],
            ..Default::default()
        })
        .set_birth_date("1985-03-22".to_string());

    assert_eq!(patient.base.base.id.as_deref(), Some("p-001"));
    assert_eq!(patient.active, Some(true));
    assert_eq!(patient.name.len(), 1);
    assert_eq!(patient.name[0].family.as_deref(), Some("Smith"));
    assert_eq!(patient.birth_date.as_deref(), Some("1985-03-22"));
}

#[test]
fn test_patient_gender_binding_enum() {
    let patient = <Patient as PatientMutators>::new().set_gender(AdministrativeGender::Female);
    let json = serde_json::to_string(&patient.gender).expect("serialize");
    assert!(
        json.contains("female"),
        "gender should serialize to 'female', got {json}"
    );
}

#[test]
fn test_patient_identifier() {
    let patient = <Patient as PatientMutators>::new().add_identifier(Identifier {
        system: Some("http://example.org/mrn".to_string()),
        value: Some("MRN-123".to_string()),
        ..Default::default()
    });

    assert_eq!(patient.identifier.len(), 1);
    assert_eq!(patient.identifier[0].value.as_deref(), Some("MRN-123"));
}

// ── Serde round-trip ──────────────────────────────────────────────────────────

#[test]
fn test_patient_serde_roundtrip() {
    let patient = <Patient as PatientMutators>::new()
        .set_id("serde-test".to_string())
        .set_active(true)
        .add_name(HumanName {
            family: Some("Jones".to_string()),
            given: vec!["Bob".to_string()],
            ..Default::default()
        });

    let json = serde_json::to_string(&patient).expect("serialize");
    assert!(
        json.contains("\"Jones\""),
        "serialized JSON should contain family name"
    );

    let restored: Patient = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored.base.base.id, patient.base.base.id);
    assert_eq!(restored.name.len(), patient.name.len());
    assert_eq!(
        restored.name[0].family.as_deref(),
        patient.name[0].family.as_deref()
    );
}

#[test]
fn test_patient_deserialize_from_fhir_json() {
    let json = r#"{
        "id": "example",
        "active": true,
        "name": [{"family": "Doe", "given": ["John"]}],
        "gender": "male",
        "birthDate": "1990-01-01"
    }"#;

    let patient: Patient = serde_json::from_str(json).expect("deserialize");
    // In R5: Patient.base = DomainResource, DomainResource.base = Resource, Resource.id
    assert_eq!(patient.base.base.id.as_deref(), Some("example"));
    assert_eq!(patient.active, Some(true));
    assert_eq!(patient.birth_date.as_deref(), Some("1990-01-01"));

    let gender_json = serde_json::to_string(&patient.gender).expect("serialize gender");
    assert!(gender_json.contains("male"));
}

// ── R5-new: CodeableReference ─────────────────────────────────────────────────

#[test]
fn test_codeable_reference_construction() {
    let cr = CodeableReference {
        concept: Some(CodeableConcept {
            text: Some("Example concept".to_string()),
            ..Default::default()
        }),
        reference: None,
        ..Default::default()
    };

    assert!(cr.concept.is_some());
    assert!(cr.reference.is_none());

    let json = serde_json::to_string(&cr).expect("serialize");
    let restored: CodeableReference = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(
        restored.concept.as_ref().and_then(|c| c.text.as_deref()),
        Some("Example concept")
    );
}

#[test]
fn test_codeable_reference_with_reference() {
    let cr = CodeableReference {
        reference: Some(Reference {
            reference: Some("Observation/obs-001".to_string()),
            ..Default::default()
        }),
        concept: None,
        ..Default::default()
    };
    assert!(cr.reference.is_some());
    assert_eq!(
        cr.reference.unwrap().reference.as_deref(),
        Some("Observation/obs-001")
    );
}

// ── R5-new: Transport ─────────────────────────────────────────────────────────

#[test]
fn test_transport_default_construction() {
    let transport = Transport::default();
    // Transport -> DomainResource -> Resource -> Base
    assert!(transport.base.base.id.is_none());
}

#[test]
fn test_transport_serde_roundtrip() {
    use rh_hl7_fhir_r5_core::bindings::transport_intent::TransportIntent;
    use rh_hl7_fhir_r5_core::bindings::transport_status::TransportStatus;

    let transport = Transport {
        status: Some(TransportStatus::InProgress),
        intent: TransportIntent::Order,
        ..Default::default()
    };
    // Set id separately since it's deeply nested
    let mut transport = transport;
    transport.base.base.id = Some("t-001".to_string());

    let json = serde_json::to_string(&transport).expect("serialize");
    assert!(
        json.contains("in-progress"),
        "status should serialize to 'in-progress'"
    );
    assert!(json.contains("order"), "intent should serialize to 'order'");

    let restored: Transport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored.base.base.id.as_deref(), Some("t-001"));

    let status_json = serde_json::to_string(&restored.status).expect("serialize status");
    assert!(status_json.contains("in-progress"));
}

// ── R5-new: Integer64Type ─────────────────────────────────────────────────────

#[test]
fn test_integer64_type_is_string() {
    // Integer64Type is a type alias for String (R5 maps it as a large integer string)
    let val: Integer64Type = "9223372036854775807".to_string();
    assert_eq!(val, "9223372036854775807");
}

#[test]
fn test_subscription_status_integer64_field() {
    let ss = SubscriptionStatus {
        events_since_subscription_start: Some("9007199254740992".to_string()),
        ..Default::default()
    };
    // Must clone (not move) — Integer64Type is String, not Copy
    assert_eq!(
        ss.events_since_subscription_start(),
        Some("9007199254740992".to_string())
    );
    // Second call still works (field wasn't consumed)
    assert!(ss.events_since_subscription_start.is_some());
}

// ── R5-new: SubscriptionStatus naming-collision fix ───────────────────────────

#[test]
fn test_subscription_status_status_is_string_type() {
    // Before the generator fix, `status` was typed as `Option<SubscriptionStatus>`
    // (self-referential / recursive). It must be `Option<StringType>` = `Option<String>`.
    let ss = SubscriptionStatus {
        status: Some("active".to_string()), // compile-time proof it's Option<String>
        type_: SubscriptionNotificationType::EventNotification,
        ..Default::default()
    };

    assert_eq!(ss.status(), Some("active".to_string()));
}

#[test]
fn test_subscription_status_serde_roundtrip() {
    let ss = SubscriptionStatus {
        status: Some("active".to_string()),
        type_: SubscriptionNotificationType::Heartbeat,
        events_since_subscription_start: Some("100".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&ss).expect("serialize");
    assert!(json.contains("\"active\""), "status should appear in JSON");
    assert!(
        json.contains("heartbeat"),
        "type should serialize to 'heartbeat'"
    );

    let restored: SubscriptionStatus = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored.status, ss.status);
    assert_eq!(
        restored.events_since_subscription_start,
        ss.events_since_subscription_start
    );
}

// ── R5-new: BiologicallyDerivedProductDispense ────────────────────────────────

#[test]
fn test_biologically_derived_product_dispense_construction() {
    use rh_hl7_fhir_r5_core::bindings::biologicallyderivedproductdispense_status::BiologicallyderivedproductdispenseStatus;
    use rh_hl7_fhir_r5_core::resources::biologically_derived_product_dispense::BiologicallyDerivedProductDispense;

    let dispense = BiologicallyDerivedProductDispense {
        status: BiologicallyderivedproductdispenseStatus::InProgress,
        product: Reference {
            reference: Some("BiologicallyDerivedProduct/bdp-001".to_string()),
            ..Default::default()
        },
        patient: Reference {
            reference: Some("Patient/p-001".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    let status_json = serde_json::to_string(&dispense.status).expect("serialize status");
    assert!(status_json.contains("in-progress"));
    assert_eq!(
        dispense.product.reference.as_deref(),
        Some("BiologicallyDerivedProduct/bdp-001")
    );
    assert_eq!(dispense.patient.reference.as_deref(), Some("Patient/p-001"));
}

// ── Metadata path resolution ──────────────────────────────────────────────────

#[test]
fn test_metadata_patient_birthdate() {
    let field_type = resolve_path("Patient.birthDate");
    assert!(
        field_type.is_some(),
        "Patient.birthDate should resolve in metadata"
    );
    assert!(
        matches!(
            field_type,
            Some(FhirFieldType::Primitive(FhirPrimitiveType::Date))
        ),
        "Patient.birthDate should be a Date primitive, got {:?}",
        field_type
    );
}

#[test]
fn test_metadata_observation_status() {
    let field_type = resolve_path("Observation.status");
    assert!(
        field_type.is_some(),
        "Observation.status should resolve in metadata"
    );
}

#[test]
fn test_metadata_nonexistent_path_returns_none() {
    assert!(
        resolve_path("Patient.nonExistentField").is_none(),
        "Unknown path should return None"
    );
    assert!(
        resolve_path("UnknownResource.field").is_none(),
        "Unknown resource should return None"
    );
}

// ── Validation: invariants and cardinalities ──────────────────────────────────

#[test]
fn test_patient_has_invariants() {
    let invariants = Patient::invariants();
    assert!(
        !invariants.is_empty(),
        "Patient should have at least one invariant"
    );
    let has_dom2 = invariants.iter().any(|i| i.key == "dom-2");
    assert!(has_dom2, "Patient should have dom-2 invariant");
}

#[test]
fn test_patient_has_cardinalities() {
    let cardinalities = Patient::cardinalities();
    assert!(
        !cardinalities.is_empty(),
        "Patient should have cardinality constraints"
    );
    let id_card = cardinalities.iter().find(|c| c.path == "Patient.id");
    assert!(
        id_card.is_some(),
        "Patient.id cardinality should be defined"
    );
    if let Some(card) = id_card {
        assert_eq!(card.min, 0);
        assert_eq!(card.max, Some(1));
    }
}

#[test]
fn test_subscription_status_has_cardinalities() {
    // Verifies Fix 1: CARDINALITIES static is always emitted even when previously
    // extract_cardinalities returned empty (it now emits an empty vec instead of
    // skipping the static entirely).
    let cardinalities = SubscriptionStatus::cardinalities();
    assert!(
        !cardinalities.is_empty(),
        "SubscriptionStatus should have cardinality constraints"
    );
}

// ── R5-specific resources present ────────────────────────────────────────────

#[test]
fn test_r5_specific_resources_compile_and_default() {
    use rh_hl7_fhir_r5_core::resources::genomic_study::GenomicStudy;
    use rh_hl7_fhir_r5_core::resources::inventory_item::InventoryItem;
    use rh_hl7_fhir_r5_core::resources::permission::Permission;
    use rh_hl7_fhir_r5_core::resources::requirements::Requirements;

    // Construction confirms they were generated and compile
    let _gs = GenomicStudy::default();
    let _ii = InventoryItem::default();
    let _perm = Permission::default();
    let _req = Requirements::default();
}

#[test]
fn test_r5_new_datatypes_compile_and_default() {
    use rh_hl7_fhir_r5_core::datatypes::availability::Availability;
    use rh_hl7_fhir_r5_core::datatypes::monetary_component::MonetaryComponent;
    use rh_hl7_fhir_r5_core::datatypes::virtual_service_detail::VirtualServiceDetail;

    let _avail = Availability::default();
    let _mc = MonetaryComponent::default();
    let _vsd = VirtualServiceDetail::default();
}

// ── Prelude imports ───────────────────────────────────────────────────────────

#[test]
fn test_prelude_has_and_existence_checks() {
    use rh_hl7_fhir_r5_core::prelude::*;
    use rh_hl7_fhir_r5_core::resources::patient::Patient;
    use rh_hl7_fhir_r5_core::traits::patient::PatientMutators;

    // PatientMutators + ValidatableResource come from the prelude
    let patient = <Patient as PatientMutators>::new().set_id("prelude-test".to_string());
    assert!(patient.has_id());
    assert!(!patient.has_meta());
    assert!(!patient.has_name());
}
