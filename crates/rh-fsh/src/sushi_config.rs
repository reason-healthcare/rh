//! Minimal `sushi-config.yaml` ingestion for project-level FSH compilation.

use crate::{FshConfig, FshDependency, FshError};
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
    Object { name: Option<String> },
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
    Ok(FshConfig {
        canonical: raw.canonical,
        fhir_version: raw.fhir_version.and_then(|v| match v {
            ScalarOrList::Scalar(s) => Some(s),
            ScalarOrList::List(values) => values.into_iter().next(),
        }),
        id: raw.id,
        name: raw.name,
        status: raw.status,
        publisher: raw.publisher.and_then(|p| match p {
            PublisherValue::Name(name) => Some(name),
            PublisherValue::Object { name } => name,
        }),
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
}
