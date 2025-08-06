//! Macros for generating primitive FHIR fields with extension companions
//!
//! These macros generate both the primitive type field and its corresponding
//! extension field for FHIR primitives.

/// Generate a string field with optional extension companion
///
/// # Usage
/// ```ignore
/// // Within a struct definition:
/// primitive_string!("description", true);  // Optional field
/// primitive_string!("name", false);        // Required field
/// ```
///
/// # Generated Code
/// For `primitive_string!("description", true)`:
/// ```ignore
/// /// Field: description
/// pub description: Option<StringType>,
/// /// Extension element for the 'description' primitive field. Contains metadata and extensions.
/// pub _description: Option<Element>,
/// ```
#[macro_export]
macro_rules! primitive_string {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::string::StringType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::string::StringType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a boolean field with optional extension companion
#[macro_export]
macro_rules! primitive_boolean {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::boolean::BooleanType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::boolean::BooleanType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate an integer field with optional extension companion
#[macro_export]
macro_rules! primitive_integer {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::integer::IntegerType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::integer::IntegerType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a decimal field with optional extension companion
#[macro_export]
macro_rules! primitive_decimal {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::decimal::DecimalType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::decimal::DecimalType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a dateTime field with optional extension companion
#[macro_export]
macro_rules! primitive_datetime {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::datetime::DateTimeType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::datetime::DateTimeType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a date field with optional extension companion
#[macro_export]
macro_rules! primitive_date {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::date::DateType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::date::DateType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a time field with optional extension companion
#[macro_export]
macro_rules! primitive_time {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::time::TimeType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::time::TimeType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a uri field with optional extension companion
#[macro_export]
macro_rules! primitive_uri {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::uri::UriType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::uri::UriType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a canonical field with optional extension companion
#[macro_export]
macro_rules! primitive_canonical {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::canonical::CanonicalType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::canonical::CanonicalType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a base64Binary field with optional extension companion
#[macro_export]
macro_rules! primitive_base64binary {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::base64binary::Base64BinaryType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::base64binary::Base64BinaryType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate an instant field with optional extension companion
#[macro_export]
macro_rules! primitive_instant {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::instant::InstantType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::instant::InstantType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a positiveInt field with optional extension companion
#[macro_export]
macro_rules! primitive_positiveint {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::positiveint::PositiveIntType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::positiveint::PositiveIntType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate an unsignedInt field with optional extension companion
#[macro_export]
macro_rules! primitive_unsignedint {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::unsignedint::UnsignedIntType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::unsignedint::UnsignedIntType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate an id field with optional extension companion
#[macro_export]
macro_rules! primitive_id {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::id::IdType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::id::IdType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate an oid field with optional extension companion
#[macro_export]
macro_rules! primitive_oid {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::oid::OidType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::oid::OidType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a uuid field with optional extension companion
#[macro_export]
macro_rules! primitive_uuid {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::uuid::UuidType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::uuid::UuidType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a code field with optional extension companion
#[macro_export]
macro_rules! primitive_code {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::code::CodeType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::code::CodeType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate a markdown field with optional extension companion
#[macro_export]
macro_rules! primitive_markdown {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::markdown::MarkdownType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::markdown::MarkdownType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}

/// Generate an url field with optional extension companion
#[macro_export]
macro_rules! primitive_url {
    ($field_name:literal, true) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: Option<$crate::primitives::url::UrlType>,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
    ($field_name:literal, false) => {
        paste::paste! {
            #[doc = concat!("Field: ", $field_name)]
            pub [<$field_name>]: $crate::primitives::url::UrlType,
            #[doc = concat!("Extension element for the '", $field_name, "' primitive field. Contains metadata and extensions.")]
            pub [<_ $field_name>]: Option<$crate::datatypes::element::Element>,
        }
    };
}
