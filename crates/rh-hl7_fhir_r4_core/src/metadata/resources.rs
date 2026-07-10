//! Field metadata for FHIR resources

use super::*;
use phf::{phf_map, Map};

/// Field metadata for Account
#[rustfmt::skip]
pub static ACCOUNT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "coverage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AccountCoverage"),
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
    "guarantor" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AccountGuarantor"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "owner" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "servicePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for ActivityDefinition
#[rustfmt::skip]
pub static ACTIVITYDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "bodySite" => FieldInfo {
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "dosage" => FieldInfo {
        field_type: FhirFieldType::Complex("Dosage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dynamicValue" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ActivityDefinitionDynamicValue"),
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
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "intent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "library" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "observationRequirement" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "observationResultRequirement" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ActivityDefinitionParticipant"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "product[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specimenRequirement" => FieldInfo {
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
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "timing[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
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
    "topic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "transform" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for Actual Group
#[rustfmt::skip]
pub static ACTUAL_GROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for AdverseEvent
#[rustfmt::skip]
pub static ADVERSEEVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actuality" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contributor" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "detected" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "event" => FieldInfo {
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
        max: Some(1),
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recordedDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recorder" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceDocument" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resultingCondition" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "seriousness" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "severity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "study" => FieldInfo {
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
    "subjectMedicalHistory" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "suspectEntity" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdverseEventSuspectEntity"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AllergyIntolerance
#[rustfmt::skip]
pub static ALLERGYINTOLERANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "asserter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "clinicalStatus" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "criticality" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "lastOccurrence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onset[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "reaction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AllergyIntoleranceReaction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "recordedDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recorder" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "verificationStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Appointment
#[rustfmt::skip]
pub static APPOINTMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "appointmentType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "cancelationReason" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "minutesDuration" => FieldInfo {
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
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AppointmentParticipant"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "patientInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
    "requestedPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "serviceCategory" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "serviceType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "slot" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialty" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "supportingInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for AppointmentResponse
#[rustfmt::skip]
pub static APPOINTMENTRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "appointment" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "participantStatus" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "participantType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
};

/// Field metadata for AuditEvent
#[rustfmt::skip]
pub static AUDITEVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "agent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AuditEventAgent"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "entity" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AuditEventEntity"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "outcomeDesc" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "purposeOfEvent" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AuditEventSource"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subtype" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
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

/// Field metadata for Basic
#[rustfmt::skip]
pub static BASIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for Binary
#[rustfmt::skip]
pub static BINARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contentType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "data" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "securityContext" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BiologicallyDerivedProduct
#[rustfmt::skip]
pub static BIOLOGICALLYDERIVEDPRODUCT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "collection" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BiologicallyDerivedProductCollection"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "manipulation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BiologicallyDerivedProductManipulation"),
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
    "parent" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "processing" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BiologicallyDerivedProductProcessing"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productCategory" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "productCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "request" => FieldInfo {
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
    "storage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BiologicallyDerivedProductStorage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BodyStructure
#[rustfmt::skip]
pub static BODYSTRUCTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "image" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "locationQualifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
    "morphology" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for Bundle
#[rustfmt::skip]
pub static BUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "entry" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BundleEntry"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "link" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BundleLink"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "signature" => FieldInfo {
        field_type: FhirFieldType::Complex("Signature"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "timestamp" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "total" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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

/// Field metadata for CDS Hooks GuidanceResponse
#[rustfmt::skip]
pub static CDS_HOOKS_GUIDANCERESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CDS Hooks RequestGroup
#[rustfmt::skip]
pub static CDS_HOOKS_REQUESTGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CDS Hooks Service PlanDefinition
#[rustfmt::skip]
pub static CDS_HOOKS_SERVICE_PLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CQF-Questionnaire
#[rustfmt::skip]
pub static CQF_QUESTIONNAIRE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CQL Library
#[rustfmt::skip]
pub static CQL_LIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CapabilityStatement
#[rustfmt::skip]
pub static CAPABILITYSTATEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "document" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementDocument"),
        min: 0,
        max: None,
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
    "fhirVersion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "format" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "implementation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementImplementation"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "implementationGuide" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "imports" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instantiates" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "messaging" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementMessaging"),
        min: 0,
        max: None,
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
    "patchFormat" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "rest" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementRest"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "software" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CapabilityStatementSoftware"),
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

/// Field metadata for CarePlan
#[rustfmt::skip]
pub static CAREPLAN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "activity" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CarePlanActivity"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "addresses" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "careTeam" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "contributor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "encounter" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "intent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "replaces" => FieldInfo {
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

/// Field metadata for CareTeam
#[rustfmt::skip]
pub static CARETEAM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "encounter" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "managingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CareTeamParticipant"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CatalogEntry
#[rustfmt::skip]
pub static CATALOGENTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additionalCharacteristic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "additionalClassification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "additionalIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "classification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "lastUpdated" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "orderable" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "referencedItem" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedEntry" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CatalogEntryRelatedEntry"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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
    "validTo" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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

/// Field metadata for ChargeItem
#[rustfmt::skip]
pub static CHARGEITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "account" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "bodysite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "costCenter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "definitionCanonical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "definitionUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "enteredDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "enterer" => FieldInfo {
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
    "factorOverride" => FieldInfo {
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "overrideReason" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ChargeItemPerformer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priceOverride" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "product[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requestingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "service" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ChargeItemDefinition
#[rustfmt::skip]
pub static CHARGEITEMDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "applicability" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ChargeItemDefinitionApplicability"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instance" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "propertyGroup" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ChargeItemDefinitionPropertyGroup"),
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
        min: 1,
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

/// Field metadata for Claim
#[rustfmt::skip]
pub static CLAIM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "accident" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimAccident"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "billablePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "careTeam" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimCareTeam"),
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
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "diagnosis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimDiagnosis"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "enterer" => FieldInfo {
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
    "facility" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "fundsReserve" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "insurance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimInsurance"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "insurer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimItem"),
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
    "originalPrescription" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "payee" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimPayee"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "prescription" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "procedure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimProcedure"),
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
    "referral" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "related" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimRelated"),
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
    "subType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInfo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimSupportingInfo"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "total" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
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
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ClaimResponse
#[rustfmt::skip]
pub static CLAIMRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "addItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseAddItem"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "communicationRequest" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "disposition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "error" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseError"),
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
    "form" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "formCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "fundsReserve" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "insurance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseInsurance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "insurer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseItem"),
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "payeeType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "payment" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponsePayment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "preAuthPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "preAuthRef" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "processNote" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseProcessNote"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "request" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "requestor" => FieldInfo {
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
    "subType" => FieldInfo {
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
    "total" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseTotal"),
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
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Clinical Document
#[rustfmt::skip]
pub static CLINICAL_DOCUMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ClinicalImpression
#[rustfmt::skip]
pub static CLINICALIMPRESSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "assessor" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "effective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "encounter" => FieldInfo {
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
    "finding" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClinicalImpressionFinding"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "investigation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClinicalImpressionInvestigation"),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "previous" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "problem" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "prognosisCodeableConcept" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "prognosisReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "protocol" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "summary" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CodeSystem
#[rustfmt::skip]
pub static CODESYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "caseSensitive" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "concept" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CodeSystemConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "content" => FieldInfo {
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
    "count" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
    "filter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CodeSystemFilter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "hierarchyMeaning" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CodeSystemProperty"),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "supplements" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "valueSet" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "versionNeeded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Communication
#[rustfmt::skip]
pub static COMMUNICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "about" => FieldInfo {
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
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "encounter" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "inResponseTo" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "medium" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "payload" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CommunicationPayload"),
        min: 0,
        max: None,
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
    "received" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "sender" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "topic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CommunicationRequest
#[rustfmt::skip]
pub static COMMUNICATIONREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "about" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "medium" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
    "payload" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CommunicationRequestPayload"),
        min: 0,
        max: None,
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
    "recipient" => FieldInfo {
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
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sender" => FieldInfo {
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
};

/// Field metadata for CompartmentDefinition
#[rustfmt::skip]
pub static COMPARTMENTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "resource" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CompartmentDefinitionResource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "search" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
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

/// Field metadata for Composition
#[rustfmt::skip]
pub static COMPOSITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attester" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CompositionAttester"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "confidentiality" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "custodian" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "event" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CompositionEvent"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "relatesTo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CompositionRelatesTo"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "section" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CompositionSection"),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
        min: 1,
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

/// Field metadata for Computable PlanDefinition
#[rustfmt::skip]
pub static COMPUTABLE_PLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ConceptMap
#[rustfmt::skip]
pub static CONCEPTMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "group" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConceptMapGroup"),
        min: 0,
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "target[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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

/// Field metadata for Condition
#[rustfmt::skip]
pub static CONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "abatement[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "asserter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "clinicalStatus" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "evidence" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionEvidence"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onset[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "recordedDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recorder" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "severity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "stage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionStage"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "verificationStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Consent
#[rustfmt::skip]
pub static CONSENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dateTime" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "organization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "policy" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConsentPolicy"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "policyRule" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "provision" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConsentProvision"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "scope" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "source[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "verification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConsentVerification"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Contract
#[rustfmt::skip]
pub static CONTRACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "authority" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "contentDefinition" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractContentDefinition"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contentDerivative" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "domain" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "expirationType" => FieldInfo {
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
    "friendly" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractFriendly"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instantiatesCanonical" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instantiatesUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "legal" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractLegal"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "legalState" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "legallyBinding[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "relevantHistory" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "rule" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractRule"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "scope" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "signer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractSigner"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "site" => FieldInfo {
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
    "subType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "term" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ContractTerm"),
        min: 0,
        max: None,
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
    "url" => FieldInfo {
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

/// Field metadata for Coverage
#[rustfmt::skip]
pub static COVERAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "beneficiary" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "class" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageClass"),
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
    "contract" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "costToBeneficiary" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageCostToBeneficiary"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dependent" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "network" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "order" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "payor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "policyHolder" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subrogation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subscriber" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subscriberId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityRequest
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "enterer" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "insurance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityRequestInsurance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "insurer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityRequestItem"),
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "priority" => FieldInfo {
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "serviced[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "supportingInfo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityRequestSupportingInfo"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CoverageEligibilityResponse
#[rustfmt::skip]
pub static COVERAGEELIGIBILITYRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "disposition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "error" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityResponseError"),
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
    "form" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "insurance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityResponseInsurance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "insurer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "preAuthRef" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "request" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "requestor" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for DetectedIssue
#[rustfmt::skip]
pub static DETECTEDISSUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "evidence" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DetectedIssueEvidence"),
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
    "identified[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicated" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mitigation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DetectedIssueMitigation"),
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "severity" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Device
#[rustfmt::skip]
pub static DEVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
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
    "definition" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "deviceName" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDeviceName"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "distinctIdentifier" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "manufactureDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "modelNumber" => FieldInfo {
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "owner" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "parent" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceProperty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "safety" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "serialNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "specialization" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceSpecialization"),
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
    "statusReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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
    "udiCarrier" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceUdiCarrier"),
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
    "version" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceVersion"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Device Metric Observation Profile
#[rustfmt::skip]
pub static DEVICE_METRIC_OBSERVATION_PROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for DeviceDefinition
#[rustfmt::skip]
pub static DEVICEDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "capability" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionCapability"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
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
    "deviceName" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionDeviceName"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "languageCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "manufacturer[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "material" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionMaterial"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modelNumber" => FieldInfo {
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "onlineInformation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "owner" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "parentDevice" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "physicalCharacteristics" => FieldInfo {
        field_type: FhirFieldType::Complex("ProdCharacteristic"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionProperty"),
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
    "safety" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "shelfLifeStorage" => FieldInfo {
        field_type: FhirFieldType::Complex("ProductShelfLife"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialization" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionSpecialization"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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
    "udiDeviceIdentifier" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionUdiDeviceIdentifier"),
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
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DeviceMetric
#[rustfmt::skip]
pub static DEVICEMETRIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "calibration" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceMetricCalibration"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "color" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "measurementPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
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
    "operationalStatus" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "parent" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
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

/// Field metadata for DeviceRequest
#[rustfmt::skip]
pub static DEVICEREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "code[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceRequestParameter"),
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
    "performerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priorRequest" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DeviceUseStatement
#[rustfmt::skip]
pub static DEVICEUSESTATEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "derivedFrom" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "device" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "recordedOn" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "timing[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for DiagnosticReport
#[rustfmt::skip]
pub static DIAGNOSTICREPORT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "conclusion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "conclusionCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "effective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "encounter" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "imagingStudy" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "issued" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "media" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DiagnosticReportMedia"),
        min: 0,
        max: None,
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
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "presentedForm" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "result" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resultsInterpreter" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for DiagnosticReport-Genetics
#[rustfmt::skip]
pub static DIAGNOSTICREPORT_GENETICS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for DocumentManifest
#[rustfmt::skip]
pub static DOCUMENTMANIFEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "content" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "masterIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
    "recipient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "related" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DocumentManifestRelated"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DocumentReference
#[rustfmt::skip]
pub static DOCUMENTREFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authenticator" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "content" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DocumentReferenceContent"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "context" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DocumentReferenceContext"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "custodian" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "docStatus" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "masterIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
    "relatesTo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DocumentReferenceRelatesTo"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "securityLabel" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for DomainResource
#[rustfmt::skip]
pub static DOMAINRESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EHRS FM Record Lifecycle Event - Audit Event
#[rustfmt::skip]
pub static EHRS_FM_RECORD_LIFECYCLE_EVENT___AUDIT_EVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for EHRS FM Record Lifecycle Event - Provenance
#[rustfmt::skip]
pub static EHRS_FM_RECORD_LIFECYCLE_EVENT___PROVENANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for EffectEvidenceSynthesis
#[rustfmt::skip]
pub static EFFECTEVIDENCESYNTHESIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "certainty" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EffectEvidenceSynthesisCertainty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectEstimate" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EffectEvidenceSynthesisEffectEstimate"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "exposure" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "exposureAlternative" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "resultsByExposure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EffectEvidenceSynthesisResultsByExposure"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sampleSize" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EffectEvidenceSynthesisSampleSize"),
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
    "studyType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "synthesisType" => FieldInfo {
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
    "topic" => FieldInfo {
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

/// Field metadata for Encounter
#[rustfmt::skip]
pub static ENCOUNTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "account" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "appointment" => FieldInfo {
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
    "class" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "classHistory" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterClassHistory"),
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
    "diagnosis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterDiagnosis"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "episodeOfCare" => FieldInfo {
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
    "hospitalization" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterHospitalization"),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "length" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterLocation"),
        min: 0,
        max: None,
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterParticipant"),
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
    "priority" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "serviceProvider" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "serviceType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "statusHistory" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterStatusHistory"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Endpoint
#[rustfmt::skip]
pub static ENDPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "address" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "connectionType" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "managingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "payloadMimeType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "payloadType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
};

/// Field metadata for EnrollmentRequest
#[rustfmt::skip]
pub static ENROLLMENTREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "candidate" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "coverage" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "created" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "insurer" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "provider" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EnrollmentResponse
#[rustfmt::skip]
pub static ENROLLMENTRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "disposition" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "organization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "requestProvider" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for EpisodeOfCare
#[rustfmt::skip]
pub static EPISODEOFCARE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "account" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "careManager" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "diagnosis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EpisodeOfCareDiagnosis"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "managingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "patient" => FieldInfo {
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
    "referralRequest" => FieldInfo {
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
    "statusHistory" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EpisodeOfCareStatusHistory"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "team" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for EventDefinition
#[rustfmt::skip]
pub static EVENTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
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
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "topic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "trigger" => FieldInfo {
        field_type: FhirFieldType::Complex("TriggerDefinition"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for Evidence
#[rustfmt::skip]
pub static EVIDENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "exposureBackground" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "exposureVariant" => FieldInfo {
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "shortTitle" => FieldInfo {
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
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "topic" => FieldInfo {
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

/// Field metadata for Evidence Synthesis Profile
#[rustfmt::skip]
pub static EVIDENCE_SYNTHESIS_PROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for EvidenceVariable
#[rustfmt::skip]
pub static EVIDENCEVARIABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EvidenceVariableCharacteristic"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "shortTitle" => FieldInfo {
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
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "topic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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

/// Field metadata for Example Lipid Profile
#[rustfmt::skip]
pub static EXAMPLE_LIPID_PROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ExampleScenario
#[rustfmt::skip]
pub static EXAMPLESCENARIO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioActor"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioInstance"),
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
    "process" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExampleScenarioProcess"),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "workflow" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ExplanationOfBenefit
#[rustfmt::skip]
pub static EXPLANATIONOFBENEFIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "accident" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitAccident"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "addItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitAddItem"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "benefitBalance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitBenefitBalance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "benefitPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "billablePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "careTeam" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitCareTeam"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "claim" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "diagnosis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitDiagnosis"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "disposition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "enterer" => FieldInfo {
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
    "facility" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "form" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "formCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "fundsReserve" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "fundsReserveRequested" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "insurance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitInsurance"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "insurer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitItem"),
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
    "originalPrescription" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "payee" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitPayee"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "payment" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitPayment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "preAuthRef" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preAuthRefPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "precedence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "prescription" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "procedure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitProcedure"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "processNote" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitProcessNote"),
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
    "referral" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "related" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitRelated"),
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
    "subType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInfo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitSupportingInfo"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "total" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitTotal"),
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
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Family member history for genetics analysis
#[rustfmt::skip]
pub static FAMILY_MEMBER_HISTORY_FOR_GENETICS_ANALYSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Flag
#[rustfmt::skip]
pub static FLAG_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for Goal
#[rustfmt::skip]
pub static GOAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "achievementStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "addresses" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "expressedBy" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "lifecycleStatus" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcomeCode" => FieldInfo {
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
    "priority" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "start[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "statusDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "statusReason" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "target" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GoalTarget"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for GraphDefinition
#[rustfmt::skip]
pub static GRAPHDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "link" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GraphDefinitionLink"),
        min: 0,
        max: None,
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for Group
#[rustfmt::skip]
pub static GROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "actual" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GroupCharacteristic"),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "managingEntity" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "member" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GroupMember"),
        min: 0,
        max: None,
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
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Group Definition
#[rustfmt::skip]
pub static GROUP_DEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for GuidanceResponse
#[rustfmt::skip]
pub static GUIDANCERESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dataRequirement" => FieldInfo {
        field_type: FhirFieldType::Complex("DataRequirement"),
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
    "evaluationMessage" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "module[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "occurrenceDateTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "outputParameters" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "requestIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "result" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for HealthcareService
#[rustfmt::skip]
pub static HEALTHCARESERVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "appointmentRequired" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availabilityExceptions" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availableTime" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("HealthcareServiceAvailableTime"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "communication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "coverageArea" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "eligibility" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("HealthcareServiceEligibility"),
        min: 0,
        max: None,
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
    "extraDetails" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "notAvailable" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("HealthcareServiceNotAvailable"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "photo" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "program" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "providedBy" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referralMethod" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "serviceProvisionCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialty" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for ImagingStudy
#[rustfmt::skip]
pub static IMAGINGSTUDY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "interpreter" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "modality" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "numberOfInstances" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "numberOfSeries" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "procedureCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "procedureReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "referrer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "series" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImagingStudySeries"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for Immunization
#[rustfmt::skip]
pub static IMMUNIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "doseQuantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "education" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImmunizationEducation"),
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
    "expirationDate" => FieldInfo {
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
    "fundingSource" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isSubpotent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImmunizationPerformer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "primarySource" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "programEligibility" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "protocolApplied" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImmunizationProtocolApplied"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reaction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImmunizationReaction"),
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
    "reportOrigin" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
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
    "subpotentReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "vaccineCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImmunizationEvaluation
#[rustfmt::skip]
pub static IMMUNIZATIONEVALUATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authority" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
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
    "doseStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "doseStatusReason" => FieldInfo {
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
    "immunizationEvent" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "targetDisease" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
};

/// Field metadata for ImmunizationRecommendation
#[rustfmt::skip]
pub static IMMUNIZATIONRECOMMENDATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authority" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "recommendation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImmunizationRecommendationRecommendation"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ImplementationGuide
#[rustfmt::skip]
pub static IMPLEMENTATIONGUIDE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "definition" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideDefinition"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "dependsOn" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideDependsOn"),
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
    "fhirVersion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "global" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideGlobal"),
        min: 0,
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
    "license" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "manifest" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImplementationGuideManifest"),
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "packageId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 1,
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
        min: 1,
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

/// Field metadata for InsurancePlan
#[rustfmt::skip]
pub static INSURANCEPLAN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "administeredBy" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanContact"),
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
    "coverage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanCoverage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "coverageArea" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "network" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "ownedBy" => FieldInfo {
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
    "plan" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InsurancePlanPlan"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for Invoice
#[rustfmt::skip]
pub static INVOICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "account" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "cancelledReason" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "issuer" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "lineItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InvoiceLineItem"),
        min: 0,
        max: None,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InvoiceParticipant"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "paymentTerms" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "totalGross" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "totalNet" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
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

/// Field metadata for Library
#[rustfmt::skip]
pub static LIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "content" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
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
    "dataRequirement" => FieldInfo {
        field_type: FhirFieldType::Complex("DataRequirement"),
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
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "parameter" => FieldInfo {
        field_type: FhirFieldType::Complex("ParameterDefinition"),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
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
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "topic" => FieldInfo {
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
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for Linkage
#[rustfmt::skip]
pub static LINKAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("LinkageItem"),
        min: 1,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for List
#[rustfmt::skip]
pub static LIST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
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
    "emptyReason" => FieldInfo {
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
    "entry" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ListEntry"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "source" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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

/// Field metadata for Location
#[rustfmt::skip]
pub static LOCATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "availabilityExceptions" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "hoursOfOperation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("LocationHoursOfOperation"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "managingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "operationalStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "position" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("LocationPosition"),
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
    "telecom" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for Measure
#[rustfmt::skip]
pub static MEASURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "clinicalRecommendationStatement" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "compositeScoring" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "disclaimer" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
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
    "group" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureGroup"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "guidance" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "improvementNotation" => FieldInfo {
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "library" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "rateAggregation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "rationale" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "riskAdjustment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "scoring" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supplementalData" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureSupplementalData"),
        min: 0,
        max: None,
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
    "topic" => FieldInfo {
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
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for MeasureReport
#[rustfmt::skip]
pub static MEASUREREPORT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "evaluatedResource" => FieldInfo {
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
    "group" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureReportGroup"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "improvementNotation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "measure" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "reporter" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Media
#[rustfmt::skip]
pub static MEDIA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "content" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "created[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "device" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "deviceName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "duration" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "frames" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "height" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "issued" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modality" => FieldInfo {
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "operator" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "view" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "width" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Medication
#[rustfmt::skip]
pub static MEDICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "batch" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationBatch"),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "form" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ingredient" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationIngredient"),
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
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
};

/// Field metadata for MedicationAdministration
#[rustfmt::skip]
pub static MEDICATIONADMINISTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "device" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dosage" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationAdministrationDosage"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "effective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "eventHistory" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instantiates" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "medication[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationAdministrationPerformer"),
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
    "request" => FieldInfo {
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
        max: None,
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationDispense
#[rustfmt::skip]
pub static MEDICATIONDISPENSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authorizingPrescription" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "daysSupply" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
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
    "detectedIssue" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dosageInstruction" => FieldInfo {
        field_type: FhirFieldType::Complex("Dosage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "eventHistory" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationDispensePerformer"),
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
    "receiver" => FieldInfo {
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
    "statusReason[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "substitution" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationDispenseSubstitution"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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
    "whenHandedOver" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "whenPrepared" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationKnowledge
#[rustfmt::skip]
pub static MEDICATIONKNOWLEDGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "administrationGuidelines" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeAdministrationGuidelines"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "associatedMedication" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contraindication" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "cost" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeCost"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "doseForm" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "drugCharacteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeDrugCharacteristic"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ingredient" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeIngredient"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "intendedRoute" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "kinetics" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeKinetics"),
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
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "medicineClassification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeMedicineClassification"),
        min: 0,
        max: None,
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
    "monitoringProgram" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeMonitoringProgram"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "monograph" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeMonograph"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "packaging" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgePackaging"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "preparationInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "productType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "regulatory" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeRegulatory"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relatedMedicationKnowledge" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeRelatedMedicationKnowledge"),
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
    "synonym" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationRequest
#[rustfmt::skip]
pub static MEDICATIONREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "courseOfTherapyType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "detectedIssue" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dispenseRequest" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationRequestDispenseRequest"),
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
    "dosageInstruction" => FieldInfo {
        field_type: FhirFieldType::Complex("Dosage"),
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
    "eventHistory" => FieldInfo {
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
    "groupIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "performerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "priorPrescription" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "recorder" => FieldInfo {
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
    "substitution" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationRequestSubstitution"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supportingInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationStatement
#[rustfmt::skip]
pub static MEDICATIONSTATEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "dateAsserted" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "derivedFrom" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dosage" => FieldInfo {
        field_type: FhirFieldType::Complex("Dosage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "informationSource" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "medication[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "statusReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProduct
#[rustfmt::skip]
pub static MEDICINALPRODUCT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additionalMonitoringIndicator" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "attachedDocument" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "clinicalTrial" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "combinedPharmaceuticalDoseForm" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "crossReference" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "domain" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "legalStatusOfSupply" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "manufacturingBusinessOperation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductManufacturingBusinessOperation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "marketingStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("MarketingStatus"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "masterFile" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
        field_type: FhirFieldType::BackboneElement("MedicinalProductName"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "packagedMedicinalProduct" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "paediatricUseIndicator" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "pharmaceuticalProduct" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productClassification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialDesignation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductSpecialDesignation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialMeasures" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for MedicinalProductAuthorization
#[rustfmt::skip]
pub static MEDICINALPRODUCTAUTHORIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "country" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dataExclusivityPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "dateOfFirstAuthorization" => FieldInfo {
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
    "holder" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "internationalBirthDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "jurisdictionalAuthorization" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductAuthorizationJurisdictionalAuthorization"),
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
    "legalBasis" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "procedure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductAuthorizationProcedure"),
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
    "restoreDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "statusDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "validityPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductContraindication
#[rustfmt::skip]
pub static MEDICINALPRODUCTCONTRAINDICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comorbidity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "disease" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "diseaseStatus" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "otherTherapy" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductContraindicationOtherTherapy"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::Complex("Population"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "therapeuticIndication" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductIndication
#[rustfmt::skip]
pub static MEDICINALPRODUCTINDICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comorbidity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "diseaseStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "diseaseSymptomProcedure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "intendedEffect" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "otherTherapy" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductIndicationOtherTherapy"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::Complex("Population"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "undesirableEffect" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductIngredient
#[rustfmt::skip]
pub static MEDICINALPRODUCTINGREDIENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allergenicIndicator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "specifiedSubstance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductIngredientSpecifiedSubstance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "substance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductIngredientSubstance"),
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
};

/// Field metadata for MedicinalProductInteraction
#[rustfmt::skip]
pub static MEDICINALPRODUCTINTERACTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "effect" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "incidence" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "interactant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductInteractionInteractant"),
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
    "management" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for MedicinalProductManufactured
#[rustfmt::skip]
pub static MEDICINALPRODUCTMANUFACTURED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ingredient" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "manufacturedDoseForm" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unitOfPresentation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPackaged
#[rustfmt::skip]
pub static MEDICINALPRODUCTPACKAGED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "batchIdentifier" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductPackagedBatchIdentifier"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "legalStatusOfSupply" => FieldInfo {
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
    "marketingAuthorization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "marketingStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("MarketingStatus"),
        min: 0,
        max: None,
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
    "packageItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductPackagedPackageItem"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductPharmaceutical
#[rustfmt::skip]
pub static MEDICINALPRODUCTPHARMACEUTICAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "administrableDoseForm" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "characteristics" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductPharmaceuticalCharacteristics"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ingredient" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "routeOfAdministration" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductPharmaceuticalRouteOfAdministration"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unitOfPresentation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicinalProductUndesirableEffect
#[rustfmt::skip]
pub static MEDICINALPRODUCTUNDESIRABLEEFFECT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "classification" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "frequencyOfOccurrence" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "population" => FieldInfo {
        field_type: FhirFieldType::Complex("Population"),
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
    "symptomConditionEffect" => FieldInfo {
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
};

/// Field metadata for MessageDefinition
#[rustfmt::skip]
pub static MESSAGEDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allowedResponse" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MessageDefinitionAllowedResponse"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "base" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "event[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
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
    "focus" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MessageDefinitionFocus"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "graph" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
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
    "parent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "responseRequired" => FieldInfo {
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

/// Field metadata for MessageHeader
#[rustfmt::skip]
pub static MESSAGEHEADER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "destination" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MessageHeaderDestination"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "enterer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "event[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "focus" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "response" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MessageHeaderResponse"),
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
    "sender" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MessageHeaderSource"),
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
};

/// Field metadata for MolecularSequence
#[rustfmt::skip]
pub static MOLECULARSEQUENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "coordinateSystem" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "device" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "observedSeq" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "pointer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quality" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceQuality"),
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
    "readCoverage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceSeq" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceReferenceSeq"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "repository" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceRepository"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specimen" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "structureVariant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceStructureVariant"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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
    "variant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceVariant"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for NamingSystem
#[rustfmt::skip]
pub static NAMINGSYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responsible" => FieldInfo {
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "uniqueId" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NamingSystemUniqueId"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
};

/// Field metadata for NutritionOrder
#[rustfmt::skip]
pub static NUTRITIONORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allergyIntolerance" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "dateTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "enteralFormula" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionOrderEnteralFormula"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "excludeFoodModifier" => FieldInfo {
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
    "foodPreferenceModifier" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instantiates" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "intent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "oralDiet" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionOrderOralDiet"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "orderer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "patient" => FieldInfo {
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
    "supplement" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionOrderSupplement"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Observation
#[rustfmt::skip]
pub static OBSERVATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "component" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ObservationComponent"),
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
    "dataAbsentReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "derivedFrom" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "device" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "effective[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "encounter" => FieldInfo {
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
    "focus" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "hasMember" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "issued" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "referenceRange" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ObservationReferenceRange"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specimen" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for Observation-genetics
#[rustfmt::skip]
pub static OBSERVATION_GENETICS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ObservationDefinition
#[rustfmt::skip]
pub static OBSERVATIONDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "abnormalCodedValueSet" => FieldInfo {
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "criticalCodedValueSet" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
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
    "multipleResultsAllowed" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "normalCodedValueSet" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "permittedDataType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preferredReportName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "qualifiedInterval" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ObservationDefinitionQualifiedInterval"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "quantitativeDetails" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ObservationDefinitionQuantitativeDetails"),
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
    "validCodedValueSet" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for OperationDefinition
#[rustfmt::skip]
pub static OPERATIONDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "affectsState" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "base" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "inputProfile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instance" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "jurisdiction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "outputProfile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "overload" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("OperationDefinitionOverload"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("OperationDefinitionParameter"),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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

/// Field metadata for OperationOutcome
#[rustfmt::skip]
pub static OPERATIONOUTCOME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "issue" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("OperationOutcomeIssue"),
        min: 1,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Organization
#[rustfmt::skip]
pub static ORGANIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("OrganizationContact"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for OrganizationAffiliation
#[rustfmt::skip]
pub static ORGANIZATIONAFFILIATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
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
    "healthcareService" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "network" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "organization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "participatingOrganization" => FieldInfo {
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
    "specialty" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PICO Element Profile
#[rustfmt::skip]
pub static PICO_ELEMENT_PROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Parameters
#[rustfmt::skip]
pub static PARAMETERS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ParametersParameter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Patient
#[rustfmt::skip]
pub static PATIENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
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
    "communication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PatientCommunication"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PatientContact"),
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
    "deceased[x]" => FieldInfo {
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
    "gender" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "generalPractitioner" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "link" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PatientLink"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "managingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maritalStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "multipleBirth[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Complex("HumanName"),
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
    "telecom" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PaymentNotice
#[rustfmt::skip]
pub static PAYMENTNOTICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "payee" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "payment" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "paymentDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "paymentStatus" => FieldInfo {
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
    "recipient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
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
};

/// Field metadata for PaymentReconciliation
#[rustfmt::skip]
pub static PAYMENTRECONCILIATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "detail" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PaymentReconciliationDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "disposition" => FieldInfo {
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
    "formCode" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "paymentAmount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "paymentDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "paymentIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "paymentIssuer" => FieldInfo {
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
    "processNote" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PaymentReconciliationProcessNote"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "request" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "requestor" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Person
#[rustfmt::skip]
pub static PERSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "link" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PersonLink"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "managingOrganization" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
        field_type: FhirFieldType::Complex("HumanName"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "photo" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PlanDefinition
#[rustfmt::skip]
pub static PLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionAction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
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
    "goal" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionGoal"),
        min: 0,
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "library" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
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
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "topic" => FieldInfo {
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
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for Practitioner
#[rustfmt::skip]
pub static PRACTITIONER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
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
    "communication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
        field_type: FhirFieldType::Complex("HumanName"),
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
    "qualification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PractitionerQualification"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PractitionerRole
#[rustfmt::skip]
pub static PRACTITIONERROLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availabilityExceptions" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "availableTime" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PractitionerRoleAvailableTime"),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
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
    "healthcareService" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "notAvailable" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PractitionerRoleNotAvailable"),
        min: 0,
        max: None,
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
    "practitioner" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "specialty" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Procedure
#[rustfmt::skip]
pub static PROCEDURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "asserter" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "bodySite" => FieldInfo {
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
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "complication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "complicationDetail" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "encounter" => FieldInfo {
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
    "focalDevice" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ProcedureFocalDevice"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "followUp" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "language" => FieldInfo {
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performed[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ProcedurePerformer"),
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
    "recorder" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "report" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usedCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "usedReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Profile for Catalog
#[rustfmt::skip]
pub static PROFILE_FOR_CATALOG_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Profile for HLA Genotyping Results
#[rustfmt::skip]
pub static PROFILE_FOR_HLA_GENOTYPING_RESULTS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Provenance
#[rustfmt::skip]
pub static PROVENANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "activity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "agent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ProvenanceAgent"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "entity" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ProvenanceEntity"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "occurred[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "policy" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "signature" => FieldInfo {
        field_type: FhirFieldType::Complex("Signature"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Provenance Relevant History
#[rustfmt::skip]
pub static PROVENANCE_RELEVANT_HISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Questionnaire
#[rustfmt::skip]
pub static QUESTIONNAIRE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "derivedFrom" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("QuestionnaireItem"),
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "subjectType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
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

/// Field metadata for QuestionnaireResponse
#[rustfmt::skip]
pub static QUESTIONNAIRERESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "authored" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "item" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("QuestionnaireResponseItem"),
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "questionnaire" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for RelatedPerson
#[rustfmt::skip]
pub static RELATEDPERSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "address" => FieldInfo {
        field_type: FhirFieldType::Complex("Address"),
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
    "communication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RelatedPersonCommunication"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
        field_type: FhirFieldType::Complex("HumanName"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "patient" => FieldInfo {
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
    "photo" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: None,
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for RequestGroup
#[rustfmt::skip]
pub static REQUESTGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RequestGroupAction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "author" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "groupIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "intent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
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
    "replaces" => FieldInfo {
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for ResearchDefinition
#[rustfmt::skip]
pub static RESEARCHDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "experimental" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "exposure" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "exposureAlternative" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "library" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "shortTitle" => FieldInfo {
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
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "topic" => FieldInfo {
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
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for ResearchElementDefinition
#[rustfmt::skip]
pub static RESEARCHELEMENTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchElementDefinitionCharacteristic"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "library" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "shortTitle" => FieldInfo {
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
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "topic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "usage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "variableType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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

/// Field metadata for ResearchStudy
#[rustfmt::skip]
pub static RESEARCHSTUDY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "arm" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyArm"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "enrollment" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "keyword" => FieldInfo {
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "objective" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyObjective"),
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "phase" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "primaryPurposeType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "principalInvestigator" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "protocol" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reasonStopped" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "site" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sponsor" => FieldInfo {
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

/// Field metadata for ResearchSubject
#[rustfmt::skip]
pub static RESEARCHSUBJECT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actualArm" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "assignedArm" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "consent" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "individual" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
    "study" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for Resource
#[rustfmt::skip]
pub static RESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
};

/// Field metadata for RiskAssessment
#[rustfmt::skip]
pub static RISKASSESSMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "basis" => FieldInfo {
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
    "condition" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
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
    "mitigation" => FieldInfo {
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
    "parent" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "prediction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RiskAssessmentPrediction"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
};

/// Field metadata for RiskEvidenceSynthesis
#[rustfmt::skip]
pub static RISKEVIDENCESYNTHESIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "certainty" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RiskEvidenceSynthesisCertainty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "editor" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "effectivePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "endorser" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "exposure" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "lastReviewDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "population" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relatedArtifact" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reviewer" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "riskEstimate" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RiskEvidenceSynthesisRiskEstimate"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sampleSize" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RiskEvidenceSynthesisSampleSize"),
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
    "studyType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "synthesisType" => FieldInfo {
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
    "topic" => FieldInfo {
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

/// Field metadata for Schedule
#[rustfmt::skip]
pub static SCHEDULE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "active" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "actor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "planningHorizon" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "serviceCategory" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "serviceType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialty" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SearchParameter
#[rustfmt::skip]
pub static SEARCHPARAMETER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "base" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "chain" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "component" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SearchParameterComponent"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "derivedFrom" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "experimental" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "modifier" => FieldInfo {
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
    "multipleAnd" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "multipleOr" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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
        min: 1,
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
    "xpath" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "xpathUsage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ServiceRequest
#[rustfmt::skip]
pub static SERVICEREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "asNeeded[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
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
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "encounter" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "locationCode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "locationReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "orderDetail" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "patientInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "quantity[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "requisition" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "specimen" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ServiceRequest-Genetics
#[rustfmt::skip]
pub static SERVICEREQUEST_GENETICS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Shareable ActivityDefinition
#[rustfmt::skip]
pub static SHAREABLE_ACTIVITYDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Shareable CodeSystem
#[rustfmt::skip]
pub static SHAREABLE_CODESYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Shareable Library
#[rustfmt::skip]
pub static SHAREABLE_LIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Shareable Measure
#[rustfmt::skip]
pub static SHAREABLE_MEASURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Shareable PlanDefinition
#[rustfmt::skip]
pub static SHAREABLE_PLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Shareable ValueSet
#[rustfmt::skip]
pub static SHAREABLE_VALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Slot
#[rustfmt::skip]
pub static SLOT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "appointmentType" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "overbooked" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "schedule" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "serviceCategory" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "serviceType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialty" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Specimen
#[rustfmt::skip]
pub static SPECIMEN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "accessionIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "collection" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenCollection"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "container" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenContainer"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "parent" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "processing" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenProcessing"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "receivedTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "request" => FieldInfo {
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
        max: Some(1),
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
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

/// Field metadata for SpecimenDefinition
#[rustfmt::skip]
pub static SPECIMENDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "collection" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "patientPreparation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "timeAspect" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "typeCollected" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "typeTested" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenDefinitionTypeTested"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for StructureDefinition
#[rustfmt::skip]
pub static STRUCTUREDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "abstract" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "baseDefinition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "context" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureDefinitionContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contextInvariant" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "derivation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "differential" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureDefinitionDifferential"),
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
    "fhirVersion" => FieldInfo {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "keyword" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mapping" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureDefinitionMapping"),
        min: 0,
        max: None,
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "snapshot" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureDefinitionSnapshot"),
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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

/// Field metadata for StructureMap
#[rustfmt::skip]
pub static STRUCTUREMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "group" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapGroup"),
        min: 1,
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "import" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "publisher" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "structure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapStructure"),
        min: 0,
        max: None,
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
        min: 1,
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

/// Field metadata for Subscription
#[rustfmt::skip]
pub static SUBSCRIPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "channel" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubscriptionChannel"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
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
    "criteria" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "error" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Substance
#[rustfmt::skip]
pub static SUBSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ingredient" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceIngredient"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceInstance"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
};

/// Field metadata for SubstanceNucleicAcid
#[rustfmt::skip]
pub static SUBSTANCENUCLEICACID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "areaOfHybridisation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "numberOfSubunits" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "oligoNucleotideType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequenceType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subunit" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceNucleicAcidSubunit"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstancePolymer
#[rustfmt::skip]
pub static SUBSTANCEPOLYMER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "class" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "copolymerConnectivity" => FieldInfo {
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
    "geometry" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "meta" => FieldInfo {
        field_type: FhirFieldType::Complex("Meta"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "modification" => FieldInfo {
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
    "monomerSet" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstancePolymerMonomerSet"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "repeat" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstancePolymerRepeat"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceProtein
#[rustfmt::skip]
pub static SUBSTANCEPROTEIN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "disulfideLinkage" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "numberOfSubunits" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sequenceType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "subunit" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceProteinSubunit"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceReferenceInformation
#[rustfmt::skip]
pub static SUBSTANCEREFERENCEINFORMATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "classification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceReferenceInformationClassification"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "gene" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceReferenceInformationGene"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "geneElement" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceReferenceInformationGeneElement"),
        min: 0,
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
    "target" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceReferenceInformationTarget"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubstanceSourceMaterial
#[rustfmt::skip]
pub static SUBSTANCESOURCEMATERIAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "countryOfOrigin" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "developmentStage" => FieldInfo {
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
    "fractionDescription" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSourceMaterialFractionDescription"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "geographicalLocation" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "organism" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSourceMaterialOrganism"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "organismId" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "organismName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "parentSubstanceId" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "parentSubstanceName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "partDescription" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSourceMaterialPartDescription"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sourceMaterialClass" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceMaterialState" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sourceMaterialType" => FieldInfo {
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
};

/// Field metadata for SubstanceSpecification
#[rustfmt::skip]
pub static SUBSTANCESPECIFICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationCode"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
    "moiety" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationMoiety"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationName"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "nucleicAcid" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "polymer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationProperty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "protein" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "relationship" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationRelationship"),
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
    "sourceMaterial" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "structure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceSpecificationStructure"),
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SupplyDelivery
#[rustfmt::skip]
pub static SUPPLYDELIVERY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "destination" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "receiver" => FieldInfo {
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
    "suppliedItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SupplyDeliverySuppliedItem"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supplier" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SupplyRequest
#[rustfmt::skip]
pub static SUPPLYREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "authoredOn" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "deliverFrom" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "deliverTo" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "parameter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SupplyRequestParameter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "priority" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "supplier" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Task
#[rustfmt::skip]
pub static TASK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "businessStatus" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "executionPeriod" => FieldInfo {
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
    "focus" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "for" => FieldInfo {
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "input" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TaskInput"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instantiatesCanonical" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "instantiatesUri" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "intent" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "lastModified" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "output" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TaskOutput"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "owner" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
        max: Some(1),
        is_choice_type: false,
    },
    "reasonReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "restriction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TaskRestriction"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TerminologyCapabilities
#[rustfmt::skip]
pub static TERMINOLOGYCAPABILITIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "closure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesClosure"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "codeSearch" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "codeSystem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesCodeSystem"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expansion" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesExpansion"),
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
    "implementation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesImplementation"),
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
    "kind" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "lockedDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "software" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesSoftware"),
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
    "translation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesTranslation"),
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
    "validateCode" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TerminologyCapabilitiesValidateCode"),
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

/// Field metadata for TestReport
#[rustfmt::skip]
pub static TESTREPORT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportParticipant"),
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
    "score" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "setup" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportSetup"),
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
    "teardown" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportTeardown"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "test" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestReportTest"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "testScript" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "tester" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
};

/// Field metadata for TestScript
#[rustfmt::skip]
pub static TESTSCRIPT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "destination" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptDestination"),
        min: 0,
        max: None,
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
    "fixture" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptFixture"),
        min: 0,
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
    "metadata" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptMetadata"),
        min: 0,
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
    "origin" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptOrigin"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "setup" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptSetup"),
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
    "teardown" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptTeardown"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "test" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptTest"),
        min: 0,
        max: None,
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "useContext" => FieldInfo {
        field_type: FhirFieldType::Complex("UsageContext"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "variable" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptVariable"),
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

/// Field metadata for ValueSet
#[rustfmt::skip]
pub static VALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "compose" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetCompose"),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "expansion" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetExpansion"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "immutable" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "purpose" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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

/// Field metadata for VerificationResult
#[rustfmt::skip]
pub static VERIFICATIONRESULT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attestation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("VerificationResultAttestation"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "failureAction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "frequency" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "lastPerformed" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "need" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "nextScheduled" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "primarySource" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("VerificationResultPrimarySource"),
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
    "statusDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "target" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "targetLocation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "validationProcess" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "validationType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "validator" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("VerificationResultValidator"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for VisionPrescription
#[rustfmt::skip]
pub static VISIONPRESCRIPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "created" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "dateWritten" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
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
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "lensSpecification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("VisionPrescriptionLensSpecification"),
        min: 1,
        max: None,
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "prescriber" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for observation-bmi
#[rustfmt::skip]
pub static OBSERVATION_BMI_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-bodyheight
#[rustfmt::skip]
pub static OBSERVATION_BODYHEIGHT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-bodytemp
#[rustfmt::skip]
pub static OBSERVATION_BODYTEMP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-bodyweight
#[rustfmt::skip]
pub static OBSERVATION_BODYWEIGHT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-bp
#[rustfmt::skip]
pub static OBSERVATION_BP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-headcircum
#[rustfmt::skip]
pub static OBSERVATION_HEADCIRCUM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-heartrate
#[rustfmt::skip]
pub static OBSERVATION_HEARTRATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-oxygensat
#[rustfmt::skip]
pub static OBSERVATION_OXYGENSAT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-resprate
#[rustfmt::skip]
pub static OBSERVATION_RESPRATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-vitalsigns
#[rustfmt::skip]
pub static OBSERVATION_VITALSIGNS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for observation-vitalspanel
#[rustfmt::skip]
pub static OBSERVATION_VITALSPANEL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};
