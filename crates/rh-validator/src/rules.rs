use anyhow::Result;
use lru::LruCache;
use rh_foundation::snapshot::{ElementDefinition, StructureDefinition};
use serde_json::Value;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct CompiledValidationRules {
    pub profile_url: String,
    pub element_paths: Vec<String>,
    pub cardinality_rules: Vec<CardinalityRule>,
    pub type_rules: Vec<TypeRule>,
    pub reference_target_rules: Vec<ReferenceTargetRule>,
    pub binding_rules: Vec<BindingRule>,
    pub fixed_pattern_rules: Vec<FixedPatternRule>,
    pub invariant_rules: Vec<InvariantRule>,
    pub extension_rules: Vec<ExtensionRule>,
    pub slicing_rules: Vec<SlicingRule>,
}

#[derive(Debug, Clone)]
pub struct CardinalityRule {
    pub path: String,
    pub min: Option<u32>,
    pub max: Option<String>,
    pub slice_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeRule {
    pub path: String,
    pub types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReferenceTargetRule {
    pub path: String,
    pub target_profiles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BindingRule {
    pub path: String,
    pub value_set_url: String,
    pub strength: String,
    pub slice_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FixedPatternRule {
    pub path: String,
    pub value: Value,
    pub is_fixed: bool,
    pub slice_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InvariantRule {
    pub path: String,
    pub key: String,
    pub severity: String,
    pub human: String,
    pub expression: String,
}

#[derive(Debug, Clone)]
pub struct ExtensionRule {
    pub path: String,
    pub slice_name: String,
    pub url: String,
    pub min: u32,
    pub max: String,
}

#[derive(Debug, Clone)]
pub struct SlicingRule {
    pub path: String,
    pub discriminators: Vec<Discriminator>,
    pub rules: String,
    pub ordered: bool,
    pub slices: Vec<SliceDefinition>,
}

#[derive(Debug, Clone)]
pub struct Discriminator {
    pub type_: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct SliceDefinition {
    pub name: String,
    pub min: u32,
    pub max: String,
    pub discriminator_constraints: Vec<DiscriminatorConstraint>,
    pub target_profiles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DiscriminatorConstraint {
    pub path: String,
    pub value: Value,
}

pub struct RuleCompiler {
    cache: Mutex<LruCache<String, CompiledValidationRules>>,
    cache_hits: Mutex<usize>,
    cache_misses: Mutex<usize>,
}

impl RuleCompiler {
    pub fn new(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity).expect("Capacity must be non-zero");
        Self {
            cache: Mutex::new(LruCache::new(capacity)),
            cache_hits: Mutex::new(0),
            cache_misses: Mutex::new(0),
        }
    }

    pub fn compile(&self, snapshot: &StructureDefinition) -> Result<CompiledValidationRules> {
        let profile_url = snapshot.url.clone();

        if let Some(cached) = self.cache.lock().unwrap().get(&profile_url) {
            *self.cache_hits.lock().unwrap() += 1;
            return Ok(cached.clone());
        }

        *self.cache_misses.lock().unwrap() += 1;

        let mut cardinality_rules = Vec::new();
        let mut type_rules = Vec::new();
        let mut reference_target_rules = Vec::new();
        let mut binding_rules = Vec::new();
        let mut fixed_pattern_rules = Vec::new();
        let mut invariant_rules = Vec::new();
        let mut extension_rules = Vec::new();
        let mut element_paths = Vec::new();

        if let Some(snapshot_data) = &snapshot.snapshot {
            for element in &snapshot_data.element {
                let path = &element.path;
                element_paths.push(path.clone());

                if element.min.is_some() || element.max.is_some() {
                    cardinality_rules.push(CardinalityRule {
                        path: path.clone(),
                        min: element.min,
                        max: element.max.clone(),
                        slice_name: effective_slice_name(element),
                    });
                }

                if let Some(types) = &element.type_ {
                    let type_codes: Vec<String> = types.iter().map(|t| t.code.clone()).collect();
                    if !type_codes.is_empty() {
                        type_rules.push(TypeRule {
                            path: path.clone(),
                            types: type_codes,
                        });
                    }

                    let target_profiles: Vec<String> = types
                        .iter()
                        .filter(|type_def| type_def.code == "Reference")
                        .filter_map(|type_def| type_def.target_profile.as_ref())
                        .flatten()
                        .cloned()
                        .collect();
                    if !target_profiles.is_empty() {
                        reference_target_rules.push(ReferenceTargetRule {
                            path: path.clone(),
                            target_profiles,
                        });
                    }
                }

                if let Some(binding) = &element.binding {
                    if let Some(vs_url) = &binding.value_set {
                        binding_rules.push(BindingRule {
                            path: path.clone(),
                            value_set_url: vs_url.clone(),
                            strength: binding.strength.clone(),
                            slice_name: effective_slice_name(element),
                        });
                    }
                }

                for (key, value) in fixed_or_pattern_entries(&element.additional) {
                    fixed_pattern_rules.push(FixedPatternRule {
                        path: path.clone(),
                        value: value.clone(),
                        is_fixed: key.starts_with("fixed"),
                        slice_name: effective_slice_name(element),
                    });
                }

                if let Some(constraints) = &element.constraint {
                    for constraint in constraints {
                        if let Some(expression) = &constraint.expression {
                            invariant_rules.push(InvariantRule {
                                path: path.clone(),
                                key: constraint.key.clone(),
                                severity: constraint.severity.clone(),
                                human: constraint.human.clone(),
                                expression: expression.clone(),
                            });
                        }
                    }
                }

                if path.ends_with(".extension") || path.ends_with(".modifierExtension") {
                    if let Some(slice_name) = &element.slice_name {
                        if let Some(types) = &element.type_ {
                            for type_def in types {
                                if type_def.code == "Extension" {
                                    if let Some(profiles) = &type_def.profile {
                                        for profile_url in profiles {
                                            extension_rules.push(ExtensionRule {
                                                path: path.clone(),
                                                slice_name: slice_name.clone(),
                                                url: profile_url.clone(),
                                                min: element.min.unwrap_or(0),
                                                max: element
                                                    .max
                                                    .clone()
                                                    .unwrap_or_else(|| "*".to_string()),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            add_differential_binding_rules(snapshot, &mut binding_rules);

            let mut slicing_rules = Vec::new();
            for element in &snapshot_data.element {
                if let Some(slicing) = &element.slicing {
                    let discriminators = if let Some(discs) = &slicing.discriminator {
                        discs
                            .iter()
                            .map(|d| Discriminator {
                                type_: d.type_.clone(),
                                path: d.path.clone(),
                            })
                            .collect()
                    } else {
                        Vec::new()
                    };

                    let mut slices = Vec::new();
                    for slice_element in &snapshot_data.element {
                        if slice_element.path == element.path {
                            if let Some(slice_name) = &slice_element.slice_name {
                                let discriminator_constraints = collect_discriminator_constraints(
                                    &snapshot_data.element,
                                    element,
                                    slice_name,
                                    &discriminators,
                                );
                                let target_profiles = collect_profile_constraints(
                                    &snapshot_data.element,
                                    element,
                                    slice_name,
                                    &discriminators,
                                );
                                slices.push(SliceDefinition {
                                    name: slice_name.clone(),
                                    min: slice_element.min.unwrap_or(0),
                                    max: slice_element
                                        .max
                                        .clone()
                                        .unwrap_or_else(|| "*".to_string()),
                                    discriminator_constraints,
                                    target_profiles,
                                });
                            }
                        }
                    }

                    if !slices.is_empty() {
                        slicing_rules.push(SlicingRule {
                            path: element.path.clone(),
                            discriminators,
                            rules: slicing.rules.clone().unwrap_or_else(|| "open".to_string()),
                            ordered: slicing.ordered.unwrap_or(false),
                            slices,
                        });
                    }
                }
            }

            let rules = CompiledValidationRules {
                profile_url: profile_url.clone(),
                element_paths,
                cardinality_rules,
                type_rules,
                reference_target_rules,
                binding_rules,
                fixed_pattern_rules,
                invariant_rules,
                extension_rules,
                slicing_rules,
            };

            self.cache.lock().unwrap().put(profile_url, rules.clone());

            return Ok(rules);
        }

        add_differential_rules(snapshot, &mut cardinality_rules, &mut binding_rules);

        let rules = CompiledValidationRules {
            profile_url: profile_url.clone(),
            element_paths,
            cardinality_rules,
            type_rules,
            reference_target_rules,
            binding_rules,
            fixed_pattern_rules,
            invariant_rules,
            extension_rules,
            slicing_rules: Vec::new(),
        };

        self.cache.lock().unwrap().put(profile_url, rules.clone());

        Ok(rules)
    }

    pub fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().unwrap();
        (cache.len(), cache.cap().get())
    }

    pub fn cache_metrics(&self) -> (usize, usize, f64) {
        let hits = *self.cache_hits.lock().unwrap();
        let misses = *self.cache_misses.lock().unwrap();
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        (hits, misses, hit_rate)
    }

    pub fn reset_cache_metrics(&self) {
        *self.cache_hits.lock().unwrap() = 0;
        *self.cache_misses.lock().unwrap() = 0;
    }
}

fn add_differential_binding_rules(
    structure_definition: &StructureDefinition,
    binding_rules: &mut Vec<BindingRule>,
) {
    let Some(differential) = &structure_definition.differential else {
        return;
    };

    for element in &differential.element {
        let Some(binding) = &element.binding else {
            continue;
        };
        let Some(value_set_url) = &binding.value_set else {
            continue;
        };

        if binding_rules.iter().any(|rule| {
            rule.path == element.path
                && rule.value_set_url == *value_set_url
                && rule.strength == binding.strength
        }) {
            continue;
        }

        binding_rules.push(BindingRule {
            path: element.path.clone(),
            value_set_url: value_set_url.clone(),
            strength: binding.strength.clone(),
            slice_name: effective_slice_name(element),
        });
    }
}

fn add_differential_rules(
    structure_definition: &StructureDefinition,
    cardinality_rules: &mut Vec<CardinalityRule>,
    binding_rules: &mut Vec<BindingRule>,
) {
    let Some(differential) = &structure_definition.differential else {
        return;
    };

    for element in &differential.element {
        if element.path.split('.').count() == 2 && (element.min.is_some() || element.max.is_some())
        {
            cardinality_rules.push(CardinalityRule {
                path: element.path.clone(),
                min: element.min,
                max: element.max.clone(),
                slice_name: effective_slice_name(element),
            });
        }

        if let Some(binding) = &element.binding {
            if let Some(value_set_url) = &binding.value_set {
                binding_rules.push(BindingRule {
                    path: element.path.clone(),
                    value_set_url: value_set_url.clone(),
                    strength: binding.strength.clone(),
                    slice_name: effective_slice_name(element),
                });
            }
        }
    }
}

fn effective_slice_name(element: &ElementDefinition) -> Option<String> {
    if let Some(slice_name) = &element.slice_name {
        return Some(slice_name.clone());
    }

    let id = element.id.as_deref()?;
    id.split('.').find_map(|segment| {
        segment
            .split_once(':')
            .map(|(_, slice)| slice.to_string())
            .filter(|slice| !slice.is_empty())
    })
}

fn collect_discriminator_constraints(
    elements: &[ElementDefinition],
    slicing_root: &ElementDefinition,
    slice_name: &str,
    discriminators: &[Discriminator],
) -> Vec<DiscriminatorConstraint> {
    let mut constraints = Vec::new();

    for element in elements {
        if element.slice_name.as_deref() != Some(slice_name) {
            continue;
        }

        let Some(relative_path) = relative_slice_path(&element.path, &slicing_root.path) else {
            continue;
        };

        for discriminator in discriminators {
            if !path_matches_discriminator(relative_path, &discriminator.path) {
                continue;
            }

            for value in fixed_or_pattern_values(&element.additional) {
                constraints.push(DiscriminatorConstraint {
                    path: relative_path.to_string(),
                    value: value.clone(),
                });
            }
        }
    }

    constraints
}

fn collect_profile_constraints(
    elements: &[ElementDefinition],
    slicing_root: &ElementDefinition,
    slice_name: &str,
    discriminators: &[Discriminator],
) -> Vec<String> {
    let mut profiles = Vec::new();

    for element in elements {
        if element.slice_name.as_deref() != Some(slice_name) {
            continue;
        }

        let Some(relative_path) = relative_slice_path(&element.path, &slicing_root.path) else {
            continue;
        };

        let is_profile_discriminator_path = discriminators.iter().any(|discriminator| {
            discriminator.type_ == "profile"
                && path_matches_discriminator(relative_path, &discriminator.path)
        });
        if !is_profile_discriminator_path {
            continue;
        }

        if let Some(types) = &element.type_ {
            for type_def in types {
                if let Some(type_profiles) = &type_def.profile {
                    profiles.extend(type_profiles.iter().cloned());
                }
                if let Some(type_profiles) = &type_def.target_profile {
                    profiles.extend(type_profiles.iter().cloned());
                }
            }
        }
    }

    profiles.sort();
    profiles.dedup();
    profiles
}

fn relative_slice_path<'a>(element_path: &'a str, slicing_path: &str) -> Option<&'a str> {
    if element_path == slicing_path {
        return Some("$this");
    }

    element_path
        .strip_prefix(slicing_path)
        .and_then(|suffix| suffix.strip_prefix('.'))
}

fn path_matches_discriminator(element_relative_path: &str, discriminator_path: &str) -> bool {
    (discriminator_path == "$this" && element_relative_path == "$this")
        || element_relative_path == discriminator_path
        || discriminator_path.starts_with(&format!("{element_relative_path}."))
        || element_relative_path.starts_with(&format!("{discriminator_path}."))
}

fn fixed_or_pattern_values(additional: &HashMap<String, Value>) -> impl Iterator<Item = &Value> {
    fixed_or_pattern_entries(additional).map(|(_, value)| value)
}

fn fixed_or_pattern_entries(
    additional: &HashMap<String, Value>,
) -> impl Iterator<Item = (&String, &Value)> {
    additional
        .iter()
        .filter(|(key, _)| key.starts_with("fixed") || key.starts_with("pattern"))
}

impl Default for RuleCompiler {
    fn default() -> Self {
        Self::new(100)
    }
}
