//! Internal DataType system for CQL type checking and resolution.
//!
//! This module provides an internal representation of CQL types used during
//! semantic analysis and type checking. While ELM `TypeSpecifier` is used for
//! serialization, `DataType` is the internal workhorse for type operations.
//!
//! # Type Hierarchy
//!
//! CQL has a rich type system with:
//! - **System types**: Primitives like Boolean, Integer, String, DateTime, etc.
//! - **Model types**: FHIR resources like Patient, Observation, etc.
//! - **Collection types**: List<T>, Interval<T>
//! - **Structural types**: Tuple with named elements, Choice of multiple types
//!
//! # Type Compatibility
//!
//! Types have compatibility rules for:
//! - Assignment (can a value of type A be assigned to type B?)
//! - Implicit conversion (can type A be automatically converted to type B?)
//! - Explicit conversion (can type A be explicitly cast to type B?)
//!
//! # Example
//!
//! ```
//! use rh_cql::datatype::{DataType, SystemType};
//!
//! // Create system types
//! let int_type = DataType::system(SystemType::Integer);
//! let decimal_type = DataType::system(SystemType::Decimal);
//!
//! // Integer can be implicitly converted to Decimal
//! assert!(int_type.can_convert_to(&decimal_type));
//!
//! // Create a list type
//! let list_of_int = DataType::list(int_type.clone());
//! assert!(list_of_int.is_list());
//! ```

use std::fmt;

/// System (primitive) types defined by CQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemType {
    /// The root type - all types are subtypes of Any.
    Any,
    /// Boolean true/false.
    Boolean,
    /// 32-bit signed integer.
    Integer,
    /// 64-bit signed integer.
    Long,
    /// Decimal number (arbitrary precision).
    Decimal,
    /// Unicode string.
    String,
    /// Date with year, month, day.
    Date,
    /// Date and time with timezone.
    DateTime,
    /// Time of day.
    Time,
    /// Quantity with value and unit.
    Quantity,
    /// Ratio of two quantities.
    Ratio,
    /// Code from a code system.
    Code,
    /// Concept - a set of codes.
    Concept,
    /// Vocabulary binding (code system or value set reference).
    Vocabulary,
}

impl SystemType {
    /// Get the fully qualified name for this system type.
    pub fn qualified_name(&self) -> &'static str {
        match self {
            SystemType::Any => "{urn:hl7-org:elm-types:r1}Any",
            SystemType::Boolean => "{urn:hl7-org:elm-types:r1}Boolean",
            SystemType::Integer => "{urn:hl7-org:elm-types:r1}Integer",
            SystemType::Long => "{urn:hl7-org:elm-types:r1}Long",
            SystemType::Decimal => "{urn:hl7-org:elm-types:r1}Decimal",
            SystemType::String => "{urn:hl7-org:elm-types:r1}String",
            SystemType::Date => "{urn:hl7-org:elm-types:r1}Date",
            SystemType::DateTime => "{urn:hl7-org:elm-types:r1}DateTime",
            SystemType::Time => "{urn:hl7-org:elm-types:r1}Time",
            SystemType::Quantity => "{urn:hl7-org:elm-types:r1}Quantity",
            SystemType::Ratio => "{urn:hl7-org:elm-types:r1}Ratio",
            SystemType::Code => "{urn:hl7-org:elm-types:r1}Code",
            SystemType::Concept => "{urn:hl7-org:elm-types:r1}Concept",
            SystemType::Vocabulary => "{urn:hl7-org:elm-types:r1}Vocabulary",
        }
    }

    /// Get the simple name for this system type.
    pub fn simple_name(&self) -> &'static str {
        match self {
            SystemType::Any => "Any",
            SystemType::Boolean => "Boolean",
            SystemType::Integer => "Integer",
            SystemType::Long => "Long",
            SystemType::Decimal => "Decimal",
            SystemType::String => "String",
            SystemType::Date => "Date",
            SystemType::DateTime => "DateTime",
            SystemType::Time => "Time",
            SystemType::Quantity => "Quantity",
            SystemType::Ratio => "Ratio",
            SystemType::Code => "Code",
            SystemType::Concept => "Concept",
            SystemType::Vocabulary => "Vocabulary",
        }
    }

    /// Parse a system type from a qualified or simple name.
    pub fn from_name(name: &str) -> Option<SystemType> {
        // Handle qualified names like "{urn:hl7-org:elm-types:r1}Integer"
        let simple = if let Some(pos) = name.rfind('}') {
            &name[pos + 1..]
        } else if let Some(pos) = name.rfind('.') {
            &name[pos + 1..]
        } else {
            name
        };

        match simple {
            "Any" => Some(SystemType::Any),
            "Boolean" => Some(SystemType::Boolean),
            "Integer" => Some(SystemType::Integer),
            "Long" => Some(SystemType::Long),
            "Decimal" => Some(SystemType::Decimal),
            "String" => Some(SystemType::String),
            "Date" => Some(SystemType::Date),
            "DateTime" => Some(SystemType::DateTime),
            "Time" => Some(SystemType::Time),
            "Quantity" => Some(SystemType::Quantity),
            "Ratio" => Some(SystemType::Ratio),
            "Code" => Some(SystemType::Code),
            "Concept" => Some(SystemType::Concept),
            "Vocabulary" => Some(SystemType::Vocabulary),
            _ => None,
        }
    }

    /// Check if this is a numeric type (Integer, Long, or Decimal).
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            SystemType::Integer | SystemType::Long | SystemType::Decimal
        )
    }

    /// Check if this is a temporal type (Date, DateTime, or Time).
    pub fn is_temporal(&self) -> bool {
        matches!(
            self,
            SystemType::Date | SystemType::DateTime | SystemType::Time
        )
    }
}

impl fmt::Display for SystemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.simple_name())
    }
}

/// A tuple element with name and type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleElement {
    /// The name of the element.
    pub name: String,
    /// The type of the element.
    pub element_type: Box<DataType>,
}

/// Internal representation of a CQL data type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum DataType {
    /// A system (primitive) type.
    System(SystemType),

    /// A model type (e.g., FHIR.Patient).
    /// Contains (namespace, name).
    Model { namespace: String, name: String },

    /// A list type: List<T>.
    List(Box<DataType>),

    /// An interval type: Interval<T>.
    Interval(Box<DataType>),

    /// A tuple type with named elements.
    Tuple(Vec<TupleElement>),

    /// A choice type (union of multiple types).
    Choice(Vec<DataType>),

    /// A type parameter (generic type variable).
    TypeParameter(String),

    /// Unknown/unresolved type.
    #[default]
    Unknown,
}

impl DataType {
    /// Create a system type.
    pub fn system(sys_type: SystemType) -> Self {
        DataType::System(sys_type)
    }

    /// Create a model type.
    pub fn model(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        DataType::Model {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    /// Create a list type.
    pub fn list(element_type: DataType) -> Self {
        DataType::List(Box::new(element_type))
    }

    /// Create an interval type.
    pub fn interval(point_type: DataType) -> Self {
        DataType::Interval(Box::new(point_type))
    }

    /// Create a tuple type.
    pub fn tuple(elements: Vec<TupleElement>) -> Self {
        DataType::Tuple(elements)
    }

    /// Create a choice type.
    pub fn choice(types: Vec<DataType>) -> Self {
        DataType::Choice(types)
    }

    /// Create a type parameter.
    pub fn type_parameter(name: impl Into<String>) -> Self {
        DataType::TypeParameter(name.into())
    }

    /// The Any type (supertype of all types).
    pub fn any() -> Self {
        DataType::System(SystemType::Any)
    }

    /// The Boolean type.
    pub fn boolean() -> Self {
        DataType::System(SystemType::Boolean)
    }

    /// The Integer type.
    pub fn integer() -> Self {
        DataType::System(SystemType::Integer)
    }

    /// The Long type.
    pub fn long() -> Self {
        DataType::System(SystemType::Long)
    }

    /// The Decimal type.
    pub fn decimal() -> Self {
        DataType::System(SystemType::Decimal)
    }

    /// The String type.
    pub fn string() -> Self {
        DataType::System(SystemType::String)
    }

    /// The Date type.
    pub fn date() -> Self {
        DataType::System(SystemType::Date)
    }

    /// The DateTime type.
    pub fn date_time() -> Self {
        DataType::System(SystemType::DateTime)
    }

    /// The Time type.
    pub fn time() -> Self {
        DataType::System(SystemType::Time)
    }

    /// The Quantity type.
    pub fn quantity() -> Self {
        DataType::System(SystemType::Quantity)
    }

    /// The Code type.
    pub fn code() -> Self {
        DataType::System(SystemType::Code)
    }

    /// The Concept type.
    pub fn concept() -> Self {
        DataType::System(SystemType::Concept)
    }

    /// Check if this is the Any type.
    pub fn is_any(&self) -> bool {
        matches!(self, DataType::System(SystemType::Any))
    }

    /// Check if this is a system (primitive) type.
    pub fn is_system(&self) -> bool {
        matches!(self, DataType::System(_))
    }

    /// Check if this is a model type.
    pub fn is_model(&self) -> bool {
        matches!(self, DataType::Model { .. })
    }

    /// Check if this is a list type.
    pub fn is_list(&self) -> bool {
        matches!(self, DataType::List(_))
    }

    /// Check if this is an interval type.
    pub fn is_interval(&self) -> bool {
        matches!(self, DataType::Interval(_))
    }

    /// Check if this is a tuple type.
    pub fn is_tuple(&self) -> bool {
        matches!(self, DataType::Tuple(_))
    }

    /// Check if this is a choice type.
    pub fn is_choice(&self) -> bool {
        matches!(self, DataType::Choice(_))
    }

    /// Check if this is a numeric type.
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            DataType::System(SystemType::Integer)
                | DataType::System(SystemType::Long)
                | DataType::System(SystemType::Decimal)
        )
    }

    /// Check if this is a temporal type.
    pub fn is_temporal(&self) -> bool {
        matches!(
            self,
            DataType::System(SystemType::Date)
                | DataType::System(SystemType::DateTime)
                | DataType::System(SystemType::Time)
        )
    }

    /// Get the element type if this is a List.
    pub fn element_type(&self) -> Option<&DataType> {
        match self {
            DataType::List(t) => Some(t),
            _ => None,
        }
    }

    /// Get the point type if this is an Interval.
    pub fn point_type(&self) -> Option<&DataType> {
        match self {
            DataType::Interval(t) => Some(t),
            _ => None,
        }
    }

    /// Get the tuple elements if this is a Tuple.
    pub fn tuple_elements(&self) -> Option<&[TupleElement]> {
        match self {
            DataType::Tuple(elements) => Some(elements),
            _ => None,
        }
    }

    /// Get the choice types if this is a Choice.
    pub fn choice_types(&self) -> Option<&[DataType]> {
        match self {
            DataType::Choice(types) => Some(types),
            _ => None,
        }
    }

    /// Check if this type is a subtype of another.
    ///
    /// Subtype relationships:
    /// - All types are subtypes of Any
    /// - Integer is a subtype of Long
    /// - Integer and Long are subtypes of Decimal
    /// - Date is a subtype of DateTime
    /// - A type is a subtype of itself
    pub fn is_subtype_of(&self, other: &DataType) -> bool {
        if self == other {
            return true;
        }

        // Everything is a subtype of Any
        if other.is_any() {
            return true;
        }

        match (self, other) {
            // Integer <: Long <: Decimal
            (DataType::System(SystemType::Integer), DataType::System(SystemType::Long)) => true,
            (DataType::System(SystemType::Integer), DataType::System(SystemType::Decimal)) => true,
            (DataType::System(SystemType::Long), DataType::System(SystemType::Decimal)) => true,

            // Date <: DateTime
            (DataType::System(SystemType::Date), DataType::System(SystemType::DateTime)) => true,

            // List covariance: List<A> <: List<B> if A <: B
            (DataType::List(a), DataType::List(b)) => a.is_subtype_of(b),

            // Interval covariance: Interval<A> <: Interval<B> if A <: B
            (DataType::Interval(a), DataType::Interval(b)) => a.is_subtype_of(b),

            // A type is a subtype of a choice if it's a subtype of any choice member
            (_, DataType::Choice(choices)) => choices.iter().any(|c| self.is_subtype_of(c)),

            // Model type inheritance would require ModelInfo lookup
            // For now, model types only match exactly
            _ => false,
        }
    }

    /// Check if this type can be implicitly converted to another.
    ///
    /// Implicit conversions happen automatically during type checking.
    /// They include subtype relationships plus additional conversions:
    /// - Integer -> Decimal
    /// - Long -> Decimal  
    /// - Code -> Concept
    /// - String -> DateTime (if valid format)
    /// - String -> Date (if valid format)
    /// - String -> Time (if valid format)
    pub fn can_convert_to(&self, target: &DataType) -> bool {
        // Subtype relationship implies convertibility
        if self.is_subtype_of(target) {
            return true;
        }

        // Unknown can convert to anything (for error recovery)
        if matches!(self, DataType::Unknown) || matches!(target, DataType::Unknown) {
            return true;
        }

        match (self, target) {
            // Numeric promotions (already handled by subtype, but explicit)
            (DataType::System(SystemType::Integer), DataType::System(SystemType::Decimal)) => true,
            (DataType::System(SystemType::Long), DataType::System(SystemType::Decimal)) => true,

            // Code -> Concept
            (DataType::System(SystemType::Code), DataType::System(SystemType::Concept)) => true,

            // List element conversion: List<A> can convert to List<B> if A can convert to B
            (DataType::List(a), DataType::List(b)) => a.can_convert_to(b),

            // Interval point conversion
            (DataType::Interval(a), DataType::Interval(b)) => a.can_convert_to(b),

            _ => false,
        }
    }

    /// Get the common supertype of two types.
    ///
    /// Returns the most specific type that both types can be converted to.
    /// Returns `Any` if no more specific common type exists.
    pub fn common_type(&self, other: &DataType) -> DataType {
        if self == other {
            return self.clone();
        }

        // If one is a subtype of the other, return the supertype
        if self.is_subtype_of(other) {
            return other.clone();
        }
        if other.is_subtype_of(self) {
            return self.clone();
        }

        // Numeric types have a common type
        if self.is_numeric() && other.is_numeric() {
            return DataType::decimal();
        }

        // Temporal types
        match (self, other) {
            (DataType::System(SystemType::Date), DataType::System(SystemType::DateTime))
            | (DataType::System(SystemType::DateTime), DataType::System(SystemType::Date)) => {
                return DataType::date_time();
            }
            _ => {}
        }

        // List types
        if let (DataType::List(a), DataType::List(b)) = (self, other) {
            return DataType::list(a.common_type(b));
        }

        // Interval types
        if let (DataType::Interval(a), DataType::Interval(b)) = (self, other) {
            return DataType::interval(a.common_type(b));
        }

        // Default to Any
        DataType::any()
    }

    /// Get the qualified name for ELM serialization.
    pub fn to_qualified_name(&self) -> String {
        match self {
            DataType::System(sys) => sys.qualified_name().to_string(),
            DataType::Model { namespace, name } => {
                format!("{{{namespace}}}{name}")
            }
            DataType::List(elem) => format!("List<{}>", elem.to_qualified_name()),
            DataType::Interval(point) => format!("Interval<{}>", point.to_qualified_name()),
            DataType::Tuple(_) => "Tuple".to_string(),
            DataType::Choice(_) => "Choice".to_string(),
            DataType::TypeParameter(name) => name.clone(),
            DataType::Unknown => "Unknown".to_string(),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::System(sys) => write!(f, "{sys}"),
            DataType::Model { namespace, name } => write!(f, "{namespace}.{name}"),
            DataType::List(elem) => write!(f, "List<{elem}>"),
            DataType::Interval(point) => write!(f, "Interval<{point}>"),
            DataType::Tuple(elements) => {
                write!(f, "Tuple {{ ")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", elem.name, elem.element_type)?;
                }
                write!(f, " }}")
            }
            DataType::Choice(types) => {
                write!(f, "Choice<")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{t}")?;
                }
                write!(f, ">")
            }
            DataType::TypeParameter(name) => write!(f, "{name}"),
            DataType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Implicit conversion rules for CQL types.
///
/// This defines which conversions happen automatically during type checking.
#[derive(Debug, Clone)]
pub struct ImplicitConversion {
    /// Source type.
    pub from: DataType,
    /// Target type.
    pub to: DataType,
    /// The conversion function name (for ELM generation).
    pub function: &'static str,
}

/// Get all implicit conversions defined by CQL.
pub fn implicit_conversions() -> Vec<ImplicitConversion> {
    vec![
        ImplicitConversion {
            from: DataType::integer(),
            to: DataType::long(),
            function: "ToLong",
        },
        ImplicitConversion {
            from: DataType::integer(),
            to: DataType::decimal(),
            function: "ToDecimal",
        },
        ImplicitConversion {
            from: DataType::long(),
            to: DataType::decimal(),
            function: "ToDecimal",
        },
        ImplicitConversion {
            from: DataType::date(),
            to: DataType::date_time(),
            function: "ToDateTime",
        },
        ImplicitConversion {
            from: DataType::code(),
            to: DataType::concept(),
            function: "ToConcept",
        },
    ]
}

/// Find the implicit conversion between two types, if one exists.
pub fn find_conversion(from: &DataType, to: &DataType) -> Option<ImplicitConversion> {
    implicit_conversions()
        .into_iter()
        .find(|conv| &conv.from == from && &conv.to == to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_type_names() {
        assert_eq!(SystemType::Integer.simple_name(), "Integer");
        assert_eq!(
            SystemType::Integer.qualified_name(),
            "{urn:hl7-org:elm-types:r1}Integer"
        );
    }

    #[test]
    fn test_system_type_from_name() {
        assert_eq!(SystemType::from_name("Integer"), Some(SystemType::Integer));
        assert_eq!(
            SystemType::from_name("{urn:hl7-org:elm-types:r1}Integer"),
            Some(SystemType::Integer)
        );
        assert_eq!(
            SystemType::from_name("System.Integer"),
            Some(SystemType::Integer)
        );
        assert_eq!(SystemType::from_name("Unknown"), None);
    }

    #[test]
    fn test_system_type_is_numeric() {
        assert!(SystemType::Integer.is_numeric());
        assert!(SystemType::Long.is_numeric());
        assert!(SystemType::Decimal.is_numeric());
        assert!(!SystemType::String.is_numeric());
        assert!(!SystemType::Boolean.is_numeric());
    }

    #[test]
    fn test_system_type_is_temporal() {
        assert!(SystemType::Date.is_temporal());
        assert!(SystemType::DateTime.is_temporal());
        assert!(SystemType::Time.is_temporal());
        assert!(!SystemType::String.is_temporal());
    }

    #[test]
    fn test_datatype_constructors() {
        let int_type = DataType::integer();
        assert!(int_type.is_system());
        assert!(int_type.is_numeric());

        let list_type = DataType::list(DataType::string());
        assert!(list_type.is_list());
        assert_eq!(list_type.element_type(), Some(&DataType::string()));

        let interval_type = DataType::interval(DataType::date());
        assert!(interval_type.is_interval());
        assert_eq!(interval_type.point_type(), Some(&DataType::date()));
    }

    #[test]
    fn test_subtype_any() {
        // Everything is a subtype of Any
        assert!(DataType::integer().is_subtype_of(&DataType::any()));
        assert!(DataType::string().is_subtype_of(&DataType::any()));
        assert!(DataType::list(DataType::integer()).is_subtype_of(&DataType::any()));
    }

    #[test]
    fn test_subtype_numeric() {
        // Integer <: Long <: Decimal
        assert!(DataType::integer().is_subtype_of(&DataType::long()));
        assert!(DataType::integer().is_subtype_of(&DataType::decimal()));
        assert!(DataType::long().is_subtype_of(&DataType::decimal()));

        // Not the other way
        assert!(!DataType::decimal().is_subtype_of(&DataType::integer()));
        assert!(!DataType::long().is_subtype_of(&DataType::integer()));
    }

    #[test]
    fn test_subtype_temporal() {
        // Date <: DateTime
        assert!(DataType::date().is_subtype_of(&DataType::date_time()));
        assert!(!DataType::date_time().is_subtype_of(&DataType::date()));
    }

    #[test]
    fn test_subtype_list_covariance() {
        // List<Integer> <: List<Decimal>
        let list_int = DataType::list(DataType::integer());
        let list_decimal = DataType::list(DataType::decimal());
        assert!(list_int.is_subtype_of(&list_decimal));
    }

    #[test]
    fn test_subtype_choice() {
        // Integer is a subtype of Choice<Integer, String>
        let choice = DataType::choice(vec![DataType::integer(), DataType::string()]);
        assert!(DataType::integer().is_subtype_of(&choice));
        assert!(DataType::string().is_subtype_of(&choice));
        assert!(!DataType::boolean().is_subtype_of(&choice));
    }

    #[test]
    fn test_can_convert_to() {
        assert!(DataType::integer().can_convert_to(&DataType::decimal()));
        assert!(DataType::code().can_convert_to(&DataType::concept()));
        assert!(!DataType::string().can_convert_to(&DataType::integer()));
    }

    #[test]
    fn test_common_type_same() {
        let int_type = DataType::integer();
        assert_eq!(int_type.common_type(&int_type), int_type);
    }

    #[test]
    fn test_common_type_numeric() {
        assert_eq!(
            DataType::integer().common_type(&DataType::decimal()),
            DataType::decimal()
        );
        assert_eq!(
            DataType::integer().common_type(&DataType::long()),
            DataType::long()
        );
        assert_eq!(
            DataType::long().common_type(&DataType::decimal()),
            DataType::decimal()
        );
    }

    #[test]
    fn test_common_type_temporal() {
        assert_eq!(
            DataType::date().common_type(&DataType::date_time()),
            DataType::date_time()
        );
    }

    #[test]
    fn test_common_type_unrelated() {
        assert_eq!(
            DataType::string().common_type(&DataType::boolean()),
            DataType::any()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(DataType::integer().to_string(), "Integer");
        assert_eq!(
            DataType::list(DataType::string()).to_string(),
            "List<String>"
        );
        assert_eq!(
            DataType::interval(DataType::date()).to_string(),
            "Interval<Date>"
        );
        assert_eq!(
            DataType::model("FHIR", "Patient").to_string(),
            "FHIR.Patient"
        );
    }

    #[test]
    fn test_qualified_name() {
        assert_eq!(
            DataType::integer().to_qualified_name(),
            "{urn:hl7-org:elm-types:r1}Integer"
        );
        assert_eq!(
            DataType::model("FHIR", "Patient").to_qualified_name(),
            "{FHIR}Patient"
        );
    }

    #[test]
    fn test_tuple_type() {
        let tuple = DataType::tuple(vec![
            TupleElement {
                name: "name".to_string(),
                element_type: Box::new(DataType::string()),
            },
            TupleElement {
                name: "age".to_string(),
                element_type: Box::new(DataType::integer()),
            },
        ]);

        assert!(tuple.is_tuple());
        let elements = tuple.tuple_elements().unwrap();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0].name, "name");
    }

    #[test]
    fn test_choice_type() {
        let choice = DataType::choice(vec![DataType::integer(), DataType::string()]);
        assert!(choice.is_choice());
        let types = choice.choice_types().unwrap();
        assert_eq!(types.len(), 2);
    }

    #[test]
    fn test_implicit_conversions() {
        let conversions = implicit_conversions();
        assert!(!conversions.is_empty());

        let int_to_decimal = find_conversion(&DataType::integer(), &DataType::decimal());
        assert!(int_to_decimal.is_some());
        assert_eq!(int_to_decimal.unwrap().function, "ToDecimal");
    }

    #[test]
    fn test_unknown_type() {
        let unknown = DataType::Unknown;
        assert!(unknown.can_convert_to(&DataType::integer()));
        assert!(DataType::integer().can_convert_to(&unknown));
    }
}
