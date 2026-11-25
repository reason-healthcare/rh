//! Adapter for the validator's profile registry, terminology and slice rules.
use super::{matches_slice, FhirValidator};
use crate::valueset::CodeInValueSetResult;
use rh_fhirpath::{
    extensions::fhir::host::FhirHost, EvaluationContext, FhirPathError, FhirPathResult,
    FhirPathValue,
};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Arc;

fn error(e: impl std::fmt::Display) -> FhirPathError {
    FhirPathError::EvaluationError {
        message: e.to_string(),
    }
}

impl std::fmt::Debug for FhirValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FhirValidator")
            .field("fhir_version", &self.fhir_version)
            .finish_non_exhaustive()
    }
}

impl FhirValidator {
    fn validate_conformance(
        &self,
        mut context: EvaluationContext,
        declared: Option<&str>,
        url: &str,
    ) -> FhirPathResult<Option<bool>> {
        let resource = &context.root;
        let Some(profile) = self.profile_registry.get_snapshot(url).map_err(error)? else {
            return Ok(None);
        };
        if self
            .active_conformance_checks
            .iter()
            .any(|(active, element)| active == url && element == resource)
        {
            return Err(error("Recursive conformsTo profile validation"));
        }
        let actual_type = declared.or_else(|| resource["resourceType"].as_str());
        if actual_type.is_some_and(|ty| {
            !rh_fhirpath::evaluator::metadata::FhirModel::R4.is_subtype(ty, &profile.type_)
        }) {
            return Ok(Some(false));
        }
        let mut nested = self.clone();
        nested
            .active_conformance_checks
            .push((url.to_owned(), resource.clone()));
        let nested = Arc::new(nested);
        context.host = Some(nested.clone());
        let result = if resource.get("resourceType").is_some() {
            nested.validate_with_profile_inner(resource, url, Some(&context))
        } else {
            nested.validate_profile_rules(resource, url, Some(&context))
        }
        .map_err(error)?;
        if result.valid {
            if let Some(issue) = result.issues.iter().find(|issue| {
                issue.message.contains("Failed to evaluate invariant")
                    || issue.message.contains("Failed to parse invariant")
            }) {
                return Err(error(&issue.message));
            }
        }
        Ok(Some(result.valid))
    }
}

impl FhirHost for FhirValidator {
    fn conforms_to(
        &self,
        resource: &Value,
        declared: Option<&str>,
        url: &str,
    ) -> FhirPathResult<Option<bool>> {
        self.validate_conformance(EvaluationContext::new(resource.clone()), declared, url)
    }

    fn conforms_to_element(
        &self,
        element: &FhirPathValue,
        declared: Option<&str>,
        url: &str,
    ) -> FhirPathResult<Option<bool>> {
        self.validate_conformance(
            EvaluationContext::from_fhirpath_value(element)?,
            declared,
            url,
        )
    }

    fn member_of(&self, value: &Value, url: &str) -> FhirPathResult<Option<bool>> {
        let codings = value
            .get("coding")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or_else(|| std::slice::from_ref(value));
        let mut unknown = false;
        for coding in codings {
            let Some(code) = coding.as_str().or_else(|| coding["code"].as_str()) else {
                return Ok(None);
            };
            let system = if let Some(s) = coding["system"].as_str() {
                s.to_owned()
            } else {
                let Some(vs) = self.valueset_loader.load_valueset(url).map_err(error)? else {
                    return Ok(None);
                };
                let mut systems = HashSet::new();
                if let Some(entries) = vs.expansion.and_then(|e| e.contains) {
                    systems.extend(entries.into_iter().filter_map(|c| c.system));
                }
                if let Some(entries) = vs.compose.and_then(|c| c.include) {
                    systems.extend(entries.into_iter().filter_map(|c| c.system));
                }
                let mut systems = systems.into_iter();
                let (Some(system), None) = (systems.next(), systems.next()) else {
                    return Ok(None);
                };
                system
            };
            match self
                .valueset_loader
                .contains_code(url, &system, code)
                .map_err(error)?
            {
                CodeInValueSetResult::Found => return Ok(Some(true)),
                CodeInValueSetResult::NotFound => (),
                CodeInValueSetResult::ValueSetNotFound => {
                    if let Some(tx) = self
                        .terminology_service
                        .as_deref()
                        .filter(|tx| tx.supports_value_set(url))
                    {
                        if tx
                            .validate_code_in_valueset(url, &system, code, None)
                            .map_err(error)?
                            .result
                        {
                            return Ok(Some(true));
                        }
                    } else {
                        unknown = true;
                    }
                }
            }
        }
        Ok(if unknown { None } else { Some(false) })
    }

    fn subsumes(&self, a: &Value, b: &Value) -> FhirPathResult<Option<bool>> {
        let (Some(system), Some(parent), Some(child)) =
            (a["system"].as_str(), a["code"].as_str(), b["code"].as_str())
        else {
            return Ok(None);
        };
        if b["system"].as_str() != Some(system) {
            return Ok(None);
        }
        let systems = self.codesystems.read().map_err(error)?;
        let Some(cs) = systems.get(system) else {
            return Ok(None);
        };
        if cs["hierarchyMeaning"] != "is-a" || cs["content"] != "complete" {
            return Ok(None);
        }
        for c in [a, b] {
            if let Some(version) = c["version"].as_str() {
                if cs["version"].as_str() != Some(version) {
                    return Ok(None);
                }
            }
        }
        fn find<'a>(node: &'a Value, code: &str) -> Option<&'a Value> {
            for c in node["concept"].as_array()? {
                if c["code"] == code {
                    return Some(c);
                }
                if let Some(n) = find(c, code) {
                    return Some(n);
                }
            }
            None
        }
        Ok(find(cs, parent).map(|p| parent == child || find(p, child).is_some()))
    }

    fn element_definition(&self, path: &str) -> FhirPathResult<Option<Value>> {
        let ty = path.split('.').next().unwrap_or(path);
        let url = format!("http://hl7.org/fhir/StructureDefinition/{ty}");
        let Some(sd) = self.profile_registry.get_snapshot(&url).map_err(error)? else {
            return Ok(None);
        };
        sd.snapshot
            .as_ref()
            .and_then(|s| s.element.iter().find(|e| e.path == path))
            .map(serde_json::to_value)
            .transpose()
            .map_err(error)
    }

    fn matches_slice(
        &self,
        element: &Value,
        source_paths: &[&str],
        url: &str,
        name: &str,
    ) -> FhirPathResult<Option<bool>> {
        let Some(profile) = self.profile_registry.get_snapshot(url).map_err(error)? else {
            return Ok(None);
        };
        let rules = self.rule_compiler.compile(&profile).map_err(error)?;
        let mut candidates = rules
            .slicing_rules
            .iter()
            .filter(|rule| source_paths.contains(&rule.path.as_str()))
            .flat_map(|rule| {
                rule.slices
                    .iter()
                    .filter(move |slice| slice.name == name)
                    .map(move |slice| (rule, slice))
            });
        let Some((rule, slice)) = candidates.next() else {
            return Ok(None);
        };
        if candidates.next().is_some() {
            return Ok(None);
        }
        Ok(Some(matches_slice(element, slice, &rule.discriminators)))
    }
}
