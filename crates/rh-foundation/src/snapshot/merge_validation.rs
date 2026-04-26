use std::collections::HashMap;

use tracing::trace;

use crate::snapshot::error::SnapshotError;
use crate::snapshot::types::{ElementBinding, ElementConstraint, ElementType};

pub(super) fn merge_cardinality(
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

    let base_max_numeric = parse_max_cardinality(base_max);
    let diff_max_numeric = parse_max_cardinality(diff_max);

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

pub(super) fn merge_types(
    base_types: Option<&Vec<ElementType>>,
    diff_types: Option<&Vec<ElementType>>,
    path: &str,
) -> Result<Option<Vec<ElementType>>, SnapshotError> {
    match (base_types, diff_types) {
        (Some(base), Some(diff)) => {
            for diff_type in diff {
                if !base
                    .iter()
                    .any(|base_type| base_type.code == diff_type.code)
                {
                    return Err(SnapshotError::MergeError(format!(
                        "Invalid type restriction for {path}: differential type '{}' is not in base types",
                        diff_type.code
                    )));
                }
            }

            trace!(
                "Merged types for {path}: {} differential types (restricted from {} base types)",
                diff.len(),
                base.len()
            );
            Ok(Some(diff.clone()))
        }
        (Some(base), None) => Ok(Some(base.clone())),
        (None, Some(diff)) => Ok(Some(diff.clone())),
        (None, None) => Ok(None),
    }
}

pub(super) fn merge_binding(
    base_binding: Option<&ElementBinding>,
    diff_binding: Option<&ElementBinding>,
    path: &str,
) -> Result<Option<ElementBinding>, SnapshotError> {
    match (base_binding, diff_binding) {
        (Some(base), Some(diff)) => {
            let base_strength = parse_binding_strength(&base.strength);
            let diff_strength = parse_binding_strength(&diff.strength);

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

pub(super) fn merge_constraints(
    base: &Option<Vec<ElementConstraint>>,
    diff: &Option<Vec<ElementConstraint>>,
    path: &str,
) -> Result<Option<Vec<ElementConstraint>>, SnapshotError> {
    match (base, diff) {
        (Some(base_constraints), Some(diff_constraints)) => {
            let mut merged = base_constraints.clone();
            let mut seen_keys: HashMap<String, String> = base_constraints
                .iter()
                .map(|constraint| {
                    (
                        constraint.key.clone(),
                        constraint.expression.clone().unwrap_or_default(),
                    )
                })
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

            trace!(
                "Merged constraints for {path}: {} total constraints (from {} base + {} differential)",
                merged.len(),
                base_constraints.len(),
                diff_constraints.len()
            );
            Ok(Some(merged))
        }
        (Some(base_constraints), None) => Ok(Some(base_constraints.clone())),
        (None, Some(diff_constraints)) => Ok(Some(diff_constraints.clone())),
        (None, None) => Ok(None),
    }
}

fn parse_max_cardinality(max: &str) -> Option<u32> {
    if max == "*" {
        None
    } else {
        max.parse::<u32>().ok()
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
