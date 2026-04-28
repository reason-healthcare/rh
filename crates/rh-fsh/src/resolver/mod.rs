//! FSH Resolver — alias expansion, RuleSet inlining, cycle detection

use crate::error::FshError;
use crate::parser::ast::*;
use crate::parser::rules::{parse_instance_rule, parse_sd_rule};
use crate::parser::span::Span;
use crate::tank::FshTank;
use std::collections::{HashMap, HashSet};

pub struct FshResolver;

impl FshResolver {
    /// Run all resolution passes on the tank in place
    pub fn resolve(tank: &mut FshTank) -> Result<(), Vec<FshError>> {
        let mut errors = Vec::new();

        // Pass 1: alias expansion (pre-inline)
        Self::expand_aliases(tank);

        // Pass 2 & 3: inline RuleSets with cycle detection
        if let Err(mut e) = Self::inline_rule_sets(tank) {
            errors.append(&mut e);
        }

        // Pass 4: alias expansion again to catch aliases introduced via parameterized inlining
        Self::expand_aliases(tank);

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

        // Value sets
        for vs in tank.value_sets.values_mut() {
            for comp in &mut vs.components {
                let c = &mut comp.value;
                // Expand system
                if let Some(sys) = &c.system {
                    if let Some(resolved) = aliases.get(sys.as_str()) {
                        c.system = Some(resolved.clone());
                    }
                }
                // Expand concept systems
                for concept in &mut c.concepts {
                    if let Some(sys) = &concept.system {
                        if let Some(resolved) = aliases.get(sys.as_str()) {
                            concept.system = Some(resolved.clone());
                        }
                    }
                }
                // Expand from_vs
                for vs_url in &mut c.from_vs {
                    if let Some(resolved) = aliases.get(vs_url.as_str()) {
                        *vs_url = resolved.clone();
                    }
                }
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

        let param_rule_sets: HashMap<String, ParamRuleSet> = tank
            .param_rule_sets
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Inline in profiles
        let profile_names: Vec<String> = tank.profiles.keys().cloned().collect();
        for name in profile_names {
            let rules = tank.profiles[&name].rules.clone();
            match inline_sd_rules(rules, &rule_sets, &param_rule_sets, &mut HashSet::new()) {
                Ok(inlined) => tank.profiles.get_mut(&name).unwrap().rules = inlined,
                Err(e) => errors.push(e),
            }
        }

        // Inline in extensions
        let ext_names: Vec<String> = tank.extensions.keys().cloned().collect();
        for name in ext_names {
            let rules = tank.extensions[&name].rules.clone();
            match inline_sd_rules(rules, &rule_sets, &param_rule_sets, &mut HashSet::new()) {
                Ok(inlined) => tank.extensions.get_mut(&name).unwrap().rules = inlined,
                Err(e) => errors.push(e),
            }
        }

        // Inline in logicals
        let logical_names: Vec<String> = tank.logicals.keys().cloned().collect();
        for name in logical_names {
            let rules = tank.logicals[&name].rules.clone();
            match inline_sd_rules(rules, &rule_sets, &param_rule_sets, &mut HashSet::new()) {
                Ok(inlined) => tank.logicals.get_mut(&name).unwrap().rules = inlined,
                Err(e) => errors.push(e),
            }
        }

        // Inline in resources
        let resource_names: Vec<String> = tank.resources.keys().cloned().collect();
        for name in resource_names {
            let rules = tank.resources[&name].rules.clone();
            match inline_sd_rules(rules, &rule_sets, &param_rule_sets, &mut HashSet::new()) {
                Ok(inlined) => tank.resources.get_mut(&name).unwrap().rules = inlined,
                Err(e) => errors.push(e),
            }
        }

        // Inline in instances (parameterized RuleSets may appear in instance context too)
        let inst_names: Vec<String> = tank.instances.keys().cloned().collect();
        for name in inst_names {
            let rules = tank.instances[&name].rules.clone();
            match inline_instance_rules(rules, &rule_sets, &param_rule_sets, &mut HashSet::new()) {
                Ok(inlined) => tank.instances.get_mut(&name).unwrap().rules = inlined,
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
        FshValue::Code {
            system: Some(sys), ..
        } => {
            if let Some(resolved) = aliases.get(sys.as_str()) {
                *sys = resolved.clone();
            }
        }
        FshValue::Date(_) | FshValue::DateTime(_) => {}
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

/// Perform text-level parameter substitution on a raw rule line.
/// Replaces `{paramName}` with the corresponding parameter value.
fn substitute_params(raw: &str, subs: &HashMap<String, String>) -> String {
    let mut result = raw.to_string();
    for (param, value) in subs {
        let placeholder = format!("{{{}}}", param);
        result = result.replace(&placeholder, value);
    }
    result
}

/// Expand a parameterized rule set into SD rules by substituting params and re-parsing.
fn expand_param_rule_set_sd(prs: &ParamRuleSet, params: &[String]) -> Vec<Spanned<SdRule>> {
    let subs: HashMap<String, String> = prs
        .params
        .iter()
        .zip(params.iter())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    let mut result = Vec::new();
    for raw in &prs.raw_rules {
        let substituted = substitute_params(raw, &subs);
        // Re-parse the substituted rule line
        let input_str = format!("{}\n", substituted);
        let span = Span::new(&input_str);
        if let Ok((_, rule)) = parse_sd_rule(span) {
            result.push(rule);
        }
    }
    result
}

/// Expand a parameterized rule set into instance rules by substituting params and re-parsing.
fn expand_param_rule_set_instance(
    prs: &ParamRuleSet,
    params: &[String],
) -> Vec<Spanned<InstanceRule>> {
    let subs: HashMap<String, String> = prs
        .params
        .iter()
        .zip(params.iter())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    let mut result = Vec::new();
    for raw in &prs.raw_rules {
        let substituted = substitute_params(raw, &subs);
        let input_str = format!("{}\n", substituted);
        let span = Span::new(&input_str);
        if let Ok((_, rule)) = parse_instance_rule(span) {
            result.push(rule);
        }
    }
    result
}

fn inline_sd_rules(
    rules: Vec<Spanned<SdRule>>,
    rule_sets: &HashMap<String, Vec<Spanned<SdRule>>>,
    param_rule_sets: &HashMap<String, ParamRuleSet>,
    in_progress: &mut HashSet<String>,
) -> Result<Vec<Spanned<SdRule>>, FshError> {
    let mut result = Vec::new();
    for rule in rules {
        match &rule.value {
            SdRule::Insert(insert) => {
                let rs_name = insert.rule_set.clone();
                let insert_params = insert.params.clone();
                if in_progress.contains(&rs_name) {
                    let cycle: Vec<String> = in_progress.iter().cloned().collect();
                    return Err(FshError::RuleSetCycle { names: cycle });
                }
                if !insert_params.is_empty() {
                    // Parameterized rule set
                    if let Some(prs) = param_rule_sets.get(&rs_name) {
                        let expanded = expand_param_rule_set_sd(prs, &insert_params);
                        in_progress.insert(rs_name.clone());
                        let inlined =
                            inline_sd_rules(expanded, rule_sets, param_rule_sets, in_progress)?;
                        in_progress.remove(&rs_name);
                        result.extend(inlined);
                    } else {
                        // Unknown parameterized ruleset — keep as-is
                        result.push(rule);
                    }
                } else if let Some(rs_rules) = rule_sets.get(&rs_name) {
                    in_progress.insert(rs_name.clone());
                    let inlined =
                        inline_sd_rules(rs_rules.clone(), rule_sets, param_rule_sets, in_progress)?;
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

fn inline_instance_rules(
    rules: Vec<Spanned<InstanceRule>>,
    rule_sets: &HashMap<String, Vec<Spanned<SdRule>>>,
    param_rule_sets: &HashMap<String, ParamRuleSet>,
    in_progress: &mut HashSet<String>,
) -> Result<Vec<Spanned<InstanceRule>>, FshError> {
    let mut result = Vec::new();
    for rule in rules {
        match &rule.value {
            InstanceRule::Insert(insert) => {
                let rs_name = insert.rule_set.clone();
                let insert_params = insert.params.clone();
                if in_progress.contains(&rs_name) {
                    let cycle: Vec<String> = in_progress.iter().cloned().collect();
                    return Err(FshError::RuleSetCycle { names: cycle });
                }
                if !insert_params.is_empty() {
                    if let Some(prs) = param_rule_sets.get(&rs_name) {
                        let expanded = expand_param_rule_set_instance(prs, &insert_params);
                        in_progress.insert(rs_name.clone());
                        let inlined = inline_instance_rules(
                            expanded,
                            rule_sets,
                            param_rule_sets,
                            in_progress,
                        )?;
                        in_progress.remove(&rs_name);
                        result.extend(inlined);
                    } else {
                        result.push(rule);
                    }
                } else {
                    // Simple (non-parameterized) rule set in instance context:
                    // SD rule sets can be inlined as instance assignment rules if they match
                    if let Some(rs_rules) = rule_sets.get(&rs_name) {
                        in_progress.insert(rs_name.clone());
                        // Convert SD assignment rules to instance rules
                        let inst_rules: Vec<Spanned<InstanceRule>> = rs_rules
                            .iter()
                            .filter_map(|r| {
                                if let SdRule::Assignment(a) = &r.value {
                                    Some(Spanned::new(
                                        InstanceRule::Assignment(a.clone()),
                                        r.location,
                                    ))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let inlined = inline_instance_rules(
                            inst_rules,
                            rule_sets,
                            param_rule_sets,
                            in_progress,
                        )?;
                        in_progress.remove(&rs_name);
                        result.extend(inlined);
                    } else {
                        result.push(rule);
                    }
                }
            }
            _ => result.push(rule),
        }
    }
    Ok(result)
}
