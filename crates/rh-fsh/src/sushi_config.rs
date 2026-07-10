//! Minimal `sushi-config.yaml` ingestion for project-level FSH compilation.

use crate::{FshConfig, FshContact, FshDependency, FshError, FshTelecom};
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
    publisher: Option<PublisherValue>,
    contact: Option<ScalarOrListContact>,
    version: Option<String>,
    #[serde(rename = "fhirVersion")]
    fhir_version: Option<ScalarOrList>,
    dependencies: Option<IndexMap<String, DependencyValue>>,
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
    let contacts = explicit_contacts
        .map(|contacts| contacts.into_iter().map(normalize_contact).collect())
        .unwrap_or_else(|| publisher_contact.into_iter().collect());

    Ok(FshConfig {
        canonical: raw.canonical,
        fhir_version: raw.fhir_version.and_then(|v| match v {
            ScalarOrList::Scalar(s) => Some(s),
            ScalarOrList::List(values) => values.into_iter().next(),
        }),
        id: raw.id,
        name: raw.name,
        status: raw.status,
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
    fn explicit_contacts_override_publisher_contact() {
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

        assert_eq!(config.contacts.len(), 1);
        assert_eq!(config.contacts[0].name, None);
        assert_eq!(
            config.contacts[0].telecom[0].value,
            "http://example.org/committee"
        );
    }
}
