use super::schema::{TerminologyRequirement, TerminologyRequirements};
use crate::eval::context::{EvalError, TerminologyProvider};
use crate::eval::value::{CqlCode, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Read-only terminology provider loaded from a verified snapshot directory.
#[derive(Debug, Clone, Default)]
pub struct SnapshotTerminologyProvider {
    pub(super) valuesets: HashMap<String, Vec<CqlCode>>,
    pub(super) ambiguous_valuesets: HashSet<String>,
    pub(super) valuesets_by_requirement: HashMap<String, Vec<CqlCode>>,
    pub(super) requirements: BTreeMap<String, TerminologyRequirement>,
}

impl TerminologyProvider for SnapshotTerminologyProvider {
    fn in_valueset(&self, code: &CqlCode, valueset_url: &str) -> Result<bool, EvalError> {
        if self.ambiguous_valuesets.contains(valueset_url) {
            return Err(EvalError::TerminologyError(format!(
                "ValueSet alias '{valueset_url}' is ambiguous; use a qualified reference"
            )));
        }
        self.valuesets
            .get(valueset_url)
            .map(|codes| {
                codes
                    .iter()
                    .any(|member| member.code == code.code && member.system == code.system)
            })
            .ok_or_else(|| {
                EvalError::TerminologyError(format!("Valueset not found: '{valueset_url}'"))
            })
    }

    fn expand_valueset(&self, valueset_url: &str) -> Result<Vec<CqlCode>, EvalError> {
        if self.ambiguous_valuesets.contains(valueset_url) {
            return Err(EvalError::TerminologyError(format!(
                "ValueSet alias '{valueset_url}' is ambiguous; use a qualified reference"
            )));
        }
        self.valuesets.get(valueset_url).cloned().ok_or_else(|| {
            EvalError::TerminologyError(format!("Valueset not found: '{valueset_url}'"))
        })
    }

    fn lookup(&self, _code: &CqlCode, _property: &str) -> Result<Option<Value>, EvalError> {
        Ok(None)
    }

    fn validate_valueset_ref(
        &self,
        reference: &crate::eval::value::ValueSetReference,
    ) -> Result<(), EvalError> {
        let requirement = self
            .requirements
            .get(&reference.requirement_id)
            .ok_or_else(|| {
                EvalError::TerminologyError(format!(
                    "terminology requirement '{}' is not bound in the snapshot",
                    reference.requirement_id
                ))
            })?;
        if requirement.canonical != reference.canonical
            || reference
                .version
                .as_ref()
                .is_some_and(|version| requirement.version.as_ref() != Some(version))
            || requirement
                .code_systems
                .iter()
                .map(|item| (&item.canonical, &item.version))
                .collect::<Vec<_>>()
                != reference
                    .code_systems
                    .iter()
                    .map(|item| (&item.0, &item.1))
                    .collect::<Vec<_>>()
        {
            return Err(EvalError::TerminologyError(format!(
                "terminology reference '{}' does not match its declared requirement",
                reference.requirement_id
            )));
        }
        Ok(())
    }

    fn in_valueset_ref(
        &self,
        code: &CqlCode,
        reference: &crate::eval::value::ValueSetReference,
    ) -> Result<bool, EvalError> {
        self.validate_valueset_ref(reference)?;
        self.valuesets_by_requirement
            .get(&reference.requirement_id)
            .map(|codes| {
                codes
                    .iter()
                    .any(|member| member.code == code.code && member.system == code.system)
            })
            .ok_or_else(|| {
                EvalError::TerminologyError(format!(
                    "terminology requirement '{}' has no expansion",
                    reference.requirement_id
                ))
            })
    }

    fn expand_valueset_ref(
        &self,
        reference: &crate::eval::value::ValueSetReference,
    ) -> Result<Vec<CqlCode>, EvalError> {
        self.validate_valueset_ref(reference)?;
        self.valuesets_by_requirement
            .get(&reference.requirement_id)
            .cloned()
            .ok_or_else(|| {
                EvalError::TerminologyError(format!(
                    "terminology requirement '{}' has no expansion",
                    reference.requirement_id
                ))
            })
    }
}

impl SnapshotTerminologyProvider {
    /// Verify that this snapshot contains compatible bindings for every declared requirement.
    pub fn validate_requirements(
        &self,
        requirements: &TerminologyRequirements,
    ) -> Result<(), String> {
        let expected: BTreeSet<_> = requirements
            .requirements
            .iter()
            .map(|item| item.id.as_str())
            .collect();
        if expected.len() != requirements.requirements.len() {
            return Err("compiled requirements contain duplicate IDs".into());
        }
        for requirement in &requirements.requirements {
            let Some(snapshot_requirement) = self.requirements.get(&requirement.id) else {
                return Err(format!(
                    "terminology snapshot is missing requirement '{}'",
                    requirement.id
                ));
            };
            if snapshot_requirement.canonical != requirement.canonical
                || requirement
                    .version
                    .as_ref()
                    .is_some_and(|version| snapshot_requirement.version.as_ref() != Some(version))
                || snapshot_requirement.code_systems != requirement.code_systems
            {
                return Err(format!("terminology snapshot binding for '{}' does not satisfy the compiled requirement", requirement.id));
            }
        }
        Ok(())
    }
}
