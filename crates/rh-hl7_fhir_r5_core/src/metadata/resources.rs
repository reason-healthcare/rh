//! Field metadata for FHIR resources

use super::*;
use phf::{phf_map, Map};

/// Field metadata for Account
#[rustfmt::skip]
pub static ACCOUNT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "balance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AccountBalance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "billingStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "calculatedAt" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
        field_type: FhirFieldType::BackboneElement("AccountCoverage"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "currency" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "diagnosis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AccountDiagnosis"),
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
    "procedure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AccountProcedure"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relatedAccount" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AccountRelatedAccount"),
        min: 0,
        max: None,
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
    "asNeeded[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "observationResultRequirement" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ActorDefinition
#[rustfmt::skip]
pub static ACTORDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "capabilities" => FieldInfo {
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
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
    "documentation" => FieldInfo {
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
    "reference" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ActualGroup
#[rustfmt::skip]
pub static ACTUALGROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for AdministrableProductDefinition
#[rustfmt::skip]
pub static ADMINISTRABLEPRODUCTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "administrableDoseForm" => FieldInfo {
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
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
    "formOf" => FieldInfo {
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
    "ingredient" => FieldInfo {
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
    "producedFrom" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdministrableProductDefinitionProperty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "routeOfAdministration" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdministrableProductDefinitionRouteOfAdministration"),
        min: 1,
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
    "unitOfPresentation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
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
    "contributingFactor" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdverseEventContributingFactor"),
        min: 0,
        max: None,
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
    "expectedInResearchStudy" => FieldInfo {
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
    "mitigatingAction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdverseEventMitigatingAction"),
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdverseEventParticipant"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preventiveAction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdverseEventPreventiveAction"),
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
    "resultingEffect" => FieldInfo {
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
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
    "supportingInfo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AdverseEventSupportingInfo"),
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
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AllergyIntoleranceParticipant"),
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
    "account" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
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
    "cancellationDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "cancellationReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "class" => FieldInfo {
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "occurrenceChanged" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "originatingAppointment" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AppointmentParticipant"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "patientInstruction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "previousAppointment" => FieldInfo {
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "recurrenceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recurrenceTemplate" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AppointmentRecurrenceTemplate"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "virtualService" => FieldInfo {
        field_type: FhirFieldType::Complex("VirtualServiceDetail"),
        min: 0,
        max: None,
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "occurrenceDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
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
    "proposedNewTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recurrenceId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "recurring" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
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

/// Field metadata for ArtifactAssessment
#[rustfmt::skip]
pub static ARTIFACTASSESSMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "artifact[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "citeAs[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "content" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ArtifactAssessmentContent"),
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
    "disposition" => FieldInfo {
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
    "workflowStatus" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "authorization" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "occurred[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "outcome" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AuditEventOutcome"),
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
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "severity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "source" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("AuditEventSource"),
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

/// Field metadata for BatchBundle
#[rustfmt::skip]
pub static BATCHBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for BatchResponseBundle
#[rustfmt::skip]
pub static BATCHRESPONSEBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "biologicalSourceEvent" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
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
    "division" => FieldInfo {
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
    "processingFacility" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "productCategory" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "productStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BiologicallyDerivedProductProperty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "request" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "storageTempRequirements" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
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

/// Field metadata for BiologicallyDerivedProductDispense
#[rustfmt::skip]
pub static BIOLOGICALLYDERIVEDPRODUCTDISPENSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "matchStatus" => FieldInfo {
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "originRelationshipType" => FieldInfo {
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BiologicallyDerivedProductDispensePerformer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preparedDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "product" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "usageInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "excludedStructure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BodyStructureIncludedStructure"),
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
    "includedStructure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("BodyStructureIncludedStructure"),
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
    "issues" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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

/// Field metadata for CDSHooksGuidanceResponse
#[rustfmt::skip]
pub static CDSHOOKSGUIDANCERESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CDSHooksRequestOrchestration
#[rustfmt::skip]
pub static CDSHOOKSREQUESTORCHESTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CDSHooksServicePlanDefinition
#[rustfmt::skip]
pub static CDSHOOKSSERVICEPLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CQLLibrary
#[rustfmt::skip]
pub static CQLLIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CanonicalResource
#[rustfmt::skip]
pub static CANONICALRESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for CapabilityStatement
#[rustfmt::skip]
pub static CAPABILITYSTATEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "acceptLanguage" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "custodian" => FieldInfo {
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
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
    "product" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "totalPriceComponent" => FieldInfo {
        field_type: FhirFieldType::Complex("MonetaryComponent"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unitPriceComponent" => FieldInfo {
        field_type: FhirFieldType::Complex("MonetaryComponent"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for Citation
#[rustfmt::skip]
pub static CITATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "citedArtifact" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CitationCitedArtifact"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "classification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CitationClassification"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "currentState" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "statusDate" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CitationStatusDate"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "summary" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CitationSummary"),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "diagnosisRelatedGroup" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "event" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimEvent"),
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
        min: 0,
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
    "patientPaid" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
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
        min: 0,
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
        min: 0,
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
    "traceNumber" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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

/// Field metadata for ClaimResponse
#[rustfmt::skip]
pub static CLAIMRESPONSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "addItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseAddItem"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "adjudication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseItemAdjudication"),
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
    "decision" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "diagnosisRelatedGroup" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "error" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseError"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "event" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClaimResponseEvent"),
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
        min: 0,
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
    "traceNumber" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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

/// Field metadata for ClinicalDocument
#[rustfmt::skip]
pub static CLINICALDOCUMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ClinicalImpression
#[rustfmt::skip]
pub static CLINICALIMPRESSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "changePattern" => FieldInfo {
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
    "performer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
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

/// Field metadata for ClinicalUseDefinition
#[rustfmt::skip]
pub static CLINICALUSEDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "contraindication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClinicalUseDefinitionContraindication"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "indication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClinicalUseDefinitionIndication"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "interaction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClinicalUseDefinitionInteraction"),
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
    "population" => FieldInfo {
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "undesirableEffect" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClinicalUseDefinitionUndesirableEffect"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "warning" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ClinicalUseDefinitionWarning"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for CodeSystem
#[rustfmt::skip]
pub static CODESYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "informationProvider" => FieldInfo {
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relatesTo" => FieldInfo {
        field_type: FhirFieldType::Complex("RelatedArtifact"),
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

/// Field metadata for ComputablePlanDefinition
#[rustfmt::skip]
pub static COMPUTABLEPLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ComputableValueSet
#[rustfmt::skip]
pub static COMPUTABLEVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ConceptMap
#[rustfmt::skip]
pub static CONCEPTMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additionalAttribute" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConceptMapAdditionalAttribute"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConceptMapProperty"),
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
    "sourceScope[x]" => FieldInfo {
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
    "targetScope[x]" => FieldInfo {
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "participant" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionParticipant"),
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

/// Field metadata for ConditionDefinition
#[rustfmt::skip]
pub static CONDITIONDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "definition" => FieldInfo {
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
    "hasBodySite" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "hasSeverity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "hasStage" => FieldInfo {
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
    "medication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionDefinitionMedication"),
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
    "observation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionDefinitionObservation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "plan" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionDefinitionPlan"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "precondition" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionDefinitionPrecondition"),
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
    "questionnaire" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConditionDefinitionQuestionnaire"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "severity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "stage" => FieldInfo {
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
    "subtitle" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for Consent
#[rustfmt::skip]
pub static CONSENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "controller" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "decision" => FieldInfo {
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
    "grantee" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "grantor" => FieldInfo {
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
    "manager" => FieldInfo {
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "policyBasis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConsentPolicyBasis"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "policyText" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "provision" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ConsentProvision"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "regulatoryBasis" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sourceAttachment" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sourceReference" => FieldInfo {
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
    "insurancePlan" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "paymentBy" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoveragePaymentBy"),
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
        field_type: FhirFieldType::Complex("Identifier"),
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
    "event" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityRequestEvent"),
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
    "event" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("CoverageEligibilityResponseEvent"),
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
    "detail" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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

/// Field metadata for Device
#[rustfmt::skip]
pub static DEVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "availabilityStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "biologicalSourceEvent" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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
    "conformsTo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceConformsTo"),
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
    "cycle" => FieldInfo {
        field_type: FhirFieldType::Complex("Count"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "definition" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "displayName" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "endpoint" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
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
    "gateway" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
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
    "mode" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "name" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceName"),
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

/// Field metadata for DeviceAssociation
#[rustfmt::skip]
pub static DEVICEASSOCIATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "bodyStructure" => FieldInfo {
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
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
    "operation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceAssociationOperation"),
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
        field_type: FhirFieldType::Complex("CodeableConcept"),
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

/// Field metadata for DeviceDefinition
#[rustfmt::skip]
pub static DEVICEDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "chargeItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionChargeItem"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "classification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionClassification"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "conformsTo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionConformsTo"),
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
    "correctiveAction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionCorrectiveAction"),
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
    "guideline" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionGuideline"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "hasPart" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionHasPart"),
        min: 0,
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
    "link" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionLink"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "manufacturer" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
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
    "owner" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "packaging" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionPackaging"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "partNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "productionIdentifierInUDI" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionProperty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "regulatoryIdentifier" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionRegulatoryIdentifier"),
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
    "shelfLifeStorage" => FieldInfo {
        field_type: FhirFieldType::Complex("ProductShelfLife"),
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
    "udiDeviceIdentifier" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionUdiDeviceIdentifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "version" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceDefinitionVersion"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for DeviceDispense
#[rustfmt::skip]
pub static DEVICEDISPENSE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "destination" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "device" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::BackboneElement("DeviceDispensePerformer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "preparedDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "receiver" => FieldInfo {
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "usageInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "measurementFrequency" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
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

/// Field metadata for DeviceMetricObservationProfile
#[rustfmt::skip]
pub static DEVICEMETRICOBSERVATIONPROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for DeviceRequest
#[rustfmt::skip]
pub static DEVICEREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "asNeeded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "asNeededFor" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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

/// Field metadata for DeviceUsage
#[rustfmt::skip]
pub static DEVICEUSAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "adherence" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DeviceUsageAdherence"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "device" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "timing[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "usageReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "usageStatus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
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
    "composition" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "conclusion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "note" => FieldInfo {
        field_type: FhirFieldType::Complex("Annotation"),
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
    "study" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "supportingInfo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DiagnosticReportSupportingInfo"),
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

/// Field metadata for DocumentBundle
#[rustfmt::skip]
pub static DOCUMENTBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for DocumentReference
#[rustfmt::skip]
pub static DOCUMENTREFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attester" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DocumentReferenceAttester"),
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
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Reference,
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
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
    "docStatus" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "event" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "modality" => FieldInfo {
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
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for EBMRecommendation
#[rustfmt::skip]
pub static EBMRECOMMENDATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ELMLibrary
#[rustfmt::skip]
pub static ELMLIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "actualPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "admission" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterAdmission"),
        min: 0,
        max: Some(1),
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
    "careTeam" => FieldInfo {
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
    "dietPreference" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "plannedEndDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "plannedStartDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterReason"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
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
    "subjectStatus" => FieldInfo {
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "virtualService" => FieldInfo {
        field_type: FhirFieldType::Complex("VirtualServiceDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for EncounterHistory
#[rustfmt::skip]
pub static ENCOUNTERHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actualPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "class" => FieldInfo {
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
    "length" => FieldInfo {
        field_type: FhirFieldType::Complex("Duration"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "location" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EncounterHistoryLocation"),
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
    "plannedEndDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "plannedStartDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "serviceType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "subjectStatus" => FieldInfo {
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
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
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
    "description" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "environmentType" => FieldInfo {
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
    "payload" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EndpointPayload"),
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
    "careTeam" => FieldInfo {
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EpisodeOfCareReason"),
        min: 0,
        max: None,
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "assertion" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
        field_type: FhirFieldType::BackboneElement("EvidenceCertainty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "citeAs[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "statistic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EvidenceStatistic"),
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
    "studyDesign" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
    "variableDefinition" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EvidenceVariableDefinition"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
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
};

/// Field metadata for EvidenceReport
#[rustfmt::skip]
pub static EVIDENCEREPORT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "citeAs[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "editor" => FieldInfo {
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "relatedIdentifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "relatesTo" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EvidenceReportRelatesTo"),
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
    "section" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EvidenceReportSection"),
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
        field_type: FhirFieldType::BackboneElement("EvidenceReportSubject"),
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
};

/// Field metadata for EvidenceVariable
#[rustfmt::skip]
pub static EVIDENCEVARIABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actual" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "author" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "category" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EvidenceVariableCategory"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("EvidenceVariableCharacteristic"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "handling" => FieldInfo {
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ExampleLipidProfile
#[rustfmt::skip]
pub static EXAMPLELIPIDPROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ExecutableValueSet
#[rustfmt::skip]
pub static EXECUTABLEVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "adjudication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitItemAdjudication"),
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
    "decision" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "diagnosis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitDiagnosis"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "diagnosisRelatedGroup" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "encounter" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "event" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitEvent"),
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
        min: 0,
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
    "patientPaid" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 0,
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
        min: 0,
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
    "traceNumber" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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

/// Field metadata for FHIRPathLibrary
#[rustfmt::skip]
pub static FHIRPATHLIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for FamilyMemberHistory
#[rustfmt::skip]
pub static FAMILYMEMBERHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "age[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Age"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "born[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("FamilyMemberHistoryCondition"),
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
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "deceased[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "estimatedAge" => FieldInfo {
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
        field_type: FhirFieldType::BackboneElement("FamilyMemberHistoryParticipant"),
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
    "procedure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("FamilyMemberHistoryProcedure"),
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
    "relationship" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "sex" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for FamilyMemberHistoryForGeneticsAnalysis
#[rustfmt::skip]
pub static FAMILYMEMBERHISTORYFORGENETICSANALYSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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

/// Field metadata for FormularyItem
#[rustfmt::skip]
pub static FORMULARYITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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

/// Field metadata for GenomicStudy
#[rustfmt::skip]
pub static GENOMICSTUDY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "analysis" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GenomicStudyAnalysis"),
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
        max: Some(1),
        is_choice_type: false,
    },
    "instantiatesUri" => FieldInfo {
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "startDate" => FieldInfo {
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
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
    "continuous" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "source" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
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
    "node" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("GraphDefinitionNode"),
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
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "membership" => FieldInfo {
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

/// Field metadata for GroupDefinition
#[rustfmt::skip]
pub static GROUPDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "availability" => FieldInfo {
        field_type: FhirFieldType::Complex("Availability"),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ExtendedContactDetail"),
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
    "offeredIn" => FieldInfo {
        field_type: FhirFieldType::Reference,
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

/// Field metadata for HistoryBundle
#[rustfmt::skip]
pub static HISTORYBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ImagingSelection
#[rustfmt::skip]
pub static IMAGINGSELECTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "basedOn" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "bodySite" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "derivedFrom" => FieldInfo {
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
    "focus" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "frameOfReferenceUid" => FieldInfo {
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
        field_type: FhirFieldType::BackboneElement("ImagingSelectionInstance"),
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
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ImagingSelectionPerformer"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "seriesNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "seriesUid" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
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
    "studyUid" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "procedure" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "administeredProduct" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "doseQuantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
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
    "informationSource" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::BackboneElement("ImmunizationProgramEligibility"),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "doseNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
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
    "seriesDoses" => FieldInfo {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for Ingredient
#[rustfmt::skip]
pub static INGREDIENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allergenicIndicator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "for" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "function" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "group" => FieldInfo {
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
        field_type: FhirFieldType::BackboneElement("IngredientManufacturer"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "substance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("IngredientSubstance"),
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
        field_type: FhirFieldType::Complex("ExtendedContactDetail"),
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

/// Field metadata for InventoryItem
#[rustfmt::skip]
pub static INVENTORYITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "association" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InventoryItemAssociation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "baseUnit" => FieldInfo {
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
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InventoryItemCharacteristic"),
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
    "description" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InventoryItemDescription"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::BackboneElement("InventoryItemInstance"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "inventoryStatus" => FieldInfo {
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
        field_type: FhirFieldType::BackboneElement("InventoryItemName"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "netContent" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "productReference" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "responsibleOrganization" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InventoryItemResponsibleOrganization"),
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
};

/// Field metadata for InventoryReport
#[rustfmt::skip]
pub static INVENTORYREPORT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "countType" => FieldInfo {
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
    "inventoryListing" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("InventoryReportInventoryListing"),
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
    "operationType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "operationTypeReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reportedDateTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "reportingPeriod" => FieldInfo {
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
    "creation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "period[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "totalPriceComponent" => FieldInfo {
        field_type: FhirFieldType::Complex("MonetaryComponent"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ExtendedContactDetail"),
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
    "form" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "hoursOfOperation" => FieldInfo {
        field_type: FhirFieldType::Complex("Availability"),
        min: 0,
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
    "virtualService" => FieldInfo {
        field_type: FhirFieldType::Complex("VirtualServiceDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for LogicLibrary
#[rustfmt::skip]
pub static LOGICLIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ManufacturedItemDefinition
#[rustfmt::skip]
pub static MANUFACTUREDITEMDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "component" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ManufacturedItemDefinitionComponent"),
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
    "ingredient" => FieldInfo {
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "property" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ManufacturedItemDefinitionProperty"),
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
    "unitOfPresentation" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
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
    "basis" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "scoringUnit" => FieldInfo {
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
    "term" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MeasureTerm"),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "dataUpdateType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "inputParameters" => FieldInfo {
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "measure" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "reportingVendor" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "subject" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "supplementalData" => FieldInfo {
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Medication
#[rustfmt::skip]
pub static MEDICATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "definition" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "doseForm" => FieldInfo {
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
    "marketingAuthorizationHolder" => FieldInfo {
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
    "totalVolume" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MedicationAdministration
#[rustfmt::skip]
pub static MEDICATIONADMINISTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "device" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "isSubPotent" => FieldInfo {
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
    "medication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "occurence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 1,
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
        field_type: FhirFieldType::BackboneElement("MedicationAdministrationPerformer"),
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
    "subPotentReason" => FieldInfo {
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
    "medication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "notPerformedReason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "renderedDosageInstruction" => FieldInfo {
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
    "statusChanged" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "associatedMedication" => FieldInfo {
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
    "clinicalUseIssue" => FieldInfo {
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
    "cost" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeCost"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "definitional" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeDefinitional"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "indicationGuideline" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeIndicationGuideline"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "intendedJurisdiction" => FieldInfo {
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "packaging" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgePackaging"),
        min: 0,
        max: None,
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
    "storageGuideline" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationKnowledgeStorageGuideline"),
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
    "device" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "effectiveDosePeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
    "informationSource" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "medication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "renderedDosageInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reported" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "statusChanged" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "adherence" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicationStatementAdherence"),
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
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
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
    "informationSource" => FieldInfo {
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
    "medication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "relatedClinicalInformation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "renderedDosageInstruction" => FieldInfo {
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

/// Field metadata for MedicinalProductDefinition
#[rustfmt::skip]
pub static MEDICINALPRODUCTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductDefinitionCharacteristic"),
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
    "clinicalTrial" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "comprisedOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductDefinitionContact"),
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
        field_type: FhirFieldType::BackboneElement("MedicinalProductDefinitionCrossReference"),
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
        max: None,
        is_choice_type: false,
    },
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "impurity" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "indication" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ingredient" => FieldInfo {
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
    "legalStatusOfSupply" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
        field_type: FhirFieldType::BackboneElement("MedicinalProductDefinitionName"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "operation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MedicinalProductDefinitionOperation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "packagedMedicinalProduct" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "pediatricUseIndicator" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "route" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "specialMeasures" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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

/// Field metadata for MetadataResource
#[rustfmt::skip]
pub static METADATARESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ModelInfoLibrary
#[rustfmt::skip]
pub static MODELINFOLIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ModuleDefinitionLibrary
#[rustfmt::skip]
pub static MODULEDEFINITIONLIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "focus" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "formatted" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
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
    "literal" => FieldInfo {
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
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
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
    "relative" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("MolecularSequenceRelative"),
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
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for NamingSystem
#[rustfmt::skip]
pub static NAMINGSYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "responsible" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "uniqueId" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NamingSystemUniqueId"),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for NutritionIntake
#[rustfmt::skip]
pub static NUTRITIONINTAKE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "consumedItem" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionIntakeConsumedItem"),
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
    "derivedFrom" => FieldInfo {
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "ingredientLabel" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionIntakeIngredientLabel"),
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
        field_type: FhirFieldType::BackboneElement("NutritionIntakePerformer"),
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
    "reported[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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

/// Field metadata for NutritionOrder
#[rustfmt::skip]
pub static NUTRITIONORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "allergyIntolerance" => FieldInfo {
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
    "outsideFoodAllowed" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "supplement" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionOrderSupplement"),
        min: 0,
        max: None,
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

/// Field metadata for NutritionProduct
#[rustfmt::skip]
pub static NUTRITIONPRODUCT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionProductCharacteristic"),
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
    "implicitRules" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "ingredient" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionProductIngredient"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "instance" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionProductInstance"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "knownAllergen" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "nutrient" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("NutritionProductNutrient"),
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
    "bodyStructure" => FieldInfo {
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
    "instantiates[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "triggeredBy" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ObservationTriggeredBy"),
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

/// Field metadata for ObservationDefinition
#[rustfmt::skip]
pub static OBSERVATIONDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
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
        field_type: FhirFieldType::BackboneElement("ObservationDefinitionComponent"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "device" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "permittedDataType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "permittedUnit" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "qualifiedValue" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ObservationDefinitionQualifiedValue"),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for Observationbmi
#[rustfmt::skip]
pub static OBSERVATIONBMI_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationbodyheight
#[rustfmt::skip]
pub static OBSERVATIONBODYHEIGHT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationbodytemp
#[rustfmt::skip]
pub static OBSERVATIONBODYTEMP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationbodyweight
#[rustfmt::skip]
pub static OBSERVATIONBODYWEIGHT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationbp
#[rustfmt::skip]
pub static OBSERVATIONBP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationheadcircum
#[rustfmt::skip]
pub static OBSERVATIONHEADCIRCUM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationheartrate
#[rustfmt::skip]
pub static OBSERVATIONHEARTRATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationoxygensat
#[rustfmt::skip]
pub static OBSERVATIONOXYGENSAT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationresprate
#[rustfmt::skip]
pub static OBSERVATIONRESPRATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationvitalsigns
#[rustfmt::skip]
pub static OBSERVATIONVITALSIGNS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Observationvitalspanel
#[rustfmt::skip]
pub static OBSERVATIONVITALSPANEL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ExtendedContactDetail"),
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
    "qualification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("OrganizationQualification"),
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
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ExtendedContactDetail"),
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Complex("Narrative"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PackagedProductDefinition
#[rustfmt::skip]
pub static PACKAGEDPRODUCTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attachedDocument" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "characteristic" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PackagedProductDefinitionPackagingProperty"),
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
    "containedItemQuantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "copackagedIndicator" => FieldInfo {
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
        field_type: FhirFieldType::BackboneElement("PackagedProductDefinitionLegalStatusOfSupply"),
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "packageFor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "packaging" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PackagedProductDefinitionPackaging"),
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
        min: 0,
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
    "recipient" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "accountNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "allocation" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PaymentReconciliationAllocation"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "amount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "authorization" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "cardBrand" => FieldInfo {
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "enterer" => FieldInfo {
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
    "issuerType" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "kind" => FieldInfo {
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
    "outcome" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
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
    "processor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "referenceNumber" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "requestor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "returnedAmount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
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
    "tenderedAmount" => FieldInfo {
        field_type: FhirFieldType::Complex("Money"),
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
};

/// Field metadata for Permission
#[rustfmt::skip]
pub static PERMISSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "asserter" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "combining" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "date" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "justification" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PermissionJustification"),
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
    "rule" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PermissionRule"),
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
    "validity" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
    "communication" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PersonCommunication"),
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

/// Field metadata for PlanDefinition
#[rustfmt::skip]
pub static PLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionAction"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "actor" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("PlanDefinitionActor"),
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
    "asNeeded[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
        field_type: FhirFieldType::BackboneElement("PractitionerCommunication"),
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
    "availability" => FieldInfo {
        field_type: FhirFieldType::Complex("Availability"),
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
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "communication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contact" => FieldInfo {
        field_type: FhirFieldType::Complex("ExtendedContactDetail"),
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
    "complication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "focus" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
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
    "occurrence[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "partOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ProcedurePerformer"),
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
    "reported[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "used" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ProfileForCatalog
#[rustfmt::skip]
pub static PROFILEFORCATALOG_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "authorization" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "patient" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "recorded" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
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

/// Field metadata for ProvenanceRelevantHistory
#[rustfmt::skip]
pub static PROVENANCERELEVANTHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishableActivityDefinition
#[rustfmt::skip]
pub static PUBLISHABLEACTIVITYDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishableCodeSystem
#[rustfmt::skip]
pub static PUBLISHABLECODESYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishableConceptMap
#[rustfmt::skip]
pub static PUBLISHABLECONCEPTMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishableLibrary
#[rustfmt::skip]
pub static PUBLISHABLELIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishableMeasure
#[rustfmt::skip]
pub static PUBLISHABLEMEASURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishableNamingSystem
#[rustfmt::skip]
pub static PUBLISHABLENAMINGSYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishablePlanDefinition
#[rustfmt::skip]
pub static PUBLISHABLEPLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PublishableValueSet
#[rustfmt::skip]
pub static PUBLISHABLEVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
        min: 1,
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

/// Field metadata for RegulatedAuthorization
#[rustfmt::skip]
pub static REGULATEDAUTHORIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "attachedDocument" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "basis" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "case" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RegulatedAuthorizationCase"),
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
    "indication" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "intendedUse" => FieldInfo {
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
    "region" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "regulator" => FieldInfo {
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
    "statusDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "validityPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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

/// Field metadata for RequestOrchestration
#[rustfmt::skip]
pub static REQUESTORCHESTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "action" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RequestOrchestrationAction"),
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
    "goal" => FieldInfo {
        field_type: FhirFieldType::Reference,
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

/// Field metadata for Requirements
#[rustfmt::skip]
pub static REQUIREMENTS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "reference" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "statement" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("RequirementsStatement"),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ResearchStudy
#[rustfmt::skip]
pub static RESEARCHSTUDY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "associatedParty" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyAssociatedParty"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "classifier" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "comparisonGroup" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyComparisonGroup"),
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
    "descriptionSummary" => FieldInfo {
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
    "focus" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
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
    "label" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyLabel"),
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
    "outcomeMeasure" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyOutcomeMeasure"),
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
    "progressStatus" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyProgressStatus"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "protocol" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "recruitment" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchStudyRecruitment"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "region" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "result" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "studyDesign" => FieldInfo {
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
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "whyStopped" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ResearchSubject
#[rustfmt::skip]
pub static RESEARCHSUBJECT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "actualComparisonGroup" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "assignedComparisonGroup" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "consent" => FieldInfo {
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "progress" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ResearchSubjectProgress"),
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
    "study" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "constraint" => FieldInfo {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "processingMode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "title" => FieldInfo {
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for SearchSetBundle
#[rustfmt::skip]
pub static SEARCHSETBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "bodyStructure" => FieldInfo {
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "location" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::BackboneElement("ServiceRequestOrderDetail"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "patientInstruction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ServiceRequestPatientInstruction"),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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

/// Field metadata for ShareableActivityDefinition
#[rustfmt::skip]
pub static SHAREABLEACTIVITYDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareableCodeSystem
#[rustfmt::skip]
pub static SHAREABLECODESYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareableConceptMap
#[rustfmt::skip]
pub static SHAREABLECONCEPTMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareableLibrary
#[rustfmt::skip]
pub static SHAREABLELIBRARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareableMeasure
#[rustfmt::skip]
pub static SHAREABLEMEASURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareableNamingSystem
#[rustfmt::skip]
pub static SHAREABLENAMINGSYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareablePlanDefinition
#[rustfmt::skip]
pub static SHAREABLEPLANDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareableTestScript
#[rustfmt::skip]
pub static SHAREABLETESTSCRIPT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ShareableValueSet
#[rustfmt::skip]
pub static SHAREABLEVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Slot
#[rustfmt::skip]
pub static SLOT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "appointmentType" => FieldInfo {
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "combined" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "feature" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SpecimenFeature"),
        min: 0,
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
    "role" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "collection" => FieldInfo {
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "patientPreparation" => FieldInfo {
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
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
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
    "timeAspect" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for StructureMap
#[rustfmt::skip]
pub static STRUCTUREMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "const" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("StructureMapConst"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for Subscription
#[rustfmt::skip]
pub static SUBSCRIPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "channelType" => FieldInfo {
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
    "content" => FieldInfo {
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
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
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
    "filterBy" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubscriptionFilterBy"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "heartbeatPeriod" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
    "managingEntity" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxCount" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
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
        field_type: FhirFieldType::BackboneElement("SubscriptionParameter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "reason" => FieldInfo {
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
    "timeout" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "topic" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for SubscriptionNotificationBundle
#[rustfmt::skip]
pub static SUBSCRIPTIONNOTIFICATIONBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for SubscriptionStatus
#[rustfmt::skip]
pub static SUBSCRIPTIONSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contained" => FieldInfo {
        field_type: FhirFieldType::Complex("Resource"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "error" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "eventsSinceSubscriptionStart" => FieldInfo {
        field_type: FhirFieldType::Complex("integer64"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "notificationEvent" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubscriptionStatusNotificationEvent"),
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
    "subscription" => FieldInfo {
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
    "topic" => FieldInfo {
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

/// Field metadata for SubscriptionTopic
#[rustfmt::skip]
pub static SUBSCRIPTIONTOPIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "approvalDate" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "canFilterBy" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubscriptionTopicCanFilterBy"),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "eventTrigger" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubscriptionTopicEventTrigger"),
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
    "notificationShape" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubscriptionTopicNotificationShape"),
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
    "resourceTrigger" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubscriptionTopicResourceTrigger"),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "quantity" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
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

/// Field metadata for SubstanceDefinition
#[rustfmt::skip]
pub static SUBSTANCEDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "characterization" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionCharacterization"),
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
    "code" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionCode"),
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
    "grade" => FieldInfo {
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
    "informationSource" => FieldInfo {
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
    "moiety" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionMoiety"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "molecularWeight" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionMolecularWeight"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionName"),
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
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionProperty"),
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
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionRelationship"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "sourceMaterial" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionSourceMaterial"),
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
        field_type: FhirFieldType::BackboneElement("SubstanceDefinitionStructure"),
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
    "version" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "modification" => FieldInfo {
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
        max: None,
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
    "deliverFor" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
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
    "item" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "performer" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TaskPerformer"),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "requestedPerformer" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requestedPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: None,
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for TestPlan
#[rustfmt::skip]
pub static TESTPLAN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "category" => FieldInfo {
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
    "copyright" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
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
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "dependency" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestPlanDependency"),
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
    "exitCriteria" => FieldInfo {
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
    "scope" => FieldInfo {
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
    "testCase" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestPlanTestCase"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "testTools" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "scope" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TestScriptScope"),
        min: 0,
        max: None,
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for TransactionBundle
#[rustfmt::skip]
pub static TRANSACTIONBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for TransactionResponseBundle
#[rustfmt::skip]
pub static TRANSACTIONRESPONSEBUNDLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Transport
#[rustfmt::skip]
pub static TRANSPORT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "completionTime" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "currentLocation" => FieldInfo {
        field_type: FhirFieldType::Reference,
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
    "history" => FieldInfo {
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
    "input" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TransportInput"),
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
        field_type: FhirFieldType::BackboneElement("TransportOutput"),
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
    "reason" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableReference"),
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
    "requestedLocation" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "requester" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "restriction" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TransportRestriction"),
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

/// Field metadata for ValueSet
#[rustfmt::skip]
pub static VALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "copyrightLabel" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "scope" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ValueSetScope"),
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
    "versionAlgorithm[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: true,
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
