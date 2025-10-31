use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "test-cases")]
    pub test_cases: Vec<TestCase>,
    #[serde(default)]
    pub versions: HashMap<String, String>,
    #[serde(default)]
    pub modules: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub file: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    #[serde(default)]
    pub profiles: Vec<String>,
    #[serde(default)]
    pub supporting: Vec<String>,
    #[serde(default)]
    pub profile: Option<ProfileTest>,
    #[serde(default)]
    pub logical: Option<LogicalTest>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub questionnaire: Option<String>,
    #[serde(rename = "use-test", default = "default_true")]
    pub use_test: bool,
    #[serde(default)]
    pub java: Option<ValidatorOutcome>,
    #[serde(rename = "firely-sdk-current", default)]
    pub firely_sdk_current: Option<ValidatorOutcome>,
    #[serde(rename = "firely-sdk-wip", default)]
    pub firely_sdk_wip: Option<ValidatorOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorOutcome {
    Path(String),
    Inline(InlineOutcome),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineOutcome {
    #[serde(default)]
    pub error_count: Option<usize>,
    #[serde(default)]
    pub warning_count: Option<usize>,
    #[serde(default)]
    pub info_count: Option<usize>,
    #[serde(default)]
    pub output: Vec<serde_json::Value>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileTest {
    pub source: String,
    #[serde(default)]
    pub supporting: Vec<String>,
    #[serde(default)]
    pub java: Option<ValidatorOutcome>,
    #[serde(rename = "firely-sdk-current", default)]
    pub firely_sdk_current: Option<ValidatorOutcome>,
    #[serde(rename = "firely-sdk-wip", default)]
    pub firely_sdk_wip: Option<ValidatorOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalTest {
    #[serde(default)]
    pub supporting: Vec<String>,
    #[serde(default)]
    pub expressions: Vec<String>,
    #[serde(default)]
    pub packages: Vec<String>,
    #[serde(default)]
    pub java: Option<ValidatorOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedOutcome {
    pub resource_type: String,
    #[serde(default)]
    pub issue: Vec<OutcomeIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeIssue {
    pub severity: String,
    pub code: String,
    #[serde(default)]
    pub details: Option<IssueDetails>,
    #[serde(default)]
    pub diagnostics: Option<String>,
    #[serde(default)]
    pub expression: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueDetails {
    #[serde(default)]
    pub text: Option<String>,
}

#[allow(dead_code)] // Methods will be used in test runner (task 12.3)
impl TestCase {
    pub fn is_r4(&self) -> bool {
        self.version.as_deref() == Some("4.0")
    }

    pub fn is_json(&self) -> bool {
        self.file.ends_with(".json")
    }

    pub fn should_run(&self) -> bool {
        self.use_test && self.is_r4() && self.is_json()
    }

    pub fn get_expected_outcome_path(&self, validator_dir: &Path) -> Option<PathBuf> {
        match &self.java {
            Some(ValidatorOutcome::Path(path)) => Some(validator_dir.join("outcomes").join(path)),
            _ => None,
        }
    }

    pub fn get_inline_outcome(&self) -> Option<&InlineOutcome> {
        match &self.java {
            Some(ValidatorOutcome::Inline(outcome)) => Some(outcome),
            _ => None,
        }
    }

    pub fn get_test_file_path(&self, validator_dir: &Path) -> PathBuf {
        validator_dir.join(&self.file)
    }

    pub fn get_profile_expected_outcome_path(&self, validator_dir: &Path) -> Option<PathBuf> {
        let profile = self.profile.as_ref()?;
        match &profile.java {
            Some(ValidatorOutcome::Path(path)) => Some(validator_dir.join("outcomes").join(path)),
            _ => None,
        }
    }

    pub fn get_profile_source_path(&self, validator_dir: &Path) -> Option<PathBuf> {
        self.profile.as_ref().map(|p| validator_dir.join(&p.source))
    }

    pub fn get_supporting_files(&self, validator_dir: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for profile in &self.profiles {
            files.push(validator_dir.join(profile));
        }

        for supporting in &self.supporting {
            files.push(validator_dir.join(supporting));
        }

        if let Some(profile) = &self.profile {
            for supporting in &profile.supporting {
                files.push(validator_dir.join(supporting));
            }
        }

        files
    }

    /// Get expected validity for Java validator.
    /// Returns None if no outcome is specified for this validator.
    pub fn get_java_expected_valid(&self, validator_dir: &Path) -> Option<bool> {
        match &self.java {
            Some(ValidatorOutcome::Path(path)) => {
                let outcome_path = validator_dir.join("outcomes").join(path);
                if outcome_path.exists() {
                    load_expected_outcome(&outcome_path)
                        .ok()
                        .map(|o| o.is_valid())
                } else {
                    None
                }
            }
            Some(ValidatorOutcome::Inline(outcome)) => Some(outcome.is_valid()),
            None => None,
        }
    }

    /// Get expected validity for Firely SDK (current) validator.
    pub fn get_firely_current_expected_valid(&self, validator_dir: &Path) -> Option<bool> {
        match &self.firely_sdk_current {
            Some(ValidatorOutcome::Path(path)) => {
                let outcome_path = validator_dir.join("outcomes").join(path);
                if outcome_path.exists() {
                    load_expected_outcome(&outcome_path)
                        .ok()
                        .map(|o| o.is_valid())
                } else {
                    None
                }
            }
            Some(ValidatorOutcome::Inline(outcome)) => Some(outcome.is_valid()),
            None => None,
        }
    }

    /// Get expected validity for Firely SDK (WIP) validator.
    pub fn get_firely_wip_expected_valid(&self, validator_dir: &Path) -> Option<bool> {
        match &self.firely_sdk_wip {
            Some(ValidatorOutcome::Path(path)) => {
                let outcome_path = validator_dir.join("outcomes").join(path);
                if outcome_path.exists() {
                    load_expected_outcome(&outcome_path)
                        .ok()
                        .map(|o| o.is_valid())
                } else {
                    None
                }
            }
            Some(ValidatorOutcome::Inline(outcome)) => Some(outcome.is_valid()),
            None => None,
        }
    }
}

pub fn load_manifest(validator_dir: &Path) -> Result<Manifest> {
    let manifest_path = validator_dir.join("manifest.json");

    let content = fs::read_to_string(&manifest_path)
        .with_context(|| format!("Failed to read manifest at {}", manifest_path.display()))?;

    let manifest: Manifest =
        serde_json::from_str(&content).context("Failed to parse manifest.json")?;

    Ok(manifest)
}

pub fn load_expected_outcome(path: &Path) -> Result<ExpectedOutcome> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read expected outcome at {}", path.display()))?;

    let outcome: ExpectedOutcome =
        serde_json::from_str(&content).context("Failed to parse expected outcome JSON")?;

    Ok(outcome)
}

#[allow(dead_code)] // Will be used in test runner (task 12.3)
pub fn load_test_resource(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read test resource at {}", path.display()))
}

#[allow(dead_code)] // Methods will be used in test runner (task 12.3)
impl Manifest {
    pub fn r4_test_cases(&self) -> impl Iterator<Item = &TestCase> {
        self.test_cases
            .iter()
            .filter(|tc| tc.is_r4() && tc.should_run())
    }

    pub fn count_by_module(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for test in self.r4_test_cases() {
            let module = test.module.as_deref().unwrap_or("(default)");
            *counts.entry(module.to_string()).or_insert(0) += 1;
        }
        counts
    }

    pub fn modules(&self) -> Vec<String> {
        let mut modules: Vec<_> = self
            .r4_test_cases()
            .filter_map(|tc| tc.module.clone())
            .collect();
        modules.sort();
        modules.dedup();
        modules
    }
}

impl ExpectedOutcome {
    pub fn error_count(&self) -> usize {
        self.issue.iter().filter(|i| i.severity == "error").count()
    }

    pub fn warning_count(&self) -> usize {
        self.issue
            .iter()
            .filter(|i| i.severity == "warning")
            .count()
    }

    pub fn info_count(&self) -> usize {
        self.issue
            .iter()
            .filter(|i| i.severity == "information")
            .count()
    }

    pub fn total_issues(&self) -> usize {
        self.issue.len()
    }

    /// Returns true if the resource is expected to be valid (no errors).
    /// Per FHIR spec, a resource is invalid if there are any error-level issues.
    pub fn is_valid(&self) -> bool {
        self.error_count() == 0
    }
}

impl InlineOutcome {
    /// Returns true if the inline outcome indicates validity (no errors).
    pub fn is_valid(&self) -> bool {
        self.error_count.unwrap_or(0) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    fn test_load_manifest() {
        use crate::fhir_test_cases::ensure_test_cases;

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        println!("\n=== Manifest Statistics ===");
        println!("Total test cases: {}", manifest.test_cases.len());

        let r4_count: usize = manifest.test_cases.iter().filter(|tc| tc.is_r4()).count();
        println!("R4 test cases: {r4_count}");

        let runnable = manifest.r4_test_cases().count();
        println!("R4 runnable tests: {runnable}");

        let modules = manifest.count_by_module();
        println!("\nTests by module:");
        let mut sorted: Vec<_> = modules.iter().collect();
        sorted.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (module, count) in sorted.iter().take(10) {
            println!("  {module}: {count}");
        }

        assert!(!manifest.test_cases.is_empty());
        assert!(r4_count > 0);
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    fn test_parse_test_case() {
        use crate::fhir_test_cases::ensure_test_cases;

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let test = manifest
            .r4_test_cases()
            .next()
            .expect("No R4 test cases found");

        println!("\n=== Sample Test Case ===");
        println!("Name: {}", test.name);
        println!("File: {}", test.file);
        println!("Version: {:?}", test.version);
        println!("Module: {:?}", test.module);

        let test_file = test.get_test_file_path(&validator_dir);
        assert!(test_file.exists(), "Test file should exist");

        if let Some(outcome_path) = test.get_expected_outcome_path(&validator_dir) {
            if outcome_path.exists() {
                let outcome =
                    load_expected_outcome(&outcome_path).expect("Failed to load expected outcome");
                println!("Expected errors: {}", outcome.error_count());
                println!("Expected warnings: {}", outcome.warning_count());
                println!("Expected info: {}", outcome.info_count());
            }
        }
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    fn test_parse_expected_outcome() {
        use crate::fhir_test_cases::ensure_test_cases;

        let cache_dir = ensure_test_cases().expect("Failed to get test cases");
        let validator_dir = cache_dir.join("validator");

        let manifest = load_manifest(&validator_dir).expect("Failed to load manifest");

        let test = manifest
            .r4_test_cases()
            .find(|tc| {
                tc.get_expected_outcome_path(&validator_dir)
                    .map(|p| p.exists())
                    .unwrap_or(false)
            })
            .expect("No test with expected outcome found");

        let outcome_path = test.get_expected_outcome_path(&validator_dir).unwrap();
        let outcome =
            load_expected_outcome(&outcome_path).expect("Failed to parse expected outcome");

        println!("\n=== Expected Outcome ===");
        println!("Test: {}", test.name);
        println!("Resource type: {}", outcome.resource_type);
        println!("Total issues: {}", outcome.total_issues());
        println!("Errors: {}", outcome.error_count());
        println!("Warnings: {}", outcome.warning_count());
        println!("Info: {}", outcome.info_count());

        if let Some(first_issue) = outcome.issue.first() {
            println!("\nFirst issue:");
            println!("  Severity: {}", first_issue.severity);
            println!("  Code: {}", first_issue.code);
            if let Some(details) = &first_issue.details {
                if let Some(text) = &details.text {
                    println!("  Details: {text}");
                }
            }
        }

        assert_eq!(outcome.resource_type, "OperationOutcome");
    }
}
