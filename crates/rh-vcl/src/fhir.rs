//! FHIR ValueSet Compose types
//!
//! This module defines Rust types that match the FHIR R4/R5 ValueSet.compose structure
//! for representing the result of VCL to FHIR translation.

use serde::{Deserialize, Serialize};

/// FHIR ValueSet compose structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSetCompose {
    /// Date/time that the value set is effective
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,

    /// Include concepts from code systems
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub include: Vec<ValueSetInclude>,

    /// Exclude concepts from code systems
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub exclude: Vec<ValueSetInclude>,
}

/// Include/exclude entry in ValueSet compose
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSetInclude {
    /// The code system from which codes are included/excluded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// The version of the code system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Specific concepts to include/exclude
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub concept: Vec<ValueSetConcept>,

    /// Filters to apply to the code system
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub filter: Vec<ValueSetFilter>,

    /// ValueSets to include
    #[serde(skip_serializing_if = "Vec::is_empty", default, rename = "valueSet")]
    pub value_set: Vec<String>,
}

/// Specific concept in ValueSet include/exclude
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSetConcept {
    /// Code value
    pub code: String,

    /// Display text for the code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// Additional designations for the concept
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub designation: Vec<ValueSetDesignation>,
}

/// Filter to apply to code system concepts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSetFilter {
    /// Property to filter on
    pub property: String,

    /// Filter operation
    pub op: FilterOperator,

    /// Value to filter with
    pub value: String,
}

/// FHIR ValueSet filter operators
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilterOperator {
    /// Equal
    #[serde(rename = "=")]
    Equal,
    /// Is-a (subsumes)
    #[serde(rename = "is-a")]
    IsA,
    /// Descendent-of (child of, recursive)
    #[serde(rename = "descendent-of")]
    DescendentOf,
    /// Is-not-a (not subsumed by)
    #[serde(rename = "is-not-a")]
    IsNotA,
    /// Regular expression
    Regex,
    /// In (member of)
    #[serde(rename = "in")]
    In,
    /// Not-in (not member of)
    #[serde(rename = "not-in")]
    NotIn,
    /// Generalizes (parent of)
    Generalizes,
    /// Child-of (direct child only)
    #[serde(rename = "child-of")]
    ChildOf,
    /// Descendent-leaf (descendent and leaf node)
    #[serde(rename = "descendent-leaf")]
    DescendentLeaf,
    /// Exists
    Exists,
}

/// Designation for a concept
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSetDesignation {
    /// Language of designation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Use context for designation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_: Option<Coding>,

    /// Designation value
    pub value: String,
}

/// FHIR Coding type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coding {
    /// Code system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Version of code system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Code value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Display text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// User selected flag
    #[serde(skip_serializing_if = "Option::is_none", rename = "userSelected")]
    pub user_selected: Option<bool>,
}

impl ValueSetCompose {
    /// Create a new empty ValueSet compose
    pub fn new() -> Self {
        Self {
            effective_date: None,
            include: Vec::new(),
            exclude: Vec::new(),
        }
    }

    /// Add an include entry
    pub fn add_include(&mut self, include: ValueSetInclude) {
        self.include.push(include);
    }

    /// Add an exclude entry
    pub fn add_exclude(&mut self, exclude: ValueSetInclude) {
        self.exclude.push(exclude);
    }

    /// Check if compose is empty (no includes or excludes)
    pub fn is_empty(&self) -> bool {
        self.include.is_empty() && self.exclude.is_empty()
    }
}

impl ValueSetInclude {
    /// Create a new include for a code system
    pub fn new_system(system: String) -> Self {
        Self {
            system: Some(system),
            version: None,
            concept: Vec::new(),
            filter: Vec::new(),
            value_set: Vec::new(),
        }
    }

    /// Create a new include for a ValueSet reference
    pub fn new_valueset(valueset_url: String) -> Self {
        Self {
            system: None,
            version: None,
            concept: Vec::new(),
            filter: Vec::new(),
            value_set: vec![valueset_url],
        }
    }

    /// Add a concept
    pub fn add_concept(&mut self, code: String, display: Option<String>) {
        self.concept.push(ValueSetConcept {
            code,
            display,
            designation: Vec::new(),
        });
    }

    /// Add a filter
    pub fn add_filter(&mut self, property: String, op: FilterOperator, value: String) {
        self.filter.push(ValueSetFilter {
            property,
            op,
            value,
        });
    }
}

impl Default for ValueSetCompose {
    fn default() -> Self {
        Self::new()
    }
}
