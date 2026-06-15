//! Field metadata for FHIR primitive types

use super::*;
use phf::{phf_map, Map};

/// Field metadata for base64Binary
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
