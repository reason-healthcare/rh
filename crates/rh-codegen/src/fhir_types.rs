//! FHIR-specific data structures
//!
//! This module contains data structures that represent FHIR StructureDefinitions,
//! ElementDefinitions, and related FHIR concepts.

#![allow(dead_code)]

use serde::Deserialize;

/// Represents a FHIR ValueSet resource
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ValueSet {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub expansion: Option<ValueSetExpansion>,
    pub compose: Option<ValueSetCompose>,
}

/// Represents a ValueSet expansion
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetExpansion {
    pub timestamp: Option<String>,
    pub total: Option<u32>,
    pub contains: Option<Vec<ValueSetExpansionContains>>,
}

/// Represents a concept in a ValueSet expansion
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetExpansionContains {
    pub system: Option<String>,
    pub code: String,
    pub display: Option<String>,
}

/// Represents a ValueSet compose
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetCompose {
    pub include: Option<Vec<ValueSetComposeInclude>>,
    pub exclude: Option<Vec<ValueSetComposeInclude>>,
}

/// Represents an include/exclude in ValueSet compose
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetComposeInclude {
    pub system: Option<String>,
    pub version: Option<String>,
    pub concept: Option<Vec<ValueSetComposeConcept>>,
    pub filter: Option<Vec<ValueSetComposeFilter>>,
}

/// Represents a concept in ValueSet compose
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetComposeConcept {
    pub code: String,
    pub display: Option<String>,
}

/// Represents a filter in ValueSet compose
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetComposeFilter {
    pub property: String,
    pub op: String,
    pub value: String,
}

/// Represents a FHIR CodeSystem resource
#[derive(Debug, Deserialize, Clone)]
pub struct CodeSystem {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub concept: Option<Vec<CodeSystemConcept>>,
}

/// Represents a concept in a CodeSystem
#[derive(Debug, Deserialize, Clone)]
pub struct CodeSystemConcept {
    pub code: String,
    pub display: Option<String>,
    pub definition: Option<String>,
}

/// Represents a FHIR element definition
#[derive(Debug, Deserialize, Clone)]
pub struct ElementDefinition {
    pub id: Option<String>,
    pub path: String,
    pub short: Option<String>,
    pub definition: Option<String>,
    pub min: Option<u32>,
    pub max: Option<String>,
    #[serde(rename = "type")]
    pub element_type: Option<Vec<ElementType>>,
    pub fixed: Option<serde_json::Value>,
    pub pattern: Option<serde_json::Value>,
    pub binding: Option<ElementBinding>,
}

/// Represents a FHIR element binding to a value set
#[derive(Debug, Deserialize, Clone)]
pub struct ElementBinding {
    pub strength: String,
    pub description: Option<String>,
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,
}

/// Represents a FHIR element type
#[derive(Debug, Deserialize, Clone)]
pub struct ElementType {
    pub code: Option<String>,
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<String>>,
}

/// Represents a FHIR StructureDefinition
#[derive(Debug, Deserialize)]
pub struct StructureDefinition {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub url: String,
    pub version: Option<String>,
    pub name: String,
    pub title: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub purpose: Option<String>,
    pub kind: String,
    #[serde(rename = "abstract")]
    pub is_abstract: bool,
    #[serde(rename = "type")]
    pub base_type: String,
    #[serde(rename = "baseDefinition")]
    pub base_definition: Option<String>,
    pub differential: Option<StructureDefinitionDifferential>,
    pub snapshot: Option<StructureDefinitionSnapshot>,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinitionDifferential {
    pub element: Vec<ElementDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinitionSnapshot {
    pub element: Vec<ElementDefinition>,
}
