//! Field metadata for other FHIR types

use super::*;
use phf::{phf_map, Map};

/// Field metadata for ParticipantContactable
pub static PARTICIPANTCONTACTABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "telecom" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Product
pub static PRODUCT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instance" => FieldInfo {
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
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Definition
pub static DEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "partOf" => FieldInfo {
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "derivedFromUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "product" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "useContext" => FieldInfo {
        field_type: FhirFieldType::Complex("UsageContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "derivedFromCanonical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "topic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ParticipantLiving
pub static PARTICIPANTLIVING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "communication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "photo" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "birthDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "gender" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Participant
pub static PARTICIPANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
};

/// Field metadata for FiveWs
pub static FIVEWS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subject" => FieldInfo {
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
    "witness" => FieldInfo {
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
    "why[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: true,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
    "context" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "cause" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "who" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "done[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "grade" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Event
pub static EVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "statusReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "researchStudy" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "relevantHistory" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "product" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
};

/// Field metadata for Request
pub static REQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "groupIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "statusReason" => FieldInfo {
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
    "supportingInfo" => FieldInfo {
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
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "performerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relevantHistory" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "authoredOn" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "deliverTo" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "product" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "insurance" => FieldInfo {
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
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Publishable
pub static PUBLISHABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "useContext" => FieldInfo {
        field_type: FhirFieldType::Complex("UsageContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "editor" => FieldInfo {
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
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "topic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
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
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Shareable
pub static SHAREABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "title" => FieldInfo {
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "knowledgeRepresentationLevel" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
};
