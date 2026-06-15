//! Field metadata for FHIR datatypes

use super::*;
use phf::{phf_map, Map};

/// Field metadata for bestpractice-explanation
pub static BESTPRACTICE_EXPLANATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for match-grade
pub static MATCH_GRADE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-careOf
pub static ADXP_CAREOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for profile-element
pub static PROFILE_ELEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for recipientType
pub static RECIPIENTTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for initialValue
pub static INITIALVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for partner-prefix
pub static PARTNER_PREFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for EN-use
pub static EN_USE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for instantiatesUri
pub static INSTANTIATESURI_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for otherName
pub static OTHERNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for question
pub static QUESTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Duration
pub static DURATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
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

/// Field metadata for rules-text
pub static RULES_TEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for dayOfMonth
pub static DAYOFMONTH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for expirationDate
pub static EXPIRATIONDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for applicable-version
pub static APPLICABLE_VERSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for specialHandling
pub static SPECIALHANDLING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for activityStatusDate
pub static ACTIVITYSTATUSDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ContactDetail
pub static CONTACTDETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
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
};

/// Field metadata for outcome
pub static OUTCOME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for addendumOf
pub static ADDENDUMOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for preferredContact
pub static PREFERREDCONTACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for styleSensitive
pub static STYLESENSITIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reasonCancelled
pub static REASONCANCELLED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Item
pub static ITEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Population
pub static POPULATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "race" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "physiologicalCondition" => FieldInfo {
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
    "gender" => FieldInfo {
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
    "age[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Range"),
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
};

/// Field metadata for primaryInd
pub static PRIMARYIND_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for delta
pub static DELTA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for UsageContext
pub static USAGECONTEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
};

/// Field metadata for trusted-expansion
pub static TRUSTED_EXPANSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for basedOn
pub static BASEDON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-delimiter
pub static ADXP_DELIMITER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for PhaseSet
pub static PHASESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for style
pub static STYLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for recipientLanguage
pub static RECIPIENTLANGUAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for referenceResource
pub static REFERENCERESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for objectClass
pub static OBJECTCLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for targetBodyStructure
pub static TARGETBODYSTRUCTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for baseType
pub static BASETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Quantity
pub static QUANTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "comparator" => FieldInfo {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
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

/// Field metadata for parent
pub static PARENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for cdsHooksEndpoint
pub static CDSHOOKSENDPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for standards-status
pub static STANDARDS_STATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for precondition
pub static PRECONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for performerFunction
pub static PERFORMERFUNCTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for SC-coding
pub static SC_CODING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for mothers-family
pub static MOTHERS_FAMILY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for extension
pub static EXTENSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for validDate
pub static VALIDDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for MoneyQuantity
pub static MONEYQUANTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for certainty
pub static CERTAINTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for completionMode
pub static COMPLETIONMODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for genderIdentity
pub static GENDERIDENTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for doNotPerform
pub static DONOTPERFORM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for specimenCode
pub static SPECIMENCODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for animalSpecies
pub static ANIMALSPECIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for profile
pub static PROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Address
pub static ADDRESS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "postalCode" => FieldInfo {
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
    "text" => FieldInfo {
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
    "city" => FieldInfo {
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
    "country" => FieldInfo {
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "line" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "district" => FieldInfo {
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
};

/// Field metadata for management
pub static MANAGEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for warning
pub static WARNING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for deviceCode
pub static DEVICECODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for related
pub static RELATED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-deliveryInstallationType
pub static ADXP_DELIVERYINSTALLATIONTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for search-parameter-combination
pub static SEARCH_PARAMETER_COMBINATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for bindingName
pub static BINDINGNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ParameterDefinition
pub static PARAMETERDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "max" => FieldInfo {
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
    "name" => FieldInfo {
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
    "min" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "use" => FieldInfo {
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
};

/// Field metadata for reagent
pub static REAGENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for patientInstruction
pub static PATIENTINSTRUCTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for calculatedValue
pub static CALCULATEDVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for haploid
pub static HAPLOID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for TriggerDefinition
pub static TRIGGERDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "data" => FieldInfo {
        field_type: FhirFieldType::Complex("DataRequirement"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::Complex("Expression"),
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

/// Field metadata for DNARegionName
pub static DNAREGIONNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for allele-database
pub static ALLELE_DATABASE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-censusTract
pub static ADXP_CENSUSTRACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for birthPlace
pub static BIRTHPLACE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for priority
pub static PRIORITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for expression
pub static EXPRESSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Display Name
pub static DISPLAY_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for resolutionAge
pub static RESOLUTIONAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for type
pub static TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Original Text
pub static ORIGINAL_TEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for preferenceType
pub static PREFERENCETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for fathers-family
pub static FATHERS_FAMILY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for extensible
pub static EXTENSIBLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for authoritativeSource
pub static AUTHORITATIVESOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Anonymized
pub static ANONYMIZED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for permitted-value-conceptmap
pub static PERMITTED_VALUE_CONCEPTMAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for referenceProfile
pub static REFERENCEPROFILE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for otherConfidentiality
pub static OTHERCONFIDENTIALITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reasonRejected
pub static REASONREJECTED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for json-type
pub static JSON_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for nullFlavor
pub static NULLFLAVOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for SubstanceAmount
pub static SUBSTANCEAMOUNT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "amountText" => FieldInfo {
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
    "referenceRange" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "amount[x]" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
};

/// Field metadata for performerOrder
pub static PERFORMERORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for xml-no-order
pub static XML_NO_ORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for daysOfCycle
pub static DAYSOFCYCLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ContactPoint
pub static CONTACTPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "rank" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
};

/// Field metadata for ADXP-postBox
pub static ADXP_POSTBOX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for dependencies
pub static DEPENDENCIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for incisionDateTime
pub static INCISIONDATETIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for adoptionInfo
pub static ADOPTIONINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for mimeType
pub static MIMETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ValidityPeriod
pub static VALIDITYPERIOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ProdCharacteristic
pub static PRODCHARACTERISTIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "image" => FieldInfo {
        field_type: FhirFieldType::Complex("Attachment"),
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
    "width" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "nominalVolume" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
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
    "weight" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "color" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "shape" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "imprint" => FieldInfo {
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
};

/// Field metadata for concept-comments
pub static CONCEPT_COMMENTS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for exposureDuration
pub static EXPOSUREDURATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for abatement
pub static ABATEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ElementDefinition
pub static ELEMENTDEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "isModifier" => FieldInfo {
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
    "code" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "definition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "alias" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "example" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 1,
        max: Some(1),
        is_choice_type: true,
    },
    "condition" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
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
    "pattern[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "mustSupport" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "binding" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "maxValue[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "maxLength" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Integer),
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
    "requirements" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
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
    "min" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
    "representation" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "meaningWhenMissing" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "base" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
    "constraint" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
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
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
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
    "comment" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Markdown),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "fixed[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "short" => FieldInfo {
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
    "minValue[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Date),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "mapping" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for encounterClass
pub static ENCOUNTERCLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Transcriber
pub static TRANSCRIBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ProductShelfLife
pub static PRODUCTSHELFLIFE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
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
    "identifier" => FieldInfo {
        field_type: FhirFieldType::Complex("Identifier"),
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

/// Field metadata for workflowStatus
pub static WORKFLOWSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for EN-qualifier
pub static EN_QUALIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for minOccurs
pub static MINOCCURS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for disability
pub static DISABILITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for http-response-header
pub static HTTP_RESPONSE_HEADER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for fhirType
pub static FHIRTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for issue-source
pub static ISSUE_SOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for MPPS
pub static MPPS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for signatureRequired
pub static SIGNATUREREQUIRED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Interpretation
pub static INTERPRETATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for isCommonBinding
pub static ISCOMMONBINDING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for own-prefix
pub static OWN_PREFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for religion
pub static RELIGION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for unitOption
pub static UNITOPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Distance
pub static DISTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
};

/// Field metadata for initiatingPerson
pub static INITIATINGPERSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for fmm
pub static FMM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for systemUserTaskContext
pub static SYSTEMUSERTASKCONTEXT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for concept-definition
pub static CONCEPT_DEFINITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for section-subject
pub static SECTION_SUBJECT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Meta
pub static META_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "security" => FieldInfo {
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
    "profile" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "tag" => FieldInfo {
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
    "versionId" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Id),
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
};

/// Field metadata for maxSize
pub static MAXSIZE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for minValueSet
pub static MINVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for identifier
pub static IDENTIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-unitType
pub static ADXP_UNITTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for receivingOrganization
pub static RECEIVINGORGANIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for HumanName
pub static HUMANNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "use" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "suffix" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "prefix" => FieldInfo {
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for category
pub static CATEGORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for versionNumber
pub static VERSIONNUMBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for NumberOfInstances
pub static NUMBEROFINSTANCES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for maxDecimalPlaces
pub static MAXDECIMALPLACES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for replacedby
pub static REPLACEDBY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for congregation
pub static CONGREGATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for pertainsToGoal
pub static PERTAINSTOGOAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Relative Date Criteria
pub static RELATIVE_DATE_CRITERIA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reasonReference
pub static REASONREFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Contributor
pub static CONTRIBUTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
};

/// Field metadata for optionExclusive
pub static OPTIONEXCLUSIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for allowed-type
pub static ALLOWED_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Accession
pub static ACCESSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for activity-title
pub static ACTIVITY_TITLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Data Absent Reason
pub static DATA_ABSENT_REASON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for sctdescid
pub static SCTDESCID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Timezone Code
pub static TIMEZONE_CODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for effectiveDate
pub static EFFECTIVEDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for acceptance
pub static ACCEPTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reasonRefuted
pub static REASONREFUTED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for prohibited
pub static PROHIBITED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for optionPrefix
pub static OPTIONPREFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for FamilyMemberHistory
pub static FAMILYMEMBERHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for country
pub static COUNTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for xml-type
pub static XML_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for qualityOfEvidence
pub static QUALITYOFEVIDENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for local
pub static LOCAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for interpreterRequired
pub static INTERPRETERREQUIRED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for allowedUnits
pub static ALLOWEDUNITS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for administration
pub static ADMINISTRATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for referenceFilter
pub static REFERENCEFILTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-precinct
pub static ADXP_PRECINCT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for GenomicSourceClass
pub static GENOMICSOURCECLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for display-hint
pub static DISPLAY_HINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for severity
pub static SEVERITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for wg
pub static WG_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for table-name
pub static TABLE_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for expand-rules
pub static EXPAND_RULES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for messageheader-response-request
pub static MESSAGEHEADER_RESPONSE_REQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for unclosed
pub static UNCLOSED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for BodyStructure Reference
pub static BODYSTRUCTURE_REFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-buildingNumberSuffix
pub static ADXP_BUILDINGNUMBERSUFFIX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for AssessedCondition
pub static ASSESSEDCONDITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for capabilities
pub static CAPABILITIES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for CodeableConcept
pub static CODEABLECONCEPT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "coding" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
        min: 0,
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for maxValueSet
pub static MAXVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for map
pub static MAP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for constraint
pub static CONSTRAINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-streetNameBase
pub static ADXP_STREETNAMEBASE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for patient-record
pub static PATIENT_RECORD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for deprecated
pub static DEPRECATED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reviewer
pub static REVIEWER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for minValue
pub static MINVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for modeOfArrival
pub static MODEOFARRIVAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for websocket
pub static WEBSOCKET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for fullUrl
pub static FULLURL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for rdf-type
pub static RDF_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for TEL-address
pub static TEL_ADDRESS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Money
pub static MONEY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "value" => FieldInfo {
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
    "currency" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for precision
pub static PRECISION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for bodyPosition
pub static BODYPOSITION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for usage
pub static USAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for regex
pub static REGEX_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for birthTime
pub static BIRTHTIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for bidirectional
pub static BIDIRECTIONAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for occurredFollowing
pub static OCCURREDFOLLOWING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for processingTime
pub static PROCESSINGTIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for uncertainty
pub static UNCERTAINTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for NotificationEndpoint
pub static NOTIFICATIONENDPOINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for systemUserType
pub static SYSTEMUSERTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-additionalLocator
pub static ADXP_ADDITIONALLOCATOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for partner-name
pub static PARTNER_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for assembly-order
pub static ASSEMBLY_ORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Variant
pub static VARIANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-houseNumberNumeric
pub static ADXP_HOUSENUMBERNUMERIC_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for isDryWeight
pub static ISDRYWEIGHT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for initiatingLocation
pub static INITIATINGLOCATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for sourceReference
pub static SOURCEREFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for directedBy
pub static DIRECTEDBY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for collectionPriority
pub static COLLECTIONPRIORITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for statusReason
pub static STATUSREASON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for eventHistory
pub static EVENTHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for AD-use
pub static AD_USE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for implantStatus
pub static IMPLANTSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-streetNameType
pub static ADXP_STREETNAMETYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for inheritedExtensibleValueSet
pub static INHERITEDEXTENSIBLEVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reference
pub static REFERENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for usageMode
pub static USAGEMODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ParticipantObjectContainsStudy
pub static PARTICIPANTOBJECTCONTAINSSTUDY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for translatable
pub static TRANSLATABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for importance
pub static IMPORTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for CopyNumberEvent
pub static COPYNUMBEREVENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Annotation
pub static ANNOTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "text" => FieldInfo {
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
    "author[x]" => FieldInfo {
        field_type: FhirFieldType::Reference,
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "time" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for label
pub static LABEL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-unitID
pub static ADXP_UNITID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for entryFormat
pub static ENTRYFORMAT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Dosage
pub static DOSAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "patientInstruction" => FieldInfo {
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
    "site" => FieldInfo {
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
    "timing" => FieldInfo {
        field_type: FhirFieldType::Complex("Timing"),
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
    "doseAndRate" => FieldInfo {
        field_type: FhirFieldType::Complex("Ratio"),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "sequence" => FieldInfo {
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
    "method" => FieldInfo {
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "asNeeded[x]" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Boolean),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "additionalInstruction" => FieldInfo {
        field_type: FhirFieldType::Complex("CodeableConcept"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for exposureDescription
pub static EXPOSUREDESCRIPTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for supportLink
pub static SUPPORTLINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for relativeDateTime
pub static RELATIVEDATETIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for sibling
pub static SIBLING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Timezone Offset
pub static TIMEZONE_OFFSET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for permitted-value-valueset
pub static PERMITTED_VALUE_VALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for author
pub static AUTHOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for focusCode
pub static FOCUSCODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for SampledData
pub static SAMPLEDDATA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "upperLimit" => FieldInfo {
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "factor" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "period" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Decimal),
        min: 1,
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
    "data" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
};

/// Field metadata for codegen-super
pub static CODEGEN_SUPER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for unitValueSet
pub static UNITVALUESET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for xhtml
pub static XHTML_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for expectation
pub static EXPECTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Element
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

/// Field metadata for causedBy
pub static CAUSEDBY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for expansionSource
pub static EXPANSIONSOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-deliveryInstallationQualifier
pub static ADXP_DELIVERYINSTALLATIONQUALIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Analysis
pub static ANALYSIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for SimpleQuantity
pub static SIMPLEQUANTITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for parameterSource
pub static PARAMETERSOURCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Narrative Link
pub static NARRATIVE_LINK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for DataElement constraint on ElementDefinition data type
pub static DATAELEMENT_CONSTRAINT_ON_ELEMENTDEFINITION_DATA_TYPE_FIELDS: Map<
    &'static str,
    FieldInfo,
> = phf_map! {};

/// Field metadata for choiceOrientation
pub static CHOICEORIENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for steward
pub static STEWARD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for hierarchy
pub static HIERARCHY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for method
pub static METHOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for extends
pub static EXTENDS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for history
pub static HISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for insurance
pub static INSURANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for relatedArtifact
pub static RELATEDARTIFACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for maxValue
pub static MAXVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for relationship
pub static RELATIONSHIP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for hidden
pub static HIDDEN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Age
pub static AGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "comparator" => FieldInfo {
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
    "unit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
};

/// Field metadata for expand-group
pub static EXPAND_GROUP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for fhir-type
pub static FHIR_TYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for toocostly
pub static TOOCOSTLY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for PQ-translation
pub static PQ_TRANSLATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for timeOffset
pub static TIMEOFFSET_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Rendered Value
pub static RENDERED_VALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for candidateList
pub static CANDIDATELIST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for signature
pub static SIGNATURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for relevantHistory
pub static RELEVANTHISTORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for boundary-geojson
pub static BOUNDARY_GEOJSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for security-category
pub static SECURITY_CATEGORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for uncertaintyType
pub static UNCERTAINTYTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Count
pub static COUNT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "code" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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
    "comparator" => FieldInfo {
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for MarketingStatus
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
    "modifierExtension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
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
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for maxOccurs
pub static MAXOCCURS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-deliveryInstallationArea
pub static ADXP_DELIVERYINSTALLATIONAREA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for citizenship
pub static CITIZENSHIP_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for oauth-uris
pub static OAUTH_URIS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Allele
pub static ALLELE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for caseSensitive
pub static CASESENSITIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for bestpractice
pub static BESTPRACTICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for location-distance
pub static LOCATION_DISTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Ratio
pub static RATIO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
    "denominator" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for explicit-type-name
pub static EXPLICIT_TYPE_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for References
pub static REFERENCES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for preferred
pub static PREFERRED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for displayCategory
pub static DISPLAYCATEGORY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for itemControl
pub static ITEMCONTROL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Human Language
pub static HUMAN_LANGUAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for detail
pub static DETAIL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for supported-system
pub static SUPPORTED_SYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Translation
pub static TRANSLATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for keyWord
pub static KEYWORD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for conceptOrder
pub static CONCEPTORDER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for sequelTo
pub static SEQUELTO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for sliderStepValue
pub static SLIDERSTEPVALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reasonCode
pub static REASONCODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Design Note
pub static DESIGN_NOTE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Ordinal Value
pub static ORDINAL_VALUE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for approachBodyStructure
pub static APPROACHBODYSTRUCTURE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for partOf
pub static PARTOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for associatedEncounter
pub static ASSOCIATEDENCOUNTER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-streetAddressLine
pub static ADXP_STREETADDRESSLINE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Gene
pub static GENE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for supportingInfo
pub static SUPPORTINGINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for equivalence
pub static EQUIVALENCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for EN-representation
pub static EN_REPRESENTATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for summary
pub static SUMMARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for lastReviewDate
pub static LASTREVIEWDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for selector
pub static SELECTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Narrative
pub static NARRATIVE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
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
    "div" => FieldInfo {
        field_type: FhirFieldType::Complex("xhtml"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for proficiency
pub static PROFICIENCY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for progressStatus
pub static PROGRESSSTATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for animal
pub static ANIMAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Ancestry
pub static ANCESTRY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-streetName
pub static ADXP_STREETNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for exposureDate
pub static EXPOSUREDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for exact
pub static EXACT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for authority
pub static AUTHORITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for strengthOfRecommendation
pub static STRENGTHOFRECOMMENDATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for template-status
pub static TEMPLATE_STATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Timing
pub static TIMING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "event" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "repeat" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
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

/// Field metadata for Period
pub static PERIOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "end" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "start" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for citation
pub static CITATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for gatewayDevice
pub static GATEWAYDEVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-deliveryMode
pub static ADXP_DELIVERYMODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for encounterType
pub static ENCOUNTERTYPE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for AminoAcidChange
pub static AMINOACIDCHANGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-direction
pub static ADXP_DIRECTION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for approvalDate
pub static APPROVALDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for secondaryFinding
pub static SECONDARYFINDING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for effectivePeriod
pub static EFFECTIVEPERIOD_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for mothersMaidenName
pub static MOTHERSMAIDENNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for unit
pub static UNIT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for test
pub static TEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for initiatingOrganization
pub static INITIATINGORGANIZATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for nationality
pub static NATIONALITY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for minLength
pub static MINLENGTH_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Range
pub static RANGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "high" => FieldInfo {
        field_type: FhirFieldType::Complex("Quantity"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Instance
pub static INSTANCE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for systemUserLanguage
pub static SYSTEMUSERLANGUAGE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for system
pub static SYSTEM_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for DataRequirement
pub static DATAREQUIREMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "codeFilter" => FieldInfo {
        field_type: FhirFieldType::Complex("Coding"),
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "mustSupport" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
        min: 0,
        max: None,
        is_choice_type: false,
    },
    "dateFilter" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::DateTime),
        min: 0,
        max: Some(1),
        is_choice_type: true,
    },
    "limit" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt),
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
    "sort" => FieldInfo {
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
};

/// Field metadata for area
pub static AREA_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ancestor
pub static ANCESTOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for systemName
pub static SYSTEMNAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Encrypted
pub static ENCRYPTED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for measureInfo
pub static MEASUREINFO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for fmm-no-warnings
pub static FMM_NO_WARNINGS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-deliveryModeIdentifier
pub static ADXP_DELIVERYMODEIDENTIFIER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for objectClassProperty
pub static OBJECTCLASSPROPERTY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for receivingPerson
pub static RECEIVINGPERSON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Coding
pub static CODING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "display" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
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
    "system" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Uri),
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
    "version" => FieldInfo {
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
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: None,
        is_choice_type: false,
    },
};

/// Field metadata for Variable
pub static VARIABLE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for systemRef
pub static SYSTEMREF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for substanceExposureRisk
pub static SUBSTANCEEXPOSURERISK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for risk
pub static RISK_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for alternate
pub static ALTERNATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for BackboneElement
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

/// Field metadata for adaptiveFeedingDevice
pub static ADAPTIVEFEEDINGDEVICE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for reason
pub static REASON_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Attachment
pub static ATTACHMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "size" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt),
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
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
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
    "language" => FieldInfo {
        field_type: FhirFieldType::Primitive(FhirPrimitiveType::Code),
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

/// Field metadata for Witness
pub static WITNESS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for replaces
pub static REPLACES_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for summaryOf
pub static SUMMARYOF_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for questionnaireRequest
pub static QUESTIONNAIREREQUEST_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for dueTo
pub static DUETO_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for changeBase
pub static CHANGEBASE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for assertedDate
pub static ASSERTEDDATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for SOPClass
pub static SOPCLASS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for glstring
pub static GLSTRING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for namespace
pub static NAMESPACE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for Geolocation
pub static GEOLOCATION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-deliveryAddressLine
pub static ADXP_DELIVERYADDRESSLINE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for locationPerformed
pub static LOCATIONPERFORMED_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for own-name
pub static OWN_NAME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for cadavericDonor
pub static CADAVERICDONOR_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for instantiatesCanonical
pub static INSTANTIATESCANONICAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ADXP-houseNumber
pub static ADXP_HOUSENUMBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for sequenceNumber
pub static SEQUENCENUMBER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for special-status
pub static SPECIAL_STATUS_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for ruledOut
pub static RULEDOUT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for normative-version
pub static NORMATIVE_VERSION_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};

/// Field metadata for supplement
pub static SUPPLEMENT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {};
