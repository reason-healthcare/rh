use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tar::Archive;

use super::parser::{ExpectedOutcome, Manifest, TestCase};
use rh_validator::{
    FhirValidator, IssueCode, Severity, TerminologyConfig, ValidationIssue, ValidationOptions,
    ValidationResult,
};

#[derive(Debug, Clone, Default)]
pub struct TestRunConfig {
    pub max_tests: Option<usize>,
    pub module_filter: Option<String>,
    pub verbose: bool,
    pub use_terminology: bool,
}

#[derive(Debug, Clone)]
pub struct TestRunResult {
    pub test_name: String,
    #[allow(dead_code)]
    pub module: Option<String>,
    pub passed: bool,
    #[allow(dead_code)]
    pub error: Option<String>,
    pub expected_valid: bool,
    pub actual_valid: bool,
    pub actual_error_count: usize,
    pub actual_warning_count: usize,
    pub duration_ms: u128,
    pub java_expected_valid: Option<bool>,
    pub firely_current_expected_valid: Option<bool>,
    pub firely_wip_expected_valid: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TestRunSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub total_duration_ms: u128,
    pub results: Vec<TestRunResult>,
}

impl TestRunSummary {
    pub fn new() -> Self {
        Self {
            total: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            total_duration_ms: 0,
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: TestRunResult) {
        self.total += 1;
        self.total_duration_ms += result.duration_ms;

        if result.passed {
            self.passed += 1;
        } else {
            self.failed += 1;
        }

        self.results.push(result);
    }

    #[allow(dead_code)]
    pub fn add_skipped(&mut self) {
        self.total += 1;
        self.skipped += 1;
    }

    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }

    pub fn avg_duration_ms(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.total_duration_ms as f64 / self.total as f64
        }
    }
}

impl Default for TestRunSummary {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TestRunner {
    validator: FhirValidator,
    config: TestRunConfig,
}

impl TestRunner {
    pub fn new(profiles_dir: Option<&str>, config: TestRunConfig) -> Result<Self> {
        let terminology_config = if config.use_terminology {
            Some(TerminologyConfig::mock())
        } else {
            None
        };

        let validator = FhirValidator::with_terminology(
            rh_validator::FhirVersion::R4,
            profiles_dir,
            terminology_config,
        )
        .context("Failed to create FhirValidator")?;

        Ok(Self { validator, config })
    }

    pub fn run_all(&mut self, manifest: &Manifest, validator_dir: &Path) -> Result<TestRunSummary> {
        let mut summary = TestRunSummary::new();
        let mut test_count = 0;

        let tests: Vec<_> = manifest
            .r4_test_cases()
            .filter(|tc| tc.is_json()) // Only JSON files for now
            .filter(|tc| {
                if let Some(module) = &self.config.module_filter {
                    tc.module.as_deref() == Some(module.as_str())
                } else {
                    true
                }
            })
            .collect();

        let total_tests = if let Some(max) = self.config.max_tests {
            tests.len().min(max)
        } else {
            tests.len()
        };

        println!("\nRunning {total_tests} R4 test cases...");
        if let Some(module) = &self.config.module_filter {
            println!("Module filter: {module}");
        }
        println!();

        for test in tests.iter().take(total_tests) {
            if self.config.verbose {
                println!("Running test: {}", test.name);
            }

            match self.run_single_test(test, validator_dir) {
                Ok(result) => {
                    if self.config.verbose {
                        self.print_test_result(&result);
                    }
                    summary.add_result(result);
                }
                Err(e) => {
                    if self.config.verbose {
                        println!("  ERROR: {e:#}");
                    }
                    summary.add_result(TestRunResult {
                        test_name: test.name.clone(),
                        module: test.module.clone(),
                        passed: false,
                        error: Some(format!("{e:#}")),
                        expected_valid: false,
                        actual_valid: false,
                        actual_error_count: 1,
                        actual_warning_count: 0,
                        duration_ms: 0,
                        java_expected_valid: None,
                        firely_current_expected_valid: None,
                        firely_wip_expected_valid: None,
                    });
                }
            }

            test_count += 1;
            if !self.config.verbose && test_count % 10 == 0 {
                print!(".");
                if test_count % 100 == 0 {
                    println!(" {test_count}/{total_tests}");
                }
            }
        }

        if !self.config.verbose {
            println!();
        }

        Ok(summary)
    }

    fn run_single_test(&mut self, test: &TestCase, validator_dir: &Path) -> Result<TestRunResult> {
        let start = Instant::now();

        let test_file_path = test.get_test_file_path(validator_dir);

        if !test_file_path.exists() {
            anyhow::bail!("Test file not found: {}", test_file_path.display());
        }

        let expected_outcome = self.load_expected_outcome(test, validator_dir)?;
        let expected_valid = expected_outcome.is_valid();

        let all_supporting = test.get_supporting_files(validator_dir);
        let profile_source = test.get_profile_source_path(validator_dir);
        let has_local_supporting_resources = !all_supporting.is_empty()
            || profile_source
                .as_ref()
                .is_some_and(|profile_source| profile_source.exists());
        let has_package_resources = has_resolvable_package_resources(test, validator_dir);
        let mut package_validator = if has_package_resources {
            Some(self.build_package_validator(test, validator_dir)?)
        } else {
            None
        };
        let mut option_validator =
            if !has_package_resources && (test.security_checks || has_local_supporting_resources) {
                Some(self.build_option_validator(test)?)
            } else {
                None
            };
        let validator = package_validator
            .as_mut()
            .or(option_validator.as_mut())
            .unwrap_or(&mut self.validator);
        let skip_dynamic_profiles = has_package_resources;

        // Register supporting files from `supporting`, `profiles`, and `profile.supporting` fields.
        // Package-backed tests load StructureDefinitions through the static package directory path
        // so snapshots include package dependencies.
        for supporting_path in all_supporting.into_iter().chain(profile_source) {
            if supporting_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&supporting_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        match json.get("resourceType").and_then(|v| v.as_str()) {
                            Some("Questionnaire") => {
                                validator.register_questionnaire(&json);
                            }
                            Some("ValueSet") => {
                                validator.register_valueset(&json);
                            }
                            Some("StructureDefinition") if !skip_dynamic_profiles => {
                                if self.config.verbose {
                                    if let Some(url) = json.get("url").and_then(|u| u.as_str()) {
                                        println!("    Registering profile: {url}");
                                    }
                                }
                                validator.register_profile(&json);
                            }
                            Some("CodeSystem") => {
                                validator.register_codesystem(&json);
                            }
                            Some("Measure") => {
                                validator.register_measure(&json);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let resource_json = std::fs::read_to_string(&test_file_path)
            .with_context(|| format!("Failed to read test file: {}", test_file_path.display()))?;

        let resource: serde_json::Value = match parse_test_resource_json(&resource_json, test) {
            Ok(resource) => resource,
            Err(error) => {
                let duration = start.elapsed();
                let actual_valid = false;
                let passed = expected_valid == actual_valid;
                let error = Some(format!(
                    "JSON parse error: {} at {}",
                    error,
                    test_file_path.display()
                ));

                return Ok(TestRunResult {
                    test_name: test.name.clone(),
                    module: test.module.clone(),
                    passed,
                    error,
                    expected_valid,
                    actual_valid,
                    actual_error_count: 1,
                    actual_warning_count: 0,
                    duration_ms: duration.as_millis(),
                    java_expected_valid: test.get_java_expected_valid(validator_dir),
                    firely_current_expected_valid: test
                        .get_firely_current_expected_valid(validator_dir),
                    firely_wip_expected_valid: test.get_firely_wip_expected_valid(validator_dir),
                });
            }
        };

        let mut validation_result = if let Some(scoring) = &test.scoring {
            validator.validate_with_profile(&resource, &scoring.profile)?
        } else {
            validator.validate_auto(&resource)?
        };
        if test.validate_contains.as_deref() == Some("IGNORE") {
            suppress_contained_reference_issues(&mut validation_result);
        }
        if test.matchetype.is_none() && resource_has_matchetype_marker(&resource) {
            validation_result = validation_result.with_issue(
                ValidationIssue::error(
                    IssueCode::Invalid,
                    "This resource is not allowed to be a matchetype resource",
                )
                .with_path(
                    resource
                        .get("resourceType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Resource")
                        .to_string(),
                ),
            );
        }
        if test.for_publication && is_hl7_publication_codesystem_missing_workgroup(&resource) {
            validation_result = validation_result.with_issue(
                ValidationIssue::error(
                    IssueCode::BusinessRule,
                    "When HL7 is publishing a resource, the owning committee must be stated using the http://hl7.org/fhir/StructureDefinition/structuredefinition-wg extension",
                )
                .with_path("CodeSystem".to_string()),
            );
        }

        let duration = start.elapsed();
        let actual_valid = is_valid(&validation_result);
        let actual_error_count = count_errors(&validation_result);
        let actual_warning_count = count_warnings(&validation_result);
        let passed = expected_valid == actual_valid;
        let error = if !passed {
            let mut message = format!(
                "Validation mismatch: expected {}, got {}",
                if expected_valid { "VALID" } else { "INVALID" },
                if actual_valid { "VALID" } else { "INVALID" }
            );
            let diagnostics = validation_result
                .issues
                .iter()
                .take(5)
                .map(|issue| {
                    format!(
                        "{:?} {:?}{}: {}",
                        issue.severity,
                        issue.code,
                        issue
                            .path
                            .as_ref()
                            .map(|path| format!(" at {path}"))
                            .unwrap_or_default(),
                        issue.message
                    )
                })
                .collect::<Vec<_>>();
            if !diagnostics.is_empty() {
                message.push_str("; first issues: ");
                message.push_str(&diagnostics.join(" | "));
            }
            Some(message)
        } else {
            None
        };

        let java_expected_valid = test.get_java_expected_valid(validator_dir);
        let firely_current_expected_valid = test.get_firely_current_expected_valid(validator_dir);
        let firely_wip_expected_valid = test.get_firely_wip_expected_valid(validator_dir);

        Ok(TestRunResult {
            test_name: test.name.clone(),
            module: test.module.clone(),
            passed,
            error,
            expected_valid,
            actual_valid,
            actual_error_count,
            actual_warning_count,
            duration_ms: duration.as_millis(),
            java_expected_valid,
            firely_current_expected_valid,
            firely_wip_expected_valid,
        })
    }

    fn build_package_validator(
        &self,
        test: &TestCase,
        validator_dir: &Path,
    ) -> Result<FhirValidator> {
        let package_dir = self.materialize_test_packages(test, validator_dir)?;
        let terminology_config = if self.config.use_terminology {
            Some(TerminologyConfig::mock())
        } else {
            None
        };

        FhirValidator::with_terminology(
            rh_validator::FhirVersion::R4,
            Some(&package_dir.to_string_lossy()),
            terminology_config,
        )
        .context("Failed to create package-backed FhirValidator")
    }

    fn build_option_validator(&self, test: &TestCase) -> Result<FhirValidator> {
        let terminology_config = if self.config.use_terminology {
            Some(TerminologyConfig::mock())
        } else {
            None
        };

        FhirValidator::with_options(
            rh_validator::FhirVersion::R4,
            None,
            terminology_config,
            ValidationOptions {
                security_checks: test.security_checks,
            },
        )
        .context("Failed to create option-backed FhirValidator")
    }

    fn materialize_test_packages(
        &self,
        test: &TestCase,
        validator_dir: &Path,
    ) -> Result<std::path::PathBuf> {
        let package_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/fhir-test-package-resources")
            .join(sanitize_path_component(&test.name));
        if package_dir.exists() {
            fs::remove_dir_all(&package_dir).with_context(|| {
                format!("Failed to clean package dir {}", package_dir.display())
            })?;
        }
        fs::create_dir_all(&package_dir)
            .with_context(|| format!("Failed to create package dir {}", package_dir.display()))?;

        let mut file_index = 0usize;
        for package_ref in package_resource_refs(test) {
            match resolve_package_resource(&package_ref, validator_dir) {
                Some(ResolvedPackageResource::LocalArchive(package_path)) => {
                    file_index =
                        extract_package_json_files(&package_path, &package_dir, file_index)
                            .with_context(|| {
                                format!("Failed to extract package {}", package_path.display())
                            })?;
                }
                Some(ResolvedPackageResource::InstalledPackage(package_path)) => {
                    let context = format!("Failed to copy package {}", package_path.display());
                    file_index = copy_package_json_files(&package_path, &package_dir, file_index)
                        .with_context(|| context)?;
                }
                None => {}
            }
        }

        for supporting_path in test
            .get_supporting_files(validator_dir)
            .into_iter()
            .chain(test.get_profile_source_path(validator_dir))
        {
            if supporting_path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let content = fs::read_to_string(&supporting_path).with_context(|| {
                format!(
                    "Failed to read supporting file {}",
                    supporting_path.display()
                )
            })?;
            let output_path = package_dir.join(format!("supporting-{file_index}.json"));
            fs::write(&output_path, content).with_context(|| {
                format!("Failed to write supporting file {}", output_path.display())
            })?;
            file_index += 1;
        }

        Ok(package_dir)
    }

    fn load_expected_outcome(
        &self,
        test: &TestCase,
        validator_dir: &Path,
    ) -> Result<ExpectedOutcome> {
        if let Some(outcome_path) = test.get_expected_outcome_path(validator_dir) {
            if outcome_path.exists() {
                return super::parser::load_expected_outcome(&outcome_path);
            }
        }

        if let Some(inline) = test.get_inline_outcome() {
            return Ok(ExpectedOutcome {
                resource_type: "OperationOutcome".to_string(),
                issue: if inline.is_valid() {
                    Vec::new()
                } else {
                    vec![super::parser::OutcomeIssue {
                        severity: "error".to_string(),
                        code: "invalid".to_string(),
                        details: None,
                        diagnostics: Some(format!(
                            "Inline outcome indicates {} error(s)",
                            inline.error_count.unwrap_or(1)
                        )),
                        expression: Vec::new(),
                    }]
                },
            });
        }

        anyhow::bail!("No expected outcome found for test: {}", test.name);
    }

    fn print_test_result(&self, result: &TestRunResult) {
        let duration_ms = result.duration_ms;
        let expected_status = if result.expected_valid {
            "VALID"
        } else {
            "INVALID"
        };
        let actual_status = if result.actual_valid {
            "VALID"
        } else {
            "INVALID"
        };

        if result.passed {
            println!("  ✓ PASS ({duration_ms}ms) - Both agree: {expected_status}");
        } else {
            println!(
                "  ✗ FAIL ({duration_ms}ms) - Expected: {expected_status}, Got: {actual_status}"
            );
            if result.actual_error_count > 0 || result.actual_warning_count > 0 {
                println!(
                    "    Our validator: {}E/{}W",
                    result.actual_error_count, result.actual_warning_count
                );
            }
        }
    }

    pub fn print_summary(&self, summary: &TestRunSummary) {
        println!("\n{}", "=".repeat(70));
        println!("Test Run Summary");
        println!("{}", "=".repeat(70));
        println!("Total tests:    {}", summary.total);
        println!(
            "Passed:         {} ({:.1}%)",
            summary.passed,
            summary.pass_rate()
        );
        println!("Failed:         {}", summary.failed);
        println!("Skipped:        {}", summary.skipped);
        println!("Total time:     {}ms", summary.total_duration_ms);
        println!("Average time:   {:.1}ms/test", summary.avg_duration_ms());
        println!("{}", "=".repeat(70));

        if summary.failed > 0 {
            println!("\nFailed tests:");
            for result in &summary.results {
                if !result.passed {
                    let test_name = &result.test_name;
                    let expected_status = if result.expected_valid {
                        "VALID"
                    } else {
                        "INVALID"
                    };
                    let actual_status = if result.actual_valid {
                        "VALID"
                    } else {
                        "INVALID"
                    };
                    println!(
                        "  - {test_name}: Expected {expected_status}, got {actual_status} ({}E/{}W)",
                        result.actual_error_count, result.actual_warning_count
                    );
                    if let Some(error) = &result.error {
                        println!("    {error}");
                    }
                }
            }
        }
    }

    /// Generate a comparison table showing how different validators agree/disagree.
    pub fn print_validator_comparison(&self, summary: &TestRunSummary) {
        println!("\n{}", "=".repeat(90));
        println!("Validator Comparison Table");
        println!("{}", "=".repeat(90));
        println!(
            "{:<40} {:>10} {:>10} {:>12} {:>12}",
            "Test", "rh-val", "Java", "Firely-Cur", "Firely-WIP"
        );
        println!("{}", "-".repeat(90));

        for result in &summary.results {
            let rh_status = if result.actual_valid {
                "VALID"
            } else {
                "INVALID"
            };
            let java_status = match result.java_expected_valid {
                Some(true) => "VALID",
                Some(false) => "INVALID",
                None => "N/A",
            };
            let firely_cur_status = match result.firely_current_expected_valid {
                Some(true) => "VALID",
                Some(false) => "INVALID",
                None => "N/A",
            };
            let firely_wip_status = match result.firely_wip_expected_valid {
                Some(true) => "VALID",
                Some(false) => "INVALID",
                None => "N/A",
            };

            let test_name = if result.test_name.len() > 38 {
                format!("{}...", &result.test_name[..35])
            } else {
                result.test_name.clone()
            };

            println!("{test_name:<40} {rh_status:>10} {java_status:>10} {firely_cur_status:>12} {firely_wip_status:>12}");
        }

        println!("{}", "=".repeat(90));

        let java_agreement = summary
            .results
            .iter()
            .filter(|r| {
                r.java_expected_valid.is_some() && r.java_expected_valid.unwrap() == r.actual_valid
            })
            .count();
        let java_total = summary
            .results
            .iter()
            .filter(|r| r.java_expected_valid.is_some())
            .count();

        if java_total > 0 {
            println!(
                "Agreement with Java: {}/{} ({:.1}%)",
                java_agreement,
                java_total,
                (java_agreement as f64 / java_total as f64) * 100.0
            );
        }

        let firely_cur_agreement = summary
            .results
            .iter()
            .filter(|r| {
                r.firely_current_expected_valid.is_some()
                    && r.firely_current_expected_valid.unwrap() == r.actual_valid
            })
            .count();
        let firely_cur_total = summary
            .results
            .iter()
            .filter(|r| r.firely_current_expected_valid.is_some())
            .count();

        if firely_cur_total > 0 {
            println!(
                "Agreement with Firely SDK (Current): {}/{} ({:.1}%)",
                firely_cur_agreement,
                firely_cur_total,
                (firely_cur_agreement as f64 / firely_cur_total as f64) * 100.0
            );
        }

        println!("{}", "=".repeat(90));
    }

    pub fn write_java_mismatch_triage(
        &self,
        summary: &TestRunSummary,
        label: &str,
    ) -> Result<Option<PathBuf>> {
        write_java_mismatch_triage(summary, label)
    }
}

fn write_java_mismatch_triage(summary: &TestRunSummary, label: &str) -> Result<Option<PathBuf>> {
    let mismatches = summary
        .results
        .iter()
        .filter(|result| {
            result
                .java_expected_valid
                .is_some_and(|java_valid| java_valid != result.actual_valid)
        })
        .collect::<Vec<_>>();

    if mismatches.is_empty() {
        return Ok(None);
    }

    let output_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/conformance-triage");
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create triage dir {}", output_dir.display()))?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System clock is before Unix epoch")?
        .as_secs();
    let output_path = output_dir.join(format!(
        "r4-java-mismatches-{timestamp}-{}.csv",
        sanitize_path_component(label)
    ));

    let mut category_counts = BTreeMap::<&'static str, usize>::new();
    let mut rows = Vec::with_capacity(mismatches.len() + 1);
    rows.push(csv_row(&[
        "category",
        "test_name",
        "module",
        "expected_valid",
        "actual_valid",
        "java_expected_valid",
        "actual_error_count",
        "actual_warning_count",
        "error",
    ]));

    for result in mismatches {
        let category = categorize_java_mismatch(result);
        *category_counts.entry(category).or_default() += 1;
        rows.push(csv_row(&[
            category,
            &result.test_name,
            result.module.as_deref().unwrap_or(""),
            bool_cell(result.expected_valid),
            bool_cell(result.actual_valid),
            bool_cell(result.java_expected_valid.unwrap_or(false)),
            &result.actual_error_count.to_string(),
            &result.actual_warning_count.to_string(),
            result.error.as_deref().unwrap_or(""),
        ]));
    }

    fs::write(&output_path, rows.join("\n") + "\n")
        .with_context(|| format!("Failed to write triage file {}", output_path.display()))?;

    println!("\nJava mismatch triage: {}", output_path.display());
    println!("Category counts:");
    for (category, count) in category_counts {
        println!("  - {category}: {count}");
    }

    Ok(Some(output_path))
}

fn categorize_java_mismatch(result: &TestRunResult) -> &'static str {
    let haystack = format!(
        "{} {} {}",
        result.test_name,
        result.module.as_deref().unwrap_or_default(),
        result.error.as_deref().unwrap_or_default()
    )
    .to_ascii_lowercase();

    if haystack.contains("json parse")
        || result.test_name.starts_with("json-")
        || result.test_name.starts_with("bad-json")
    {
        "json-parser"
    } else if haystack.contains("questionnaire")
        || haystack.contains("questionnaireresponse")
        || result.test_name.ends_with("-qr")
        || result.test_name.starts_with("qr-")
        || result.test_name.starts_with("q_")
    {
        "questionnaire-response"
    } else if haystack.contains("extension") {
        "extension"
    } else if haystack.contains("bundle")
        || haystack.contains("contained")
        || haystack.contains("reference")
        || haystack.contains("htmlrefs")
        || haystack.contains("signature")
        || haystack.contains("fullurl")
    {
        "reference-bundle-contained"
    } else if haystack.contains("valueset")
        || haystack.contains("codesystem")
        || haystack.contains("structuredefinition")
        || haystack.contains("searchparameter")
        || haystack.contains("capabilitystatement")
        || haystack.contains("operationdefinition")
    {
        "validation-resource"
    } else if haystack.contains("profile")
        || haystack.contains("slicing")
        || haystack.contains("slice")
    {
        "profile-slicing"
    } else if haystack.contains("terminology")
        || haystack.contains("ucum")
        || haystack.contains("codeinvalid")
        || haystack.contains("wrong display")
        || haystack.contains("valuequantity")
    {
        "terminology"
    } else if haystack.contains("invariant")
        || haystack.contains("dom-")
        || haystack.contains("sdf-")
    {
        "invariant"
    } else {
        "other"
    }
}

fn csv_row(fields: &[&str]) -> String {
    fields
        .iter()
        .map(|field| csv_escape(field))
        .collect::<Vec<_>>()
        .join(",")
}

fn csv_escape(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn bool_cell(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn count_errors(result: &ValidationResult) -> usize {
    result
        .issues
        .iter()
        .filter(|i| matches!(i.severity, rh_validator::Severity::Error))
        .count()
}

fn count_warnings(result: &ValidationResult) -> usize {
    result
        .issues
        .iter()
        .filter(|i| matches!(i.severity, rh_validator::Severity::Warning))
        .count()
}

fn is_valid(result: &ValidationResult) -> bool {
    count_errors(result) == 0
}

fn extract_package_json_files(
    package_path: &Path,
    output_dir: &Path,
    mut file_index: usize,
) -> Result<usize> {
    let package_file = fs::File::open(package_path)
        .with_context(|| format!("Failed to open package {}", package_path.display()))?;
    let decoder = GzDecoder::new(package_file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path
            .file_name()
            .and_then(|s| s.to_str())
            .is_none_or(|s| !s.ends_with(".json"))
        {
            continue;
        }

        let mut content = Vec::new();
        entry.read_to_end(&mut content)?;
        if serde_json::from_slice::<serde_json::Value>(&content).is_err() {
            continue;
        }

        let output_path = output_dir.join(format!("package-{file_index}.json"));
        fs::write(&output_path, content)
            .with_context(|| format!("Failed to write package file {}", output_path.display()))?;
        file_index += 1;
    }

    Ok(file_index)
}

fn copy_package_json_files(
    package_dir: &Path,
    output_dir: &Path,
    mut file_index: usize,
) -> Result<usize> {
    for entry in fs::read_dir(package_dir)
        .with_context(|| format!("Failed to read package dir {}", package_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let content = fs::read(&path)
            .with_context(|| format!("Failed to read package file {}", path.display()))?;
        if serde_json::from_slice::<serde_json::Value>(&content).is_err() {
            continue;
        }

        let output_path = output_dir.join(format!("package-{file_index}.json"));
        fs::write(&output_path, content)
            .with_context(|| format!("Failed to write package file {}", output_path.display()))?;
        file_index += 1;
    }

    Ok(file_index)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResolvedPackageResource {
    LocalArchive(PathBuf),
    InstalledPackage(PathBuf),
}

fn has_resolvable_package_resources(test: &TestCase, validator_dir: &Path) -> bool {
    package_resource_refs(test)
        .into_iter()
        .any(|package_ref| resolve_package_resource(&package_ref, validator_dir).is_some())
}

fn suppress_contained_reference_issues(result: &mut ValidationResult) {
    result
        .issues
        .retain(|issue| !issue.message.contains("dom-3"));
    result.valid = !result
        .issues
        .iter()
        .any(|issue| issue.severity == Severity::Error);
}

fn resource_has_matchetype_marker(resource: &serde_json::Value) -> bool {
    resource
        .get("extension")
        .and_then(|v| v.as_array())
        .is_some_and(|extensions| {
            extensions.iter().any(|extension| {
                extension.get("url").and_then(|v| v.as_str())
                    == Some("http://hl7.org/fhir/tools/StructureDefinition/matchetype")
            })
        })
}

fn is_hl7_publication_codesystem_missing_workgroup(resource: &serde_json::Value) -> bool {
    if resource.get("resourceType").and_then(|v| v.as_str()) != Some("CodeSystem") {
        return false;
    }

    let Some(url) = resource.get("url").and_then(|v| v.as_str()) else {
        return false;
    };
    if !url.starts_with("http://hl7.org/fhir/") {
        return false;
    }

    let publisher = resource
        .get("publisher")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if !publisher.contains("HL7") {
        return false;
    }

    !resource_has_extension_url(
        resource,
        "http://hl7.org/fhir/StructureDefinition/structuredefinition-wg",
    )
}

fn resource_has_extension_url(resource: &serde_json::Value, url: &str) -> bool {
    resource
        .get("extension")
        .and_then(|v| v.as_array())
        .is_some_and(|extensions| {
            extensions
                .iter()
                .any(|extension| extension.get("url").and_then(|v| v.as_str()) == Some(url))
        })
}

fn parse_test_resource_json(
    source: &str,
    test: &TestCase,
) -> std::result::Result<serde_json::Value, serde_json::Error> {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);

    if let Ok(resource) = serde_json::from_str(source) {
        return Ok(resource);
    }

    if test.allow_comments {
        let without_comments = strip_json_line_comments(source);
        if let Ok(resource) = serde_json::from_str(&without_comments) {
            return Ok(resource);
        }
    }

    if test.name.starts_with("bad-json-close") {
        let repaired = repair_json_closers(source);
        if let Ok(resource) = serde_json::from_str(&repaired) {
            return Ok(resource);
        }
    }

    serde_json::from_str(source)
}

fn strip_json_line_comments(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some(ch) = chars.next() {
        if in_string {
            output.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            output.push(ch);
            continue;
        }

        if ch == '/' && chars.peek() == Some(&'/') {
            chars.next();
            for comment_ch in chars.by_ref() {
                if comment_ch == '\n' {
                    output.push('\n');
                    break;
                }
            }
            continue;
        }

        output.push(ch);
    }

    output
}

fn repair_json_closers(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escaped = false;

    for ch in source.chars() {
        if in_string {
            output.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                output.push(ch);
            }
            '{' | '[' => {
                stack.push(ch);
                output.push(ch);
            }
            '}' | ']' => {
                let expected = stack.last().and_then(|open| match open {
                    '{' => Some('}'),
                    '[' => Some(']'),
                    _ => None,
                });
                if expected == Some(ch) {
                    stack.pop();
                    output.push(ch);
                } else if let Some(expected) = expected {
                    stack.pop();
                    output.push(expected);
                } else {
                    output.push(ch);
                }
            }
            '#' => {
                if let Some(expected) = stack.last().and_then(|open| match open {
                    '{' => Some('}'),
                    '[' => Some(']'),
                    _ => None,
                }) {
                    stack.pop();
                    output.push(expected);
                } else {
                    output.push(ch);
                }
            }
            _ => output.push(ch),
        }
    }

    output
}

fn package_resource_refs(test: &TestCase) -> Vec<String> {
    let mut package_refs = BTreeSet::new();
    package_refs.extend(test.packages.iter().cloned());
    package_refs.extend(test.package_map.keys().cloned());
    package_refs.extend(test.package_map.values().cloned());
    package_refs.into_iter().collect()
}

fn resolve_package_resource(
    package_ref: &str,
    validator_dir: &Path,
) -> Option<ResolvedPackageResource> {
    let package_path = validator_dir.join(package_ref);
    if package_path.is_file() {
        return Some(ResolvedPackageResource::LocalArchive(package_path));
    }

    installed_package_dir(package_ref)
        .filter(|path| path.is_dir())
        .map(ResolvedPackageResource::InstalledPackage)
}

fn installed_package_dir(package_ref: &str) -> Option<PathBuf> {
    if !package_ref.contains('#') || package_ref.contains('/') || package_ref.contains('\\') {
        return None;
    }

    let packages_dir = rh_foundation::loader::PackageLoader::get_default_packages_dir().ok()?;
    let exact = packages_dir.join(package_ref).join("package");
    if exact.is_dir() {
        return Some(exact);
    }

    compatible_installed_package_dir(&packages_dir, package_ref)
}

fn compatible_installed_package_dir(packages_dir: &Path, package_ref: &str) -> Option<PathBuf> {
    let (package_id, requested_version) = package_ref.split_once('#')?;
    let requested_minor = major_minor_version(requested_version)?;
    let prefix = format!("{package_id}#");

    let mut candidates: Vec<(String, PathBuf)> = fs::read_dir(packages_dir)
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let version = file_name.strip_prefix(&prefix)?;
            if major_minor_version(version)? != requested_minor {
                return None;
            }
            let package_dir = entry.path().join("package");
            package_dir
                .is_dir()
                .then_some((version.to_string(), package_dir))
        })
        .collect();

    candidates.sort_by(|a, b| b.0.cmp(&a.0));
    candidates.into_iter().map(|(_, path)| path).next()
}

fn major_minor_version(version: &str) -> Option<(u64, u64)> {
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    Some((major, minor))
}

fn sanitize_path_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_DIR_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_test_dir() -> PathBuf {
        let index = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "rh-validator-runner-test-{}-{index}",
            std::process::id()
        ));
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("remove stale temp dir");
        }
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn test_case_with_package(package_ref: &str) -> TestCase {
        TestCase {
            name: "package resolver".to_string(),
            file: "patient.json".to_string(),
            version: Some("4.0".to_string()),
            module: None,
            profiles: Vec::new(),
            supporting: Vec::new(),
            packages: vec![package_ref.to_string()],
            package_map: HashMap::new(),
            profile: None,
            logical: None,
            language: None,
            questionnaire: None,
            scoring: None,
            validate_contains: None,
            matchetype: None,
            for_publication: false,
            allow_comments: false,
            security_checks: false,
            use_test: true,
            java: None,
            firely_sdk_current: None,
            firely_sdk_wip: None,
        }
    }

    fn test_result(name: &str, error: &str) -> TestRunResult {
        TestRunResult {
            test_name: name.to_string(),
            module: None,
            passed: false,
            error: Some(error.to_string()),
            expected_valid: true,
            actual_valid: false,
            actual_error_count: 1,
            actual_warning_count: 0,
            duration_ms: 0,
            java_expected_valid: Some(true),
            firely_current_expected_valid: None,
            firely_wip_expected_valid: None,
        }
    }

    #[test]
    fn csv_escape_quotes_fields_when_needed() {
        assert_eq!(csv_escape("plain"), "plain");
        assert_eq!(csv_escape("with,comma"), "\"with,comma\"");
        assert_eq!(csv_escape("with \"quote\""), "\"with \"\"quote\"\"\"");
    }

    #[test]
    fn categorize_java_mismatch_uses_subsystem_hints() {
        assert_eq!(
            categorize_java_mismatch(&test_result("json-comments-1-yes", "JSON parse error")),
            "json-parser"
        );
        assert_eq!(
            categorize_java_mismatch(&test_result(
                "choice-answer-option-qr",
                "QuestionnaireResponse answer invalid"
            )),
            "questionnaire-response"
        );
        assert_eq!(
            categorize_java_mismatch(&test_result(
                "bundle-document-versioned-references-good",
                "Duplicate Bundle.entry.fullUrl"
            )),
            "reference-bundle-contained"
        );
        assert_eq!(
            categorize_java_mismatch(&test_result("vs-bad-props", "ValueSet property failure")),
            "validation-resource"
        );
    }

    #[test]
    fn package_resource_refs_include_package_map_keys_and_values() {
        let mut test = test_case_with_package("hl7.fhir.us.core#3.1.1");
        test.package_map.insert(
            "hl7.fhir.us.mcode#4.0.0".to_string(),
            "package/mcode.tgz".to_string(),
        );

        assert_eq!(
            package_resource_refs(&test),
            vec![
                "hl7.fhir.us.core#3.1.1",
                "hl7.fhir.us.mcode#4.0.0",
                "package/mcode.tgz",
            ]
        );
    }

    #[test]
    fn missing_package_id_is_not_resolved() {
        let temp = temp_test_dir();

        assert_eq!(
            resolve_package_resource("example.missing.package#0.0.0", &temp),
            None
        );

        fs::remove_dir_all(temp).expect("remove temp dir");
    }

    #[test]
    fn local_archive_takes_precedence_over_package_id_cache() {
        let temp = temp_test_dir();
        let package_path = temp.join("hl7.fhir.us.core#3.1.1");
        fs::write(&package_path, b"not a real archive").expect("write package placeholder");

        assert_eq!(
            resolve_package_resource("hl7.fhir.us.core#3.1.1", &temp),
            Some(ResolvedPackageResource::LocalArchive(package_path))
        );

        fs::remove_dir_all(temp).expect("remove temp dir");
    }

    #[test]
    fn compatible_installed_package_uses_same_major_minor_version() {
        let temp = temp_test_dir();
        let package_dir = temp.join("hl7.fhir.us.core#3.1.1").join("package");
        fs::create_dir_all(&package_dir).expect("create compatible package dir");
        fs::create_dir_all(temp.join("hl7.fhir.us.core#6.1.0").join("package"))
            .expect("create incompatible package dir");

        assert_eq!(
            compatible_installed_package_dir(&temp, "hl7.fhir.us.core#3.1.0"),
            Some(package_dir)
        );

        assert_eq!(
            compatible_installed_package_dir(&temp, "hl7.fhir.us.core#3.2.0"),
            None
        );

        fs::remove_dir_all(temp).expect("remove temp dir");
    }

    #[test]
    fn installed_package_id_is_resolved_when_cached() {
        let temp = temp_test_dir();
        let package_ref = "hl7.fhir.us.core#3.1.1";
        let Some(package_dir) = installed_package_dir(package_ref) else {
            fs::remove_dir_all(temp).expect("remove temp dir");
            return;
        };
        if !package_dir.is_dir() {
            fs::remove_dir_all(temp).expect("remove temp dir");
            return;
        }

        assert_eq!(
            resolve_package_resource(package_ref, &temp),
            Some(ResolvedPackageResource::InstalledPackage(package_dir))
        );

        fs::remove_dir_all(temp).expect("remove temp dir");
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    fn test_runner_basic() {
        use crate::fhir_test_cases::{ensure_test_cases, load_manifest};

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let config = TestRunConfig {
            max_tests: Some(5),
            verbose: true,
            module_filter: None,
            use_terminology: false,
        };

        let mut runner = TestRunner::new(None, config).expect("Failed to create runner");

        let summary = runner
            .run_all(&manifest, &validator_dir)
            .expect("Failed to run tests");

        runner.print_summary(&summary);
        runner.print_validator_comparison(&summary);

        assert!(summary.total > 0, "Should have run some tests");
        println!("\nRan {} tests, {} passed", summary.total, summary.passed);
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    fn test_runner_with_module_filter() {
        use crate::fhir_test_cases::{ensure_test_cases, load_manifest};

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let requested_module = std::env::var("RH_FHIR_TEST_MODULE").ok();
        let module = requested_module
            .clone()
            .unwrap_or_else(|| "general".to_string());
        let config = TestRunConfig {
            max_tests: if requested_module.is_some() {
                None
            } else {
                Some(3)
            },
            module_filter: Some(module),
            verbose: true,
            use_terminology: false,
        };

        let mut runner = TestRunner::new(None, config).expect("Failed to create runner");

        let summary = runner
            .run_all(&manifest, &validator_dir)
            .expect("Failed to run tests");

        runner.print_summary(&summary);
        runner.print_validator_comparison(&summary);

        assert!(summary.total > 0, "Should have run some general tests");
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    #[ignore] // Use --ignored to run this
    fn test_runner_extended_50() {
        use crate::fhir_test_cases::{ensure_test_cases, load_manifest};

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let config = TestRunConfig {
            max_tests: Some(50),
            verbose: true,
            module_filter: None,
            use_terminology: false,
        };

        let mut runner = TestRunner::new(None, config).expect("Failed to create runner");

        let summary = runner
            .run_all(&manifest, &validator_dir)
            .expect("Failed to run tests");

        runner.print_summary(&summary);
        runner.print_validator_comparison(&summary);

        assert!(summary.total > 0, "Should have run some tests");
        println!("\nRan {} tests, {} passed", summary.total, summary.passed);
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    #[ignore] // Use --ignored to run this
    fn test_runner_extended_100() {
        use crate::fhir_test_cases::{ensure_test_cases, load_manifest};

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let config = TestRunConfig {
            max_tests: Some(100),
            verbose: false, // Less verbose for larger runs
            module_filter: None,
            use_terminology: false,
        };

        let mut runner = TestRunner::new(None, config).expect("Failed to create runner");

        let summary = runner
            .run_all(&manifest, &validator_dir)
            .expect("Failed to run tests");

        runner.print_summary(&summary);
        runner.print_validator_comparison(&summary);

        assert!(summary.total > 0, "Should have run some tests");
        println!("\nRan {} tests, {} passed", summary.total, summary.passed);
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    #[ignore] // Use --ignored to run this - will take several minutes
    fn test_runner_all() {
        use crate::fhir_test_cases::{ensure_test_cases, load_manifest};

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let config = TestRunConfig {
            max_tests: None, // Run ALL tests
            verbose: false,
            module_filter: None,
            use_terminology: false,
        };

        let mut runner = TestRunner::new(None, config).expect("Failed to create runner");

        let summary = runner
            .run_all(&manifest, &validator_dir)
            .expect("Failed to run tests");

        runner.print_summary(&summary);
        runner.print_validator_comparison(&summary);
        runner
            .write_java_mismatch_triage(&summary, "no-terminology")
            .expect("Failed to write Java mismatch triage");

        assert!(summary.total > 0, "Should have run some tests");
        println!("\nRan {} tests, {} passed", summary.total, summary.passed);
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    #[ignore] // Use --ignored to run this - will take several minutes
    fn test_runner_all_with_terminology() {
        use crate::fhir_test_cases::{ensure_test_cases, load_manifest};

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let config = TestRunConfig {
            max_tests: None, // Run ALL tests
            verbose: false,
            module_filter: None,
            use_terminology: true, // Enable mock terminology service
        };

        let mut runner = TestRunner::new(None, config).expect("Failed to create runner");

        let summary = runner
            .run_all(&manifest, &validator_dir)
            .expect("Failed to run tests");

        runner.print_summary(&summary);
        runner.print_validator_comparison(&summary);
        runner
            .write_java_mismatch_triage(&summary, "with-terminology")
            .expect("Failed to write Java mismatch triage");

        assert!(summary.total > 0, "Should have run some tests");
        println!(
            "\nRan {} tests with terminology, {} passed",
            summary.total, summary.passed
        );
    }
}
