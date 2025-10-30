use anyhow::Result;
use lru::LruCache;
use rh_snapshot::StructureDefinition;
use std::num::NonZeroUsize;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct CompiledValidationRules {
    pub profile_url: String,
    pub cardinality_rules: Vec<CardinalityRule>,
    pub type_rules: Vec<TypeRule>,
    pub binding_rules: Vec<BindingRule>,
    pub invariant_rules: Vec<InvariantRule>,
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

pub struct RuleCompiler {
    cache: Mutex<LruCache<String, CompiledValidationRules>>,
}

impl RuleCompiler {
    pub fn new(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity).expect("Capacity must be non-zero");
        Self {
            cache: Mutex::new(LruCache::new(capacity)),
        }
    }

    pub fn compile(&self, snapshot: &StructureDefinition) -> Result<CompiledValidationRules> {
        let profile_url = snapshot.url.clone();

        if let Some(cached) = self.cache.lock().unwrap().get(&profile_url) {
            return Ok(cached.clone());
        }

        let mut cardinality_rules = Vec::new();
        let mut type_rules = Vec::new();
        let mut binding_rules = Vec::new();
        let mut invariant_rules = Vec::new();

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
            }
        }

        let rules = CompiledValidationRules {
            profile_url: profile_url.clone(),
            cardinality_rules,
            type_rules,
            binding_rules,
            invariant_rules,
        };

        self.cache.lock().unwrap().put(profile_url, rules.clone());

        Ok(rules)
    }

    pub fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().unwrap();
        (cache.len(), cache.cap().get())
    }
}

impl Default for RuleCompiler {
    fn default() -> Self {
        Self::new(100)
    }
}
