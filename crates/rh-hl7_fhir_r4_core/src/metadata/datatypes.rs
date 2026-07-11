//! Field metadata for FHIR datatypes

use super::*;
use phf::{phf_map, Map};

/// Field metadata for AD-use
#[rustfmt::skip]
pub static AD_USE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-additionalLocator
#[rustfmt::skip]
pub static ADXP_ADDITIONALLOCATOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-buildingNumberSuffix
#[rustfmt::skip]
pub static ADXP_BUILDINGNUMBERSUFFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-careOf
#[rustfmt::skip]
pub static ADXP_CAREOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-censusTract
#[rustfmt::skip]
pub static ADXP_CENSUSTRACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-delimiter
#[rustfmt::skip]
pub static ADXP_DELIMITER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-deliveryAddressLine
#[rustfmt::skip]
pub static ADXP_DELIVERYADDRESSLINE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-deliveryInstallationArea
#[rustfmt::skip]
pub static ADXP_DELIVERYINSTALLATIONAREA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-deliveryInstallationQualifier
#[rustfmt::skip]
pub static ADXP_DELIVERYINSTALLATIONQUALIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-deliveryInstallationType
#[rustfmt::skip]
pub static ADXP_DELIVERYINSTALLATIONTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-deliveryMode
#[rustfmt::skip]
pub static ADXP_DELIVERYMODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-deliveryModeIdentifier
#[rustfmt::skip]
pub static ADXP_DELIVERYMODEIDENTIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-direction
#[rustfmt::skip]
pub static ADXP_DIRECTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-houseNumber
#[rustfmt::skip]
pub static ADXP_HOUSENUMBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-houseNumberNumeric
#[rustfmt::skip]
pub static ADXP_HOUSENUMBERNUMERIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-postBox
#[rustfmt::skip]
pub static ADXP_POSTBOX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-precinct
#[rustfmt::skip]
pub static ADXP_PRECINCT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-streetAddressLine
#[rustfmt::skip]
pub static ADXP_STREETADDRESSLINE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-streetName
#[rustfmt::skip]
pub static ADXP_STREETNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-streetNameBase
#[rustfmt::skip]
pub static ADXP_STREETNAMEBASE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-streetNameType
#[rustfmt::skip]
pub static ADXP_STREETNAMETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-unitID
#[rustfmt::skip]
pub static ADXP_UNITID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ADXP-unitType
#[rustfmt::skip]
pub static ADXP_UNITTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Accession
#[rustfmt::skip]
pub static ACCESSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Address
#[rustfmt::skip]
pub static ADDRESS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "city" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "country" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "district" => FieldInfo {
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
    "line" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "postalCode" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "state" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Age
#[rustfmt::skip]
pub static AGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for Allele
#[rustfmt::skip]
pub static ALLELE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for AminoAcidChange
#[rustfmt::skip]
pub static AMINOACIDCHANGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Analysis
#[rustfmt::skip]
pub static ANALYSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Ancestry
#[rustfmt::skip]
pub static ANCESTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Annotation
#[rustfmt::skip]
pub static ANNOTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "author[x]" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 1,
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

/// Field metadata for Anonymized
#[rustfmt::skip]
pub static ANONYMIZED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for AssessedCondition
#[rustfmt::skip]
pub static ASSESSEDCONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Attachment
#[rustfmt::skip]
pub static ATTACHMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contentType" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "creation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "data" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "hash" => FieldInfo {
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "size" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for BackboneElement
#[rustfmt::skip]
pub static BACKBONEELEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
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

/// Field metadata for BodyStructure Reference
#[rustfmt::skip]
pub static BODYSTRUCTURE_REFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for CodeableConcept
#[rustfmt::skip]
pub static CODEABLECONCEPT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "coding" => FieldInfo {
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
    "text" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Coding
#[rustfmt::skip]
pub static CODING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "userSelected" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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

/// Field metadata for ContactDetail
#[rustfmt::skip]
pub static CONTACTDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "telecom" => FieldInfo {
        field_type: FhirFieldType::Complex("ContactPoint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for ContactPoint
#[rustfmt::skip]
pub static CONTACTPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "rank" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
};

/// Field metadata for Contributor
#[rustfmt::skip]
pub static CONTRIBUTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "contact" => FieldInfo {
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

/// Field metadata for CopyNumberEvent
#[rustfmt::skip]
pub static COPYNUMBEREVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Count
#[rustfmt::skip]
pub static COUNT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for DNARegionName
#[rustfmt::skip]
pub static DNAREGIONNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Data Absent Reason
#[rustfmt::skip]
pub static DATA_ABSENT_REASON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for DataElement constraint on ElementDefinition data type
#[rustfmt::skip]
pub static DATAELEMENT_CONSTRAINT_ON_ELEMENTDEFINITION_DATA_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for DataRequirement
#[rustfmt::skip]
pub static DATAREQUIREMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "codeFilter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DataRequirementCodeFilter"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dateFilter" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DataRequirementDateFilter"),
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
    "limit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "mustSupport" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "sort" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DataRequirementSort"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "subject[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Design Note
#[rustfmt::skip]
pub static DESIGN_NOTE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Display Name
#[rustfmt::skip]
pub static DISPLAY_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Distance
#[rustfmt::skip]
pub static DISTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for Dosage
#[rustfmt::skip]
pub static DOSAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "additionalInstruction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "asNeeded[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "doseAndRate" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("DosageDoseAndRate"),
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
    "maxDosePerAdministration" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxDosePerLifetime" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxDosePerPeriod" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
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
    "patientInstruction" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "sequence" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
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
    "timing" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Duration
#[rustfmt::skip]
pub static DURATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for EN-qualifier
#[rustfmt::skip]
pub static EN_QUALIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for EN-representation
#[rustfmt::skip]
pub static EN_REPRESENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for EN-use
#[rustfmt::skip]
pub static EN_USE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Element
#[rustfmt::skip]
pub static ELEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ElementDefinition
#[rustfmt::skip]
pub static ELEMENTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "base" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ElementDefinitionBase"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "binding" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ElementDefinitionBinding"),
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
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "constraint" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ElementDefinitionConstraint"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "contentReference" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "defaultValue[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "example" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ElementDefinitionExample"),
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
    "fixed[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
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
    "isModifier" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isModifierReason" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "isSummary" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "mapping" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ElementDefinitionMapping"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "max" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxLength" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "maxValue[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "meaningWhenMissing" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "min" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "minValue[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
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
    "mustSupport" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "orderMeaning" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "path" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "pattern[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "representation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "requirements" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "short" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sliceIsConstraining" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
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
    "slicing" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ElementDefinitionSlicing"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("ElementDefinitionType"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Encrypted
#[rustfmt::skip]
pub static ENCRYPTED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Expression
#[rustfmt::skip]
pub static EXPRESSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
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

/// Field metadata for Extension
#[rustfmt::skip]
pub static EXTENSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "url" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 1,
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

/// Field metadata for FamilyMemberHistory
#[rustfmt::skip]
pub static FAMILYMEMBERHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Gene
#[rustfmt::skip]
pub static GENE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for GenomicSourceClass
#[rustfmt::skip]
pub static GENOMICSOURCECLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Geolocation
#[rustfmt::skip]
pub static GEOLOCATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Human Language
#[rustfmt::skip]
pub static HUMAN_LANGUAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for HumanName
#[rustfmt::skip]
pub static HUMANNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "family" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "given" => FieldInfo {
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "prefix" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "suffix" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Identifier
#[rustfmt::skip]
pub static IDENTIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "assigner" => FieldInfo {
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
};

/// Field metadata for Instance
#[rustfmt::skip]
pub static INSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Interpretation
#[rustfmt::skip]
pub static INTERPRETATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Item
#[rustfmt::skip]
pub static ITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for MPPS
#[rustfmt::skip]
pub static MPPS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for MarketingStatus
#[rustfmt::skip]
pub static MARKETINGSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "country" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "dateRange" => FieldInfo {
        field_type: FhirFieldType::Complex("Period"),
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
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
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
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Meta
#[rustfmt::skip]
pub static META_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "lastUpdated" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "security" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "tag" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "versionId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Money
#[rustfmt::skip]
pub static MONEY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "currency" => FieldInfo {
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
    "value" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for MoneyQuantity
#[rustfmt::skip]
pub static MONEYQUANTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Narrative
#[rustfmt::skip]
pub static NARRATIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "div" => FieldInfo {
        field_type: FhirFieldType::Complex("xhtml"),
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
    "status" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Narrative Link
#[rustfmt::skip]
pub static NARRATIVE_LINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for NotificationEndpoint
#[rustfmt::skip]
pub static NOTIFICATIONENDPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for NumberOfInstances
#[rustfmt::skip]
pub static NUMBEROFINSTANCES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Ordinal Value
#[rustfmt::skip]
pub static ORDINAL_VALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Original Text
#[rustfmt::skip]
pub static ORIGINAL_TEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for PQ-translation
#[rustfmt::skip]
pub static PQ_TRANSLATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ParameterDefinition
#[rustfmt::skip]
pub static PARAMETERDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ParticipantObjectContainsStudy
#[rustfmt::skip]
pub static PARTICIPANTOBJECTCONTAINSSTUDY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Period
#[rustfmt::skip]
pub static PERIOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "end" => FieldInfo {
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
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for PhaseSet
#[rustfmt::skip]
pub static PHASESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Population
#[rustfmt::skip]
pub static POPULATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "age[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
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
    "physiologicalCondition" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "race" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ProdCharacteristic
#[rustfmt::skip]
pub static PRODCHARACTERISTIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "color" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "depth" => FieldInfo {
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
    "externalDiameter" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "height" => FieldInfo {
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
    "image" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "imprint" => FieldInfo {
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
    "nominalVolume" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
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
    "shape" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "weight" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "width" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for ProductShelfLife
#[rustfmt::skip]
pub static PRODUCTSHELFLIFE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "period" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "specialPrecautionsForStorage" => FieldInfo {
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
};

/// Field metadata for Quantity
#[rustfmt::skip]
pub static QUANTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "unit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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

/// Field metadata for Range
#[rustfmt::skip]
pub static RANGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
};

/// Field metadata for Ratio
#[rustfmt::skip]
pub static RATIO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "denominator" => FieldInfo {
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
    "numerator" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Reference
#[rustfmt::skip]
pub static REFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "reference" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for References
#[rustfmt::skip]
pub static REFERENCES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for RelatedArtifact
#[rustfmt::skip]
pub static RELATEDARTIFACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "citation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "document" => FieldInfo {
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
    "label" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "resource" => FieldInfo {
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
    "url" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Url),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Relative Date Criteria
#[rustfmt::skip]
pub static RELATIVE_DATE_CRITERIA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Rendered Value
#[rustfmt::skip]
pub static RENDERED_VALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for SC-coding
#[rustfmt::skip]
pub static SC_CODING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for SOPClass
#[rustfmt::skip]
pub static SOPCLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for SampledData
#[rustfmt::skip]
pub static SAMPLEDDATA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "data" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "dimensions" => FieldInfo {
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
    "lowerLimit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "origin" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "upperLimit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Signature
#[rustfmt::skip]
pub static SIGNATURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "data" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "onBehalfOf" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "sigFormat" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "targetFormat" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "type" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 1,
        max: None,
        is_choice_type: false,
    },
    "when" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Instant),
        min: 1,
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

/// Field metadata for SimpleQuantity
#[rustfmt::skip]
pub static SIMPLEQUANTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for SubstanceAmount
#[rustfmt::skip]
pub static SUBSTANCEAMOUNT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amountText" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "referenceRange" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("SubstanceAmountReferenceRange"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for TEL-address
#[rustfmt::skip]
pub static TEL_ADDRESS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Timezone Code
#[rustfmt::skip]
pub static TIMEZONE_CODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Timezone Offset
#[rustfmt::skip]
pub static TIMEZONE_OFFSET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Timing
#[rustfmt::skip]
pub static TIMING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "event" => FieldInfo {
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
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "repeat" => FieldInfo {
        field_type: FhirFieldType::BackboneElement("TimingRepeat"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for Transcriber
#[rustfmt::skip]
pub static TRANSCRIBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Translation
#[rustfmt::skip]
pub static TRANSLATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for TriggerDefinition
#[rustfmt::skip]
pub static TRIGGERDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "condition" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "data" => FieldInfo {
        field_type: FhirFieldType::Complex("DataRequirement"),
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
    "name" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "type" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for UsageContext
#[rustfmt::skip]
pub static USAGECONTEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
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
    "value[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for ValidityPeriod
#[rustfmt::skip]
pub static VALIDITYPERIOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Variable
#[rustfmt::skip]
pub static VARIABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Variant
#[rustfmt::skip]
pub static VARIANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for Witness
#[rustfmt::skip]
pub static WITNESS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for abatement
#[rustfmt::skip]
pub static ABATEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for acceptance
#[rustfmt::skip]
pub static ACCEPTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for activity-title
#[rustfmt::skip]
pub static ACTIVITY_TITLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for activityStatusDate
#[rustfmt::skip]
pub static ACTIVITYSTATUSDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for adaptiveFeedingDevice
#[rustfmt::skip]
pub static ADAPTIVEFEEDINGDEVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for addendumOf
#[rustfmt::skip]
pub static ADDENDUMOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for administration
#[rustfmt::skip]
pub static ADMINISTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for adoptionInfo
#[rustfmt::skip]
pub static ADOPTIONINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for allele-database
#[rustfmt::skip]
pub static ALLELE_DATABASE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for allowed-type
#[rustfmt::skip]
pub static ALLOWED_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for allowedUnits
#[rustfmt::skip]
pub static ALLOWEDUNITS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for alternate
#[rustfmt::skip]
pub static ALTERNATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ancestor
#[rustfmt::skip]
pub static ANCESTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for animal
#[rustfmt::skip]
pub static ANIMAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for animalSpecies
#[rustfmt::skip]
pub static ANIMALSPECIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for applicable-version
#[rustfmt::skip]
pub static APPLICABLE_VERSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for approachBodyStructure
#[rustfmt::skip]
pub static APPROACHBODYSTRUCTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for approvalDate
#[rustfmt::skip]
pub static APPROVALDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for area
#[rustfmt::skip]
pub static AREA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for assembly-order
#[rustfmt::skip]
pub static ASSEMBLY_ORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for assertedDate
#[rustfmt::skip]
pub static ASSERTEDDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for associatedEncounter
#[rustfmt::skip]
pub static ASSOCIATEDENCOUNTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for author
#[rustfmt::skip]
pub static AUTHOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for authoritativeSource
#[rustfmt::skip]
pub static AUTHORITATIVESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for authority
#[rustfmt::skip]
pub static AUTHORITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for baseType
#[rustfmt::skip]
pub static BASETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for basedOn
#[rustfmt::skip]
pub static BASEDON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for bestpractice
#[rustfmt::skip]
pub static BESTPRACTICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for bestpractice-explanation
#[rustfmt::skip]
pub static BESTPRACTICE_EXPLANATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for bidirectional
#[rustfmt::skip]
pub static BIDIRECTIONAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for bindingName
#[rustfmt::skip]
pub static BINDINGNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for birthPlace
#[rustfmt::skip]
pub static BIRTHPLACE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for birthTime
#[rustfmt::skip]
pub static BIRTHTIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for bodyPosition
#[rustfmt::skip]
pub static BODYPOSITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for boundary-geojson
#[rustfmt::skip]
pub static BOUNDARY_GEOJSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for cadavericDonor
#[rustfmt::skip]
pub static CADAVERICDONOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for calculatedValue
#[rustfmt::skip]
pub static CALCULATEDVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for candidateList
#[rustfmt::skip]
pub static CANDIDATELIST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for capabilities
#[rustfmt::skip]
pub static CAPABILITIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for caseSensitive
#[rustfmt::skip]
pub static CASESENSITIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for category
#[rustfmt::skip]
pub static CATEGORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for causedBy
#[rustfmt::skip]
pub static CAUSEDBY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for cdsHooksEndpoint
#[rustfmt::skip]
pub static CDSHOOKSENDPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for certainty
#[rustfmt::skip]
pub static CERTAINTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for changeBase
#[rustfmt::skip]
pub static CHANGEBASE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for choiceOrientation
#[rustfmt::skip]
pub static CHOICEORIENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for citation
#[rustfmt::skip]
pub static CITATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for citizenship
#[rustfmt::skip]
pub static CITIZENSHIP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for codegen-super
#[rustfmt::skip]
pub static CODEGEN_SUPER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for collectionPriority
#[rustfmt::skip]
pub static COLLECTIONPRIORITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for completionMode
#[rustfmt::skip]
pub static COMPLETIONMODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for concept-comments
#[rustfmt::skip]
pub static CONCEPT_COMMENTS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for concept-definition
#[rustfmt::skip]
pub static CONCEPT_DEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for conceptOrder
#[rustfmt::skip]
pub static CONCEPTORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for congregation
#[rustfmt::skip]
pub static CONGREGATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for constraint
#[rustfmt::skip]
pub static CONSTRAINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for country
#[rustfmt::skip]
pub static COUNTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for dayOfMonth
#[rustfmt::skip]
pub static DAYOFMONTH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for daysOfCycle
#[rustfmt::skip]
pub static DAYSOFCYCLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for delta
#[rustfmt::skip]
pub static DELTA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for dependencies
#[rustfmt::skip]
pub static DEPENDENCIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for deprecated
#[rustfmt::skip]
pub static DEPRECATED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for detail
#[rustfmt::skip]
pub static DETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for deviceCode
#[rustfmt::skip]
pub static DEVICECODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for directedBy
#[rustfmt::skip]
pub static DIRECTEDBY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for disability
#[rustfmt::skip]
pub static DISABILITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for display-hint
#[rustfmt::skip]
pub static DISPLAY_HINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for displayCategory
#[rustfmt::skip]
pub static DISPLAYCATEGORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for doNotPerform
#[rustfmt::skip]
pub static DONOTPERFORM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for dueTo
#[rustfmt::skip]
pub static DUETO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for effectiveDate
#[rustfmt::skip]
pub static EFFECTIVEDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for effectivePeriod
#[rustfmt::skip]
pub static EFFECTIVEPERIOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for encounterClass
#[rustfmt::skip]
pub static ENCOUNTERCLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for encounterType
#[rustfmt::skip]
pub static ENCOUNTERTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for entryFormat
#[rustfmt::skip]
pub static ENTRYFORMAT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for equivalence
#[rustfmt::skip]
pub static EQUIVALENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for eventHistory
#[rustfmt::skip]
pub static EVENTHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for exact
#[rustfmt::skip]
pub static EXACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for expand-group
#[rustfmt::skip]
pub static EXPAND_GROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for expand-rules
#[rustfmt::skip]
pub static EXPAND_RULES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for expansionSource
#[rustfmt::skip]
pub static EXPANSIONSOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for expectation
#[rustfmt::skip]
pub static EXPECTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for expirationDate
#[rustfmt::skip]
pub static EXPIRATIONDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for explicit-type-name
#[rustfmt::skip]
pub static EXPLICIT_TYPE_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for exposureDate
#[rustfmt::skip]
pub static EXPOSUREDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for exposureDescription
#[rustfmt::skip]
pub static EXPOSUREDESCRIPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for exposureDuration
#[rustfmt::skip]
pub static EXPOSUREDURATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for extends
#[rustfmt::skip]
pub static EXTENDS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for extensible
#[rustfmt::skip]
pub static EXTENSIBLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for fathers-family
#[rustfmt::skip]
pub static FATHERS_FAMILY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for fhir-type
#[rustfmt::skip]
pub static FHIR_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for fhirType
#[rustfmt::skip]
pub static FHIRTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for fmm
#[rustfmt::skip]
pub static FMM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for fmm-no-warnings
#[rustfmt::skip]
pub static FMM_NO_WARNINGS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for focusCode
#[rustfmt::skip]
pub static FOCUSCODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for fullUrl
#[rustfmt::skip]
pub static FULLURL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for gatewayDevice
#[rustfmt::skip]
pub static GATEWAYDEVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for genderIdentity
#[rustfmt::skip]
pub static GENDERIDENTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for glstring
#[rustfmt::skip]
pub static GLSTRING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for haploid
#[rustfmt::skip]
pub static HAPLOID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for hidden
#[rustfmt::skip]
pub static HIDDEN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for hierarchy
#[rustfmt::skip]
pub static HIERARCHY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for history
#[rustfmt::skip]
pub static HISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for http-response-header
#[rustfmt::skip]
pub static HTTP_RESPONSE_HEADER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for implantStatus
#[rustfmt::skip]
pub static IMPLANTSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for importance
#[rustfmt::skip]
pub static IMPORTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for incisionDateTime
#[rustfmt::skip]
pub static INCISIONDATETIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for inheritedExtensibleValueSet
#[rustfmt::skip]
pub static INHERITEDEXTENSIBLEVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for initialValue
#[rustfmt::skip]
pub static INITIALVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for initiatingLocation
#[rustfmt::skip]
pub static INITIATINGLOCATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for initiatingOrganization
#[rustfmt::skip]
pub static INITIATINGORGANIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for initiatingPerson
#[rustfmt::skip]
pub static INITIATINGPERSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for instantiatesCanonical
#[rustfmt::skip]
pub static INSTANTIATESCANONICAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for instantiatesUri
#[rustfmt::skip]
pub static INSTANTIATESURI_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for insurance
#[rustfmt::skip]
pub static INSURANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for interpreterRequired
#[rustfmt::skip]
pub static INTERPRETERREQUIRED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for isCommonBinding
#[rustfmt::skip]
pub static ISCOMMONBINDING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for isDryWeight
#[rustfmt::skip]
pub static ISDRYWEIGHT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for issue-source
#[rustfmt::skip]
pub static ISSUE_SOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for itemControl
#[rustfmt::skip]
pub static ITEMCONTROL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for json-type
#[rustfmt::skip]
pub static JSON_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for keyWord
#[rustfmt::skip]
pub static KEYWORD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for label
#[rustfmt::skip]
pub static LABEL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for lastReviewDate
#[rustfmt::skip]
pub static LASTREVIEWDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for local
#[rustfmt::skip]
pub static LOCAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for location-distance
#[rustfmt::skip]
pub static LOCATION_DISTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for locationPerformed
#[rustfmt::skip]
pub static LOCATIONPERFORMED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for management
#[rustfmt::skip]
pub static MANAGEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for map
#[rustfmt::skip]
pub static MAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for match-grade
#[rustfmt::skip]
pub static MATCH_GRADE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for maxDecimalPlaces
#[rustfmt::skip]
pub static MAXDECIMALPLACES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for maxOccurs
#[rustfmt::skip]
pub static MAXOCCURS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for maxSize
#[rustfmt::skip]
pub static MAXSIZE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for maxValue
#[rustfmt::skip]
pub static MAXVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for maxValueSet
#[rustfmt::skip]
pub static MAXVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for measureInfo
#[rustfmt::skip]
pub static MEASUREINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for messageheader-response-request
#[rustfmt::skip]
pub static MESSAGEHEADER_RESPONSE_REQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for method
#[rustfmt::skip]
pub static METHOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for mimeType
#[rustfmt::skip]
pub static MIMETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for minLength
#[rustfmt::skip]
pub static MINLENGTH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for minOccurs
#[rustfmt::skip]
pub static MINOCCURS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for minValue
#[rustfmt::skip]
pub static MINVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for minValueSet
#[rustfmt::skip]
pub static MINVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for modeOfArrival
#[rustfmt::skip]
pub static MODEOFARRIVAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for mothers-family
#[rustfmt::skip]
pub static MOTHERS_FAMILY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for mothersMaidenName
#[rustfmt::skip]
pub static MOTHERSMAIDENNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for namespace
#[rustfmt::skip]
pub static NAMESPACE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for nationality
#[rustfmt::skip]
pub static NATIONALITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for normative-version
#[rustfmt::skip]
pub static NORMATIVE_VERSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for nullFlavor
#[rustfmt::skip]
pub static NULLFLAVOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for oauth-uris
#[rustfmt::skip]
pub static OAUTH_URIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for objectClass
#[rustfmt::skip]
pub static OBJECTCLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for objectClassProperty
#[rustfmt::skip]
pub static OBJECTCLASSPROPERTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for occurredFollowing
#[rustfmt::skip]
pub static OCCURREDFOLLOWING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for optionExclusive
#[rustfmt::skip]
pub static OPTIONEXCLUSIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for optionPrefix
#[rustfmt::skip]
pub static OPTIONPREFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for otherConfidentiality
#[rustfmt::skip]
pub static OTHERCONFIDENTIALITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for otherName
#[rustfmt::skip]
pub static OTHERNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for outcome
#[rustfmt::skip]
pub static OUTCOME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for own-name
#[rustfmt::skip]
pub static OWN_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for own-prefix
#[rustfmt::skip]
pub static OWN_PREFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for parameterSource
#[rustfmt::skip]
pub static PARAMETERSOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for parent
#[rustfmt::skip]
pub static PARENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for partOf
#[rustfmt::skip]
pub static PARTOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for partner-name
#[rustfmt::skip]
pub static PARTNER_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for partner-prefix
#[rustfmt::skip]
pub static PARTNER_PREFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for patient-record
#[rustfmt::skip]
pub static PATIENT_RECORD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for patientInstruction
#[rustfmt::skip]
pub static PATIENTINSTRUCTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for performerFunction
#[rustfmt::skip]
pub static PERFORMERFUNCTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for performerOrder
#[rustfmt::skip]
pub static PERFORMERORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for permitted-value-conceptmap
#[rustfmt::skip]
pub static PERMITTED_VALUE_CONCEPTMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for permitted-value-valueset
#[rustfmt::skip]
pub static PERMITTED_VALUE_VALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for pertainsToGoal
#[rustfmt::skip]
pub static PERTAINSTOGOAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for precision
#[rustfmt::skip]
pub static PRECISION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for precondition
#[rustfmt::skip]
pub static PRECONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for preferenceType
#[rustfmt::skip]
pub static PREFERENCETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for preferred
#[rustfmt::skip]
pub static PREFERRED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for preferredContact
#[rustfmt::skip]
pub static PREFERREDCONTACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for primaryInd
#[rustfmt::skip]
pub static PRIMARYIND_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for priority
#[rustfmt::skip]
pub static PRIORITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for processingTime
#[rustfmt::skip]
pub static PROCESSINGTIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for proficiency
#[rustfmt::skip]
pub static PROFICIENCY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for profile
#[rustfmt::skip]
pub static PROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for profile-element
#[rustfmt::skip]
pub static PROFILE_ELEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for progressStatus
#[rustfmt::skip]
pub static PROGRESSSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for prohibited
#[rustfmt::skip]
pub static PROHIBITED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for qualityOfEvidence
#[rustfmt::skip]
pub static QUALITYOFEVIDENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for question
#[rustfmt::skip]
pub static QUESTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for questionnaireRequest
#[rustfmt::skip]
pub static QUESTIONNAIREREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for rdf-type
#[rustfmt::skip]
pub static RDF_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reagent
#[rustfmt::skip]
pub static REAGENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reason
#[rustfmt::skip]
pub static REASON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reasonCancelled
#[rustfmt::skip]
pub static REASONCANCELLED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reasonCode
#[rustfmt::skip]
pub static REASONCODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reasonReference
#[rustfmt::skip]
pub static REASONREFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reasonRefuted
#[rustfmt::skip]
pub static REASONREFUTED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reasonRejected
#[rustfmt::skip]
pub static REASONREJECTED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for receivingOrganization
#[rustfmt::skip]
pub static RECEIVINGORGANIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for receivingPerson
#[rustfmt::skip]
pub static RECEIVINGPERSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for recipientLanguage
#[rustfmt::skip]
pub static RECIPIENTLANGUAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for recipientType
#[rustfmt::skip]
pub static RECIPIENTTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for referenceFilter
#[rustfmt::skip]
pub static REFERENCEFILTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for referenceProfile
#[rustfmt::skip]
pub static REFERENCEPROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for referenceResource
#[rustfmt::skip]
pub static REFERENCERESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for regex
#[rustfmt::skip]
pub static REGEX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for related
#[rustfmt::skip]
pub static RELATED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for relationship
#[rustfmt::skip]
pub static RELATIONSHIP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for relativeDateTime
#[rustfmt::skip]
pub static RELATIVEDATETIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for relevantHistory
#[rustfmt::skip]
pub static RELEVANTHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for religion
#[rustfmt::skip]
pub static RELIGION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for replacedby
#[rustfmt::skip]
pub static REPLACEDBY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for replaces
#[rustfmt::skip]
pub static REPLACES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for resolutionAge
#[rustfmt::skip]
pub static RESOLUTIONAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for reviewer
#[rustfmt::skip]
pub static REVIEWER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for risk
#[rustfmt::skip]
pub static RISK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for ruledOut
#[rustfmt::skip]
pub static RULEDOUT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for rules-text
#[rustfmt::skip]
pub static RULES_TEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for sctdescid
#[rustfmt::skip]
pub static SCTDESCID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for search-parameter-combination
#[rustfmt::skip]
pub static SEARCH_PARAMETER_COMBINATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for secondaryFinding
#[rustfmt::skip]
pub static SECONDARYFINDING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for section-subject
#[rustfmt::skip]
pub static SECTION_SUBJECT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for security-category
#[rustfmt::skip]
pub static SECURITY_CATEGORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for selector
#[rustfmt::skip]
pub static SELECTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for sequelTo
#[rustfmt::skip]
pub static SEQUELTO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for sequenceNumber
#[rustfmt::skip]
pub static SEQUENCENUMBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for severity
#[rustfmt::skip]
pub static SEVERITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for sibling
#[rustfmt::skip]
pub static SIBLING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for signatureRequired
#[rustfmt::skip]
pub static SIGNATUREREQUIRED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for sliderStepValue
#[rustfmt::skip]
pub static SLIDERSTEPVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for sourceReference
#[rustfmt::skip]
pub static SOURCEREFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for special-status
#[rustfmt::skip]
pub static SPECIAL_STATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for specialHandling
#[rustfmt::skip]
pub static SPECIALHANDLING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for specimenCode
#[rustfmt::skip]
pub static SPECIMENCODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for standards-status
#[rustfmt::skip]
pub static STANDARDS_STATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for statusReason
#[rustfmt::skip]
pub static STATUSREASON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for steward
#[rustfmt::skip]
pub static STEWARD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for strengthOfRecommendation
#[rustfmt::skip]
pub static STRENGTHOFRECOMMENDATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for style
#[rustfmt::skip]
pub static STYLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for styleSensitive
#[rustfmt::skip]
pub static STYLESENSITIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for substanceExposureRisk
#[rustfmt::skip]
pub static SUBSTANCEEXPOSURERISK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for summary
#[rustfmt::skip]
pub static SUMMARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for summaryOf
#[rustfmt::skip]
pub static SUMMARYOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for supplement
#[rustfmt::skip]
pub static SUPPLEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for supportLink
#[rustfmt::skip]
pub static SUPPORTLINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for supported-system
#[rustfmt::skip]
pub static SUPPORTED_SYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for supportingInfo
#[rustfmt::skip]
pub static SUPPORTINGINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for system
#[rustfmt::skip]
pub static SYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for systemName
#[rustfmt::skip]
pub static SYSTEMNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for systemRef
#[rustfmt::skip]
pub static SYSTEMREF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for systemUserLanguage
#[rustfmt::skip]
pub static SYSTEMUSERLANGUAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for systemUserTaskContext
#[rustfmt::skip]
pub static SYSTEMUSERTASKCONTEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for systemUserType
#[rustfmt::skip]
pub static SYSTEMUSERTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for table-name
#[rustfmt::skip]
pub static TABLE_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for targetBodyStructure
#[rustfmt::skip]
pub static TARGETBODYSTRUCTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for template-status
#[rustfmt::skip]
pub static TEMPLATE_STATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for test
#[rustfmt::skip]
pub static TEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for timeOffset
#[rustfmt::skip]
pub static TIMEOFFSET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for toocostly
#[rustfmt::skip]
pub static TOOCOSTLY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for translatable
#[rustfmt::skip]
pub static TRANSLATABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for trusted-expansion
#[rustfmt::skip]
pub static TRUSTED_EXPANSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for type
#[rustfmt::skip]
pub static TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for uncertainty
#[rustfmt::skip]
pub static UNCERTAINTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for uncertaintyType
#[rustfmt::skip]
pub static UNCERTAINTYTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for unclosed
#[rustfmt::skip]
pub static UNCLOSED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for unit
#[rustfmt::skip]
pub static UNIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for unitOption
#[rustfmt::skip]
pub static UNITOPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for unitValueSet
#[rustfmt::skip]
pub static UNITVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for usage
#[rustfmt::skip]
pub static USAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for usageMode
#[rustfmt::skip]
pub static USAGEMODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for validDate
#[rustfmt::skip]
pub static VALIDDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for versionNumber
#[rustfmt::skip]
pub static VERSIONNUMBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for warning
#[rustfmt::skip]
pub static WARNING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for websocket
#[rustfmt::skip]
pub static WEBSOCKET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for wg
#[rustfmt::skip]
pub static WG_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for workflowStatus
#[rustfmt::skip]
pub static WORKFLOWSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for xml-no-order
#[rustfmt::skip]
pub static XML_NO_ORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};

/// Field metadata for xml-type
#[rustfmt::skip]
pub static XML_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
};
