//! Field metadata for other FHIR types

use super::*;
use phf::{phf_map, Map};

/// Field metadata for AccountCoverage
#[rustfmt::skip]
pub static ACCOUNTCOVERAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "coverage" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AccountGuarantor
#[rustfmt::skip]
pub static ACCOUNTGUARANTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onHold" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "party" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ActivityDefinitionDynamicValue
#[rustfmt::skip]
pub static ACTIVITYDEFINITIONDYNAMICVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "expression" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "path" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ActivityDefinitionParticipant
#[rustfmt::skip]
pub static ACTIVITYDEFINITIONPARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AdverseEventSuspectEntity
#[rustfmt::skip]
pub static ADVERSEEVENTSUSPECTENTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "causality" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdverseEventSuspectEntityCausality"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instance" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for AdverseEventSuspectEntityCausality
#[rustfmt::skip]
pub static ADVERSEEVENTSUSPECTENTITYCAUSALITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "assessment" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "method" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productRelatedness" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AllergyIntoleranceReaction
#[rustfmt::skip]
pub static ALLERGYINTOLERANCEREACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "exposureRoute" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "manifestation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onset" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "severity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "substance" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AppointmentParticipant
#[rustfmt::skip]
pub static APPOINTMENTPARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "required" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for AuditEventAgent
#[rustfmt::skip]
pub static AUDITEVENTAGENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "altId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "media" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "network" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AuditEventAgentNetwork"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "policy" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "purposeOfUse" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requestor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "who" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AuditEventAgentNetwork
#[rustfmt::skip]
pub static AUDITEVENTAGENTNETWORK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "address" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AuditEventEntity
#[rustfmt::skip]
pub static AUDITEVENTENTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AuditEventEntityDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "lifecycle" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "query" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "securityLabel" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "what" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AuditEventEntityDetail
#[rustfmt::skip]
pub static AUDITEVENTENTITYDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for AuditEventSource
#[rustfmt::skip]
pub static AUDITEVENTSOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "observer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "site" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for BiologicallyDerivedProductCollection
#[rustfmt::skip]
pub static BIOLOGICALLYDERIVEDPRODUCTCOLLECTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "collected[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "collector" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BiologicallyDerivedProductManipulation
#[rustfmt::skip]
pub static BIOLOGICALLYDERIVEDPRODUCTMANIPULATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "time[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for BiologicallyDerivedProductProcessing
#[rustfmt::skip]
pub static BIOLOGICALLYDERIVEDPRODUCTPROCESSING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additive" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "procedure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "time[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for BiologicallyDerivedProductStorage
#[rustfmt::skip]
pub static BIOLOGICALLYDERIVEDPRODUCTSTORAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "duration" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "scale" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "temperature" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BundleEntry
#[rustfmt::skip]
pub static BUNDLEENTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fullUrl" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "request" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BundleEntryRequest"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "response" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BundleEntryResponse"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "search" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BundleEntrySearch"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BundleEntryRequest
#[rustfmt::skip]
pub static BUNDLEENTRYREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ifMatch" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ifModifiedSince" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ifNoneExist" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ifNoneMatch" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "method" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BundleEntryResponse
#[rustfmt::skip]
pub static BUNDLEENTRYRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "etag" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "lastModified" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BundleEntrySearch
#[rustfmt::skip]
pub static BUNDLEENTRYSEARCH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "score" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BundleLink
#[rustfmt::skip]
pub static BUNDLELINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementDocument
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTDOCUMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementImplementation
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTIMPLEMENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "custodian" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementMessaging
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTMESSAGING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endpoint" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementMessagingEndpoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reliableCache" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supportedMessage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementMessagingSupportedMessage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementMessagingEndpoint
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTMESSAGINGENDPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "address" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "protocol" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementMessagingSupportedMessage
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTMESSAGINGSUPPORTEDMESSAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementRest
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTREST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "compartment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "interaction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementRestInteraction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementRestResource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "security" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementRestSecurity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementRestInteraction
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTRESTINTERACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementRestResource
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTRESTRESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "conditionalCreate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "conditionalDelete" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "conditionalRead" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "conditionalUpdate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "interaction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementRestResourceInteraction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementRestResourceOperation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "readHistory" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referencePolicy" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "searchInclude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "searchParam" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementRestResourceSearchParam"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "searchRevInclude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "supportedProfile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "updateCreate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "versioning" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementRestResourceInteraction
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTRESTRESOURCEINTERACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementRestResourceOperation
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTRESTRESOURCEOPERATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementRestResourceSearchParam
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTRESTRESOURCESEARCHPARAM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementRestSecurity
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTRESTSECURITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "cors" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "service" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CapabilityStatementSoftware
#[rustfmt::skip]
pub static CAPABILITYSTATEMENTSOFTWARE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "releaseDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CarePlanActivity
#[rustfmt::skip]
pub static CAREPLANACTIVITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CarePlanActivityDetail"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcomeCodeableConcept" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcomeReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "progress" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CarePlanActivityDetail
#[rustfmt::skip]
pub static CAREPLANACTIVITYDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "dailyAmount" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "doNotPerform" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "goal" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instantiatesCanonical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instantiatesUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "product[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reasonCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "scheduled[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "statusReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CareTeamParticipant
#[rustfmt::skip]
pub static CARETEAMPARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "member" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onBehalfOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CatalogEntryRelatedEntry
#[rustfmt::skip]
pub static CATALOGENTRYRELATEDENTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relationtype" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ChargeItemDefinitionApplicability
#[rustfmt::skip]
pub static CHARGEITEMDEFINITIONAPPLICABILITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ChargeItemDefinitionPropertyGroup
#[rustfmt::skip]
pub static CHARGEITEMDEFINITIONPROPERTYGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "priceComponent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ChargeItemDefinitionPropertyGroupPriceComponent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ChargeItemDefinitionPropertyGroupPriceComponent
#[rustfmt::skip]
pub static CHARGEITEMDEFINITIONPROPERTYGROUPPRICECOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ChargeItemPerformer
#[rustfmt::skip]
pub static CHARGEITEMPERFORMER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "function" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ClaimAccident
#[rustfmt::skip]
pub static CLAIMACCIDENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimCareTeam
#[rustfmt::skip]
pub static CLAIMCARETEAM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "provider" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "qualification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responsible" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimDiagnosis
#[rustfmt::skip]
pub static CLAIMDIAGNOSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "diagnosis[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onAdmission" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "packageCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ClaimInsurance
#[rustfmt::skip]
pub static CLAIMINSURANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "businessArrangement" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "claimResponse" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "coverage" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "focal" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preAuthRef" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimItem
#[rustfmt::skip]
pub static CLAIMITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "careTeamSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimItemDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "diagnosisSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "informationSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "location[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "procedureSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "revenue" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "serviced[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subSite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimItemDetail
#[rustfmt::skip]
pub static CLAIMITEMDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "revenue" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subDetail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimItemDetailSubDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimItemDetailSubDetail
#[rustfmt::skip]
pub static CLAIMITEMDETAILSUBDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "revenue" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimPayee
#[rustfmt::skip]
pub static CLAIMPAYEE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "party" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimProcedure
#[rustfmt::skip]
pub static CLAIMPROCEDURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "procedure[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ClaimRelated
#[rustfmt::skip]
pub static CLAIMRELATED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "claim" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseAddItem
#[rustfmt::skip]
pub static CLAIMRESPONSEADDITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseAddItemDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "detailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "itemSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "location[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "provider" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "serviced[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subSite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subdetailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseAddItemDetail
#[rustfmt::skip]
pub static CLAIMRESPONSEADDITEMDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subDetail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseAddItemDetailSubDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseAddItemDetailSubDetail
#[rustfmt::skip]
pub static CLAIMRESPONSEADDITEMDETAILSUBDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseError
#[rustfmt::skip]
pub static CLAIMRESPONSEERROR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "detailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "itemSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subDetailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseInsurance
#[rustfmt::skip]
pub static CLAIMRESPONSEINSURANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "businessArrangement" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "claimResponse" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "coverage" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "focal" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseItem
#[rustfmt::skip]
pub static CLAIMRESPONSEITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "adjudication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseItemAdjudication"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseItemDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "itemSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseItemAdjudication
#[rustfmt::skip]
pub static CLAIMRESPONSEITEMADJUDICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseItemDetail
#[rustfmt::skip]
pub static CLAIMRESPONSEITEMDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "detailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subDetail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseItemDetailSubDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseItemDetailSubDetail
#[rustfmt::skip]
pub static CLAIMRESPONSEITEMDETAILSUBDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subDetailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponsePayment
#[rustfmt::skip]
pub static CLAIMRESPONSEPAYMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "adjustment" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "adjustmentReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseProcessNote
#[rustfmt::skip]
pub static CLAIMRESPONSEPROCESSNOTE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "number" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponseTotal
#[rustfmt::skip]
pub static CLAIMRESPONSETOTAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ClaimSupportingInfo
#[rustfmt::skip]
pub static CLAIMSUPPORTINGINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "timing[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ClinicalImpressionFinding
#[rustfmt::skip]
pub static CLINICALIMPRESSIONFINDING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basis" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "itemCodeableConcept" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "itemReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ClinicalImpressionInvestigation
#[rustfmt::skip]
pub static CLINICALIMPRESSIONINVESTIGATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CodeSystemConcept
#[rustfmt::skip]
pub static CODESYSTEMCONCEPT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "designation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CodeSystemConceptDesignation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CodeSystemConceptProperty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CodeSystemConceptDesignation
#[rustfmt::skip]
pub static CODESYSTEMCONCEPTDESIGNATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CodeSystemConceptProperty
#[rustfmt::skip]
pub static CODESYSTEMCONCEPTPROPERTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for CodeSystemFilter
#[rustfmt::skip]
pub static CODESYSTEMFILTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CodeSystemProperty
#[rustfmt::skip]
pub static CODESYSTEMPROPERTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "uri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CommunicationPayload
#[rustfmt::skip]
pub static COMMUNICATIONPAYLOAD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "content[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CommunicationRequestPayload
#[rustfmt::skip]
pub static COMMUNICATIONREQUESTPAYLOAD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "content[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CompartmentDefinitionResource
#[rustfmt::skip]
pub static COMPARTMENTDEFINITIONRESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "param" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CompositionAttester
#[rustfmt::skip]
pub static COMPOSITIONATTESTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "party" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "time" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CompositionEvent
#[rustfmt::skip]
pub static COMPOSITIONEVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CompositionRelatesTo
#[rustfmt::skip]
pub static COMPOSITIONRELATESTO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "target[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for CompositionSection
#[rustfmt::skip]
pub static COMPOSITIONSECTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "emptyReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "entry" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "focus" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "orderedBy" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConceptMapGroup
#[rustfmt::skip]
pub static CONCEPTMAPGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "element" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConceptMapGroupElement"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceVersion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "targetVersion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unmapped" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConceptMapGroupUnmapped"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConceptMapGroupElement
#[rustfmt::skip]
pub static CONCEPTMAPGROUPELEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConceptMapGroupElementTarget"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ConceptMapGroupElementTarget
#[rustfmt::skip]
pub static CONCEPTMAPGROUPELEMENTTARGET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "dependsOn" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConceptMapGroupElementTargetDependsOn"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "equivalence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ConceptMapGroupElementTargetDependsOn
#[rustfmt::skip]
pub static CONCEPTMAPGROUPELEMENTTARGETDEPENDSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConceptMapGroupUnmapped
#[rustfmt::skip]
pub static CONCEPTMAPGROUPUNMAPPED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConditionEvidence
#[rustfmt::skip]
pub static CONDITIONEVIDENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ConditionStage
#[rustfmt::skip]
pub static CONDITIONSTAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "assessment" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "summary" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConsentPolicy
#[rustfmt::skip]
pub static CONSENTPOLICY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "uri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConsentProvision
#[rustfmt::skip]
pub static CONSENTPROVISION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "actor" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConsentProvisionActor"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "class" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "data" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConsentProvisionData"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dataPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "securityLabel" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConsentProvisionActor
#[rustfmt::skip]
pub static CONSENTPROVISIONACTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConsentProvisionData
#[rustfmt::skip]
pub static CONSENTPROVISIONDATA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "meaning" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ConsentVerification
#[rustfmt::skip]
pub static CONSENTVERIFICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "verificationDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "verified" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "verifiedWith" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractContentDefinition
#[rustfmt::skip]
pub static CONTRACTCONTENTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "publicationDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "publicationStatus" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractFriendly
#[rustfmt::skip]
pub static CONTRACTFRIENDLY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "content[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ContractLegal
#[rustfmt::skip]
pub static CONTRACTLEGAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "content[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ContractRule
#[rustfmt::skip]
pub static CONTRACTRULE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "content[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ContractSigner
#[rustfmt::skip]
pub static CONTRACTSIGNER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "party" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "signature" => FieldInfo {
        field_type: FhirFieldType::Complex("Signature"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTerm
#[rustfmt::skip]
pub static CONTRACTTERM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermAction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "applies" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "asset" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermAsset"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "issued" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "offer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermOffer"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "securityLabel" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermSecurityLabel"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "topic[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermAction
#[rustfmt::skip]
pub static CONTRACTTERMACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "context" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contextLinkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "doNotPerform" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "intent" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "linkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performerLinkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performerRole" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonLinkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requesterLinkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "securityLabelNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermActionSubject"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermActionSubject
#[rustfmt::skip]
pub static CONTRACTTERMACTIONSUBJECT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermAsset
#[rustfmt::skip]
pub static CONTRACTTERMASSET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "condition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "context" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermAssetContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "linkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "periodType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "scope" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "securityLabelNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subtype" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "typeReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "usePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "valuedItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermAssetValuedItem"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermAssetContext
#[rustfmt::skip]
pub static CONTRACTTERMASSETCONTEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermAssetValuedItem
#[rustfmt::skip]
pub static CONTRACTTERMASSETVALUEDITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "effectiveTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "entity[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "linkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "payment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "paymentDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "points" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recipient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responsible" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "securityLabelNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermOffer
#[rustfmt::skip]
pub static CONTRACTTERMOFFER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "answer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermOfferAnswer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "decision" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "decisionMode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "linkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "party" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTermOfferParty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "securityLabelNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "topic" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermOfferAnswer
#[rustfmt::skip]
pub static CONTRACTTERMOFFERANSWER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ContractTermOfferParty
#[rustfmt::skip]
pub static CONTRACTTERMOFFERPARTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ContractTermSecurityLabel
#[rustfmt::skip]
pub static CONTRACTTERMSECURITYLABEL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "classification" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "control" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "number" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CoverageClass
#[rustfmt::skip]
pub static COVERAGECLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CoverageCostToBeneficiary
#[rustfmt::skip]
pub static COVERAGECOSTTOBENEFICIARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "exception" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageCostToBeneficiaryException"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for CoverageCostToBeneficiaryException
#[rustfmt::skip]
pub static COVERAGECOSTTOBENEFICIARYEXCEPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityRequestInsurance
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYREQUESTINSURANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "businessArrangement" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "coverage" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "focal" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityRequestItem
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYREQUESTITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "diagnosis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityRequestItemDiagnosis"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "facility" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "provider" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInfoSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityRequestItemDiagnosis
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYREQUESTITEMDIAGNOSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "diagnosis[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityRequestSupportingInfo
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYREQUESTSUPPORTINGINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "appliesToAll" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "information" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityResponseError
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYRESPONSEERROR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityResponseInsurance
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYRESPONSEINSURANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "benefitPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "coverage" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "inforce" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityResponseInsuranceItem"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityResponseInsuranceItem
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYRESPONSEINSURANCEITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authorizationRequired" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "authorizationSupporting" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "authorizationUrl" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "benefit" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityResponseInsuranceItemBenefit"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "excluded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "network" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "provider" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "term" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityResponseInsuranceItemBenefit
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYRESPONSEINSURANCEITEMBENEFIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allowed[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "used[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for Definition
#[rustfmt::skip]
pub static DEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "derivedFromCanonical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "derivedFromUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "experimental" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "replaces" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "useContext" => FieldInfo {
        field_type: FhirFieldType::Complex("UsageContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DetectedIssueEvidence
#[rustfmt::skip]
pub static DETECTEDISSUEEVIDENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DetectedIssueMitigation
#[rustfmt::skip]
pub static DETECTEDISSUEMITIGATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDefinitionCapability
#[rustfmt::skip]
pub static DEVICEDEFINITIONCAPABILITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDefinitionDeviceName
#[rustfmt::skip]
pub static DEVICEDEFINITIONDEVICENAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDefinitionMaterial
#[rustfmt::skip]
pub static DEVICEDEFINITIONMATERIAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allergenicIndicator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "alternate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "substance" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDefinitionProperty
#[rustfmt::skip]
pub static DEVICEDEFINITIONPROPERTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "valueCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "valueQuantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDefinitionSpecialization
#[rustfmt::skip]
pub static DEVICEDEFINITIONSPECIALIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "systemType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDefinitionUdiDeviceIdentifier
#[rustfmt::skip]
pub static DEVICEDEFINITIONUDIDEVICEIDENTIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "deviceIdentifier" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "issuer" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDeviceName
#[rustfmt::skip]
pub static DEVICEDEVICENAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceMetricCalibration
#[rustfmt::skip]
pub static DEVICEMETRICCALIBRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "state" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "time" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceProperty
#[rustfmt::skip]
pub static DEVICEPROPERTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "valueCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "valueQuantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DeviceRequestParameter
#[rustfmt::skip]
pub static DEVICEREQUESTPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for DeviceSpecialization
#[rustfmt::skip]
pub static DEVICESPECIALIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "systemType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceUdiCarrier
#[rustfmt::skip]
pub static DEVICEUDICARRIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "carrierAIDC" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "carrierHRF" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "deviceIdentifier" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "entryType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "issuer" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DeviceVersion
#[rustfmt::skip]
pub static DEVICEVERSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "component" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DiagnosticReportMedia
#[rustfmt::skip]
pub static DIAGNOSTICREPORTMEDIA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "link" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DocumentManifestRelated
#[rustfmt::skip]
pub static DOCUMENTMANIFESTRELATED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "ref" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DocumentReferenceContent
#[rustfmt::skip]
pub static DOCUMENTREFERENCECONTENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attachment" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "format" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DocumentReferenceContext
#[rustfmt::skip]
pub static DOCUMENTREFERENCECONTEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "event" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "facilityType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "practiceSetting" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "related" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sourcePatientInfo" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DocumentReferenceRelatesTo
#[rustfmt::skip]
pub static DOCUMENTREFERENCERELATESTO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EffectEvidenceSynthesisCertainty
#[rustfmt::skip]
pub static EFFECTEVIDENCESYNTHESISCERTAINTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "certaintySubcomponent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EffectEvidenceSynthesisCertaintyCertaintySubcomponent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rating" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for EffectEvidenceSynthesisCertaintyCertaintySubcomponent
#[rustfmt::skip]
pub static EFFECTEVIDENCESYNTHESISCERTAINTYCERTAINTYSUBCOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rating" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EffectEvidenceSynthesisEffectEstimate
#[rustfmt::skip]
pub static EFFECTEVIDENCESYNTHESISEFFECTESTIMATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "precisionEstimate" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EffectEvidenceSynthesisEffectEstimatePrecisionEstimate"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unitOfMeasure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "variantState" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EffectEvidenceSynthesisEffectEstimatePrecisionEstimate
#[rustfmt::skip]
pub static EFFECTEVIDENCESYNTHESISEFFECTESTIMATEPRECISIONESTIMATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "from" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "level" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "to" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EffectEvidenceSynthesisResultsByExposure
#[rustfmt::skip]
pub static EFFECTEVIDENCESYNTHESISRESULTSBYEXPOSURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "exposureState" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "riskEvidenceSynthesis" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "variantState" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EffectEvidenceSynthesisSampleSize
#[rustfmt::skip]
pub static EFFECTEVIDENCESYNTHESISSAMPLESIZE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numberOfParticipants" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "numberOfStudies" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EncounterClassHistory
#[rustfmt::skip]
pub static ENCOUNTERCLASSHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "class" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EncounterDiagnosis
#[rustfmt::skip]
pub static ENCOUNTERDIAGNOSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "condition" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rank" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EncounterHospitalization
#[rustfmt::skip]
pub static ENCOUNTERHOSPITALIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "admitSource" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "destination" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "dietPreference" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dischargeDisposition" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "origin" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "preAdmissionIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reAdmission" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "specialArrangement" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialCourtesy" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for EncounterLocation
#[rustfmt::skip]
pub static ENCOUNTERLOCATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "physicalType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EncounterParticipant
#[rustfmt::skip]
pub static ENCOUNTERPARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "individual" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for EncounterStatusHistory
#[rustfmt::skip]
pub static ENCOUNTERSTATUSHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EpisodeOfCareDiagnosis
#[rustfmt::skip]
pub static EPISODEOFCAREDIAGNOSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "condition" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rank" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EpisodeOfCareStatusHistory
#[rustfmt::skip]
pub static EPISODEOFCARESTATUSHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Event
#[rustfmt::skip]
pub static EVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instantiatesCanonical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instantiatesUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Complex("Element"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reported[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "researchStudy" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "statusReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EvidenceVariableCharacteristic
#[rustfmt::skip]
pub static EVIDENCEVARIABLECHARACTERISTIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "definition[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "exclude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupMeasure" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participantEffective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "timeFromStart" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usageContext" => FieldInfo {
        field_type: FhirFieldType::Complex("UsageContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioActor
#[rustfmt::skip]
pub static EXAMPLESCENARIOACTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actorId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioInstance
#[rustfmt::skip]
pub static EXAMPLESCENARIOINSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "containedInstance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioInstanceContainedInstance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resourceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "resourceType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioInstanceVersion"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioInstanceContainedInstance
#[rustfmt::skip]
pub static EXAMPLESCENARIOINSTANCECONTAINEDINSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resourceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "versionId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioInstanceVersion
#[rustfmt::skip]
pub static EXAMPLESCENARIOINSTANCEVERSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "versionId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioProcess
#[rustfmt::skip]
pub static EXAMPLESCENARIOPROCESS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "postConditions" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "preConditions" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "step" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioProcessStep"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioProcessStep
#[rustfmt::skip]
pub static EXAMPLESCENARIOPROCESSSTEP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "alternative" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioProcessStepAlternative"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioProcessStepOperation"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "pause" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioProcessStepAlternative
#[rustfmt::skip]
pub static EXAMPLESCENARIOPROCESSSTEPALTERNATIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExampleScenarioProcessStepOperation
#[rustfmt::skip]
pub static EXAMPLESCENARIOPROCESSSTEPOPERATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "initiator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "initiatorActive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "number" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "receiver" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "receiverActive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitAccident
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITACCIDENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitAddItem
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITADDITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitAddItemDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "detailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "itemSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "location[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "provider" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "serviced[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subDetailSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subSite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitAddItemDetail
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITADDITEMDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subDetail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitAddItemDetailSubDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitAddItemDetailSubDetail
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITADDITEMDETAILSUBDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitBenefitBalance
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITBENEFITBALANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "excluded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "financial" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitBenefitBalanceFinancial"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "network" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "term" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitBenefitBalanceFinancial
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITBENEFITBALANCEFINANCIAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allowed[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "used[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ExplanationOfBenefitCareTeam
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITCARETEAM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "provider" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "qualification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responsible" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitDiagnosis
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITDIAGNOSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "diagnosis[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onAdmission" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "packageCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitInsurance
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITINSURANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "coverage" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "focal" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preAuthRef" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitItem
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "adjudication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitItemAdjudication"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "careTeamSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitItemDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "diagnosisSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "informationSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "location[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "procedureSequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "revenue" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "serviced[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subSite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitItemAdjudication
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITITEMADJUDICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitItemDetail
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITITEMDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "revenue" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subDetail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitItemDetailSubDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitItemDetailSubDetail
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITITEMDETAILSUBDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "net" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "noteNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productOrService" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "programCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "revenue" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unitPrice" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitPayee
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITPAYEE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "party" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitPayment
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITPAYMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "adjustment" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "adjustmentReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitProcedure
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITPROCEDURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "procedure[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "udi" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitProcessNote
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITPROCESSNOTE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "number" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitRelated
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITRELATED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "claim" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefitSupportingInfo
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITSUPPORTINGINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "timing[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ExplanationOfBenefitTotal
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFITTOTAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for FamilyMemberHistoryCondition
#[rustfmt::skip]
pub static FAMILYMEMBERHISTORYCONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contributedToDeath" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "onset[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Age"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for FiveWs
#[rustfmt::skip]
pub static FIVEWS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "cause" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "class" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "context" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "done[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "grade" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "init" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "planned" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "what[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "where[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: true,
    },
    "who" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "why[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: true,
    },
    "witness" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for GoalTarget
#[rustfmt::skip]
pub static GOALTARGET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "detail[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "due[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "measure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for GraphDefinitionLink
#[rustfmt::skip]
pub static GRAPHDEFINITIONLINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "max" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "min" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "path" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sliceName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GraphDefinitionLinkTarget"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for GraphDefinitionLinkTarget
#[rustfmt::skip]
pub static GRAPHDEFINITIONLINKTARGET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "compartment" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GraphDefinitionLinkTargetCompartment"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "params" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for GraphDefinitionLinkTargetCompartment
#[rustfmt::skip]
pub static GRAPHDEFINITIONLINKTARGETCOMPARTMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rule" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for GroupCharacteristic
#[rustfmt::skip]
pub static GROUPCHARACTERISTIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "exclude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for GroupMember
#[rustfmt::skip]
pub static GROUPMEMBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "entity" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "inactive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for HealthcareServiceAvailableTime
#[rustfmt::skip]
pub static HEALTHCARESERVICEAVAILABLETIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allDay" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availableEndTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Time),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availableStartTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Time),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "daysOfWeek" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for HealthcareServiceEligibility
#[rustfmt::skip]
pub static HEALTHCARESERVICEELIGIBILITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for HealthcareServiceNotAvailable
#[rustfmt::skip]
pub static HEALTHCARESERVICENOTAVAILABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "during" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ImagingStudySeries
#[rustfmt::skip]
pub static IMAGINGSTUDYSERIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endpoint" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImagingStudySeriesInstance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "laterality" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modality" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "number" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "numberOfInstances" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImagingStudySeriesPerformer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specimen" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "started" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "uid" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImagingStudySeriesInstance
#[rustfmt::skip]
pub static IMAGINGSTUDYSERIESINSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "number" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sopClass" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "uid" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImagingStudySeriesPerformer
#[rustfmt::skip]
pub static IMAGINGSTUDYSERIESPERFORMER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "function" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ImmunizationEducation
#[rustfmt::skip]
pub static IMMUNIZATIONEDUCATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "documentType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "presentationDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "publicationDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImmunizationPerformer
#[rustfmt::skip]
pub static IMMUNIZATIONPERFORMER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "function" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ImmunizationProtocolApplied
#[rustfmt::skip]
pub static IMMUNIZATIONPROTOCOLAPPLIED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authority" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "doseNumber[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "series" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "seriesDoses[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "targetDisease" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ImmunizationReaction
#[rustfmt::skip]
pub static IMMUNIZATIONREACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reported" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImmunizationRecommendationRecommendation
#[rustfmt::skip]
pub static IMMUNIZATIONRECOMMENDATIONRECOMMENDATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contraindicatedVaccineCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dateCriterion" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImmunizationRecommendationRecommendationDateCriterion"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "doseNumber[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "forecastReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "forecastStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "series" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "seriesDoses[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "supportingImmunization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "supportingPatientInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "targetDisease" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "vaccineCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ImmunizationRecommendationRecommendationDateCriterion
#[rustfmt::skip]
pub static IMMUNIZATIONRECOMMENDATIONRECOMMENDATIONDATECRITERION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideDefinition
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "grouping" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideDefinitionGrouping"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "page" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideDefinitionPage"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideDefinitionParameter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideDefinitionResource"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "template" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideDefinitionTemplate"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideDefinitionGrouping
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEDEFINITIONGROUPING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideDefinitionPage
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEDEFINITIONPAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "generation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideDefinitionParameter
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEDEFINITIONPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideDefinitionResource
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEDEFINITIONRESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "example[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fhirVersion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupingId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideDefinitionTemplate
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEDEFINITIONTEMPLATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "scope" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideDependsOn
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEDEPENDSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "packageId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "uri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideGlobal
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEGLOBAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideManifest
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEMANIFEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "image" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "other" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "page" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideManifestPage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rendering" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideManifestResource"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideManifestPage
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEMANIFESTPAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "anchor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuideManifestResource
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDEMANIFESTRESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "example[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "relativePath" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanContact
#[rustfmt::skip]
pub static INSURANCEPLANCONTACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Complex("HumanName"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "telecom" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanCoverage
#[rustfmt::skip]
pub static INSURANCEPLANCOVERAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "benefit" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanCoverageBenefit"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "network" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanCoverageBenefit
#[rustfmt::skip]
pub static INSURANCEPLANCOVERAGEBENEFIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "limit" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanCoverageBenefitLimit"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requirement" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanCoverageBenefitLimit
#[rustfmt::skip]
pub static INSURANCEPLANCOVERAGEBENEFITLIMIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanPlan
#[rustfmt::skip]
pub static INSURANCEPLANPLAN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "coverageArea" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "generalCost" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanPlanGeneralCost"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "network" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specificCost" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanPlanSpecificCost"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanPlanGeneralCost
#[rustfmt::skip]
pub static INSURANCEPLANPLANGENERALCOST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "cost" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupSize" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanPlanSpecificCost
#[rustfmt::skip]
pub static INSURANCEPLANPLANSPECIFICCOST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "benefit" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanPlanSpecificCostBenefit"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanPlanSpecificCostBenefit
#[rustfmt::skip]
pub static INSURANCEPLANPLANSPECIFICCOSTBENEFIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "cost" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanPlanSpecificCostBenefitCost"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InsurancePlanPlanSpecificCostBenefitCost
#[rustfmt::skip]
pub static INSURANCEPLANPLANSPECIFICCOSTBENEFITCOST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "applicability" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "qualifiers" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InvoiceLineItem
#[rustfmt::skip]
pub static INVOICELINEITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "chargeItem[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "priceComponent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InvoiceLineItemPriceComponent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InvoiceLineItemPriceComponent
#[rustfmt::skip]
pub static INVOICELINEITEMPRICECOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for InvoiceParticipant
#[rustfmt::skip]
pub static INVOICEPARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for LinkageItem
#[rustfmt::skip]
pub static LINKAGEITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ListEntry
#[rustfmt::skip]
pub static LISTENTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "deleted" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "flag" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for LocationHoursOfOperation
#[rustfmt::skip]
pub static LOCATIONHOURSOFOPERATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allDay" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "closingTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Time),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "daysOfWeek" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "openingTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Time),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for LocationPosition
#[rustfmt::skip]
pub static LOCATIONPOSITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "altitude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "latitude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "longitude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MeasureGroup
#[rustfmt::skip]
pub static MEASUREGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureGroupPopulation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "stratifier" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureGroupStratifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MeasureGroupPopulation
#[rustfmt::skip]
pub static MEASUREGROUPPOPULATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "criteria" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MeasureGroupStratifier
#[rustfmt::skip]
pub static MEASUREGROUPSTRATIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "component" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureGroupStratifierComponent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "criteria" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MeasureGroupStratifierComponent
#[rustfmt::skip]
pub static MEASUREGROUPSTRATIFIERCOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "criteria" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MeasureReportGroup
#[rustfmt::skip]
pub static MEASUREREPORTGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "measureScore" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureReportGroupPopulation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "stratifier" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureReportGroupStratifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MeasureReportGroupPopulation
#[rustfmt::skip]
pub static MEASUREREPORTGROUPPOPULATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "count" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subjectResults" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MeasureReportGroupStratifier
#[rustfmt::skip]
pub static MEASUREREPORTGROUPSTRATIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "stratum" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureReportGroupStratifierStratum"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MeasureReportGroupStratifierStratum
#[rustfmt::skip]
pub static MEASUREREPORTGROUPSTRATIFIERSTRATUM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "component" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureReportGroupStratifierStratumComponent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "measureScore" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureReportGroupStratifierStratumPopulation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MeasureReportGroupStratifierStratumComponent
#[rustfmt::skip]
pub static MEASUREREPORTGROUPSTRATIFIERSTRATUMCOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MeasureReportGroupStratifierStratumPopulation
#[rustfmt::skip]
pub static MEASUREREPORTGROUPSTRATIFIERSTRATUMPOPULATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "count" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subjectResults" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MeasureSupplementalData
#[rustfmt::skip]
pub static MEASURESUPPLEMENTALDATA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "criteria" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "usage" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationAdministrationDosage
#[rustfmt::skip]
pub static MEDICATIONADMINISTRATIONDOSAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "dose" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "method" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rate[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "route" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "site" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationAdministrationPerformer
#[rustfmt::skip]
pub static MEDICATIONADMINISTRATIONPERFORMER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "function" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationBatch
#[rustfmt::skip]
pub static MEDICATIONBATCH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "expirationDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "lotNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationDispensePerformer
#[rustfmt::skip]
pub static MEDICATIONDISPENSEPERFORMER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "function" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationDispenseSubstitution
#[rustfmt::skip]
pub static MEDICATIONDISPENSESUBSTITUTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "responsibleParty" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "wasSubstituted" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationIngredient
#[rustfmt::skip]
pub static MEDICATIONINGREDIENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isActive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "strength" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeAdministrationGuidelines
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEADMINISTRATIONGUIDELINES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "dosage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeAdministrationGuidelinesDosage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "indication[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "patientCharacteristics" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeAdministrationGuidelinesDosage
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEADMINISTRATIONGUIDELINESDOSAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "dosage" => FieldInfo {
        field_type: FhirFieldType::Complex("Dosage"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEADMINISTRATIONGUIDELINESPATIENTCHARACTERISTICS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "characteristic[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeCost
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGECOST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "cost" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeDrugCharacteristic
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEDRUGCHARACTERISTIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for MedicationKnowledgeIngredient
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEINGREDIENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isActive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "strength" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeKinetics
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEKINETICS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "areaUnderCurve" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "halfLifePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "lethalDose50" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeMedicineClassification
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEMEDICINECLASSIFICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "classification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeMonitoringProgram
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEMONITORINGPROGRAM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeMonograph
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEMONOGRAPH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgePackaging
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEPACKAGING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeRegulatory
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEREGULATORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxDispense" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeRegulatoryMaxDispense"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "regulatoryAuthority" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "schedule" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeRegulatorySchedule"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "substitution" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeRegulatorySubstitution"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeRegulatoryMaxDispense
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEREGULATORYMAXDISPENSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeRegulatorySchedule
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEREGULATORYSCHEDULE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "schedule" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeRegulatorySubstitution
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGEREGULATORYSUBSTITUTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allowed" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledgeRelatedMedicationKnowledge
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGERELATEDMEDICATIONKNOWLEDGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationRequestDispenseRequest
#[rustfmt::skip]
pub static MEDICATIONREQUESTDISPENSEREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "dispenseInterval" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expectedSupplyDuration" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "initialFill" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationRequestDispenseRequestInitialFill"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numberOfRepeatsAllowed" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "validityPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationRequestDispenseRequestInitialFill
#[rustfmt::skip]
pub static MEDICATIONREQUESTDISPENSEREQUESTINITIALFILL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "duration" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationRequestSubstitution
#[rustfmt::skip]
pub static MEDICATIONREQUESTSUBSTITUTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allowed[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductAuthorizationJurisdictionalAuthorization
#[rustfmt::skip]
pub static MEDICINALPRODUCTAUTHORIZATIONJURISDICTIONALAUTHORIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "country" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "legalStatusOfSupply" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "validityPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductAuthorizationProcedure
#[rustfmt::skip]
pub static MEDICINALPRODUCTAUTHORIZATIONPROCEDURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductContraindicationOtherTherapy
#[rustfmt::skip]
pub static MEDICINALPRODUCTCONTRAINDICATIONOTHERTHERAPY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "medication[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "therapyRelationshipType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductIndicationOtherTherapy
#[rustfmt::skip]
pub static MEDICINALPRODUCTINDICATIONOTHERTHERAPY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "medication[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "therapyRelationshipType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductIngredientSpecifiedSubstance
#[rustfmt::skip]
pub static MEDICINALPRODUCTINGREDIENTSPECIFIEDSUBSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "confidentiality" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "group" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "strength" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductIngredientSpecifiedSubstanceStrength"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductIngredientSpecifiedSubstanceStrength
#[rustfmt::skip]
pub static MEDICINALPRODUCTINGREDIENTSPECIFIEDSUBSTANCESTRENGTH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "concentration" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "concentrationLowLimit" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "country" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "measurementPoint" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "presentation" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "presentationLowLimit" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceStrength" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength
#[rustfmt::skip]
pub static MEDICINALPRODUCTINGREDIENTSPECIFIEDSUBSTANCESTRENGTHREFERENCESTRENGTH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "country" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "measurementPoint" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "strength" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "strengthLowLimit" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "substance" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductIngredientSubstance
#[rustfmt::skip]
pub static MEDICINALPRODUCTINGREDIENTSUBSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductInteractionInteractant
#[rustfmt::skip]
pub static MEDICINALPRODUCTINTERACTIONINTERACTANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductManufacturingBusinessOperation
#[rustfmt::skip]
pub static MEDICINALPRODUCTMANUFACTURINGBUSINESSOPERATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authorisationReferenceNumber" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "confidentialityIndicator" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "effectiveDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operationType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "regulator" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductName
#[rustfmt::skip]
pub static MEDICINALPRODUCTNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "countryLanguage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductNameCountryLanguage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "namePart" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductNameNamePart"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductNameCountryLanguage
#[rustfmt::skip]
pub static MEDICINALPRODUCTNAMECOUNTRYLANGUAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "country" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductNameNamePart
#[rustfmt::skip]
pub static MEDICINALPRODUCTNAMENAMEPART_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "part" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPackagedBatchIdentifier
#[rustfmt::skip]
pub static MEDICINALPRODUCTPACKAGEDBATCHIDENTIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "immediatePackaging" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outerPackaging" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPackagedPackageItem
#[rustfmt::skip]
pub static MEDICINALPRODUCTPACKAGEDPACKAGEITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "alternateMaterial" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "device" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "manufacturedItem" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "material" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "otherCharacteristics" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "physicalCharacteristics" => FieldInfo {
        field_type: FhirFieldType::Complex("ProdCharacteristic"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "shelfLifeStorage" => FieldInfo {
        field_type: FhirFieldType::Complex("ProductShelfLife"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPharmaceuticalCharacteristics
#[rustfmt::skip]
pub static MEDICINALPRODUCTPHARMACEUTICALCHARACTERISTICS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPharmaceuticalRouteOfAdministration
#[rustfmt::skip]
pub static MEDICINALPRODUCTPHARMACEUTICALROUTEOFADMINISTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "firstDose" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxDosePerDay" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxDosePerTreatmentPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxSingleDose" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxTreatmentPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "targetSpecies" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies
#[rustfmt::skip]
pub static MEDICINALPRODUCTPHARMACEUTICALROUTEOFADMINISTRATIONTARGETSPECIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "withdrawalPeriod" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod
#[rustfmt::skip]
pub static MEDICINALPRODUCTPHARMACEUTICALROUTEOFADMINISTRATIONTARGETSPECIESWITHDRAWALPERIOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "supportingInformation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "tissue" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductSpecialDesignation
#[rustfmt::skip]
pub static MEDICINALPRODUCTSPECIALDESIGNATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "indication[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "intendedUse" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "species" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MessageDefinitionAllowedResponse
#[rustfmt::skip]
pub static MESSAGEDEFINITIONALLOWEDRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "message" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "situation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MessageDefinitionFocus
#[rustfmt::skip]
pub static MESSAGEDEFINITIONFOCUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "max" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "min" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MessageHeaderDestination
#[rustfmt::skip]
pub static MESSAGEHEADERDESTINATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "endpoint" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "receiver" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MessageHeaderResponse
#[rustfmt::skip]
pub static MESSAGEHEADERRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "details" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MessageHeaderSource
#[rustfmt::skip]
pub static MESSAGEHEADERSOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endpoint" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "software" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MetadataResource
#[rustfmt::skip]
pub static METADATARESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "experimental" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "useContext" => FieldInfo {
        field_type: FhirFieldType::Complex("UsageContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceQuality
#[rustfmt::skip]
pub static MOLECULARSEQUENCEQUALITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fScore" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "gtFP" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "method" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "precision" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "queryFP" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "queryTP" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recall" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "roc" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceQualityRoc"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "score" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "standardSequence" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "truthFN" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "truthTP" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceQualityRoc
#[rustfmt::skip]
pub static MOLECULARSEQUENCEQUALITYROC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fMeasure" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numFN" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numFP" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numTP" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "precision" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "score" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sensitivity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceReferenceSeq
#[rustfmt::skip]
pub static MOLECULARSEQUENCEREFERENCESEQ_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "chromosome" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "genomeBuild" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "orientation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceSeqId" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceSeqPointer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceSeqString" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "strand" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "windowEnd" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "windowStart" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceRepository
#[rustfmt::skip]
pub static MOLECULARSEQUENCEREPOSITORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "datasetId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "readsetId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "variantsetId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceStructureVariant
#[rustfmt::skip]
pub static MOLECULARSEQUENCESTRUCTUREVARIANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "exact" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "inner" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceStructureVariantInner"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "length" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceStructureVariantOuter"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "variantType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceStructureVariantInner
#[rustfmt::skip]
pub static MOLECULARSEQUENCESTRUCTUREVARIANTINNER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceStructureVariantOuter
#[rustfmt::skip]
pub static MOLECULARSEQUENCESTRUCTUREVARIANTOUTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MolecularSequenceVariant
#[rustfmt::skip]
pub static MOLECULARSEQUENCEVARIANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "cigar" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "observedAllele" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceAllele" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "variantPointer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for NamingSystemUniqueId
#[rustfmt::skip]
pub static NAMINGSYSTEMUNIQUEID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "preferred" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for NutritionOrderEnteralFormula
#[rustfmt::skip]
pub static NUTRITIONORDERENTERALFORMULA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additiveProductName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "additiveType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "administration" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionOrderEnteralFormulaAdministration"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "administrationInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "baseFormulaProductName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "baseFormulaType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "caloricDensity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxVolumeToDeliver" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "routeofAdministration" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for NutritionOrderEnteralFormulaAdministration
#[rustfmt::skip]
pub static NUTRITIONORDERENTERALFORMULAADMINISTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "rate[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "schedule" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for NutritionOrderOralDiet
#[rustfmt::skip]
pub static NUTRITIONORDERORALDIET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fluidConsistencyType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "nutrient" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionOrderOralDietNutrient"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "schedule" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "texture" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionOrderOralDietTexture"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for NutritionOrderOralDietNutrient
#[rustfmt::skip]
pub static NUTRITIONORDERORALDIETNUTRIENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for NutritionOrderOralDietTexture
#[rustfmt::skip]
pub static NUTRITIONORDERORALDIETTEXTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "foodType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for NutritionOrderSupplement
#[rustfmt::skip]
pub static NUTRITIONORDERSUPPLEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "schedule" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ObservationComponent
#[rustfmt::skip]
pub static OBSERVATIONCOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "dataAbsentReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "interpretation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ObservationDefinitionQualifiedInterval
#[rustfmt::skip]
pub static OBSERVATIONDEFINITIONQUALIFIEDINTERVAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "age" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "appliesTo" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "context" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "gender" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "gestationalAge" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "range" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ObservationDefinitionQuantitativeDetails
#[rustfmt::skip]
pub static OBSERVATIONDEFINITIONQUANTITATIVEDETAILS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "conversionFactor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "customaryUnit" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "decimalPrecision" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ObservationReferenceRange
#[rustfmt::skip]
pub static OBSERVATIONREFERENCERANGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "age" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "appliesTo" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "high" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "low" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for OperationDefinitionOverload
#[rustfmt::skip]
pub static OPERATIONDEFINITIONOVERLOAD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "parameterName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for OperationDefinitionParameter
#[rustfmt::skip]
pub static OPERATIONDEFINITIONPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "binding" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("OperationDefinitionParameterBinding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "max" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "min" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "referencedFrom" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("OperationDefinitionParameterReferencedFrom"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "searchType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "targetProfile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for OperationDefinitionParameterBinding
#[rustfmt::skip]
pub static OPERATIONDEFINITIONPARAMETERBINDING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "strength" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "valueSet" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for OperationDefinitionParameterReferencedFrom
#[rustfmt::skip]
pub static OPERATIONDEFINITIONPARAMETERREFERENCEDFROM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for OperationOutcomeIssue
#[rustfmt::skip]
pub static OPERATIONOUTCOMEISSUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "details" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "diagnostics" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "severity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for OrganizationContact
#[rustfmt::skip]
pub static ORGANIZATIONCONTACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Complex("HumanName"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "telecom" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ParametersParameter
#[rustfmt::skip]
pub static PARAMETERSPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for PatientCommunication
#[rustfmt::skip]
pub static PATIENTCOMMUNICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preferred" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PatientContact
#[rustfmt::skip]
pub static PATIENTCONTACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "gender" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Complex("HumanName"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "organization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "telecom" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for PatientLink
#[rustfmt::skip]
pub static PATIENTLINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "other" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PaymentReconciliationDetail
#[rustfmt::skip]
pub static PAYMENTRECONCILIATIONDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "payee" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "predecessor" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "request" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "response" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responsible" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "submitter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PaymentReconciliationProcessNote
#[rustfmt::skip]
pub static PAYMENTRECONCILIATIONPROCESSNOTE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PersonLink
#[rustfmt::skip]
pub static PERSONLINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "assurance" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinitionAction
#[rustfmt::skip]
pub static PLANDEFINITIONACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "cardinalityBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionActionCondition"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "definition[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dynamicValue" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionActionDynamicValue"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "goalId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupingBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "input" => FieldInfo {
        field_type: FhirFieldType::Complex("DataRequirement"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "output" => FieldInfo {
        field_type: FhirFieldType::Complex("DataRequirement"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionActionParticipant"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "precheckBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "prefix" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relatedAction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionActionRelatedAction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requiredBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "selectionBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "textEquivalent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "timing[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "transform" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "trigger" => FieldInfo {
        field_type: FhirFieldType::Complex("TriggerDefinition"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinitionActionCondition
#[rustfmt::skip]
pub static PLANDEFINITIONACTIONCONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "expression" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinitionActionDynamicValue
#[rustfmt::skip]
pub static PLANDEFINITIONACTIONDYNAMICVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "expression" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "path" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinitionActionParticipant
#[rustfmt::skip]
pub static PLANDEFINITIONACTIONPARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinitionActionRelatedAction
#[rustfmt::skip]
pub static PLANDEFINITIONACTIONRELATEDACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actionId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "offset[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinitionGoal
#[rustfmt::skip]
pub static PLANDEFINITIONGOAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "addresses" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionGoalTarget"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinitionGoalTarget
#[rustfmt::skip]
pub static PLANDEFINITIONGOALTARGET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "detail[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "due" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "measure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for PractitionerQualification
#[rustfmt::skip]
pub static PRACTITIONERQUALIFICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "issuer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PractitionerRoleAvailableTime
#[rustfmt::skip]
pub static PRACTITIONERROLEAVAILABLETIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allDay" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availableEndTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Time),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availableStartTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Time),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "daysOfWeek" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for PractitionerRoleNotAvailable
#[rustfmt::skip]
pub static PRACTITIONERROLENOTAVAILABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "during" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ProcedureFocalDevice
#[rustfmt::skip]
pub static PROCEDUREFOCALDEVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "manipulated" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ProcedurePerformer
#[rustfmt::skip]
pub static PROCEDUREPERFORMER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "function" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onBehalfOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ProvenanceAgent
#[rustfmt::skip]
pub static PROVENANCEAGENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onBehalfOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "who" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ProvenanceEntity
#[rustfmt::skip]
pub static PROVENANCEENTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "what" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for QuestionnaireItem
#[rustfmt::skip]
pub static QUESTIONNAIREITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "answerOption" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("QuestionnaireItemAnswerOption"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "answerValueSet" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "enableBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "enableWhen" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("QuestionnaireItemEnableWhen"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "initial" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("QuestionnaireItemInitial"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "linkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "maxLength" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "prefix" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "readOnly" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "repeats" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "required" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for QuestionnaireItemAnswerOption
#[rustfmt::skip]
pub static QUESTIONNAIREITEMANSWEROPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "initialSelected" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for QuestionnaireItemEnableWhen
#[rustfmt::skip]
pub static QUESTIONNAIREITEMENABLEWHEN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "answer[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "question" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for QuestionnaireItemInitial
#[rustfmt::skip]
pub static QUESTIONNAIREITEMINITIAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for QuestionnaireResponseItem
#[rustfmt::skip]
pub static QUESTIONNAIRERESPONSEITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "answer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("QuestionnaireResponseItemAnswer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "linkId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for QuestionnaireResponseItemAnswer
#[rustfmt::skip]
pub static QUESTIONNAIRERESPONSEITEMANSWER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for RelatedPersonCommunication
#[rustfmt::skip]
pub static RELATEDPERSONCOMMUNICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preferred" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Request
#[rustfmt::skip]
pub static REQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authoredOn" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "doNotPerform" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "groupIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instantiatesCanonical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instantiatesUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "insurance" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "intent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reasonCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relevantHistory" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "replaces" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reported[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "statusReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInfo" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for RequestGroupAction
#[rustfmt::skip]
pub static REQUESTGROUPACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "cardinalityBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RequestGroupActionCondition"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupingBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "precheckBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "prefix" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedAction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RequestGroupActionRelatedAction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requiredBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "selectionBehavior" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "textEquivalent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "timing[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "title" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for RequestGroupActionCondition
#[rustfmt::skip]
pub static REQUESTGROUPACTIONCONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "expression" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for RequestGroupActionRelatedAction
#[rustfmt::skip]
pub static REQUESTGROUPACTIONRELATEDACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actionId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "offset[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ResearchElementDefinitionCharacteristic
#[rustfmt::skip]
pub static RESEARCHELEMENTDEFINITIONCHARACTERISTIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "definition[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "exclude" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participantEffectiveDescription" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "participantEffectiveGroupMeasure" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "participantEffectiveTimeFromStart" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "participantEffective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "studyEffectiveDescription" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "studyEffectiveGroupMeasure" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "studyEffectiveTimeFromStart" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "studyEffective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "unitOfMeasure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usageContext" => FieldInfo {
        field_type: FhirFieldType::Complex("UsageContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ResearchStudyArm
#[rustfmt::skip]
pub static RESEARCHSTUDYARM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ResearchStudyObjective
#[rustfmt::skip]
pub static RESEARCHSTUDYOBJECTIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for RiskAssessmentPrediction
#[rustfmt::skip]
pub static RISKASSESSMENTPREDICTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "probability[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "qualitativeRisk" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "rationale" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relativeRisk" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "when[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for RiskEvidenceSynthesisCertainty
#[rustfmt::skip]
pub static RISKEVIDENCESYNTHESISCERTAINTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "certaintySubcomponent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RiskEvidenceSynthesisCertaintyCertaintySubcomponent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rating" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for RiskEvidenceSynthesisCertaintyCertaintySubcomponent
#[rustfmt::skip]
pub static RISKEVIDENCESYNTHESISCERTAINTYCERTAINTYSUBCOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rating" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for RiskEvidenceSynthesisRiskEstimate
#[rustfmt::skip]
pub static RISKEVIDENCESYNTHESISRISKESTIMATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "denominatorCount" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numeratorCount" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "precisionEstimate" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RiskEvidenceSynthesisRiskEstimatePrecisionEstimate"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unitOfMeasure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for RiskEvidenceSynthesisRiskEstimatePrecisionEstimate
#[rustfmt::skip]
pub static RISKEVIDENCESYNTHESISRISKESTIMATEPRECISIONESTIMATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "from" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "level" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "to" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for RiskEvidenceSynthesisSampleSize
#[rustfmt::skip]
pub static RISKEVIDENCESYNTHESISSAMPLESIZE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numberOfParticipants" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "numberOfStudies" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SearchParameterComponent
#[rustfmt::skip]
pub static SEARCHPARAMETERCOMPONENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "expression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SpecimenCollection
#[rustfmt::skip]
pub static SPECIMENCOLLECTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "collected[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "collector" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "duration" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fastingStatus[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "method" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SpecimenContainer
#[rustfmt::skip]
pub static SPECIMENCONTAINER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additive[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "capacity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specimenQuantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SpecimenDefinitionTypeTested
#[rustfmt::skip]
pub static SPECIMENDEFINITIONTYPETESTED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "container" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenDefinitionTypeTestedContainer"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "handling" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenDefinitionTypeTestedHandling"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isDerived" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preference" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "rejectionCriterion" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requirement" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "retentionTime" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SpecimenDefinitionTypeTestedContainer
#[rustfmt::skip]
pub static SPECIMENDEFINITIONTYPETESTEDCONTAINER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additive" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenDefinitionTypeTestedContainerAdditive"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "cap" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "capacity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "material" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "minimumVolume[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preparation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SpecimenDefinitionTypeTestedContainerAdditive
#[rustfmt::skip]
pub static SPECIMENDEFINITIONTYPETESTEDCONTAINERADDITIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additive[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SpecimenDefinitionTypeTestedHandling
#[rustfmt::skip]
pub static SPECIMENDEFINITIONTYPETESTEDHANDLING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxDuration" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "temperatureQualifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "temperatureRange" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SpecimenProcessing
#[rustfmt::skip]
pub static SPECIMENPROCESSING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additive" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "procedure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "time[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for StructureDefinitionContext
#[rustfmt::skip]
pub static STRUCTUREDEFINITIONCONTEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "expression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for StructureDefinitionDifferential
#[rustfmt::skip]
pub static STRUCTUREDEFINITIONDIFFERENTIAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "element" => FieldInfo {
        field_type: FhirFieldType::Complex("ElementDefinition"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for StructureDefinitionMapping
#[rustfmt::skip]
pub static STRUCTUREDEFINITIONMAPPING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "uri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for StructureDefinitionSnapshot
#[rustfmt::skip]
pub static STRUCTUREDEFINITIONSNAPSHOT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "element" => FieldInfo {
        field_type: FhirFieldType::Complex("ElementDefinition"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for StructureMapGroup
#[rustfmt::skip]
pub static STRUCTUREMAPGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extends" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "input" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapGroupInput"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "rule" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapGroupRule"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "typeMode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for StructureMapGroupInput
#[rustfmt::skip]
pub static STRUCTUREMAPGROUPINPUT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for StructureMapGroupRule
#[rustfmt::skip]
pub static STRUCTUREMAPGROUPRULE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "dependent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapGroupRuleDependent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapGroupRuleSource"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapGroupRuleTarget"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for StructureMapGroupRuleDependent
#[rustfmt::skip]
pub static STRUCTUREMAPGROUPRULEDEPENDENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "variable" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for StructureMapGroupRuleSource
#[rustfmt::skip]
pub static STRUCTUREMAPGROUPRULESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "check" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "context" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "defaultValue[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "element" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "listMode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "logMessage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "max" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "min" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "variable" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for StructureMapGroupRuleTarget
#[rustfmt::skip]
pub static STRUCTUREMAPGROUPRULETARGET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "context" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contextType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "element" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "listMode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "listRuleId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapGroupRuleTargetParameter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "transform" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "variable" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for StructureMapGroupRuleTargetParameter
#[rustfmt::skip]
pub static STRUCTUREMAPGROUPRULETARGETPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for StructureMapStructure
#[rustfmt::skip]
pub static STRUCTUREMAPSTRUCTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubscriptionChannel
#[rustfmt::skip]
pub static SUBSCRIPTIONCHANNEL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "endpoint" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "header" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "payload" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceIngredient
#[rustfmt::skip]
pub static SUBSTANCEINGREDIENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "substance[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for SubstanceInstance
#[rustfmt::skip]
pub static SUBSTANCEINSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "expiry" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceNucleicAcidSubunit
#[rustfmt::skip]
pub static SUBSTANCENUCLEICACIDSUBUNIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fivePrime" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "length" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "linkage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceNucleicAcidSubunitLinkage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequenceAttachment" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subunit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sugar" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceNucleicAcidSubunitSugar"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "threePrime" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceNucleicAcidSubunitLinkage
#[rustfmt::skip]
pub static SUBSTANCENUCLEICACIDSUBUNITLINKAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "connectivity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "residueSite" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceNucleicAcidSubunitSugar
#[rustfmt::skip]
pub static SUBSTANCENUCLEICACIDSUBUNITSUGAR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "residueSite" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstancePolymerMonomerSet
#[rustfmt::skip]
pub static SUBSTANCEPOLYMERMONOMERSET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "ratioType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "startingMaterial" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstancePolymerMonomerSetStartingMaterial"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SubstancePolymerMonomerSetStartingMaterial
#[rustfmt::skip]
pub static SUBSTANCEPOLYMERMONOMERSETSTARTINGMATERIAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("SubstanceAmount"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isDefining" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "material" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstancePolymerRepeat
#[rustfmt::skip]
pub static SUBSTANCEPOLYMERREPEAT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "averageMolecularFormula" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "numberOfUnits" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "repeatUnit" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstancePolymerRepeatRepeatUnit"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "repeatUnitAmountType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstancePolymerRepeatRepeatUnit
#[rustfmt::skip]
pub static SUBSTANCEPOLYMERREPEATREPEATUNIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("SubstanceAmount"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "degreeOfPolymerisation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "orientationOfPolymerisation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "repeatUnit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "structuralRepresentation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstancePolymerRepeatRepeatUnitStructuralRepresentation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation
#[rustfmt::skip]
pub static SUBSTANCEPOLYMERREPEATREPEATUNITDEGREEOFPOLYMERISATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("SubstanceAmount"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "degree" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SubstancePolymerRepeatRepeatUnitStructuralRepresentation
#[rustfmt::skip]
pub static SUBSTANCEPOLYMERREPEATREPEATUNITSTRUCTURALREPRESENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attachment" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "representation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceProteinSubunit
#[rustfmt::skip]
pub static SUBSTANCEPROTEINSUBUNIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "cTerminalModification" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "cTerminalModificationId" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "length" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "nTerminalModification" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "nTerminalModificationId" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequenceAttachment" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subunit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceReferenceInformationClassification
#[rustfmt::skip]
pub static SUBSTANCEREFERENCEINFORMATIONCLASSIFICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "classification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "domain" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subtype" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceReferenceInformationGene
#[rustfmt::skip]
pub static SUBSTANCEREFERENCEINFORMATIONGENE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "gene" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "geneSequenceOrigin" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceReferenceInformationGeneElement
#[rustfmt::skip]
pub static SUBSTANCEREFERENCEINFORMATIONGENEELEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "element" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceReferenceInformationTarget
#[rustfmt::skip]
pub static SUBSTANCEREFERENCEINFORMATIONTARGET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amountType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "amount[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "interaction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "organism" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "organismType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSourceMaterialFractionDescription
#[rustfmt::skip]
pub static SUBSTANCESOURCEMATERIALFRACTIONDESCRIPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "fraction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "materialType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSourceMaterialOrganism
#[rustfmt::skip]
pub static SUBSTANCESOURCEMATERIALORGANISM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSourceMaterialOrganismAuthor"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "family" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "genus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "hybrid" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSourceMaterialOrganismHybrid"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "intraspecificDescription" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "intraspecificType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "organismGeneral" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSourceMaterialOrganismOrganismGeneral"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "species" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSourceMaterialOrganismAuthor
#[rustfmt::skip]
pub static SUBSTANCESOURCEMATERIALORGANISMAUTHOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authorDescription" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "authorType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSourceMaterialOrganismHybrid
#[rustfmt::skip]
pub static SUBSTANCESOURCEMATERIALORGANISMHYBRID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "hybridType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maternalOrganismId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maternalOrganismName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "paternalOrganismId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "paternalOrganismName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSourceMaterialOrganismOrganismGeneral
#[rustfmt::skip]
pub static SUBSTANCESOURCEMATERIALORGANISMORGANISMGENERAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "class" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "kingdom" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "order" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "phylum" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSourceMaterialPartDescription
#[rustfmt::skip]
pub static SUBSTANCESOURCEMATERIALPARTDESCRIPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "part" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partLocation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationCode
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONCODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "statusDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationMoiety
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONMOIETY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "molecularFormula" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "opticalActivity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "stereochemistry" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationName
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "domain" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "official" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationNameOfficial"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preferred" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationNameOfficial
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONNAMEOFFICIAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authority" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "status" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationProperty
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONPROPERTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "definingSubstance[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "parameters" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationRelationship
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONRELATIONSHIP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amountRatioLowLimit" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "amountType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "amount[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isDefining" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "substance[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for SubstanceSpecificationStructure
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONSTRUCTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isotope" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationStructureIsotope"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "molecularFormula" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "molecularFormulaByMoiety" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "opticalActivity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "representation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationStructureRepresentation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "stereochemistry" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationStructureIsotope
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONSTRUCTUREISOTOPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "halfLife" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "molecularWeight" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationStructureIsotopeMolecularWeight"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "substitution" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationStructureIsotopeMolecularWeight
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONSTRUCTUREISOTOPEMOLECULARWEIGHT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "method" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSpecificationStructureRepresentation
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATIONSTRUCTUREREPRESENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attachment" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "representation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SupplyDeliverySuppliedItem
#[rustfmt::skip]
pub static SUPPLYDELIVERYSUPPLIEDITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SupplyRequestParameter
#[rustfmt::skip]
pub static SUPPLYREQUESTPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for TaskInput
#[rustfmt::skip]
pub static TASKINPUT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for TaskOutput
#[rustfmt::skip]
pub static TASKOUTPUT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for TaskRestriction
#[rustfmt::skip]
pub static TASKRESTRICTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recipient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "repetitions" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesClosure
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESCLOSURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "translation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesCodeSystem
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESCODESYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subsumption" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "uri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesCodeSystemVersion"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesCodeSystemVersion
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESCODESYSTEMVERSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "compositional" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "filter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesCodeSystemVersionFilter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isDefault" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesCodeSystemVersionFilter
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESCODESYSTEMVERSIONFILTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "op" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesExpansion
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESEXPANSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "hierarchical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "incomplete" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "paging" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesExpansionParameter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "textFilter" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesExpansionParameter
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESEXPANSIONPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "documentation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesImplementation
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESIMPLEMENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesSoftware
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESSOFTWARE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesTranslation
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESTRANSLATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "needsMap" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilitiesValidateCode
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIESVALIDATECODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "translations" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestReportParticipant
#[rustfmt::skip]
pub static TESTREPORTPARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "uri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestReportSetup
#[rustfmt::skip]
pub static TESTREPORTSETUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportSetupAction"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestReportSetupAction
#[rustfmt::skip]
pub static TESTREPORTSETUPACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "assert" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportSetupActionAssert"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportSetupActionOperation"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestReportSetupActionAssert
#[rustfmt::skip]
pub static TESTREPORTSETUPACTIONASSERT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "detail" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "message" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "result" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestReportSetupActionOperation
#[rustfmt::skip]
pub static TESTREPORTSETUPACTIONOPERATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "detail" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "message" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "result" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestReportTeardown
#[rustfmt::skip]
pub static TESTREPORTTEARDOWN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportTeardownAction"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestReportTeardownAction
#[rustfmt::skip]
pub static TESTREPORTTEARDOWNACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestReportTest
#[rustfmt::skip]
pub static TESTREPORTTEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportTestAction"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestReportTestAction
#[rustfmt::skip]
pub static TESTREPORTTESTACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptDestination
#[rustfmt::skip]
pub static TESTSCRIPTDESTINATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "index" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptFixture
#[rustfmt::skip]
pub static TESTSCRIPTFIXTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "autocreate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "autodelete" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptMetadata
#[rustfmt::skip]
pub static TESTSCRIPTMETADATA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "capability" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptMetadataCapability"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "link" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptMetadataLink"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptMetadataCapability
#[rustfmt::skip]
pub static TESTSCRIPTMETADATACAPABILITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "capabilities" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "destination" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "link" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "origin" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "required" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "validated" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptMetadataLink
#[rustfmt::skip]
pub static TESTSCRIPTMETADATALINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptOrigin
#[rustfmt::skip]
pub static TESTSCRIPTORIGIN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "index" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptSetup
#[rustfmt::skip]
pub static TESTSCRIPTSETUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptSetupAction"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptSetupAction
#[rustfmt::skip]
pub static TESTSCRIPTSETUPACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "assert" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptSetupActionAssert"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptSetupActionOperation"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptSetupActionAssert
#[rustfmt::skip]
pub static TESTSCRIPTSETUPACTIONASSERT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "compareToSourceExpression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "compareToSourceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "compareToSourcePath" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contentType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "direction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "headerField" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "label" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "minimumId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "navigationLinks" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "operator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "path" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "requestMethod" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "requestURL" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "response" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responseCode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "validateProfileId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "warningOnly" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptSetupActionOperation
#[rustfmt::skip]
pub static TESTSCRIPTSETUPACTIONOPERATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "accept" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contentType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "destination" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "encodeRequestUrl" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "label" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "method" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "origin" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "params" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "requestHeader" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptSetupActionOperationRequestHeader"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requestId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responseId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "targetId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptSetupActionOperationRequestHeader
#[rustfmt::skip]
pub static TESTSCRIPTSETUPACTIONOPERATIONREQUESTHEADER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "field" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptTeardown
#[rustfmt::skip]
pub static TESTSCRIPTTEARDOWN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptTeardownAction"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptTeardownAction
#[rustfmt::skip]
pub static TESTSCRIPTTEARDOWNACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptTest
#[rustfmt::skip]
pub static TESTSCRIPTTEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptTestAction"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptTestAction
#[rustfmt::skip]
pub static TESTSCRIPTTESTACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for TestScriptVariable
#[rustfmt::skip]
pub static TESTSCRIPTVARIABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "defaultValue" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expression" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "headerField" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "hint" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "path" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetCompose
#[rustfmt::skip]
pub static VALUESETCOMPOSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "inactive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "include" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetComposeInclude"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "lockedDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetComposeInclude
#[rustfmt::skip]
pub static VALUESETCOMPOSEINCLUDE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "concept" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetComposeIncludeConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "filter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetComposeIncludeFilter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "valueSet" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetComposeIncludeConcept
#[rustfmt::skip]
pub static VALUESETCOMPOSEINCLUDECONCEPT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "designation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetComposeIncludeConceptDesignation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetComposeIncludeConceptDesignation
#[rustfmt::skip]
pub static VALUESETCOMPOSEINCLUDECONCEPTDESIGNATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetComposeIncludeFilter
#[rustfmt::skip]
pub static VALUESETCOMPOSEINCLUDEFILTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "op" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetExpansion
#[rustfmt::skip]
pub static VALUESETEXPANSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contains" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetExpansionContains"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "offset" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetExpansionParameter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "timestamp" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "total" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetExpansionContains
#[rustfmt::skip]
pub static VALUESETEXPANSIONCONTAINS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "abstract" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "inactive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ValueSetExpansionParameter
#[rustfmt::skip]
pub static VALUESETEXPANSIONPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for VerificationResultAttestation
#[rustfmt::skip]
pub static VERIFICATIONRESULTATTESTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "communicationMethod" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onBehalfOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "proxyIdentityCertificate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "proxySignature" => FieldInfo {
        field_type: FhirFieldType::Complex("Signature"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceIdentityCertificate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceSignature" => FieldInfo {
        field_type: FhirFieldType::Complex("Signature"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "who" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for VerificationResultPrimarySource
#[rustfmt::skip]
pub static VERIFICATIONRESULTPRIMARYSOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "canPushUpdates" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "communicationMethod" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "pushTypeAvailable" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "validationDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "validationStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "who" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for VerificationResultValidator
#[rustfmt::skip]
pub static VERIFICATIONRESULTVALIDATOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attestationSignature" => FieldInfo {
        field_type: FhirFieldType::Complex("Signature"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "identityCertificate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "organization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for VisionPrescriptionLensSpecification
#[rustfmt::skip]
pub static VISIONPRESCRIPTIONLENSSPECIFICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "add" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "axis" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "backCurve" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "brand" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "color" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "cylinder" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "diameter" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "duration" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "eye" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "power" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "prism" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("VisionPrescriptionLensSpecificationPrism"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "product" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "sphere" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for VisionPrescriptionLensSpecificationPrism
#[rustfmt::skip]
pub static VISIONPRESCRIPTIONLENSSPECIFICATIONPRISM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "base" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};
