use crate::snapshot::error::{SnapshotError, SnapshotResult};
use crate::snapshot::path::ElementPath;
use crate::snapshot::types::ElementDefinition;
use std::collections::HashMap;
use tracing::{debug, trace};

pub struct ElementMerger;

impl ElementMerger {
    pub fn merge_elements(
        base: &[ElementDefinition],
        differential: &[ElementDefinition],
    ) -> SnapshotResult<Vec<ElementDefinition>> {
        debug!(
            "Merging {} base elements with {} differential elements",
            base.len(),
            differential.len()
        );

        let mut element_map = Self::create_element_map(base);
        Self::apply_differential(&mut element_map, differential)?;
        Self::expand_slice_children(&mut element_map);

        let mut result: Vec<ElementDefinition> = element_map.into_values().collect();
        Self::sort_elements_by_path(&mut result);

        debug!("Merge complete: {} elements in result", result.len());
        Ok(result)
    }

    fn create_element_map(
        base: &[ElementDefinition],
    ) -> HashMap<(String, Option<String>), ElementDefinition> {
        base.iter()
            .map(|e| ((e.path.clone(), e.slice_name.clone()), e.clone()))
            .collect()
    }

    fn apply_differential(
        element_map: &mut HashMap<(String, Option<String>), ElementDefinition>,
        differential: &[ElementDefinition],
    ) -> SnapshotResult<()> {
        for diff_element in differential {
            let key = (diff_element.path.clone(), diff_element.slice_name.clone());

            if let Some(existing) = element_map.get(&key) {
                let merged = Self::merge_element(existing, diff_element)?;
                element_map.insert(key, merged);
            } else if diff_element.slice_name.is_some() {
                let base_key = (diff_element.path.clone(), None);
                if let Some(base_element) = element_map.get(&base_key) {
                    let merged = Self::merge_element(base_element, diff_element)?;
                    element_map.insert(key, merged);
                } else {
                    element_map.insert(key, diff_element.clone());
                }
            } else {
                element_map.insert(key, diff_element.clone());
            }
        }
        Ok(())
    }

    fn sort_elements_by_path(elements: &mut [ElementDefinition]) {
        elements.sort_by(|a, b| match a.path.cmp(&b.path) {
            std::cmp::Ordering::Equal => match (&a.slice_name, &b.slice_name) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(a_name), Some(b_name)) => a_name.cmp(b_name),
            },
            other => other,
        });
    }

    fn expand_slice_children(
        element_map: &mut HashMap<(String, Option<String>), ElementDefinition>,
    ) {
        let mut new_elements = Vec::new();

        // Cache ElementPaths for base elements to avoid repeated parsing
        let base_elements: Vec<(&String, ElementPath, &ElementDefinition)> = element_map
            .iter()
            .filter(|((_, sn), _)| sn.is_none())
            .map(|((path, _), elem)| (path, ElementPath::new(path), elem))
            .collect();

        // Collect slice roots
        let slice_roots: Vec<(String, String)> = element_map
            .keys()
            .filter_map(|(path, slice_name)| {
                slice_name.as_ref().map(|name| (path.clone(), name.clone()))
            })
            .collect();

        for (slice_path_str, slice_name) in slice_roots {
            let slice_base_path = ElementPath::new(&slice_path_str);

            for (path, elem_path, elem) in &base_elements {
                if elem_path.is_child_of(&slice_base_path) {
                    let slice_child_key = ((*path).clone(), Some(slice_name.clone()));

                    if !element_map.contains_key(&slice_child_key) {
                        let mut new_elem = (*elem).clone();
                        new_elem.slice_name = Some(slice_name.clone());
                        new_elements.push((slice_child_key, new_elem));
                    }
                }
            }
        }

        for (key, elem) in new_elements {
            element_map.insert(key, elem);
        }
    }

    fn merge_element(
        base: &ElementDefinition,
        diff: &ElementDefinition,
    ) -> Result<ElementDefinition, SnapshotError> {
        let (merged_min, merged_max) = Self::merge_cardinality(
            base.min,
            base.max.as_deref(),
            diff.min,
            diff.max.as_deref(),
            &diff.path,
        )?;

        let merged_type = Self::merge_types(base.type_.as_ref(), diff.type_.as_ref(), &diff.path)?;

        let merged_binding =
            Self::merge_binding(base.binding.as_ref(), diff.binding.as_ref(), &diff.path)?;

        let merged_constraints =
            Self::merge_constraints(&base.constraint, &diff.constraint, &diff.path)?;

        Ok(ElementDefinition {
            path: diff.path.clone(),
            id: diff.id.clone().or_else(|| base.id.clone()),
            min: Some(merged_min),
            max: Some(merged_max.to_string()),
            type_: merged_type,
            binding: merged_binding,
            constraint: merged_constraints,
            definition: diff.definition.clone().or_else(|| base.definition.clone()),
            short: diff.short.clone().or_else(|| base.short.clone()),
            comment: diff.comment.clone().or_else(|| base.comment.clone()),
            requirements: diff
                .requirements
                .clone()
                .or_else(|| base.requirements.clone()),
            must_support: diff.must_support.or(base.must_support),
            is_summary: diff.is_summary.or(base.is_summary),
            is_modifier: diff.is_modifier.or(base.is_modifier),
            is_modifier_reason: diff
                .is_modifier_reason
                .clone()
                .or_else(|| base.is_modifier_reason.clone()),
            slicing: diff.slicing.clone().or_else(|| base.slicing.clone()),
            slice_name: diff.slice_name.clone().or_else(|| base.slice_name.clone()),
        })
    }

    fn merge_cardinality(
        base_min: Option<u32>,
        base_max: Option<&str>,
        diff_min: Option<u32>,
        diff_max: Option<&str>,
        path: &str,
    ) -> Result<(u32, String), SnapshotError> {
        let base_min = base_min.unwrap_or(0);
        let base_max = base_max.unwrap_or("*");
        let diff_min = diff_min.unwrap_or(base_min);
        let diff_max = diff_max.unwrap_or(base_max);

        if diff_min < base_min {
            return Err(SnapshotError::MergeError(format!(
                "Invalid cardinality for {path}: differential min ({diff_min}) is less than base min ({base_min})"
            )));
        }

        let base_max_numeric = Self::parse_max_cardinality(base_max);
        let diff_max_numeric = Self::parse_max_cardinality(diff_max);

        match (base_max_numeric, diff_max_numeric) {
            (Some(base_max_val), Some(diff_max_val)) => {
                if diff_max_val > base_max_val {
                    return Err(SnapshotError::MergeError(format!(
                        "Invalid cardinality for {path}: differential max ({diff_max}) is greater than base max ({base_max})"
                    )));
                }
            }
            (Some(_), None) => {
                return Err(SnapshotError::MergeError(format!(
                    "Invalid cardinality for {path}: differential max ({diff_max}) is greater than base max ({base_max})"
                )));
            }
            (None, _) => {}
        }

        if let Some(diff_max_val) = diff_max_numeric {
            if diff_min > diff_max_val {
                return Err(SnapshotError::MergeError(format!(
                    "Invalid cardinality for {path}: min ({diff_min}) is greater than max ({diff_max})"
                )));
            }
        }

        trace!(
            "Merged cardinality for {}: {}..{} (from base {}..{}, diff {}..{})",
            path,
            diff_min,
            diff_max,
            base_min,
            base_max,
            diff_min,
            diff_max
        );

        Ok((diff_min, diff_max.to_string()))
    }

    fn parse_max_cardinality(max: &str) -> Option<u32> {
        if max == "*" {
            None
        } else {
            max.parse::<u32>().ok()
        }
    }

    fn merge_types(
        base_types: Option<&Vec<crate::snapshot::types::ElementType>>,
        diff_types: Option<&Vec<crate::snapshot::types::ElementType>>,
        path: &str,
    ) -> Result<Option<Vec<crate::snapshot::types::ElementType>>, SnapshotError> {
        match (base_types, diff_types) {
            (Some(base), Some(diff)) => {
                for diff_type in diff {
                    let is_valid = base.iter().any(|base_type| {
                        if base_type.code != diff_type.code {
                            return false;
                        }
                        true
                    });

                    if !is_valid {
                        return Err(SnapshotError::MergeError(format!(
                            "Invalid type restriction for {path}: differential type '{}' is not in base types",
                            diff_type.code
                        )));
                    }
                }

                trace!("Merged types for {path}: {} differential types (restricted from {} base types)", diff.len(), base.len());
                Ok(Some(diff.clone()))
            }
            (Some(base), None) => Ok(Some(base.clone())),
            (None, Some(diff)) => Ok(Some(diff.clone())),
            (None, None) => Ok(None),
        }
    }

    fn merge_binding(
        base_binding: Option<&crate::snapshot::types::ElementBinding>,
        diff_binding: Option<&crate::snapshot::types::ElementBinding>,
        path: &str,
    ) -> Result<Option<crate::snapshot::types::ElementBinding>, SnapshotError> {
        match (base_binding, diff_binding) {
            (Some(base), Some(diff)) => {
                let base_strength = Self::parse_binding_strength(&base.strength);
                let diff_strength = Self::parse_binding_strength(&diff.strength);

                if diff_strength < base_strength {
                    return Err(SnapshotError::MergeError(format!(
                        "Invalid binding for {path}: differential strength '{}' is weaker than base strength '{}'",
                        diff.strength, base.strength
                    )));
                }

                trace!(
                    "Merged binding for {path}: {} (from base {})",
                    diff.strength,
                    base.strength
                );
                Ok(Some(diff.clone()))
            }
            (Some(base), None) => Ok(Some(base.clone())),
            (None, Some(diff)) => Ok(Some(diff.clone())),
            (None, None) => Ok(None),
        }
    }

    fn parse_binding_strength(strength: &str) -> u8 {
        match strength.to_lowercase().as_str() {
            "example" => 0,
            "preferred" => 1,
            "extensible" => 2,
            "required" => 3,
            _ => 0,
        }
    }

    fn merge_constraints(
        base: &Option<Vec<crate::snapshot::types::ElementConstraint>>,
        diff: &Option<Vec<crate::snapshot::types::ElementConstraint>>,
        path: &str,
    ) -> Result<Option<Vec<crate::snapshot::types::ElementConstraint>>, SnapshotError> {
        match (base, diff) {
            (Some(base_constraints), Some(diff_constraints)) => {
                let mut merged = base_constraints.clone();
                let mut seen_keys: HashMap<String, String> = base_constraints
                    .iter()
                    .map(|c| (c.key.clone(), c.expression.clone().unwrap_or_default()))
                    .collect();

                for diff_constraint in diff_constraints {
                    if let Some(existing_expr) = seen_keys.get(&diff_constraint.key) {
                        let diff_expr = diff_constraint.expression.as_deref().unwrap_or("");
                        if existing_expr != diff_expr {
                            return Err(SnapshotError::MergeError(format!(
                                "Duplicate constraint key '{}' for {path} with different expressions",
                                diff_constraint.key
                            )));
                        }
                        continue;
                    }
                    seen_keys.insert(
                        diff_constraint.key.clone(),
                        diff_constraint.expression.clone().unwrap_or_default(),
                    );
                    merged.push(diff_constraint.clone());
                }

                trace!("Merged constraints for {path}: {} total constraints (from {} base + {} differential)", 
                    merged.len(), base_constraints.len(), diff_constraints.len());
                Ok(Some(merged))
            }
            (Some(base_constraints), None) => Ok(Some(base_constraints.clone())),
            (None, Some(diff_constraints)) => Ok(Some(diff_constraints.clone())),
            (None, None) => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::types::ElementDefinition;

    fn create_element(path: &str, slice_name: Option<&str>) -> ElementDefinition {
        ElementDefinition {
            path: path.to_string(),
            id: None,
            min: None,
            max: None,
            type_: None,
            binding: None,
            constraint: None,
            definition: None,
            short: None,
            comment: None,
            requirements: None,
            must_support: None,
            is_summary: None,
            is_modifier: None,
            is_modifier_reason: None,
            slicing: None,
            slice_name: slice_name.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_merge_elements_basic() {
        let base_elem = create_element("Patient", None);
        let diff_elem = create_element("Patient", None);

        let base = vec![base_elem];
        let diff = vec![diff_elem];

        let result = ElementMerger::merge_elements(&base, &diff).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "Patient");
    }

    #[test]
    fn test_expand_slice_children() {
        let mut map = HashMap::new();

        // Base elements
        let base_root = create_element("Patient.identifier", None);
        let base_child = create_element("Patient.identifier.system", None);

        map.insert(("Patient.identifier".to_string(), None), base_root);
        map.insert(("Patient.identifier.system".to_string(), None), base_child);

        // Slice root
        let slice_root = create_element("Patient.identifier", Some("MRN"));
        map.insert(
            ("Patient.identifier".to_string(), Some("MRN".to_string())),
            slice_root,
        );

        ElementMerger::expand_slice_children(&mut map);

        // Check if child was copied to slice
        assert!(map.contains_key(&(
            "Patient.identifier.system".to_string(),
            Some("MRN".to_string())
        )));

        let slice_child = map
            .get(&(
                "Patient.identifier.system".to_string(),
                Some("MRN".to_string()),
            ))
            .unwrap();
        assert_eq!(slice_child.slice_name.as_deref(), Some("MRN"));
    }
}
