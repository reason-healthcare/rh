use anyhow::{Context, Result};
use serde_json::Value;
use std::path::PathBuf;

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};

use crate::profile::ProfileRegistry;
use crate::rules::RuleCompiler;
use crate::types::{IssueCode, ValidationIssue, ValidationResult};
use crate::valueset::ValueSetLoader;

pub struct FhirValidator {
    profile_registry: ProfileRegistry,
    rule_compiler: RuleCompiler,
    valueset_loader: ValueSetLoader,
    fhirpath_parser: FhirPathParser,
    fhirpath_evaluator: FhirPathEvaluator,
}

impl FhirValidator {
    pub fn new(packages_dir: Option<&str>) -> Result<Self> {
        let package_dirs = if let Some(dir) = packages_dir {
            vec![PathBuf::from(dir)]
        } else {
            vec![]
        };

        Ok(Self {
            profile_registry: ProfileRegistry::new(packages_dir)?,
            rule_compiler: RuleCompiler::default(),
            valueset_loader: ValueSetLoader::new(package_dirs, 100),
            fhirpath_parser: FhirPathParser::new(),
            fhirpath_evaluator: FhirPathEvaluator::new(),
        })
    }

    pub fn validate(&self, resource: &Value) -> Result<ValidationResult> {
        let mut result = ValidationResult::valid();

        if !resource.is_object() {
            return Ok(ValidationResult::invalid(vec![ValidationIssue::error(
                IssueCode::Structure,
                "Resource must be a JSON object",
            )]));
        }

        let resource_type = resource.get("resourceType").and_then(|v| v.as_str());
        if resource_type.is_none() {
            result = result.with_issue(ValidationIssue::error(
                IssueCode::Required,
                "Missing required field 'resourceType'",
            ));
        }

        Ok(result)
    }

    pub fn validate_with_profile(
        &self,
        resource: &Value,
        profile_url: &str,
    ) -> Result<ValidationResult> {
        let mut result = self.validate(resource)?;

        let snapshot = self
            .profile_registry
            .get_snapshot(profile_url)
            .context("Failed to get snapshot from registry")?;

        let snapshot = match snapshot {
            Some(s) => s,
            None => {
                return Ok(result.with_issue(ValidationIssue::error(
                    IssueCode::NotFound,
                    format!("Profile not found: {profile_url}"),
                )));
            }
        };

        let rules = self
            .rule_compiler
            .compile(&snapshot)
            .context("Failed to compile validation rules")?;

        for rule in &rules.cardinality_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            let violations =
                validate_cardinality_at_path(resource, &rule.path, rule.min, &rule.max);

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Type validation
        for rule in &rules.type_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            let violations = validate_type_at_path(resource, &rule.path, &rule.types);

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Binding validation
        for rule in &rules.binding_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            let violations = self.validate_binding_at_path(resource, rule)?;

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Invariant validation
        for rule in &rules.invariant_rules {
            let violations = self.validate_invariant(resource, rule)?;

            for violation in violations {
                result = result.with_issue(violation);
            }
        }

        Ok(result)
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profile_registry.list_profiles()
    }

    pub fn search_profiles(&self, query: &str) -> Vec<String> {
        self.profile_registry.search_profiles(query)
    }

    pub fn cache_stats(&self) -> ((usize, usize), (usize, usize)) {
        (
            self.profile_registry.cache_stats(),
            self.rule_compiler.cache_stats(),
        )
    }
}

fn validate_cardinality_at_path(
    resource: &Value,
    path: &str,
    min: Option<u32>,
    max: &Option<String>,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return issues;
    }

    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return issues;
    };

    if parts[0] != resource_type {
        return issues;
    }

    // Find if there's an array in the path before the final element
    let array_index = find_array_in_path(resource, &parts);

    if let Some(arr_idx) = array_index {
        // Path crosses an array boundary - validate per-item
        validate_per_array_item(resource, &parts, arr_idx, min, max.as_deref(), &mut issues);
    } else {
        // Simple path - validate directly
        let count = count_simple_path(resource, &parts);
        check_cardinality(path, count, min, max.as_deref(), &mut issues);
    }

    issues
}

fn find_array_in_path(resource: &Value, parts: &[&str]) -> Option<usize> {
    if parts.len() <= 2 {
        // No intermediate path to check (e.g., "Patient.identifier")
        return None;
    }

    let mut current = resource;

    for (i, part) in parts[1..parts.len() - 1].iter().enumerate() {
        match current.get(part) {
            Some(Value::Array(_)) => return Some(i + 1), // +1 because we skipped resource type
            Some(other) => current = other,
            None => return None,
        }
    }

    None
}

fn validate_per_array_item(
    resource: &Value,
    parts: &[&str],
    array_index: usize,
    min: Option<u32>,
    max: Option<&str>,
    issues: &mut Vec<ValidationIssue>,
) {
    // Navigate to the array
    let mut current = resource;
    for part in &parts[1..array_index] {
        match current.get(part) {
            Some(value) => current = value,
            None => return,
        }
    }

    // Get the array
    let Some(array) = current.get(parts[array_index]).and_then(|v| v.as_array()) else {
        return;
    };

    // Validate cardinality for each array item
    for item in array.iter() {
        let remaining_path = &parts[array_index + 1..];
        let count = count_in_item(item, remaining_path);

        let full_path = parts.join(".");

        check_cardinality(&full_path, count, min, max, issues);
    }
}

fn count_in_item(item: &Value, remaining_path: &[&str]) -> usize {
    if remaining_path.is_empty() {
        return 1;
    }

    let mut current = item;
    for (i, part) in remaining_path.iter().enumerate() {
        match current.get(part) {
            Some(Value::Array(arr)) => {
                if i == remaining_path.len() - 1 {
                    return arr.len();
                }
                return arr.len(); // Count items in nested arrays
            }
            Some(other) => {
                if i == remaining_path.len() - 1 {
                    return 1;
                }
                current = other;
            }
            None => return 0,
        }
    }

    0
}

fn count_simple_path(resource: &Value, parts: &[&str]) -> usize {
    let mut current = resource;

    for (i, part) in parts[1..].iter().enumerate() {
        match current.get(part) {
            Some(Value::Array(arr)) => {
                if i == parts.len() - 2 {
                    return arr.len();
                }
                if arr.is_empty() {
                    return 0;
                }
                return arr.len();
            }
            Some(other) => {
                if i == parts.len() - 2 {
                    return 1;
                }
                current = other;
            }
            None => return 0,
        }
    }

    1
}

fn check_cardinality(
    path: &str,
    count: usize,
    min: Option<u32>,
    max: Option<&str>,
    issues: &mut Vec<ValidationIssue>,
) {
    if let Some(min_val) = min {
        if count < min_val as usize {
            issues.push(ValidationIssue::error(
                IssueCode::Required,
                format!(
                    "Cardinality violation at '{path}': expected at least {min_val} but found {count}"
                ),
            ));
        }
    }

    if let Some(max_str) = max {
        if max_str != "*" {
            if let Ok(max_num) = max_str.parse::<usize>() {
                if count > max_num {
                    issues.push(ValidationIssue::error(
                        IssueCode::Value,
                        format!(
                            "Cardinality violation at '{path}': expected at most {max_num} but found {count}"
                        ),
                    ));
                }
            }
        }
    }
}

fn validate_type_at_path(
    resource: &Value,
    path: &str,
    expected_types: &[String],
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if expected_types.is_empty() {
        return issues;
    }

    let values = get_values_at_path(resource, path);

    for value in values {
        if !matches_any_type(value, expected_types) {
            let actual_type = get_json_type_name(value);
            issues.push(ValidationIssue::error(
                IssueCode::Value,
                format!(
                    "Type mismatch at '{path}': expected one of [{}] but found {actual_type}",
                    expected_types.join(", ")
                ),
            ));
        }
    }

    issues
}

fn get_values_at_path<'a>(resource: &'a Value, path: &str) -> Vec<&'a Value> {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return vec![];
    }

    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return vec![];
    };

    if parts[0] != resource_type {
        return vec![];
    }

    if parts.len() == 1 {
        return vec![resource];
    }

    let mut current = vec![resource];

    for part in &parts[1..] {
        let mut next = Vec::new();

        for value in current {
            match value.get(part) {
                Some(Value::Array(arr)) => {
                    for item in arr {
                        next.push(item);
                    }
                }
                Some(other) => {
                    next.push(other);
                }
                None => {}
            }
        }

        if next.is_empty() {
            return vec![];
        }

        current = next;
    }

    current
}

fn matches_any_type(value: &Value, expected_types: &[String]) -> bool {
    for expected_type in expected_types {
        if matches_fhir_type(value, expected_type) {
            return true;
        }
    }
    false
}

fn matches_fhir_type(value: &Value, fhir_type: &str) -> bool {
    match fhir_type {
        // Primitive types
        "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical" | "oid" | "uuid" => {
            value.is_string()
        }
        "boolean" => value.is_boolean(),
        "integer" | "unsignedInt" | "positiveInt" => value.is_i64() || value.is_u64(),
        "decimal" => value.is_f64() || value.is_i64() || value.is_u64(),
        "date" | "dateTime" | "instant" | "time" => value.is_string(),
        "base64Binary" => value.is_string(),

        // Complex types - must be objects
        "HumanName"
        | "Address"
        | "ContactPoint"
        | "Identifier"
        | "CodeableConcept"
        | "Coding"
        | "Reference"
        | "Period"
        | "Quantity"
        | "Range"
        | "Ratio"
        | "Attachment"
        | "Annotation"
        | "Signature"
        | "SampledData"
        | "Age"
        | "Distance"
        | "Duration"
        | "Count"
        | "Money"
        | "MoneyQuantity"
        | "SimpleQuantity"
        | "Meta"
        | "Dosage"
        | "Extension"
        | "Narrative"
        | "ContactDetail"
        | "Contributor"
        | "DataRequirement"
        | "ParameterDefinition"
        | "RelatedArtifact"
        | "TriggerDefinition"
        | "UsageContext"
        | "Expression"
        | "Timing" => value.is_object(),

        // BackboneElement and other structured types
        "BackboneElement" | "Element" => value.is_object(),

        // Resource types (all should be objects)
        _ if fhir_type.chars().next().unwrap_or('a').is_uppercase() => value.is_object(),

        // Unknown type - be lenient
        _ => true,
    }
}

fn get_json_type_name(value: &Value) -> &str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(n) if n.is_i64() || n.is_u64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn should_validate_path(path: &str, resource: &Value) -> bool {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.len() <= 1 {
        return true;
    }

    let parent_path = parts[..parts.len() - 1].join(".");
    let parent_value = get_value_at_path(resource, &parent_path);

    match parent_value {
        Some(Value::Array(arr)) => !arr.is_empty(),
        Some(Value::Object(_)) => true,
        Some(_) => false,
        None => parts.len() == 2,
    }
}

fn get_value_at_path<'a>(resource: &'a Value, path: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return None;
    }

    let resource_type = resource.get("resourceType").and_then(|v| v.as_str())?;

    if parts[0] != resource_type {
        return None;
    }

    if parts.len() == 1 {
        return Some(resource);
    }

    let mut current = resource;
    for part in &parts[1..] {
        current = current.get(part)?;
    }

    Some(current)
}

impl FhirValidator {
    fn validate_binding_at_path(
        &self,
        resource: &Value,
        rule: &crate::rules::BindingRule,
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Skip preferred and example bindings
        if rule.strength == "preferred" || rule.strength == "example" {
            return Ok(issues);
        }

        // Check if ValueSet is extensional
        let is_extensional = self
            .valueset_loader
            .is_extensional(&rule.value_set_url)
            .unwrap_or(false);

        if !is_extensional {
            // Skip intensional ValueSets (deferred to Phase 3.5)
            return Ok(issues);
        }

        // Get values at path
        let values = get_values_at_path(resource, &rule.path);

        for value in values {
            // Extract codes based on type
            let codes = extract_codes_from_value(value);

            for (system, code) in codes {
                let is_valid =
                    self.valueset_loader
                        .contains_code(&rule.value_set_url, &system, &code)?;

                if !is_valid {
                    let message = format!(
                        "Code '{}' from system '{}' is not in {} ValueSet '{}'",
                        code,
                        if system.is_empty() {
                            "(no system)"
                        } else {
                            &system
                        },
                        rule.strength,
                        rule.value_set_url
                    );

                    let issue = if rule.strength == "required" {
                        ValidationIssue::error(IssueCode::CodeInvalid, message)
                    } else {
                        ValidationIssue::warning(IssueCode::CodeInvalid, message)
                    };

                    issues.push(issue);
                }
            }
        }

        Ok(issues)
    }

    fn validate_invariant(
        &self,
        resource: &Value,
        rule: &crate::rules::InvariantRule,
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Parse the FHIRPath expression
        let expression = match self.fhirpath_parser.parse(&rule.expression) {
            Ok(expr) => expr,
            Err(e) => {
                // Handle parse errors gracefully - skip validation but log
                return Ok(vec![ValidationIssue::warning(
                    IssueCode::Invariant,
                    format!("Failed to parse invariant {}: {}", rule.key, e),
                )]);
            }
        };

        // Create evaluation context with the resource
        // For resource-level invariants (path is resource type), use resource as context
        // For element-level invariants, use the specific element as context
        let parts: Vec<&str> = rule.path.split('.').collect();
        let is_resource_level = parts.len() == 1;

        let context = if is_resource_level {
            // Resource-level invariant: both root and current are the resource
            EvaluationContext::new(resource.clone())
        } else {
            // Element-level invariant: set current to the element
            // For element-level invariants, we would ideally validate against each instance
            // For now, use resource as root and current (simplified approach)
            // Full implementation would:
            // 1. Get all values at the element path: get_values_at_path(resource, &rule.path)
            // 2. Create context for each element value
            // 3. Validate each instance separately
            EvaluationContext::new(resource.clone())
        };

        // Evaluate the expression
        let result = match self.fhirpath_evaluator.evaluate(&expression, &context) {
            Ok(value) => value,
            Err(e) => {
                // Handle evaluation errors gracefully
                return Ok(vec![ValidationIssue::warning(
                    IssueCode::Invariant,
                    format!("Failed to evaluate invariant {}: {}", rule.key, e),
                )]);
            }
        };

        // Convert result to boolean
        let is_valid = match result {
            FhirPathValue::Boolean(b) => b,
            FhirPathValue::Empty => true, // Empty is treated as true (constraint not applicable)
            FhirPathValue::Collection(ref items) if items.is_empty() => true, // Empty collection = true
            FhirPathValue::Collection(ref items) if items.len() == 1 => {
                // Single item collection - check if it's a boolean
                match &items[0] {
                    FhirPathValue::Boolean(b) => *b,
                    _ => true, // Non-empty non-boolean collection = true
                }
            }
            _ => true, // Any other non-empty result = true (constraint satisfied)
        };

        if !is_valid {
            let issue = if rule.severity == "error" {
                ValidationIssue::error(
                    IssueCode::Invariant,
                    format!("{}: {}", rule.key, rule.human),
                )
            } else {
                ValidationIssue::warning(
                    IssueCode::Invariant,
                    format!("{}: {}", rule.key, rule.human),
                )
            };
            issues.push(issue);
        }

        Ok(issues)
    }
}

fn extract_codes_from_value(value: &Value) -> Vec<(String, String)> {
    let mut codes = Vec::new();

    match value {
        // Simple code (string)
        Value::String(code) => {
            codes.push((String::new(), code.clone()));
        }
        // Coding
        Value::Object(obj) if obj.contains_key("code") => {
            if let Some(code) = obj.get("code").and_then(|v| v.as_str()) {
                let system = obj
                    .get("system")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                codes.push((system, code.to_string()));
            }
        }
        // CodeableConcept
        Value::Object(obj) if obj.contains_key("coding") => {
            if let Some(Value::Array(codings)) = obj.get("coding") {
                for coding in codings {
                    if let Value::Object(coding_obj) = coding {
                        if let Some(code) = coding_obj.get("code").and_then(|v| v.as_str()) {
                            let system = coding_obj
                                .get("system")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            codes.push((system, code.to_string()));
                        }
                    }
                }
            }
        }
        _ => {}
    }

    codes
}

impl Default for FhirValidator {
    fn default() -> Self {
        Self::new(None).expect("Failed to initialize FhirValidator")
    }
}
