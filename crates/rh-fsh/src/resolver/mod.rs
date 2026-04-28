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

        expand_sd_rules_collection(&mut tank.profiles, &aliases, |e| &mut e.rules);
        expand_sd_rules_collection(&mut tank.extensions, &aliases, |e| &mut e.rules);
        expand_sd_rules_collection(&mut tank.logicals, &aliases, |e| &mut e.rules);
        expand_sd_rules_collection(&mut tank.resources, &aliases, |e| &mut e.rules);

        // Instances use instance rules (different rule type)
        for inst in tank.instances.values_mut() {
            for rule in &mut inst.rules {
                expand_instance_rule(&mut rule.value, &aliases);
            }
        }

        // Value sets
        for vs in tank.value_sets.values_mut() {
            for comp in &mut vs.components {
                let c = &mut comp.value;
                if let Some(sys) = &c.system {
                    if let Some(resolved) = aliases.get(sys.as_str()) {
                        c.system = Some(resolved.clone());
                    }
                }
                for concept in &mut c.concepts {
                    if let Some(sys) = &concept.system {
                        if let Some(resolved) = aliases.get(sys.as_str()) {
                            concept.system = Some(resolved.clone());
                        }
                    }
                }
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

        inline_sd_entity_rules(
            &mut tank.profiles,
            &rule_sets,
            &param_rule_sets,
            &mut errors,
        );
        inline_sd_entity_rules(
            &mut tank.extensions,
            &rule_sets,
            &param_rule_sets,
            &mut errors,
        );
        inline_sd_entity_rules(
            &mut tank.logicals,
            &rule_sets,
            &param_rule_sets,
            &mut errors,
        );
        inline_sd_entity_rules(
            &mut tank.resources,
            &rule_sets,
            &param_rule_sets,
            &mut errors,
        );

        // Instances use instance rules
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

/// Expand SD-rule aliases across every entity in an IndexMap.
fn expand_sd_rules_collection<T, F>(
    collection: &mut indexmap::IndexMap<String, T>,
    aliases: &HashMap<String, String>,
    get_rules_mut: F,
) where
    F: Fn(&mut T) -> &mut Vec<Spanned<SdRule>>,
{
    for entity in collection.values_mut() {
        for rule in get_rules_mut(entity) {
            expand_sd_rule(&mut rule.value, aliases);
        }
    }
}

/// Inline SD ruleset rules across every entity in an IndexMap.
fn inline_sd_entity_rules<T>(
    collection: &mut indexmap::IndexMap<String, T>,
    rule_sets: &HashMap<String, Vec<Spanned<SdRule>>>,
    param_rule_sets: &HashMap<String, ParamRuleSet>,
    errors: &mut Vec<FshError>,
) where
    T: HasSdRules,
{
    let names: Vec<String> = collection.keys().cloned().collect();
    for name in names {
        let rules = collection[&name].sd_rules().to_vec();
        match inline_sd_rules(rules, rule_sets, param_rule_sets, &mut HashSet::new()) {
            Ok(inlined) => *collection.get_mut(&name).unwrap().sd_rules_mut() = inlined,
            Err(e) => errors.push(e),
        }
    }
}

/// Trait to abstract mutable access to `rules: Vec<Spanned<SdRule>>` across entity types.
trait HasSdRules {
    fn sd_rules(&self) -> &[Spanned<SdRule>];
    fn sd_rules_mut(&mut self) -> &mut Vec<Spanned<SdRule>>;
}

macro_rules! impl_has_sd_rules {
    ($t:ty) => {
        impl HasSdRules for $t {
            fn sd_rules(&self) -> &[Spanned<SdRule>] {
                &self.rules
            }
            fn sd_rules_mut(&mut self) -> &mut Vec<Spanned<SdRule>> {
                &mut self.rules
            }
        }
    };
}

impl_has_sd_rules!(Profile);
impl_has_sd_rules!(Extension);
impl_has_sd_rules!(Logical);
impl_has_sd_rules!(ResourceDef);

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
