use std::collections::HashMap;

use tracing::debug;

use crate::snapshot::error::{SnapshotError, SnapshotResult};
use crate::snapshot::merge_validation::{
    merge_binding, merge_cardinality, merge_constraints, merge_types,
};
use crate::snapshot::path::ElementPath;
use crate::snapshot::types::ElementDefinition;

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
            .map(|element| {
                (
                    (element.path.clone(), element.slice_name.clone()),
                    element.clone(),
                )
            })
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
        elements.sort_by(|left, right| match left.path.cmp(&right.path) {
            std::cmp::Ordering::Equal => match (&left.slice_name, &right.slice_name) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(left_name), Some(right_name)) => left_name.cmp(right_name),
            },
            other => other,
        });
    }

    fn expand_slice_children(
        element_map: &mut HashMap<(String, Option<String>), ElementDefinition>,
    ) {
        let mut new_elements = Vec::new();

        let base_elements: Vec<(&String, ElementPath, &ElementDefinition)> = element_map
            .iter()
            .filter(|((_, slice_name), _)| slice_name.is_none())
            .map(|((path, _), element)| (path, ElementPath::new(path), element))
            .collect();

        let slice_roots: Vec<(String, String)> = element_map
            .keys()
            .filter_map(|(path, slice_name)| {
                slice_name.as_ref().map(|name| (path.clone(), name.clone()))
            })
            .collect();

        for (slice_path_str, slice_name) in slice_roots {
            let slice_base_path = ElementPath::new(&slice_path_str);

            for (path, element_path, element) in &base_elements {
                if element_path.is_child_of(&slice_base_path) {
                    let slice_child_key = ((*path).clone(), Some(slice_name.clone()));

                    if !element_map.contains_key(&slice_child_key) {
                        let mut new_element = (*element).clone();
                        new_element.slice_name = Some(slice_name.clone());
                        new_elements.push((slice_child_key, new_element));
                    }
                }
            }
        }

        for (key, element) in new_elements {
            element_map.insert(key, element);
        }
    }

    fn merge_element(
        base: &ElementDefinition,
        diff: &ElementDefinition,
    ) -> Result<ElementDefinition, SnapshotError> {
        let (merged_min, merged_max) = merge_cardinality(
            base.min,
            base.max.as_deref(),
            diff.min,
            diff.max.as_deref(),
            &diff.path,
        )?;

        let merged_type = merge_types(base.type_.as_ref(), diff.type_.as_ref(), &diff.path)?;
        let merged_binding =
            merge_binding(base.binding.as_ref(), diff.binding.as_ref(), &diff.path)?;
        let merged_constraints = merge_constraints(&base.constraint, &diff.constraint, &diff.path)?;

        Ok(ElementDefinition {
            path: diff.path.clone(),
            id: diff.id.clone().or_else(|| base.id.clone()),
            min: Some(merged_min),
            max: Some(merged_max),
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
            slice_name: slice_name.map(|slice| slice.to_string()),
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

        let base_root = create_element("Patient.identifier", None);
        let base_child = create_element("Patient.identifier.system", None);

        map.insert(("Patient.identifier".to_string(), None), base_root);
        map.insert(("Patient.identifier.system".to_string(), None), base_child);

        let slice_root = create_element("Patient.identifier", Some("MRN"));
        map.insert(
            ("Patient.identifier".to_string(), Some("MRN".to_string())),
            slice_root,
        );

        ElementMerger::expand_slice_children(&mut map);

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
