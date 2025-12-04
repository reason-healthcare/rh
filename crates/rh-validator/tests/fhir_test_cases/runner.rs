use anyhow::{Context, Result};
use std::path::Path;
use std::time::Instant;

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
                        actual_error_count: 0,
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

        // Register supporting files from `supporting`, `profiles`, and `profile.supporting` fields
        // Also include the profile source if present
        let all_supporting = test.get_supporting_files(validator_dir);

        // Also add the profile source itself if present
        let profile_source = test.get_profile_source_path(validator_dir);

        for supporting_path in all_supporting.into_iter().chain(profile_source.into_iter()) {
            if supporting_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&supporting_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        match json.get("resourceType").and_then(|v| v.as_str()) {
                            Some("Questionnaire") => {
                                self.validator.register_questionnaire(&json);
                            }
                            Some("ValueSet") => {
                                self.validator.register_valueset(&json);
                            }
                            Some("StructureDefinition") => {
                                if self.config.verbose {
                                    if let Some(url) = json.get("url").and_then(|u| u.as_str()) {
                                        println!("    Registering profile: {url}");
                                    }
                                }
                                self.validator.register_profile(&json);
                            }
                            Some("CodeSystem") => {
                                self.validator.register_codesystem(&json);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let resource_json = std::fs::read_to_string(&test_file_path)
            .with_context(|| format!("Failed to read test file: {}", test_file_path.display()))?;

        let resource: serde_json::Value = serde_json::from_str(&resource_json)
            .with_context(|| format!("Failed to parse JSON: {}", test_file_path.display()))?;

        let validation_result = self.validator.validate_auto(&resource)?;

        let expected_outcome = self.load_expected_outcome(test, validator_dir)?;

        let duration = start.elapsed();

        let expected_valid = expected_outcome.is_valid();
        let actual_valid = is_valid(&validation_result);

        let passed = expected_valid == actual_valid;
        let error = if !passed {
            Some(format!(
                "Validation mismatch: expected {}, got {}",
                if expected_valid { "VALID" } else { "INVALID" },
                if actual_valid { "VALID" } else { "INVALID" }
            ))
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
            actual_error_count: count_errors(&validation_result),
            actual_warning_count: count_warnings(&validation_result),
            duration_ms: duration.as_millis(),
            java_expected_valid,
            firely_current_expected_valid,
            firely_wip_expected_valid,
        })
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

        let config = TestRunConfig {
            max_tests: Some(3),
            module_filter: Some("general".to_string()),
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
