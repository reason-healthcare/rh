use super::schema::{TerminologyCanonical, TerminologyExpansionLock};
use crate::eval::value::CqlCode;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn validate_fhir_expansion(
    value: &serde_json::Value,
    expansion: &TerminologyExpansionLock,
) -> Result<Vec<CqlCode>, String> {
    if value.get("resourceType").and_then(|x| x.as_str()) != Some("ValueSet")
        || value.get("url").and_then(|x| x.as_str()) != Some(expansion.canonical.as_str())
    {
        return Err(format!(
            "ValueSet resource identity does not match lock entry '{}'",
            expansion.id
        ));
    }
    if value.get("version").and_then(|x| x.as_str()) != expansion.version.as_deref() {
        return Err(format!(
            "ValueSet version does not match lock entry '{}'",
            expansion.id
        ));
    }
    if value
        .get("extension")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|extensions| {
            extensions.iter().any(|item| {
                item.get("url")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|url| url.ends_with("/valueset-unclosed"))
            })
        })
    {
        return Err(format!("ValueSet '{}' is marked unclosed", expansion.id));
    }
    validate_unclosed_extensions(value, &expansion.id)?;
    let exp = value
        .get("expansion")
        .ok_or_else(|| format!("ValueSet '{}' has no expansion", expansion.id))?;
    validate_unclosed_extensions(exp, &expansion.id)?;
    if expansion.completeness.scope == "production"
        && value
            .get("experimental")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
    {
        return Err(format!(
            "production expansion '{}' is marked experimental",
            expansion.id
        ));
    }
    let mut release_evidence = BTreeMap::<String, String>::new();
    if let Some(parameters) = exp.get("parameter").and_then(serde_json::Value::as_array) {
        for parameter in parameters {
            let parameter_name = parameter
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            if matches!(
                parameter_name,
                "used-codesystem" | "system-version" | "version"
            ) {
                let raw = parameter
                    .get("valueUri")
                    .or_else(|| parameter.get("valueCanonical"))
                    .or_else(|| parameter.get("valueString"))
                    .and_then(serde_json::Value::as_str)
                    .ok_or_else(|| {
                        format!(
                            "ValueSet '{}' has malformed {parameter_name} provenance",
                            expansion.id
                        )
                    })?;
                let (system, version) = raw.split_once('|').unwrap_or((raw, ""));
                if system.is_empty() {
                    return Err(format!(
                        "ValueSet '{}' has malformed {parameter_name} provenance",
                        expansion.id
                    ));
                }
                if !version.is_empty()
                    && release_evidence
                        .insert(system.to_string(), version.to_string())
                        .is_some_and(|existing| existing != version)
                {
                    return Err(format!(
                        "ValueSet '{}' has conflicting release provenance for '{system}'",
                        expansion.id
                    ));
                }
            }
        }
    }
    let lock_release_evidence = parse_lock_system_versions(expansion)?;
    for (system, version) in &lock_release_evidence {
        if release_evidence
            .get(system)
            .is_some_and(|fhir_version| fhir_version != version)
        {
            return Err(format!(
                "ValueSet '{}' lock release provenance conflicts with FHIR response for '{system}'",
                expansion.id
            ));
        }
    }
    if exp.get("offset").is_some()
        || exp.get("filter").is_some()
        || exp.get("notClosed").and_then(serde_json::Value::as_bool) == Some(true)
        || exp
            .get("limitedExpansion")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        || exp
            .get("parameter")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|parameters| {
                parameters
                    .iter()
                    .any(|parameter| !valid_expansion_parameter(parameter))
            })
        || exp
            .get("parameter")
            .is_some_and(|parameters| !parameters.is_array())
    {
        return Err(format!(
            "ValueSet '{}' has paged or filtered expansion",
            expansion.id
        ));
    }
    let contains = exp
        .get("contains")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let total = exp
        .get("total")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| format!("ValueSet '{}' expansion lacks total", expansion.id))?;
    if total != contains.len() as u64 || total != expansion.completeness.total {
        return Err(format!(
            "ValueSet '{}' expansion is incomplete",
            expansion.id
        ));
    }
    if exp
        .get("identifier")
        .and_then(serde_json::Value::as_str)
        .is_none_or(str::is_empty)
        || exp
            .get("timestamp")
            .and_then(serde_json::Value::as_str)
            .is_none_or(str::is_empty)
    {
        return Err(format!(
            "ValueSet '{}' expansion lacks identifier or timestamp",
            expansion.id
        ));
    }
    let mut codes = Vec::new();
    let mut actual_systems = BTreeMap::<String, Option<String>>::new();
    let mut member_keys = BTreeSet::new();
    for item in contains {
        if item.get("contains").is_some() {
            return Err(format!(
                "ValueSet '{}' expansion is hierarchical",
                expansion.id
            ));
        }
        let code = item
            .get("code")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("ValueSet '{}' expansion entry has no code", expansion.id))?;
        let system = item
            .get("system")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("ValueSet '{}' expansion entry has no system", expansion.id))?;
        if code.is_empty() || system.is_empty() {
            return Err(format!(
                "ValueSet '{}' contains an empty system or code",
                expansion.id
            ));
        }
        if item.get("abstract").and_then(serde_json::Value::as_bool) == Some(true) {
            return Err(format!(
                "ValueSet '{}' contains an abstract code",
                expansion.id
            ));
        }
        if !member_keys.insert((system.to_owned(), code.to_owned())) {
            return Err(format!(
                "ValueSet '{}' contains duplicate system/code {system}|{code}",
                expansion.id
            ));
        }
        let member_version = item
            .get("version")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned);
        let parameter_version = release_evidence.get(system).cloned();
        let locked_version = lock_release_evidence.get(system).cloned();
        if member_version
            .as_ref()
            .zip(parameter_version.as_ref())
            .is_some_and(|(a, b)| a != b)
        {
            return Err(format!(
                "ValueSet '{}' has conflicting CodeSystem release evidence for '{system}'",
                expansion.id
            ));
        }
        if member_version
            .as_ref()
            .zip(locked_version.as_ref())
            .is_some_and(|(a, b)| a != b)
            || parameter_version
                .as_ref()
                .zip(locked_version.as_ref())
                .is_some_and(|(a, b)| a != b)
            || (locked_version.is_some() && member_version.is_none() && parameter_version.is_none())
        {
            return Err(format!(
                "ValueSet '{}' lacks matching FHIR CodeSystem release evidence for '{system}'",
                expansion.id
            ));
        }
        let resolved_version = member_version.or(parameter_version);
        if let Some(previous) = actual_systems.insert(system.to_owned(), resolved_version.clone()) {
            if previous != resolved_version {
                return Err(format!(
                    "ValueSet '{}' has conflicting releases for CodeSystem '{system}'",
                    expansion.id
                ));
            }
        }
        if let Some(constraint) = expansion
            .code_systems
            .iter()
            .find(|constraint| constraint.canonical == system)
        {
            if constraint.version.as_ref().is_some_and(|version| {
                item.get("version")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|actual| actual != version)
            }) {
                return Err(format!(
                    "ValueSet '{}' member has a conflicting CodeSystem version",
                    expansion.id
                ));
            }
        }
        if release_evidence.get(system).is_some_and(|expected| {
            item.get("version")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|actual| actual != expected)
        }) {
            return Err(format!(
                "ValueSet '{}' member conflicts with used CodeSystem release provenance",
                expansion.id
            ));
        }
        codes.push(CqlCode {
            code: code.into(),
            system: system.into(),
            display: item
                .get("display")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            version: item
                .get("version")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
        });
    }
    let actual_code_systems = actual_systems
        .into_iter()
        .map(|(canonical, version)| TerminologyCanonical { canonical, version })
        .collect::<BTreeSet<_>>();
    let locked_code_systems = expansion
        .code_systems
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if actual_code_systems != locked_code_systems {
        return Err(format!(
            "ValueSet '{}' lock codeSystems do not match actual expansion releases",
            expansion.id
        ));
    }
    Ok(codes)
}

fn validate_unclosed_extensions(value: &serde_json::Value, id: &str) -> Result<(), String> {
    if let Some(extension_value) = value.get("extension") {
        let extensions = extension_value
            .as_array()
            .ok_or_else(|| format!("ValueSet '{id}' has malformed extensions"))?;
        for extension in extensions {
            let url = extension
                .get("url")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| format!("ValueSet '{id}' has malformed extension URL"))?
                .to_ascii_lowercase();
            if url.contains("unclosed") || url.contains("limited-expansion") {
                return Err(format!("ValueSet '{id}' is marked unclosed or limited"));
            }
        }
    }
    Ok(())
}

fn valid_expansion_parameter(parameter: &serde_json::Value) -> bool {
    let Some(object) = parameter.as_object() else {
        return false;
    };
    if object.len() != 2 {
        return false;
    }
    let Some(name) = parameter.get("name").and_then(serde_json::Value::as_str) else {
        return false;
    };
    let values = object
        .keys()
        .filter(|key| key.starts_with("value"))
        .collect::<Vec<_>>();
    if values.len() != 1 {
        return false;
    }
    let field = values[0].as_str();
    let value = &object[field];
    match name {
        "used-codesystem" | "system-version" | "version" if field == "valueUri" => value
            .as_str()
            .and_then(|value| value.split_once('|'))
            .is_some_and(|(system, version)| {
                !system.is_empty() && !version.is_empty() && !version.contains('|')
            }),
        "includeDesignations" | "excludeNested" if field == "valueBoolean" => value.is_boolean(),
        "displayLanguage" if field == "valueCode" || field == "valueString" => {
            value.as_str().is_some_and(|value| !value.is_empty())
        }
        _ => false,
    }
}

fn parse_lock_system_versions(
    expansion: &TerminologyExpansionLock,
) -> Result<BTreeMap<String, String>, String> {
    let Some(values) = expansion.parameters.get("system-version") else {
        return Ok(BTreeMap::new());
    };
    let values = values.as_array().ok_or_else(|| {
        format!(
            "expansion '{}' lock system-version must be an array",
            expansion.id
        )
    })?;
    let mut result = BTreeMap::new();
    for value in values {
        let raw = value.as_str().ok_or_else(|| {
            format!(
                "expansion '{}' has malformed lock system-version",
                expansion.id
            )
        })?;
        let (system, version) = raw.split_once('|').ok_or_else(|| {
            format!(
                "expansion '{}' has malformed lock system-version",
                expansion.id
            )
        })?;
        if system.is_empty() || version.is_empty() || version.contains('|') {
            return Err(format!(
                "expansion '{}' has malformed lock system-version",
                expansion.id
            ));
        }
        if result
            .insert(system.to_string(), version.to_string())
            .is_some_and(|existing| existing != version)
        {
            return Err(format!(
                "expansion '{}' has conflicting lock system-version for '{system}'",
                expansion.id
            ));
        }
    }
    Ok(result)
}
