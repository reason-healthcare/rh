//! FHIR resource validation library.
pub mod fhir_version;
pub mod profile;
pub mod questionnaire;
pub mod rules;
pub mod terminology;
pub mod types;
pub mod validator;
pub mod valueset;

pub use fhir_version::FhirVersion;
pub use profile::ProfileRegistry;
pub use questionnaire::{Questionnaire, QuestionnaireLoader, QuestionnaireResponseValidator};
pub use rules::{
    BindingRule, CardinalityRule, CompiledValidationRules, InvariantRule, RuleCompiler, TypeRule,
};
pub use terminology::{
    CachedTerminologyService, MockTerminologyService, TerminologyConfig, TerminologyError,
    TerminologyService, ValidateCodeResult,
};
pub use types::{IssueCode, Location, Severity, ValidationIssue, ValidationResult};
pub use validator::FhirValidator;
pub use valueset::{ValueSet, ValueSetExpansion, ValueSetLoader};
