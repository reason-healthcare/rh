//! FSH Tank — holds all parsed FSH entities indexed by name

use crate::error::FshError;
use crate::parser::ast::*;
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

/// Container for all FSH entities, indexed by name
pub struct FshTank {
    pub profiles: IndexMap<String, Profile>,
    pub extensions: IndexMap<String, Extension>,
    pub logicals: IndexMap<String, Logical>,
    pub resources: IndexMap<String, ResourceDef>,
    pub instances: IndexMap<String, Instance>,
    pub value_sets: IndexMap<String, ValueSet>,
    pub code_systems: IndexMap<String, CodeSystem>,
    pub invariants: IndexMap<String, Invariant>,
    pub mappings: IndexMap<String, Mapping>,
    pub rule_sets: IndexMap<String, RuleSet>,
    pub param_rule_sets: IndexMap<String, ParamRuleSet>,
    pub aliases: HashMap<String, String>,
    /// All known names (for duplicate detection)
    known_names: HashSet<String>,
}

/// A reference to any FSH entity by name
pub enum FshEntityRef<'a> {
    Profile(&'a Profile),
    Extension(&'a Extension),
    Logical(&'a Logical),
    Resource(&'a ResourceDef),
    Instance(&'a Instance),
    ValueSet(&'a ValueSet),
    CodeSystem(&'a CodeSystem),
    Invariant(&'a Invariant),
    Mapping(&'a Mapping),
    RuleSet(&'a RuleSet),
    ParamRuleSet(&'a ParamRuleSet),
}

impl FshTank {
    pub fn new() -> Self {
        Self {
            profiles: IndexMap::new(),
            extensions: IndexMap::new(),
            logicals: IndexMap::new(),
            resources: IndexMap::new(),
            instances: IndexMap::new(),
            value_sets: IndexMap::new(),
            code_systems: IndexMap::new(),
            invariants: IndexMap::new(),
            mappings: IndexMap::new(),
            rule_sets: IndexMap::new(),
            param_rule_sets: IndexMap::new(),
            aliases: HashMap::new(),
            known_names: HashSet::new(),
        }
    }

    /// Add all entities from a parsed document.
    /// Collects ALL duplicate errors before returning.
    pub fn add_document(&mut self, doc: FshDocument) -> Result<(), Vec<FshError>> {
        let mut errors = Vec::new();

        for spanned in doc.entities {
            let loc = spanned.location;
            match spanned.value {
                FshEntity::Profile(p) => {
                    let name = p.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.profiles.insert(name, p);
                    }
                }
                FshEntity::Extension(e) => {
                    let name = e.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.extensions.insert(name, e);
                    }
                }
                FshEntity::Logical(l) => {
                    let name = l.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.logicals.insert(name, l);
                    }
                }
                FshEntity::Resource(r) => {
                    let name = r.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.resources.insert(name, r);
                    }
                }
                FshEntity::Instance(i) => {
                    let name = i.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.instances.insert(name, i);
                    }
                }
                FshEntity::ValueSet(vs) => {
                    let name = vs.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.value_sets.insert(name, vs);
                    }
                }
                FshEntity::CodeSystem(cs) => {
                    let name = cs.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.code_systems.insert(name, cs);
                    }
                }
                FshEntity::Invariant(inv) => {
                    let name = inv.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.invariants.insert(name, inv);
                    }
                }
                FshEntity::Mapping(m) => {
                    let name = m.metadata.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.mappings.insert(name, m);
                    }
                }
                FshEntity::RuleSet(rs) => {
                    let name = rs.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.rule_sets.insert(name, rs);
                    }
                }
                FshEntity::ParamRuleSet(prs) => {
                    let name = prs.name.clone();
                    if self.known_names.contains(&name) {
                        errors.push(FshError::DuplicateEntity {
                            name,
                            location: loc,
                        });
                    } else {
                        self.known_names.insert(name.clone());
                        self.param_rule_sets.insert(name, prs);
                    }
                }
                FshEntity::Alias(a) => {
                    self.aliases.insert(a.name, a.value);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Look up any entity by name across all maps
    pub fn fish(&self, name: &str) -> Option<FshEntityRef<'_>> {
        if let Some(p) = self.profiles.get(name) {
            return Some(FshEntityRef::Profile(p));
        }
        if let Some(e) = self.extensions.get(name) {
            return Some(FshEntityRef::Extension(e));
        }
        if let Some(l) = self.logicals.get(name) {
            return Some(FshEntityRef::Logical(l));
        }
        if let Some(r) = self.resources.get(name) {
            return Some(FshEntityRef::Resource(r));
        }
        if let Some(i) = self.instances.get(name) {
            return Some(FshEntityRef::Instance(i));
        }
        if let Some(vs) = self.value_sets.get(name) {
            return Some(FshEntityRef::ValueSet(vs));
        }
        if let Some(cs) = self.code_systems.get(name) {
            return Some(FshEntityRef::CodeSystem(cs));
        }
        if let Some(inv) = self.invariants.get(name) {
            return Some(FshEntityRef::Invariant(inv));
        }
        if let Some(m) = self.mappings.get(name) {
            return Some(FshEntityRef::Mapping(m));
        }
        if let Some(rs) = self.rule_sets.get(name) {
            return Some(FshEntityRef::RuleSet(rs));
        }
        if let Some(prs) = self.param_rule_sets.get(name) {
            return Some(FshEntityRef::ParamRuleSet(prs));
        }
        None
    }
}

impl Default for FshTank {
    fn default() -> Self {
        Self::new()
    }
}
