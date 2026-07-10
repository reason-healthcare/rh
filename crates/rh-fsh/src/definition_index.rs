//! StructureDefinition lookup index for local FSH definitions and package dependencies.

use crate::dependencies::{
    DependencyDefinitionSet, DependencyFixedValue, DependencyStructureDefinition,
};
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::SdMetadata;
use crate::tank::FshTank;
use crate::{load_dependency_structure_definitions, FshConfig, FshError};
use std::collections::{HashMap, HashSet};

/// Source of an indexed StructureDefinition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionSource {
    Local { kind: LocalDefinitionKind },
    Dependency { package_id: String, version: String },
}

/// Local FSH definition kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDefinitionKind {
    Profile,
    Extension,
    Logical,
    Resource,
}

/// Normalized StructureDefinition metadata used for name/id/url lookups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedStructureDefinition {
    pub name: Option<String>,
    pub id: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub kind: Option<String>,
    pub type_: Option<String>,
    pub base_definition: Option<String>,
    pub parent: Option<String>,
    pub source: DefinitionSource,
    pub fixed_values: Vec<DependencyFixedValue>,
}

/// Lookup index keyed by FSH name, id, canonical URL, aliases, and URL tail.
#[derive(Debug, Clone, Default)]
pub struct DefinitionIndex {
    definitions: Vec<IndexedStructureDefinition>,
    keys: HashMap<String, usize>,
    pub dependency_warnings: Vec<String>,
}

impl DefinitionIndex {
    pub fn lookup(&self, key: &str) -> Option<&IndexedStructureDefinition> {
        self.keys
            .get(key)
            .or_else(|| {
                key.split_once('|')
                    .and_then(|(canonical, _)| self.keys.get(canonical))
            })
            .and_then(|idx| self.definitions.get(*idx))
    }

    pub fn definitions(&self) -> &[IndexedStructureDefinition] {
        &self.definitions
    }

    /// Resolve a StructureDefinition reference to the underlying FHIR resource/type name.
    pub fn resolve_base_type(&self, name_or_url: &str, defs: &FhirDefs) -> Option<String> {
        self.resolve_base_type_inner(name_or_url, defs, &mut HashSet::new())
    }

    fn resolve_base_type_inner(
        &self,
        name_or_url: &str,
        defs: &FhirDefs,
        seen: &mut HashSet<String>,
    ) -> Option<String> {
        if !seen.insert(name_or_url.to_string()) {
            return None;
        }

        if let Some(sd) = defs.get_sd(name_or_url) {
            return Some(sd.base_type.clone());
        }

        let definition = self.lookup(name_or_url)?;
        if let Some(type_) = &definition.type_ {
            if defs.get_sd(type_).is_some() {
                return Some(type_.clone());
            }
        }
        if let Some(parent) = definition
            .parent
            .as_ref()
            .or(definition.base_definition.as_ref())
        {
            return self.resolve_base_type_inner(parent, defs, seen);
        }
        None
    }

    fn insert(&mut self, definition: IndexedStructureDefinition) {
        let idx = self.definitions.len();
        for key in definition_keys(&definition) {
            self.keys.entry(key).or_insert(idx);
        }
        self.definitions.push(definition);
    }

    fn add_alias_keys(&mut self, aliases: &HashMap<String, String>) {
        for (alias, target) in aliases {
            if let Some(idx) = self.keys.get(target).copied() {
                self.keys.entry(alias.clone()).or_insert(idx);
            }
        }
    }
}

/// Build an index from local definitions and dependency definitions already loaded by the caller.
pub fn build_definition_index(
    tank: &FshTank,
    config: &FshConfig,
    dependencies: &DependencyDefinitionSet,
) -> DefinitionIndex {
    let mut index = DefinitionIndex {
        dependency_warnings: dependencies.warnings.clone(),
        ..Default::default()
    };

    for profile in tank.profiles.values() {
        index.insert(index_local_sd(
            &profile.metadata,
            LocalDefinitionKind::Profile,
            config,
        ));
    }
    for extension in tank.extensions.values() {
        index.insert(index_local_sd(
            &extension.metadata,
            LocalDefinitionKind::Extension,
            config,
        ));
    }
    for logical in tank.logicals.values() {
        index.insert(index_local_sd(
            &logical.metadata,
            LocalDefinitionKind::Logical,
            config,
        ));
    }
    for resource in tank.resources.values() {
        index.insert(index_local_sd(
            &resource.metadata,
            LocalDefinitionKind::Resource,
            config,
        ));
    }
    for definition in &dependencies.structure_definitions {
        index.insert(index_dependency_sd(definition));
    }

    index.add_alias_keys(&tank.aliases);
    index
}

/// Build an index from local definitions and dependency definitions in the default package cache.
pub fn build_definition_index_with_default_dependencies(
    tank: &FshTank,
    config: &FshConfig,
) -> Result<DefinitionIndex, FshError> {
    let dependencies = load_dependency_structure_definitions(config)?;
    Ok(build_definition_index(tank, config, &dependencies))
}

fn index_local_sd(
    metadata: &SdMetadata,
    kind: LocalDefinitionKind,
    config: &FshConfig,
) -> IndexedStructureDefinition {
    let id = metadata.id.clone().or_else(|| Some(metadata.name.clone()));
    let url = config.canonical.as_ref().map(|canonical| {
        format!(
            "{}/StructureDefinition/{}",
            canonical.trim_end_matches('/'),
            id.as_deref().unwrap_or(metadata.name.as_str())
        )
    });
    let type_ = match kind {
        LocalDefinitionKind::Extension => Some("Extension".to_string()),
        LocalDefinitionKind::Logical
        | LocalDefinitionKind::Resource
        | LocalDefinitionKind::Profile => None,
    };

    IndexedStructureDefinition {
        name: Some(metadata.name.clone()),
        id,
        url,
        title: metadata.title.clone(),
        kind: Some(
            match kind {
                LocalDefinitionKind::Profile => "resource",
                LocalDefinitionKind::Extension => "complex-type",
                LocalDefinitionKind::Logical => "logical",
                LocalDefinitionKind::Resource => "resource",
            }
            .to_string(),
        ),
        type_,
        base_definition: None,
        parent: metadata.parent.clone(),
        source: DefinitionSource::Local { kind },
        fixed_values: Vec::new(),
    }
}

fn index_dependency_sd(definition: &DependencyStructureDefinition) -> IndexedStructureDefinition {
    IndexedStructureDefinition {
        name: definition.name.clone(),
        id: definition.id.clone(),
        url: definition.url.clone(),
        title: definition.title.clone(),
        kind: definition.kind.clone(),
        type_: definition.type_.clone(),
        base_definition: definition.base_definition.clone(),
        parent: definition.base_definition.clone(),
        source: DefinitionSource::Dependency {
            package_id: definition.package_id.clone(),
            version: definition.version.clone(),
        },
        fixed_values: definition.fixed_values.clone(),
    }
}

fn definition_keys(definition: &IndexedStructureDefinition) -> Vec<String> {
    let mut keys = Vec::new();
    push_key(&mut keys, definition.name.as_deref());
    push_key(&mut keys, definition.id.as_deref());
    push_key(&mut keys, definition.url.as_deref());

    if let Some(url) = &definition.url {
        push_key(&mut keys, url.rsplit('/').next());
    }

    keys
}

fn push_key(keys: &mut Vec<String>, key: Option<&str>) {
    let Some(key) = key else {
        return;
    };
    if key.is_empty() || keys.iter().any(|existing| existing == key) {
        return;
    }
    keys.push(key.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Profile, Spanned};
    use std::path::PathBuf;

    fn empty_tank() -> FshTank {
        FshTank::new()
    }

    #[test]
    fn indexes_local_definitions_by_name_id_url_and_alias() {
        let mut tank = empty_tank();
        tank.aliases.insert(
            "$MyPatient".to_string(),
            "http://example.org/fhir/StructureDefinition/my-patient".to_string(),
        );
        tank.profiles.insert(
            "MyPatient".to_string(),
            Profile {
                metadata: SdMetadata {
                    name: "MyPatient".to_string(),
                    parent: Some("Patient".to_string()),
                    id: Some("my-patient".to_string()),
                    title: Some("My Patient".to_string()),
                    description: None,
                    characteristics: Vec::new(),
                },
                rules: Vec::<Spanned<_>>::new(),
            },
        );
        let config = FshConfig {
            canonical: Some("http://example.org/fhir".to_string()),
            ..Default::default()
        };

        let index = build_definition_index(&tank, &config, &DependencyDefinitionSet::default());

        assert_eq!(
            index.lookup("MyPatient").and_then(|d| d.id.as_deref()),
            Some("my-patient")
        );
        assert!(index.lookup("my-patient").is_some());
        assert!(index
            .lookup("http://example.org/fhir/StructureDefinition/my-patient")
            .is_some());
        assert!(index.lookup("$MyPatient").is_some());
    }

    #[test]
    fn indexes_dependency_definitions_by_name_id_url_and_tail() {
        let dependencies = DependencyDefinitionSet {
            structure_definitions: vec![DependencyStructureDefinition {
                package_id: "hl7.fhir.us.core".to_string(),
                version: "6.1.0".to_string(),
                path: PathBuf::from("StructureDefinition-us-core-patient.json"),
                id: Some("us-core-patient".to_string()),
                url: Some(
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
                ),
                name: Some("USCorePatientProfile".to_string()),
                title: None,
                kind: Some("resource".to_string()),
                type_: Some("Patient".to_string()),
                base_definition: Some(
                    "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
                ),
                derivation: Some("constraint".to_string()),
                fixed_values: Vec::new(),
            }],
            warnings: vec!["missing other package".to_string()],
        };

        let index = build_definition_index(&empty_tank(), &FshConfig::default(), &dependencies);

        assert_eq!(index.dependency_warnings, vec!["missing other package"]);
        assert_eq!(
            index
                .lookup("USCorePatientProfile")
                .and_then(|d| d.type_.as_deref()),
            Some("Patient")
        );
        assert!(index.lookup("us-core-patient").is_some());
        assert!(index
            .lookup("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient")
            .is_some());
    }

    #[test]
    fn resolves_dependency_profile_to_base_resource_type() {
        let dependencies = DependencyDefinitionSet {
            structure_definitions: vec![DependencyStructureDefinition {
                package_id: "hl7.fhir.us.core".to_string(),
                version: "6.1.0".to_string(),
                path: PathBuf::from("StructureDefinition-us-core-practitioner.json"),
                id: Some("us-core-practitioner".to_string()),
                url: Some(
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitioner"
                        .to_string(),
                ),
                name: Some("USCorePractitionerProfile".to_string()),
                title: None,
                kind: Some("resource".to_string()),
                type_: Some("Practitioner".to_string()),
                base_definition: Some(
                    "http://hl7.org/fhir/StructureDefinition/Practitioner".to_string(),
                ),
                derivation: Some("constraint".to_string()),
                fixed_values: Vec::new(),
            }],
            warnings: Vec::new(),
        };
        let index = build_definition_index(&empty_tank(), &FshConfig::default(), &dependencies);
        let defs = FhirDefs::r4();

        assert_eq!(
            index.resolve_base_type("USCorePractitionerProfile", defs.as_ref()),
            Some("Practitioner".to_string())
        );
        assert_eq!(
            index.resolve_base_type(
                "http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitioner",
                defs.as_ref()
            ),
            Some("Practitioner".to_string())
        );
    }

    #[test]
    fn resolves_versioned_profile_references_to_base_resource_type() {
        let dependencies = DependencyDefinitionSet {
            structure_definitions: vec![DependencyStructureDefinition {
                package_id: "hl7.fhir.us.core".to_string(),
                version: "7.0.0".to_string(),
                path: PathBuf::from("StructureDefinition-us-core-patient.json"),
                id: Some("us-core-patient".to_string()),
                url: Some(
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
                ),
                name: Some("USCorePatientProfile".to_string()),
                title: None,
                kind: Some("resource".to_string()),
                type_: Some("Patient".to_string()),
                base_definition: Some(
                    "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
                ),
                derivation: Some("constraint".to_string()),
                fixed_values: Vec::new(),
            }],
            warnings: Vec::new(),
        };
        let mut tank = empty_tank();
        tank.profiles.insert(
            "ProjectPatient".to_string(),
            Profile {
                metadata: SdMetadata {
                    name: "ProjectPatient".to_string(),
                    parent: Some("USCorePatientProfile|7.0.0".to_string()),
                    id: Some("project-patient".to_string()),
                    title: None,
                    description: None,
                    characteristics: Vec::new(),
                },
                rules: Vec::new(),
            },
        );
        let index = build_definition_index(&tank, &FshConfig::default(), &dependencies);

        assert_eq!(
            index.resolve_base_type("ProjectPatient", FhirDefs::r4().as_ref()),
            Some("Patient".to_string())
        );
        assert!(index.lookup("USCorePatientProfile|7.0.0").is_some());
    }
}
