//! FSH Resolver — alias expansion, RuleSet inlining, cycle detection

use crate::error::FshError;
use crate::parser::ast::*;
use crate::tank::FshTank;
use std::collections::{HashMap, HashSet};

pub struct FshResolver;

impl FshResolver {
    /// Run all resolution passes on the tank in place
    pub fn resolve(tank: &mut FshTank) -> Result<(), Vec<FshError>> {
        let mut errors = Vec::new();

        // Pass 1: alias expansion
        Self::expand_aliases(tank);

        // Pass 2 & 3: inline RuleSets with cycle detection
        if let Err(mut e) = Self::inline_rule_sets(tank) {
            errors.append(&mut e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // ========================================================================
    // Pass 1: Alias expansion
    // ========================================================================

    fn expand_aliases(tank: &mut FshTank) {
        let aliases = tank.aliases.clone();

        // Expand aliases in profiles
        for profile in tank.profiles.values_mut() {
            for rule in &mut profile.rules {
                expand_sd_rule(&mut rule.value, &aliases);
            }
        }
        // Extensions
        for ext in tank.extensions.values_mut() {
            for rule in &mut ext.rules {
                expand_sd_rule(&mut rule.value, &aliases);
            }
        }
        // Logicals
        for logical in tank.logicals.values_mut() {
            for rule in &mut logical.rules {
                expand_sd_rule(&mut rule.value, &aliases);
            }
        }
        // Resources
        for res in tank.resources.values_mut() {
            for rule in &mut res.rules {
                expand_sd_rule(&mut rule.value, &aliases);
            }
        }
        // Instances
        for inst in tank.instances.values_mut() {
            for rule in &mut inst.rules {
                expand_instance_rule(&mut rule.value, &aliases);
            }
        }
    }

    // ========================================================================
    // Pass 2 & 3: RuleSet inlining
    // ========================================================================

    fn inline_rule_sets(tank: &mut FshTank) -> Result<(), Vec<FshError>> {
        let mut errors = Vec::new();

        // Build a snapshot of rule sets for lookup (before mutation)
        let rule_sets: HashMap<String, Vec<Spanned<SdRule>>> = tank
            .rule_sets
            .iter()
            .map(|(k, v)| (k.clone(), v.rules.clone()))
            .collect();

        // Inline in profiles
        let profile_names: Vec<String> = tank.profiles.keys().cloned().collect();
        for name in profile_names {
            let rules = tank.profiles[&name].rules.clone();
            match inline_sd_rules(rules, &rule_sets, &mut HashSet::new()) {
                Ok(inlined) => tank.profiles.get_mut(&name).unwrap().rules = inlined,
                Err(e) => errors.push(e),
            }
        }

        // Inline in extensions
        let ext_names: Vec<String> = tank.extensions.keys().cloned().collect();
        for name in ext_names {
            let rules = tank.extensions[&name].rules.clone();
            match inline_sd_rules(rules, &rule_sets, &mut HashSet::new()) {
                Ok(inlined) => tank.extensions.get_mut(&name).unwrap().rules = inlined,
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn expand_value(value: &mut FshValue, aliases: &HashMap<String, String>) {
    match value {
        FshValue::Canonical(ref mut url) => {
            if let Some(resolved) = aliases.get(url.as_str()) {
                *url = resolved.clone();
            }
        }
        FshValue::Reference(ref mut r) => {
            if let Some(resolved) = aliases.get(r.as_str()) {
                *r = resolved.clone();
            }
        }
        _ => {}
    }
}

fn expand_sd_rule(rule: &mut SdRule, aliases: &HashMap<String, String>) {
    match rule {
        SdRule::Assignment(r) => expand_value(&mut r.value, aliases),
        SdRule::Binding(r) => {
            if let Some(resolved) = aliases.get(r.value_set.as_str()) {
                r.value_set = resolved.clone();
            }
        }
        SdRule::CaretValue(r) => expand_value(&mut r.value, aliases),
        _ => {}
    }
}

fn expand_instance_rule(rule: &mut InstanceRule, aliases: &HashMap<String, String>) {
    if let InstanceRule::Assignment(r) = rule {
        expand_value(&mut r.value, aliases);
    }
}

fn inline_sd_rules(
    rules: Vec<Spanned<SdRule>>,
    rule_sets: &HashMap<String, Vec<Spanned<SdRule>>>,
    in_progress: &mut HashSet<String>,
) -> Result<Vec<Spanned<SdRule>>, FshError> {
    let mut result = Vec::new();
    for rule in rules {
        match &rule.value {
            SdRule::Insert(insert) => {
                let rs_name = insert.rule_set.clone();
                if in_progress.contains(&rs_name) {
                    let cycle: Vec<String> = in_progress.iter().cloned().collect();
                    return Err(FshError::RuleSetCycle { names: cycle });
                }
                if let Some(rs_rules) = rule_sets.get(&rs_name) {
                    in_progress.insert(rs_name.clone());
                    let inlined = inline_sd_rules(rs_rules.clone(), rule_sets, in_progress)?;
                    in_progress.remove(&rs_name);
                    result.extend(inlined);
                } else {
                    // Unknown ruleset — keep the insert rule as-is
                    result.push(rule);
                }
            }
            _ => result.push(rule),
        }
    }
    Ok(result)
}
