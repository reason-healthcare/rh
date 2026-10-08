use super::provider::SnapshotTerminologyProvider;
use super::schema::{
    TerminologyCanonical, TerminologyExpansionLock, TerminologyLock, TerminologyRequirements,
    TERMINOLOGY_LOCK_SCHEMA_VERSION, TERMINOLOGY_REQUIREMENTS_SCHEMA_VERSION,
};
use crate::eval::value::CqlCode;
use crate::terminology::requirement_id;
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

fn validate_requirements_document(
    requirements: &TerminologyRequirements,
) -> Result<BTreeSet<String>, String> {
    if requirements.resource_type != "ReasonHealthTerminologyRequirements"
        || requirements.schema_version != TERMINOLOGY_REQUIREMENTS_SCHEMA_VERSION
    {
        return Err("unsupported terminology requirements resource or schema version".into());
    }
    let requirement_ids = requirements
        .requirements
        .iter()
        .map(|requirement| requirement.id.clone())
        .collect::<BTreeSet<_>>();
    if requirement_ids.len() != requirements.requirements.len() {
        return Err("requirements.json contains duplicate requirement IDs".into());
    }
    for requirement in &requirements.requirements {
        if requirement.id
            != requirement_id(
                &requirement.library.name,
                requirement.library.version.as_deref(),
                &requirement.name,
            )
        {
            return Err(format!(
                "requirement '{}' has an invalid deterministic ID",
                requirement.id
            ));
        }
        if requirement.canonical.is_empty()
            || requirement.library.name.is_empty()
            || requirement.name.is_empty()
            || requirement.operations.is_empty()
            || requirement.operations.iter().any(|operation| {
                !matches!(
                    operation.as_str(),
                    "membership" | "expand" | "string-membership"
                )
            })
        {
            return Err(format!(
                "requirement '{}' has invalid identity or operations",
                requirement.id
            ));
        }
    }
    Ok(requirement_ids)
}

fn validate_bindings(
    requirements: &TerminologyRequirements,
    lock: &TerminologyLock,
    requirement_ids: &BTreeSet<String>,
) -> Result<(), String> {
    let mut bound_requirements = BTreeSet::new();
    for binding in &lock.bindings {
        if !requirement_ids.contains(&binding.requirement_id) {
            return Err(format!(
                "binding references unknown requirement '{}'",
                binding.requirement_id
            ));
        }
        if !bound_requirements.insert(binding.requirement_id.as_str()) {
            return Err(format!(
                "duplicate binding for requirement '{}'",
                binding.requirement_id
            ));
        }
        let req = requirements
            .requirements
            .iter()
            .find(|requirement| requirement.id == binding.requirement_id)
            .ok_or_else(|| format!("missing requirement '{}'", binding.requirement_id))?;
        let expansion = lock
            .expansions
            .iter()
            .find(|entry| entry.id == binding.expansion_id)
            .ok_or_else(|| {
                format!(
                    "binding references missing expansion '{}'",
                    binding.expansion_id
                )
            })?;
        if req.canonical != expansion.canonical
            || req
                .version
                .as_ref()
                .is_some_and(|version| expansion.version.as_ref() != Some(version))
        {
            return Err(format!(
                "binding for '{}' has mismatched ValueSet identity",
                binding.requirement_id
            ));
        }
        for constraint in &req.code_systems {
            if let Some(actual) = expansion
                .code_systems
                .iter()
                .find(|actual| actual.canonical == constraint.canonical)
            {
                if constraint
                    .version
                    .as_ref()
                    .is_some_and(|version| actual.version.as_ref() != Some(version))
                {
                    return Err(format!(
                        "binding for '{}' has a conflicting CodeSystem version for '{}'",
                        binding.requirement_id, constraint.canonical
                    ));
                }
            }
        }
    }
    if bound_requirements.len() != requirement_ids.len() {
        return Err("terminology lock does not bind every declared requirement".into());
    }
    Ok(())
}

/// Load a prepared snapshot's actual ValueSet resources into an immutable native provider.
pub fn load_terminology_snapshot(
    path: impl AsRef<Path>,
    allow_fixture: bool,
) -> Result<SnapshotTerminologyProvider, String> {
    let root = path.as_ref();
    let lock_path = root.join("lock.json");
    let lock: TerminologyLock = serde_json::from_slice(
        &std::fs::read(&lock_path).map_err(|e| format!("{}: {e}", lock_path.display()))?,
    )
    .map_err(|e| format!("invalid terminology lock: {e}"))?;
    if lock.resource_type != "ReasonHealthTerminologyLock"
        || lock.schema_version != TERMINOLOGY_LOCK_SCHEMA_VERSION
    {
        return Err("unsupported terminology lock resource or schema version".into());
    }
    if lock
        .bindings
        .windows(2)
        .any(|pair| pair[0].requirement_id > pair[1].requirement_id)
        || lock
            .expansions
            .windows(2)
            .any(|pair| pair[0].id > pair[1].id)
    {
        return Err("terminology lock bindings and expansions must be sorted by ID".into());
    }
    if snapshot_id(&lock)? != lock.snapshot_id {
        return Err("terminology snapshot ID does not match lock contents".into());
    }
    let requirements_path = root.join("requirements.json");
    let requirements_bytes = std::fs::read(&requirements_path)
        .map_err(|e| format!("{}: {e}", requirements_path.display()))?;
    if sha256_digest(&requirements_bytes) != lock.requirements_digest {
        return Err("requirements.json digest does not match terminology lock".into());
    }
    let requirements: TerminologyRequirements = serde_json::from_slice(&requirements_bytes)
        .map_err(|e| format!("invalid terminology requirements: {e}"))?;
    let requirement_ids = validate_requirements_document(&requirements)?;
    validate_bindings(&requirements, &lock, &requirement_ids)?;
    let root = root
        .canonicalize()
        .map_err(|e| format!("{}: {e}", root.display()))?;
    let codes_by_expansion = validate_and_load_expansions(&root, &lock, allow_fixture)?;
    materialize_provider(&requirements, &lock, &codes_by_expansion)
}

fn validate_and_load_expansions(
    root: &Path,
    lock: &TerminologyLock,
    allow_fixture: bool,
) -> Result<HashMap<String, Vec<CqlCode>>, String> {
    let mut expansion_ids = BTreeSet::new();
    let mut codes_by_expansion = HashMap::<String, Vec<CqlCode>>::new();
    for expansion in &lock.expansions {
        if !expansion_ids.insert(expansion.id.as_str()) {
            return Err(format!("duplicate expansion ID '{}'", expansion.id));
        }
        let codes = load_expansion_codes(root, expansion, allow_fixture)?;
        codes_by_expansion.insert(expansion.id.clone(), codes);
    }
    Ok(codes_by_expansion)
}

fn materialize_provider(
    requirements: &TerminologyRequirements,
    lock: &TerminologyLock,
    codes_by_expansion: &HashMap<String, Vec<CqlCode>>,
) -> Result<SnapshotTerminologyProvider, String> {
    let mut provider = SnapshotTerminologyProvider {
        requirements: requirements
            .requirements
            .iter()
            .cloned()
            .map(|item| (item.id.clone(), item))
            .collect(),
        ..SnapshotTerminologyProvider::default()
    };
    let mut alias_contexts =
        HashMap::<String, (Option<String>, Vec<TerminologyCanonical>, String)>::new();
    for binding in &lock.bindings {
        let expansion = lock
            .expansions
            .iter()
            .find(|entry| entry.id == binding.expansion_id)
            .ok_or_else(|| {
                format!(
                    "binding references missing expansion '{}'",
                    binding.expansion_id
                )
            })?;
        let codes = codes_by_expansion
            .get(&binding.expansion_id)
            .ok_or_else(|| {
                format!(
                    "missing validated codes for expansion '{}'",
                    binding.expansion_id
                )
            })?
            .clone();
        let requirement = provider
            .requirements
            .get(&binding.requirement_id)
            .ok_or_else(|| format!("missing requirement '{}'", binding.requirement_id))?;
        if provider
            .valuesets_by_requirement
            .insert(binding.requirement_id.clone(), codes)
            .is_some()
        {
            return Err(format!(
                "duplicate requirement binding '{}'",
                binding.requirement_id
            ));
        }
        let codes = provider.valuesets_by_requirement[&binding.requirement_id].clone();
        let mut aliases = vec![expansion.canonical.clone(), binding.requirement_id.clone()];
        if let Some(version) = &expansion.version {
            aliases.push(format!("{}|{}", expansion.canonical, version));
        }
        for alias in aliases {
            let context = (
                expansion.version.clone(),
                requirement.code_systems.clone(),
                expansion.id.clone(),
            );
            if alias_contexts
                .get(&alias)
                .is_some_and(|existing| existing != &context)
            {
                provider.ambiguous_valuesets.insert(alias.clone());
                provider.valuesets.remove(&alias);
                continue;
            }
            alias_contexts.insert(alias.clone(), context);
            if provider
                .valuesets
                .get(&alias)
                .is_some_and(|current| current != &codes)
            {
                provider.ambiguous_valuesets.insert(alias.clone());
                provider.valuesets.remove(&alias);
                continue;
            }
            provider
                .valuesets
                .entry(alias)
                .or_insert_with(|| codes.clone());
        }
    }
    Ok(provider)
}

fn load_expansion_codes(
    root: &Path,
    expansion: &TerminologyExpansionLock,
    allow_fixture: bool,
) -> Result<Vec<CqlCode>, String> {
    if expansion.completeness.status != "complete" {
        return Err(format!("expansion '{}' is not complete", expansion.id));
    }
    if expansion.completeness.scope == "fixture" && !allow_fixture {
        return Err(format!(
            "fixture terminology snapshot requires --allow-fixture: {}",
            expansion.id
        ));
    }
    if expansion.completeness.scope != "fixture" && expansion.completeness.scope != "production" {
        return Err(format!(
            "unsupported terminology scope '{}'",
            expansion.completeness.scope
        ));
    }
    if expansion.completeness.scope == "fixture"
        && expansion
            .completeness
            .justification
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty())
    {
        return Err(format!(
            "fixture expansion '{}' requires a justification",
            expansion.id
        ));
    }
    if expansion.source.kind.is_empty() || expansion.source.identity.is_empty() {
        return Err(format!(
            "expansion '{}' lacks source provenance",
            expansion.id
        ));
    }
    let relative = Path::new(&expansion.path);
    if relative.is_absolute()
        || relative.components().any(|part| {
            matches!(
                part,
                std::path::Component::ParentDir
                    | std::path::Component::RootDir
                    | std::path::Component::Prefix(_)
            )
        })
    {
        return Err(format!("unsafe expansion path '{}'", expansion.path));
    }
    let resource_path = root
        .join(relative)
        .canonicalize()
        .map_err(|e| format!("{}: {e}", root.join(relative).display()))?;
    if !resource_path.starts_with(root) {
        return Err(format!(
            "expansion path '{}' escapes snapshot directory",
            expansion.path
        ));
    }
    let resource_bytes =
        std::fs::read(&resource_path).map_err(|e| format!("{}: {e}", resource_path.display()))?;
    if sha256_digest(&resource_bytes) != expansion.content_digest {
        return Err(format!(
            "ValueSet '{}' content digest does not match lock",
            expansion.id
        ));
    }
    match expansion.source.kind.as_str() {
        "local" if expansion.source.identity != expansion.content_digest => {
            return Err(format!(
                "local expansion '{}' source identity must equal contentDigest",
                expansion.id
            ));
        }
        "local" => {}
        "fhir-expand" => {
            let identity = expansion.source.identity.as_str();
            let loopback_http = ["http://localhost", "http://127.0.0.1", "http://[::1]"]
                .iter()
                .any(|prefix| {
                    identity == *prefix
                        || identity.strip_prefix(prefix).is_some_and(|suffix| {
                            suffix.starts_with(':') || suffix.starts_with('/')
                        })
                });
            if !(identity.starts_with("https://") || loopback_http)
                || identity.contains('@')
                || identity.contains('#')
                || identity.contains('?')
            {
                return Err(format!(
                    "remote expansion '{}' source identity must be an HTTPS endpoint",
                    expansion.id
                ));
            }
        }
        other => {
            return Err(format!(
                "expansion '{}' has unsupported source kind '{other}'",
                expansion.id
            ));
        }
    }
    let value: serde_json::Value = serde_json::from_slice(&resource_bytes)
        .map_err(|e| format!("invalid ValueSet {}: {e}", resource_path.display()))?;
    super::validation::validate_fhir_expansion(&value, expansion)
}

pub(super) fn sha256_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!(
        "sha256:{}",
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    )
}

pub(super) fn snapshot_id(lock: &TerminologyLock) -> Result<String, String> {
    let mut value = serde_json::to_value(lock).map_err(|error| error.to_string())?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| "terminology lock is not an object".to_string())?;
    object.remove("snapshotId");
    if let Some(bindings) = object
        .get_mut("bindings")
        .and_then(serde_json::Value::as_array_mut)
    {
        bindings.sort_by(|left, right| {
            left.get("requirementId")
                .and_then(serde_json::Value::as_str)
                .cmp(
                    &right
                        .get("requirementId")
                        .and_then(serde_json::Value::as_str),
                )
        });
    }
    if let Some(expansions) = object
        .get_mut("expansions")
        .and_then(serde_json::Value::as_array_mut)
    {
        expansions.sort_by(|left, right| {
            left.get("id")
                .and_then(serde_json::Value::as_str)
                .cmp(&right.get("id").and_then(serde_json::Value::as_str))
        });
    }
    let bytes = serde_json::to_vec(&value).map_err(|error| error.to_string())?;
    Ok(sha256_digest(&bytes))
}
