//! Minimal `sushi-config.yaml` ingestion for project-level FSH compilation.

use crate::{
    FshCoding, FshConfig, FshContact, FshDependency, FshError, FshGroup, FshPage,
    FshResourceMetadata, FshTelecom,
};
use indexmap::IndexMap;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SushiConfigFile {
    id: Option<String>,
    canonical: Option<String>,
    name: Option<String>,
    status: Option<String>,
    title: Option<String>,
    description: Option<String>,
    license: Option<String>,
    experimental: Option<bool>,
    #[serde(default, rename = "extension")]
    extensions: Vec<serde_json::Value>,
    jurisdiction: Option<String>,
    publisher: Option<PublisherValue>,
    contact: Option<ScalarOrListContact>,
    version: Option<String>,
    #[serde(rename = "fhirVersion")]
    fhir_version: Option<ScalarOrList>,
    dependencies: Option<IndexMap<String, DependencyValue>>,
    pages: Option<IndexMap<String, PageValue>>,
    groups: Option<IndexMap<String, GroupValue>>,
    parameters: Option<IndexMap<String, serde_yaml::Value>>,
    resources: Option<IndexMap<String, ResourceValue>>,
    #[serde(rename = "copyrightYear")]
    copyright_year: Option<String>,
    #[serde(rename = "releaseLabel")]
    release_label: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PageValue {
    title: Option<String>,
    #[serde(default, rename = "extension")]
    extensions: Vec<serde_json::Value>,
    #[serde(flatten)]
    pages: IndexMap<String, PageValue>,
}

#[derive(Debug, Deserialize)]
struct GroupValue {
    name: String,
    description: Option<String>,
    #[serde(default)]
    resources: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResourceValue {
    name: Option<String>,
    description: Option<String>,
    example_canonical: Option<String>,
    example_boolean: Option<bool>,
    grouping_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ScalarOrList {
    Scalar(String),
    List(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PublisherValue {
    Name(String),
    Object {
        name: Option<String>,
        url: Option<String>,
        email: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ScalarOrListContact {
    Scalar(ContactValue),
    List(Vec<ContactValue>),
}

#[derive(Debug, Deserialize)]
struct ContactValue {
    name: Option<String>,
    #[serde(default)]
    telecom: Vec<TelecomValue>,
}

#[derive(Debug, Deserialize)]
struct TelecomValue {
    system: String,
    value: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DependencyValue {
    Version(String),
    Object {
        version: String,
        id: Option<String>,
        uri: Option<String>,
    },
}

/// Read and normalize a SUSHI config file into `FshConfig`.
pub fn read_sushi_config(path: &Path) -> Result<FshConfig, FshError> {
    let text = std::fs::read_to_string(path).map_err(|e| FshError::Export {
        message: format!("failed to read {}: {e}", path.display()),
    })?;
    parse_sushi_config(&text).map_err(|e| FshError::Export {
        message: format!("failed to parse {}: {e}", path.display()),
    })
}

/// Parse SUSHI config text into the compiler's normalized project config.
pub fn parse_sushi_config(input: &str) -> Result<FshConfig, serde_yaml::Error> {
    let raw: SushiConfigFile = serde_yaml::from_str(input)?;
    let (publisher, publisher_contact) = match raw.publisher {
        Some(PublisherValue::Name(name)) => (Some(name), None),
        Some(PublisherValue::Object { name, url, email }) => {
            let telecom = url
                .map(|value| FshTelecom {
                    system: "url".to_string(),
                    value,
                })
                .into_iter()
                .chain(email.map(|value| FshTelecom {
                    system: "email".to_string(),
                    value,
                }))
                .collect::<Vec<_>>();
            let contact = (!telecom.is_empty()).then(|| FshContact {
                name: name.clone(),
                telecom,
            });
            (name, contact)
        }
        None => (None, None),
    };
    let explicit_contacts = raw.contact.map(|contacts| match contacts {
        ScalarOrListContact::Scalar(contact) => vec![contact],
        ScalarOrListContact::List(contacts) => contacts,
    });
    let contacts = publisher_contact
        .into_iter()
        .chain(
            explicit_contacts
                .unwrap_or_default()
                .into_iter()
                .map(normalize_contact),
        )
        .collect();
    let mut parameters = normalize_parameters(raw.parameters.unwrap_or_default());
    if let Some(value) = raw.copyright_year {
        parameters.insert(0, ("copyrightyear".to_string(), value));
    }
    if let Some(value) = raw.release_label {
        let index = usize::from(!parameters.is_empty());
        parameters.insert(index, ("releaselabel".to_string(), value));
    }
    if let Some(canonical) = raw.canonical.as_ref() {
        if !parameters.iter().any(|(code, _)| code == "path-history") {
            parameters.push((
                "path-history".to_string(),
                format!("{}/history.html", canonical.trim_end_matches('/')),
            ));
        }
    }

    Ok(FshConfig {
        canonical: raw.canonical,
        fhir_version: raw.fhir_version.and_then(|v| match v {
            ScalarOrList::Scalar(s) => Some(s),
            ScalarOrList::List(values) => values.into_iter().next(),
        }),
        id: raw.id,
        name: raw.name,
        status: raw.status,
        title: raw.title,
        description: raw.description,
        license: raw.license,
        experimental: raw.experimental,
        extensions: raw.extensions,
        jurisdiction: raw.jurisdiction.and_then(|value| parse_coding(&value)),
        publisher,
        contacts,
        version: raw.version,
        dependencies: raw
            .dependencies
            .unwrap_or_default()
            .into_iter()
            .map(|(package_id, dep)| match dep {
                DependencyValue::Version(version) => FshDependency {
                    package_id,
                    version,
                    id: None,
                    uri: None,
                },
                DependencyValue::Object { version, id, uri } => FshDependency {
                    package_id,
                    version,
                    id,
                    uri,
                },
            })
            .collect(),
        pages: raw
            .pages
            .unwrap_or_default()
            .into_iter()
            .map(|(source, page)| normalize_page(source, page))
            .collect(),
        groups: raw
            .groups
            .unwrap_or_default()
            .into_iter()
            .map(|(id, group)| FshGroup {
                id,
                name: group.name,
                description: group.description,
                resources: group.resources,
            })
            .collect(),
        parameters,
        resources: raw
            .resources
            .unwrap_or_default()
            .into_iter()
            .map(|(reference, resource)| {
                (
                    reference,
                    FshResourceMetadata {
                        name: resource.name,
                        description: resource.description,
                        example_canonical: resource.example_canonical,
                        example_boolean: resource.example_boolean,
                        grouping_id: resource.grouping_id,
                    },
                )
            })
            .collect(),
    })
}

fn normalize_page(source: String, page: PageValue) -> FshPage {
    FshPage {
        title: page.title.unwrap_or_else(|| source.clone()),
        source,
        extensions: page.extensions,
        pages: page
            .pages
            .into_iter()
            .map(|(source, page)| normalize_page(source, page))
            .collect(),
    }
}

fn normalize_parameters(values: IndexMap<String, serde_yaml::Value>) -> Vec<(String, String)> {
    values
        .into_iter()
        .flat_map(|(code, value)| match value {
            serde_yaml::Value::Sequence(values) => values
                .into_iter()
                .filter_map(|value| scalar_string(&value).map(|value| (code.clone(), value)))
                .collect(),
            value => scalar_string(&value)
                .map(|value| vec![(code, value)])
                .unwrap_or_default(),
        })
        .collect()
}

fn scalar_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

fn parse_coding(value: &str) -> Option<FshCoding> {
    let (coding, display) = value
        .split_once(" \"")
        .map_or((value, None), |(coding, display)| {
            (coding, Some(display.trim_end_matches('"').to_string()))
        });
    let (system, code) = coding.rsplit_once('#')?;
    Some(FshCoding {
        system: system.to_string(),
        code: code.to_string(),
        display,
    })
}

fn normalize_contact(contact: ContactValue) -> FshContact {
    FshContact {
        name: contact.name,
        telecom: contact
            .telecom
            .into_iter()
            .map(|telecom| FshTelecom {
                system: telecom.system,
                value: telecom.value,
            })
            .collect(),
    }
}

/// Find the nearest SUSHI config for the provided input files.
///
/// If files come from different projects, the first nearest config is returned.
/// The caller can decide whether cross-project inputs are valid for its use case.
pub fn find_sushi_config_for_files(paths: &[PathBuf]) -> Option<PathBuf> {
    paths.iter().find_map(|path| find_sushi_config(path))
}

fn find_sushi_config(path: &Path) -> Option<PathBuf> {
    let start = if path.is_dir() {
        path
    } else {
        path.parent().unwrap_or(path)
    };
    for ancestor in start.ancestors() {
        for file_name in ["sushi-config.yaml", "sushi-config.yml"] {
            let candidate = ancestor.join(file_name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_string_and_object_dependencies() {
        let config = parse_sushi_config(
            r#"
id: example.fhir
canonical: http://example.org/fhir
version: 1.2.3
name: ExampleIG
status: active
publisher:
  name: Example Publisher
  url: http://example.org/publisher
  email: publisher@example.org
fhirVersion: 4.0.1
dependencies:
  hl7.fhir.uv.genomics-reporting: 2.0.0
  hl7.fhir.us.core:
    version: 6.1.0
    id: uscore
    uri: http://hl7.org/fhir/us/core/ImplementationGuide/hl7.fhir.us.core
"#,
        )
        .expect("config parses");

        assert_eq!(config.id.as_deref(), Some("example.fhir"));
        assert_eq!(config.canonical.as_deref(), Some("http://example.org/fhir"));
        assert_eq!(config.fhir_version.as_deref(), Some("4.0.1"));
        assert_eq!(config.publisher.as_deref(), Some("Example Publisher"));
        assert_eq!(config.contacts.len(), 1);
        assert_eq!(
            config.contacts[0].name.as_deref(),
            Some("Example Publisher")
        );
        assert_eq!(config.contacts[0].telecom[0].system, "url");
        assert_eq!(config.contacts[0].telecom[1].system, "email");
        assert_eq!(config.dependencies.len(), 2);
        assert_eq!(
            config.dependencies[0].package_id,
            "hl7.fhir.uv.genomics-reporting"
        );
        assert_eq!(config.dependencies[0].version, "2.0.0");
        assert_eq!(config.dependencies[1].package_id, "hl7.fhir.us.core");
        assert_eq!(config.dependencies[1].version, "6.1.0");
        assert_eq!(config.dependencies[1].id.as_deref(), Some("uscore"));
    }

    #[test]
    fn explicit_contacts_follow_publisher_contact() {
        let config = parse_sushi_config(
            r#"
publisher:
  name: Example Publisher
  url: http://example.org/publisher
contact:
  telecom:
    - system: url
      value: http://example.org/committee
"#,
        )
        .expect("config parses");

        assert_eq!(config.contacts.len(), 2);
        assert_eq!(
            config.contacts[0].name.as_deref(),
            Some("Example Publisher")
        );
        assert_eq!(
            config.contacts[1].telecom[0].value,
            "http://example.org/committee"
        );
    }

    #[test]
    fn parses_implementation_guide_definition_metadata() {
        let config = parse_sushi_config(
            r#"
title: Example Guide
description: An example guide.
license: CC0-1.0
experimental: false
jurisdiction: urn:iso:std:iso:3166#US "United States of America"
extension:
  - url: http://example.org/work-group
    valueCode: fhir
copyrightYear: 2026+
releaseLabel: STU 1
parameters:
  shownav: true
  special-url:
    - http://example.org/one
    - http://example.org/two
pages:
  index.md:
    title: Home
groups:
  examples:
    name: Examples
    description: Example resources.
    resources:
      - Patient/example
resources:
  Patient/example:
    name: Example Patient
    description: A patient example.
    exampleCanonical: http://example.org/StructureDefinition/patient
"#,
        )
        .expect("config parses");

        assert_eq!(config.title.as_deref(), Some("Example Guide"));
        assert_eq!(config.jurisdiction.as_ref().unwrap().code, "US");
        assert_eq!(config.pages[0].source, "index.md");
        assert_eq!(config.groups[0].resources, ["Patient/example"]);
        assert_eq!(
            config.parameters[0],
            ("copyrightyear".into(), "2026+".into())
        );
        assert_eq!(
            config.parameters[1],
            ("releaselabel".into(), "STU 1".into())
        );
        assert_eq!(config.parameters.len(), 5);
        assert_eq!(
            config.resources["Patient/example"].name.as_deref(),
            Some("Example Patient")
        );
    }
}
