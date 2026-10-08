use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The version of the serialized terminology requirements schema.
pub const TERMINOLOGY_REQUIREMENTS_SCHEMA_VERSION: u32 = 1;
/// The version of the serialized terminology lock schema.
pub const TERMINOLOGY_LOCK_SCHEMA_VERSION: u32 = 1;

/// A CQL declaration whose terminology must be present before evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyRequirement {
    /// Stable identifier derived from the declaring library and definition.
    pub id: String,
    /// Library that declares the ValueSet.
    pub library: LibraryIdentifierJson,
    /// CQL ValueSet definition name.
    pub name: String,
    /// ValueSet canonical URL.
    pub canonical: String,
    /// Optional ValueSet version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Declared CodeSystem canonical and version constraints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_systems: Vec<TerminologyCanonical>,
    /// Operations that require this ValueSet.
    pub operations: Vec<String>,
}

/// JSON form of a CQL library identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryIdentifierJson {
    /// Library name.
    pub name: String,
    /// Optional library version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// A terminology canonical with an optional version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TerminologyCanonical {
    /// Canonical URL.
    pub canonical: String,
    /// Optional version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Serialized requirements file consumed by the snapshot preparation layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminologyRequirements {
    /// FHIR-like resource discriminator.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Requirements schema version.
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    /// Requirements in deterministic order.
    pub requirements: Vec<TerminologyRequirement>,
}

impl TerminologyRequirements {
    /// Create an empty requirements document.
    pub fn empty() -> Self {
        Self {
            resource_type: "ReasonHealthTerminologyRequirements".into(),
            schema_version: 1,
            requirements: Vec::new(),
        }
    }
}

/// Snapshot binding for one declared requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyBinding {
    pub requirement_id: String,
    pub expansion_id: String,
}

/// An immutable expansion recorded in a terminology lock file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyExpansionLock {
    pub id: String,
    pub path: String,
    pub canonical: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub source: TerminologySource,
    #[serde(default)]
    pub code_systems: Vec<TerminologyCanonical>,
    #[serde(default)]
    pub parameters: BTreeMap<String, serde_json::Value>,
    pub completeness: TerminologyCompleteness,
    pub content_digest: String,
}

/// Provenance for a terminology expansion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologySource {
    pub kind: String,
    pub identity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
}

/// Completeness declaration for an expansion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCompleteness {
    pub status: String,
    pub total: u64,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
}

/// Snapshot lock document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminologyLock {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
    #[serde(rename = "requirementsDigest")]
    pub requirements_digest: String,
    pub bindings: Vec<TerminologyBinding>,
    pub expansions: Vec<TerminologyExpansionLock>,
}
