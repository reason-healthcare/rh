//! Field metadata for FHIR primitive types

use super::*;
use phf::{phf_map, Map};

/// Field metadata for base64Binary
#[rustfmt::skip]
pub static BASE64BINARY_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for boolean
#[rustfmt::skip]
pub static BOOLEAN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.Boolean"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for canonical
#[rustfmt::skip]
pub static CANONICAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for code
#[rustfmt::skip]
pub static CODE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for date
#[rustfmt::skip]
pub static DATE_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.Date"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for dateTime
#[rustfmt::skip]
pub static DATETIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.DateTime"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for decimal
#[rustfmt::skip]
pub static DECIMAL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.Decimal"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for id
#[rustfmt::skip]
pub static ID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for instant
#[rustfmt::skip]
pub static INSTANT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.DateTime"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for integer
#[rustfmt::skip]
pub static INTEGER_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.Integer"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for integer64
#[rustfmt::skip]
pub static INTEGER64_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.Integer"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for markdown
#[rustfmt::skip]
pub static MARKDOWN_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for oid
#[rustfmt::skip]
pub static OID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for positiveInt
#[rustfmt::skip]
pub static POSITIVEINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for string
#[rustfmt::skip]
pub static STRING_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for time
#[rustfmt::skip]
pub static TIME_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.Time"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for unsignedInt
#[rustfmt::skip]
pub static UNSIGNEDINT_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for uri
#[rustfmt::skip]
pub static URI_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for url
#[rustfmt::skip]
pub static URL_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for uuid
#[rustfmt::skip]
pub static UUID_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
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
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
};

/// Field metadata for xhtml
#[rustfmt::skip]
pub static XHTML_FIELDS: Map<&'static str, FieldInfo> = phf_map! {
    "extension" => FieldInfo {
        field_type: FhirFieldType::Complex("Extension"),
        min: 0,
        max: Some(0),
        is_choice_type: false,
    },
    "id" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 0,
        max: Some(1),
        is_choice_type: false,
    },
    "value" => FieldInfo {
        field_type: FhirFieldType::Complex("http://hl7.org/fhirpath/System.String"),
        min: 1,
        max: Some(1),
        is_choice_type: false,
    },
};
