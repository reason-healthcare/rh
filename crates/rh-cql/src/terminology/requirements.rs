use super::schema::{
    LibraryIdentifierJson, TerminologyCanonical, TerminologyRequirement, TerminologyRequirements,
};
use crate::library::{LibraryIdentifier, LibrarySourceProvider};
use crate::parser::{ast, CqlParser};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// Percent-encode an identifier component and join library/name components with `|`.
pub fn requirement_id(
    library_name: &str,
    library_version: Option<&str>,
    definition_name: &str,
) -> String {
    format!(
        "{}|{}|{}",
        encode_component(library_name),
        encode_component(library_version.unwrap_or("")),
        encode_component(definition_name)
    )
}

fn encode_component(input: &str) -> String {
    let mut output = String::new();
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            output.push(byte as char);
        } else {
            output.push_str(&format!("%{byte:02X}"));
        }
    }
    output
}

/// Extract requirements from a CQL library and all resolvable transitive includes.
pub fn extract_terminology_requirements(
    source: &str,
    provider: &dyn LibrarySourceProvider,
) -> Result<TerminologyRequirements, String> {
    let mut requirements = BTreeMap::new();
    let mut visited = HashSet::new();
    extract_recursive(source, None, provider, &mut visited, &mut requirements)?;
    let mut document = TerminologyRequirements::empty();
    document.requirements = requirements.into_values().collect();
    Ok(document)
}

fn extract_recursive(
    source: &str,
    requested: Option<&LibraryIdentifier>,
    provider: &dyn LibrarySourceProvider,
    visited: &mut HashSet<String>,
    result: &mut BTreeMap<String, TerminologyRequirement>,
) -> Result<(), String> {
    let library = CqlParser::new().parse(source).map_err(|e| e.to_string())?;
    let ident = library.identifier.as_ref();
    let lib_name = ident
        .map(|i| i.name.as_str())
        .or_else(|| requested.map(|i| i.name.as_str()))
        .unwrap_or("");
    let lib_version = ident
        .and_then(|i| i.version.as_deref())
        .or_else(|| requested.and_then(|i| i.version.as_deref()));
    let operations = vec![
        "membership".to_string(),
        "expand".to_string(),
        "string-membership".to_string(),
    ];
    let key = format!("{lib_name}|{}", lib_version.unwrap_or_default());
    if !visited.insert(key) {
        return Ok(());
    }
    for valueset in &library.valuesets {
        let systems: BTreeMap<&str, &ast::CodeSystemDef> = library
            .codesystems
            .iter()
            .map(|cs| (cs.name.as_str(), cs))
            .collect();
        let mut resolved_systems = BTreeSet::new();
        for name in &valueset.codesystems {
            if let Some(code_system) = systems.get(name.as_str()) {
                resolved_systems.insert(TerminologyCanonical {
                    canonical: code_system.id.clone(),
                    version: code_system.version.clone(),
                });
                continue;
            }
            let include = library
                .includes
                .iter()
                .filter_map(|include| {
                    let alias = include.alias.as_deref().unwrap_or(&include.path);
                    name.strip_prefix(alias)
                        .and_then(|rest| rest.strip_prefix('.'))
                        .map(|local_name| (alias.len(), include, local_name))
                })
                .max_by_key(|(length, _, _)| *length);
            let Some((_, include, local_name)) = include else {
                return Err(format!(
                    "ValueSet '{}' references unknown CodeSystem '{}' in library '{}'",
                    valueset.name, name, lib_name
                ));
            };
            let requested = LibraryIdentifier::new(include.path.clone(), include.version.clone());
            let source = provider
                .get_source(&requested)
                .ok_or_else(|| format!("Unable to resolve included library '{}'", requested))?;
            let included_library = CqlParser::new()
                .parse(&source.source)
                .map_err(|error| error.to_string())?;
            let code_system = included_library
                .codesystems
                .iter()
                .find(|system| system.name == local_name)
                .ok_or_else(|| {
                    format!(
                        "ValueSet '{}' references unknown included CodeSystem '{}.{}'",
                        valueset.name,
                        include.alias.as_deref().unwrap_or(&include.path),
                        local_name
                    )
                })?;
            resolved_systems.insert(TerminologyCanonical {
                canonical: code_system.id.clone(),
                version: code_system.version.clone(),
            });
        }
        let code_systems = resolved_systems.into_iter().collect();
        let id = requirement_id(lib_name, lib_version, &valueset.name);
        result.insert(
            id.clone(),
            TerminologyRequirement {
                id,
                library: LibraryIdentifierJson {
                    name: lib_name.to_owned(),
                    version: lib_version.map(str::to_owned),
                },
                name: valueset.name.clone(),
                canonical: valueset.id.clone(),
                version: valueset.version.clone(),
                code_systems,
                operations: operations.clone(),
            },
        );
    }
    for include in &library.includes {
        let requested = LibraryIdentifier::new(include.path.clone(), include.version.clone());
        let source = provider
            .get_source(&requested)
            .ok_or_else(|| format!("Unable to resolve included library '{}'", requested))?;
        extract_recursive(&source.source, Some(&requested), provider, visited, result)?;
    }
    Ok(())
}
