use anyhow::Result;
use lru::LruCache;
use rh_foundation::snapshot::StructureDefinition;
use std::num::NonZeroUsize;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct CompiledValidationRules {
    pub profile_url: String,
    pub cardinality_rules: Vec<CardinalityRule>,
    pub type_rules: Vec<TypeRule>,
    pub binding_rules: Vec<BindingRule>,
    pub invariant_rules: Vec<InvariantRule>,
    pub extension_rules: Vec<ExtensionRule>,
    pub slicing_rules: Vec<SlicingRule>,
}

#[derive(Debug, Clone)]
pub struct CardinalityRule {
    pub path: String,
    pub min: Option<u32>,
    pub max: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeRule {
    pub path: String,
    pub types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BindingRule {
    pub path: String,
    pub value_set_url: String,
    pub strength: String,
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
        let mut binding_rules = Vec::new();
        let mut invariant_rules = Vec::new();
        let mut extension_rules = Vec::new();

        if let Some(snapshot_data) = &snapshot.snapshot {
            for element in &snapshot_data.element {
                let path = &element.path;

                if let (Some(min), Some(max)) = (element.min, &element.max) {
                    cardinality_rules.push(CardinalityRule {
                        path: path.clone(),
                        min: Some(min),
                        max: Some(max.clone()),
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
                }

                if let Some(binding) = &element.binding {
                    if let Some(vs_url) = &binding.value_set {
                        binding_rules.push(BindingRule {
                            path: path.clone(),
                            value_set_url: vs_url.clone(),
                            strength: binding.strength.clone(),
                        });
                    }
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
                                slices.push(SliceDefinition {
                                    name: slice_name.clone(),
                                    min: slice_element.min.unwrap_or(0),
                                    max: slice_element
                                        .max
                                        .clone()
                                        .unwrap_or_else(|| "*".to_string()),
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
                cardinality_rules,
                type_rules,
                binding_rules,
                invariant_rules,
                extension_rules,
                slicing_rules,
            };

            self.cache.lock().unwrap().put(profile_url, rules.clone());

            return Ok(rules);
        }

        let rules = CompiledValidationRules {
            profile_url: profile_url.clone(),
            cardinality_rules,
            type_rules,
            binding_rules,
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

impl Default for RuleCompiler {
    fn default() -> Self {
        Self::new(100)
    }
}
