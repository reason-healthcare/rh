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

        let mut element_map: HashMap<(String, Option<String>), ElementDefinition> = base
            .iter()
            .map(|e| ((e.path.clone(), e.slice_name.clone()), e.clone()))
            .collect();

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

        Self::expand_slice_children(&mut element_map);

        let mut result: Vec<ElementDefinition> = element_map.into_values().collect();
        result.sort_by(|a, b| match a.path.cmp(&b.path) {
            std::cmp::Ordering::Equal => match (&a.slice_name, &b.slice_name) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(a_name), Some(b_name)) => a_name.cmp(b_name),
            },
            other => other,
        });

        debug!("Merge complete: {} elements in result", result.len());
        Ok(result)
    }

    fn expand_slice_children(
        element_map: &mut HashMap<(String, Option<String>), ElementDefinition>,
    ) {
        let mut new_elements = Vec::new();

        let slices: Vec<_> = element_map
            .iter()
            .filter(|((_path, slice_name), _elem)| slice_name.is_some())
            .map(|((path, slice_name), elem)| (path.clone(), slice_name.clone(), elem.clone()))
            .collect();

        for (slice_path, slice_name, _slice_elem) in slices {
            let slice_name = slice_name.expect("slice_name should be Some");

            let children: Vec<_> = element_map
                .iter()
                .filter(|((_p, sn), elem)| {
                    sn.is_none() && {
                        let elem_path = ElementPath::new(&elem.path);
                        let slice_base_path = ElementPath::new(&slice_path);
                        elem_path.is_child_of(&slice_base_path)
                    }
                })
                .map(|((path, _), elem)| (path.clone(), elem.clone()))
                .collect();

            for (child_path, mut child_elem) in children {
                let slice_child_key = (child_path.clone(), Some(slice_name.clone()));
                if !element_map.contains_key(&slice_child_key) {
                    child_elem.slice_name = Some(slice_name.clone());
                    new_elements.push((slice_child_key, child_elem));
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
