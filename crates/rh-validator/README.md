# rh-validator

FHIR resource validation for Rust and the `rh` CLI.

## Overview

`rh-validator` validates JSON FHIR resources against base R4 rules and loaded StructureDefinition profiles. The crate combines structural checks with profile-driven rules, FHIRPath invariant evaluation, terminology hooks, and QuestionnaireResponse validation.

Current library exports include:

- `FhirValidator`, `ValidationOptions`, and `FhirVersion`
- `ValidationResult`, `ValidationIssue`, `Severity`, and `IssueCode`
- `ProfileRegistry`, `RuleCompiler`, and ValueSet helpers
- Terminology service traits and configs such as `TerminologyConfig`
- Questionnaire loading and `QuestionnaireResponseValidator`

## What it validates

- Base JSON/FHIR structure such as `resourceType`, empty arrays, ids, attachments, and selected resource-specific rules
- Profile cardinality, type, slicing, extension, and invariant rules via StructureDefinition snapshots
- ValueSet bindings and terminology display checks when a terminology service is configured
- UCUM unit validation for supported quantity paths
- QuestionnaireResponse resources against a linked Questionnaire when that Questionnaire can be resolved

## Quick start

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
rh-validator = { path = "../rh-validator" }
serde_json = "1"
```

Validate a resource with base R4 rules:

```rust
use rh_validator::{FhirValidator, FhirVersion};
use serde_json::json;

# fn main() -> anyhow::Result<()> {
let validator = FhirValidator::new(FhirVersion::R4, None)?;

let patient = json!({
    "resourceType": "Patient",
    "id": "example",
    "name": [{"family": "Doe", "given": ["Jane"]}]
});

let result = validator.validate(&patient)?;
assert!(result.valid);
# Ok(())
# }
```

Validate with loaded packages and explicit profile selection:

```rust
use rh_validator::{FhirValidator, FhirVersion};
use serde_json::json;

# fn main() -> anyhow::Result<()> {
let validator = FhirValidator::new(FhirVersion::R4, Some("~/.fhir/packages"))?;

let patient = json!({
    "resourceType": "Patient",
    "meta": {
        "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
    },
    "name": [{"family": "Doe", "given": ["Jane"]}]
});

let result = validator.validate_with_profile(
    &patient,
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
)?;

for issue in &result.issues {
    println!("{} {} {}", issue.severity, issue.code, issue.message);
}
# Ok(())
# }
```

Enable security checks or terminology validation:

```rust
use rh_validator::{FhirValidator, FhirVersion, TerminologyConfig, ValidationOptions};

# fn main() -> anyhow::Result<()> {
let validator = FhirValidator::with_options(
    FhirVersion::R4,
    None,
    Some(TerminologyConfig::with_server("https://tx.fhir.org/r4")),
    ValidationOptions {
        security_checks: true,
    },
)?;
# Ok(())
# }
```

## Validation APIs

- `validate(&resource)` runs base validation only
- `validate_auto(&resource)` runs base validation and then applies any profiles declared in `meta.profile`
- `validate_with_profile(&resource, profile_url)` validates against one explicit profile
- `validate_with_profiles(&resource, &[...])` validates against multiple explicit profiles

`ValidationResult` contains `valid` plus a list of `ValidationIssue` values with severity, code, message, and optional path details.

## Terminology

Terminology validation is optional. Without a terminology service, the validator still performs structural and profile validation, but remote code/display checks are skipped.

Available integration points include:

- `TerminologyConfig::with_server(...)` for a remote FHIR terminology endpoint
- `TerminologyConfig::mock()` for tests
- `CachedTerminologyService` for cached lookups

## Performance and caching

The validator caches compiled profile rules and resolved snapshots. You can inspect cache behavior with:

```rust
# use rh_validator::{FhirValidator, FhirVersion};
# fn main() -> anyhow::Result<()> {
let validator = FhirValidator::new(FhirVersion::R4, None)?;
let (profile_hits, profile_misses, profile_rate, rule_hits, rule_misses, rule_rate) =
    validator.cache_metrics();

println!(
    "profiles: {} hits / {} misses ({:.1}%)",
    profile_hits,
    profile_misses,
    profile_rate * 100.0
);
println!(
    "rules: {} hits / {} misses ({:.1}%)",
    rule_hits,
    rule_misses,
    rule_rate * 100.0
);
# Ok(())
# }
```

## CLI

The workspace CLI exposes this crate through:

- [`apps/rh-cli/docs/VALIDATOR.md`](../../apps/rh-cli/docs/VALIDATOR.md)
- `rh validate resource`
- `rh validate batch`

## Planning and analysis artifacts

Longer-running planning notes and conformance analysis now live under `openspec/planning/rh-validator/`:

- [`TODO.md`](../../openspec/planning/rh-validator/TODO.md)
- [`PHASE_14_PLAN.md`](../../openspec/planning/rh-validator/PHASE_14_PLAN.md)
- [`PHASE_15_ANALYSIS.md`](../../openspec/planning/rh-validator/PHASE_15_ANALYSIS.md)
- [`PHASE_15_COMPLETE.md`](../../openspec/planning/rh-validator/PHASE_15_COMPLETE.md)
- [`FALSE_NEGATIVES_ANALYSIS.md`](FALSE_NEGATIVES_ANALYSIS.md)
