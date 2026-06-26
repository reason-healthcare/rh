use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::Instant;
use tar::Archive;

use super::parser::{ExpectedOutcome, Manifest, TestCase};
use rh_validator::{FhirValidator, TerminologyConfig, ValidationResult};

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

        let has_package_resources = has_local_package_archives(test, validator_dir);
        let mut package_validator = if has_package_resources {
            Some(self.build_package_validator(test, validator_dir)?)
        } else {
            None
        };
        let validator = package_validator.as_mut().unwrap_or(&mut self.validator);
        let skip_dynamic_profiles = has_package_resources;

        // Register supporting files from `supporting`, `profiles`, and `profile.supporting` fields.
        // Package-backed tests load StructureDefinitions through the static package directory path
        // so snapshots include package dependencies.
        let all_supporting = test.get_supporting_files(validator_dir);

        // Also add the profile source itself if present
        let profile_source = test.get_profile_source_path(validator_dir);

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
                            _ => {}
                        }
                    }
                }
            }
        }

        let resource_json = std::fs::read_to_string(&test_file_path)
            .with_context(|| format!("Failed to read test file: {}", test_file_path.display()))?;

        let resource: serde_json::Value = match serde_json::from_str(&resource_json) {
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

        let validation_result = validator.validate_auto(&resource)?;

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

        let mut package_files = BTreeSet::new();
        package_files.extend(test.packages.iter().cloned());
        package_files.extend(test.package_map.values().cloned());

        let mut file_index = 0usize;
        for package_file in package_files {
            let package_path = validator_dir.join(&package_file);
            if !package_path.is_file() {
                continue;
            }
            file_index = extract_package_json_files(&package_path, &package_dir, file_index)
                .with_context(|| format!("Failed to extract package {}", package_path.display()))?;
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

fn has_local_package_archives(test: &TestCase, validator_dir: &Path) -> bool {
    test.packages
        .iter()
        .chain(test.package_map.values())
        .any(|package_file| validator_dir.join(package_file).is_file())
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

        assert!(summary.total > 0, "Should have run some tests");
        println!(
            "\nRan {} tests with terminology, {} passed",
            summary.total, summary.passed
        );
    }
}
