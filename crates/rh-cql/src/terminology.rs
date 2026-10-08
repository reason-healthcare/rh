//! Immutable terminology requirement and snapshot support for CQL.

mod loader;
mod provider;
mod requirements;
mod schema;
mod validation;

pub use loader::load_terminology_snapshot;
#[cfg(test)]
use loader::{sha256_digest, snapshot_id};
pub use provider::SnapshotTerminologyProvider;
pub use requirements::{extract_terminology_requirements, requirement_id};
pub use schema::{
    LibraryIdentifierJson, TerminologyBinding, TerminologyCanonical, TerminologyCompleteness,
    TerminologyExpansionLock, TerminologyLock, TerminologyRequirement, TerminologyRequirements,
    TerminologySource, TERMINOLOGY_LOCK_SCHEMA_VERSION, TERMINOLOGY_REQUIREMENTS_SCHEMA_VERSION,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::context::TerminologyProvider;
    use crate::eval::value::CqlCode;
    use crate::library::{LibraryIdentifier, MemoryLibrarySourceProvider};
    use std::path::Path;

    #[test]
    fn requirement_ids_escape_delimiters_and_utf8() {
        assert_eq!(
            requirement_id("Lib|x", Some("v/1"), "é"),
            "Lib%7Cx|v%2F1|%C3%A9"
        );
    }

    #[test]
    fn requirements_include_transitive_declarations_and_codesystem_constraints() {
        let provider = MemoryLibrarySourceProvider::new();
        provider.register_source(
            LibraryIdentifier::new("Helper", Some("2")),
            "library Helper version '2'\ncodesystem CS: 'http://example.org/cs' version '2026'\nvalueset Shared: 'http://example.org/vs/shared' version '3' codesystems { CS }".into(),
        );
        let document = extract_terminology_requirements(
            "library Main version '1'\ninclude Helper version '2' called H\nvalueset Local: 'http://example.org/vs/local'",
            &provider,
        ).expect("requirements extraction");
        assert_eq!(document.requirements.len(), 2);
        let included = document
            .requirements
            .iter()
            .find(|requirement| requirement.name == "Shared")
            .expect("included requirement");
        assert_eq!(included.id, "Helper|2|Shared");
        assert_eq!(
            included.code_systems,
            vec![TerminologyCanonical {
                canonical: "http://example.org/cs".into(),
                version: Some("2026".into())
            }]
        );
    }

    #[test]
    fn golden_snapshot_is_rejected_without_fixture_opt_in_and_loads_with_it() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/terminology/golden-snapshot");
        assert!(load_terminology_snapshot(&path, false)
            .unwrap_err()
            .contains("--allow-fixture"));
        let provider = load_terminology_snapshot(&path, true).expect("fixture snapshot");
        let member = CqlCode {
            code: "a".into(),
            system: "https://example.test/CodeSystem/codes".into(),
            display: None,
            version: None,
        };
        assert!(provider
            .in_valueset(&member, "Golden|1.0|Codes")
            .expect("membership"));
        assert!(provider
            .in_valueset(&member, "https://example.test/ValueSet/codes|fixture-1")
            .expect("membership"));
    }

    #[test]
    fn snapshot_loads_two_versions_and_rejects_unqualified_canonical_alias() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/terminology/golden-snapshot");
        let temp =
            std::env::temp_dir().join(format!("rh-two-version-snapshot-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&temp);
        std::fs::create_dir_all(temp.join("expansions")).unwrap();

        let mut requirements: serde_json::Value =
            serde_json::from_slice(&std::fs::read(source.join("requirements.json")).unwrap())
                .unwrap();
        let mut second_requirement = requirements["requirements"][0].clone();
        second_requirement["id"] = serde_json::Value::String("Golden|1.0|CodesV2".into());
        second_requirement["name"] = serde_json::Value::String("CodesV2".into());
        second_requirement["version"] = serde_json::Value::String("fixture-2".into());
        requirements["requirements"]
            .as_array_mut()
            .unwrap()
            .push(second_requirement);
        requirements["requirements"]
            .as_array_mut()
            .unwrap()
            .sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
        let requirement_bytes = serde_json::to_vec_pretty(&requirements).unwrap();
        std::fs::write(temp.join("requirements.json"), &requirement_bytes).unwrap();

        let original_expansion_path = source.join("expansions/golden.json");
        let original_expansion_bytes = std::fs::read(&original_expansion_path).unwrap();
        let mut second_value: serde_json::Value =
            serde_json::from_slice(&original_expansion_bytes).unwrap();
        second_value["version"] = serde_json::Value::String("fixture-2".into());
        let second_expansion_bytes = serde_json::to_vec_pretty(&second_value).unwrap();
        std::fs::write(
            temp.join("expansions/golden-2.json"),
            &second_expansion_bytes,
        )
        .unwrap();
        std::fs::copy(original_expansion_path, temp.join("expansions/golden.json")).unwrap();

        let mut lock_value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(source.join("lock.json")).unwrap()).unwrap();
        lock_value["requirementsDigest"] =
            serde_json::Value::String(sha256_digest(&requirement_bytes));
        let mut second_expansion = lock_value["expansions"][0].clone();
        second_expansion["id"] = serde_json::Value::String("golden-2".into());
        second_expansion["path"] = serde_json::Value::String("expansions/golden-2.json".into());
        second_expansion["version"] = serde_json::Value::String("fixture-2".into());
        let second_digest = sha256_digest(&second_expansion_bytes);
        second_expansion["contentDigest"] = serde_json::Value::String(second_digest.clone());
        second_expansion["source"]["identity"] = serde_json::Value::String(second_digest);
        lock_value["expansions"]
            .as_array_mut()
            .unwrap()
            .push(second_expansion);
        lock_value["expansions"]
            .as_array_mut()
            .unwrap()
            .sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
        let mut second_binding = lock_value["bindings"][0].clone();
        second_binding["requirementId"] = serde_json::Value::String("Golden|1.0|CodesV2".into());
        second_binding["expansionId"] = serde_json::Value::String("golden-2".into());
        lock_value["bindings"]
            .as_array_mut()
            .unwrap()
            .push(second_binding);
        lock_value["bindings"]
            .as_array_mut()
            .unwrap()
            .sort_by(|a, b| {
                a["requirementId"]
                    .as_str()
                    .cmp(&b["requirementId"].as_str())
            });
        lock_value["snapshotId"] = serde_json::Value::String(String::new());
        let mut lock: TerminologyLock = serde_json::from_value(lock_value).unwrap();
        lock.snapshot_id = snapshot_id(&lock).unwrap();
        std::fs::write(
            temp.join("lock.json"),
            serde_json::to_vec_pretty(&lock).unwrap(),
        )
        .unwrap();

        let provider = load_terminology_snapshot(&temp, true).expect("both versions load");
        let reference = crate::eval::value::ValueSetReference {
            requirement_id: "Golden|1.0|Codes".into(),
            canonical: "https://example.test/ValueSet/codes".into(),
            version: Some("fixture-1".into()),
            code_systems: vec![(
                "https://example.test/CodeSystem/codes".into(),
                Some("2026".into()),
            )],
        };
        let member = CqlCode {
            code: "a".into(),
            system: "https://example.test/CodeSystem/codes".into(),
            display: None,
            version: None,
        };
        assert!(provider.in_valueset_ref(&member, &reference).unwrap());
        let second_reference = crate::eval::value::ValueSetReference {
            requirement_id: "Golden|1.0|CodesV2".into(),
            version: Some("fixture-2".into()),
            ..reference.clone()
        };
        assert!(provider
            .in_valueset_ref(&member, &second_reference)
            .unwrap());
        assert!(provider
            .expand_valueset(&reference.canonical)
            .unwrap_err()
            .to_string()
            .contains("ambiguous"));
        let _ = std::fs::remove_dir_all(temp);
    }
}
