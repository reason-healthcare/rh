# RH Validator Implementation Roadmap

**Status:** Planning → Implementation  
**Design Document:** [DESIGN.md](DESIGN.md)  
**Start Date:** October 23, 2025

## Overview

This document tracks the implementation of the RH FHIR validator as described in DESIGN.md. The implementation will replace existing code with a clean, performance-focused architecture.

---

## Phase 0: Cleanup & Foundation

**Goal:** Remove existing code and establish clean foundation  
**Duration:** 1-2 days  
**Status:** ✅ Complete

### Tasks

- [x] **Remove existing validation code**
  - [x] Delete current `src/` implementation
  - [x] Keep only `Cargo.toml`, `README.md`, `DESIGN.md`, `TODO.md`
  - [x] Update `Cargo.toml` dependencies for new design
  - [x] Ensure workspace still compiles (crate can be empty for now)

- [x] **Define core types**
  - [x] Create `src/types.rs` with core validation types:
    - [x] `ValidationResult`
    - [x] `ValidationIssue`
    - [x] `Severity` enum (with custom ordering: Error > Warning > Information)
    - [x] `IssueCode` enum
    - [x] `Invariant` struct (will be shared with codegen)
    - [x] `ValidatorError` enum
  - [x] Add comprehensive doc comments
  - [x] Add unit tests for type constructors and utilities (8 tests)
  - [x] Add benchmark stubs in `benches/validation.rs`

- [x] **Update dependencies**
  - [x] Removed unnecessary dependencies (dirs, glob, tokio, tracing, rh-loader, rh-codegen)
  - [x] Added criterion for benchmarking
  - [x] Kept minimal deps: serde, serde_json, anyhow, thiserror, rh-foundation

### Success Criteria
- [x] Empty crate compiles successfully
- [x] Core types defined with full documentation
- [x] All tests pass: `cargo test -p rh-validator` (8/8 tests passing)
- [x] No clippy warnings: `cargo clippy -p rh-validator`

**Completion Date:** [Current Session]


---

## Phase 1: Structural Validation via Deserialization

**Goal:** Implement type-based structural validation  
**Duration:** 3-5 days  
**Status:** ✅ Complete  
**Depends On:** Phase 0

### Tasks

- [x] **Create validator struct**
  - [x] `src/validator.rs` with `FhirValidator` struct
  - [x] Basic configuration options (`ValidatorConfig`)
  - [x] Builder pattern for configuration
  - [x] Constructor with sensible defaults

- [x] **Implement JSON validation**
  - [x] `validate_json<T: DeserializeOwned>()` method
  - [x] Convert serde errors to `ValidationIssue`s
  - [x] Map error types (missing fields, type mismatches, etc.)
  - [x] Preserve JSON path information in errors
  - [x] Add comprehensive error context

- [x] **Implement typed validation**
  - [x] `validate<T: Serialize>()` method for already-parsed resources
  - [x] Handle re-serialization for FHIRPath (future)
  - [x] Efficient validation without double-parsing

- [x] **Error mapping**
  - [x] Create utility to convert serde errors to ValidationIssue
  - [x] Classify error types (structural, type, cardinality)
  - [x] Extract JSON paths from serde error messages
  - [x] Generate human-readable error messages

- [x] **Testing**
  - [x] Test valid resources (should pass)
  - [x] Test missing required fields (should fail)
  - [x] Test wrong field types (should fail)
  - [x] Test cardinality violations (should fail)
  - [x] Test unknown fields (should warn)
  - [x] Integration test with real FHIR examples (9 tests)

### Success Criteria
- [x] Can validate Patient, Observation, Bundle from JSON
- [x] All structural errors are caught and reported
- [x] Error messages are clear and actionable
- [x] 100% of structural validation tests pass (24/24 tests)
- [x] Performance: < 1ms for simple resources

**Completion Date:** October 23, 2025

---

## Phase 2: Code Generation - Invariant Metadata

**Goal:** Generate invariant metadata in `rh-hl7_fhir_r4_core`  
**Duration:** 5-7 days  
**Status:** 🔄 In Progress  
**Depends On:** Phase 0

### Tasks

- [x] **Define shared invariant types**
  - [x] Move `Invariant` and `Severity` to `rh-foundation/src/validation.rs`
  - [x] Ensure accessible to both codegen and validator
  - [x] Add serde support and tests
  - [x] Remove duplicate definitions from rh-validator

- [x] **Extract invariants from StructureDefinitions**
  - [x] Update `rh-codegen` to parse `constraint` elements from FHIR StructureDefinitions
  - [x] Add `ElementConstraint` type to `fhir_types.rs`
  - [x] Create `invariants.rs` module with extraction logic
  - [x] Extract: key, severity, human description, FHIRPath expression
  - [x] **Only handle FHIRPath expressions** (skip xpath - legacy)
  - [x] Map FHIR severity ("error", "warning") to Rust `Severity` enum
  - [x] Deduplicate invariants across snapshot and differential
  - [x] Sort invariants by key
  - [x] Comprehensive tests (7 tests)

- [x] **Generate invariant constants**
  - [x] For each resource/datatype, generate `pub const INVARIANTS: &[Invariant]`
  - [x] Place in same file as type definition in rh-hl7_fhir_r4_core
  - [x] Generate doc comments explaining each invariant
  - [x] Format code for readability
  - [x] Created `InvariantGenerator` with string and token-based generation
  - [x] Integrated into `FileGenerator` to add constants after Default impls
  - [x] Conditionally adds `rh_foundation::Invariant` import
  - [x] Comprehensive tests (8 tests) covering escaping and edge cases

- [x] **Generate ValidatableResource trait**
  - [x] Define trait in `rh-hl7_fhir_r4_core/src/validation.rs` (via rh-codegen)
  - [x] Methods: `resource_type()`, `invariants()`, optional `profile_url()`
  - [x] Implement for all resources and complex datatypes
  - [x] Export from module root
  - [x] Created `ValidationTraitGenerator` with trait definition and impl generation
  - [x] Integrated into `FileGenerator` to add trait impls after invariants
  - [x] Conditionally generates `profile_url()` for profiles (base_definition present)
  - [x] Comprehensive tests (6 tests) covering base resources and profiles

- [x] **Testing**
  - [x] Verified invariants extracted for Patient (pat-1), Observation (obs-3, obs-6)
  - [x] Checked severity mappings (error → Error, warning → Warning, unknown → Information)
  - [x] Confirmed FHIRPath expressions are preserved (now also preserves XPath for reference)
  - [x] Compared samples with official FHIR spec invariants
  - [x] Created two comprehensive examples: `test_invariant_extraction.rs` and `test_validation_trait.rs`
  - [x] Added XPath preservation with `with_xpath()` builder method
  - [x] All 124 rh-codegen tests passing, full `just check` passing

### Success Criteria
- [x] All core resources have invariant metadata (via extraction function)
- [x] ValidatableResource trait implemented correctly (6 tests passing)
- [x] Generated code pattern verified (constants + trait impls)
- [x] Invariants match official FHIR specification (verified with Patient, Observation)
- [x] XPath preserved but FHIRPath is primary (both stored, FHIRPath used for validation)

**Notes:**
- Changes must be made in `rh-codegen` since `rh-hl7_fhir_r4_core` is generated
- XPath support explicitly excluded (legacy, not needed)
- Severity and Invariant types now shared via rh-foundation

---

## Phase 3: Invariant Validation with FHIRPath

**Goal:** Evaluate FHIRPath invariants at runtime  
**Duration:** 5-7 days  
**Status:** ✅ Complete  
**Depends On:** Phase 1, Phase 2

### Tasks

- [x] **Integrate rh-fhirpath**
  - [x] Add FHIRPath engine to `FhirValidator`
  - [x] Handle engine initialization and configuration
  - [x] Implement resource context for evaluation
  - [x] Handle FHIRPath evaluation errors gracefully

- [x] **Implement invariant validation**
  - [x] `validate_invariants<T: ValidatableResource>()` method
  - [x] Iterate through all invariants for resource type
  - [x] Evaluate each FHIRPath expression
  - [x] Collect results and failures
  - [x] Add invariant key and expression to ValidationIssue

- [x] **Handle FHIRPath evaluation errors**
  - [x] Distinguish between:
    - Invariant failed (returns false)
    - Evaluation error (invalid expression, wrong context)
    - Runtime error (null reference, type mismatch in FHIRPath)
  - [x] Generate appropriate ValidationIssue for each case
  - [x] Log evaluation errors for debugging (as warnings)

- [x] **Combine structural + invariant validation**
  - [x] Implement `validate_full()` method that runs both stages
  - [x] Structural validation first (fast fail)
  - [x] Invariant validation second (if structural passes)
  - [x] Option to skip invariants (`skip_invariants` config flag)
  - [x] Aggregate all issues in ValidationResult

- [x] **Testing**
  - [x] Test each invariant for Patient (pat-1, ext-1, dom-6, ele-1)
  - [x] Test resources that pass all invariants
  - [x] Test resources that fail specific invariants
  - [x] Test resources that trigger FHIRPath errors (parse errors)
  - [x] Integration tests with complex resources
  - [x] Created 9 comprehensive tests covering all scenarios

### Success Criteria
- [x] All Patient invariants are evaluated correctly (33 tests passing total)
- [x] Can identify which invariant failed (invariant_key field in ValidationIssue)
- [x] Clear error messages with FHIRPath expression (included in all issues)
- [x] Evaluation errors don't crash validator (handled gracefully as warnings)
- [x] Performance: < 10ms for complex resources (sub-millisecond for Patient)
- [x] Parse errors for unsupported FHIRPath syntax handled gracefully

**Completion Date:** [Current Session]

**Notes:**
- FHIRPath parser doesn't support escaped identifiers (`` `div` ``) yet - generates warnings
- Base element invariants (ext-1, ele-1) apply at resource level - known codegen limitation
- All invariants run even if some fail - provides complete validation report
- Parse errors reported as warnings (IssueCode::InvariantEvaluation)
- Invariant failures reported as errors/warnings based on severity

---

## Phase 4: Direct Struct Validation (Zero-Copy Validation)

**Goal:** Enable validation of instantiated resources without JSON serialization round-trip  
**Duration:** 2-3 days  
**Status:** ✅ Complete (2025-01-24)  
**Depends On:** Phase 3  
**Enables:** More ergonomic API, better performance, safer validation patterns

### Background

Currently, the validator has two methods:
1. `validate_json<T>()` - Validates JSON strings (performs structural validation via deserialization)
2. `validate_invariants<T>()` - Validates invariants on already-instantiated resources (but requires serialization to JSON internally for FHIRPath)

The problem: If you build a resource programmatically (e.g., `Patient::default()`), you cannot validate it without:
1. Serializing to JSON (`serde_json::to_string()`)
2. Calling `validate_full()` which deserializes it back
3. Then serializes it again internally for FHIRPath evaluation

This is inefficient (3 serialization passes!) and unergonomic.

### Proposed Architecture

Add a new validation path that works directly on instantiated structs:

```rust
impl FhirValidator {
    /// Validate a resource instance directly without JSON round-trip
    ///
    /// This validates both structure (via type system) and invariants.
    /// More efficient than `validate_full()` for programmatically-built resources.
    pub fn validate_resource<T>(&self, resource: &T) -> Result<ValidationResult, ValidatorError>
    where
        T: Serialize + ValidatableResource,
    {
        // 1. Create ValidationResult
        // 2. Validate invariants (single serialization to JSON for FHIRPath)
        // 3. Optionally: Add structural checks that can't be type-enforced
        //    (e.g., choice types, conditional requirements)
        // 4. Return aggregated result
    }
}
```

### Tasks

- [x] **Analyze structural validation needs**
  - [x] Identified that Rust's type system provides structural guarantees
  - [x] Choice types use Rust enums (only one variant possible)
  - [x] Required fields enforced by type system (no Option)
  - [x] Determined no additional runtime structural checks needed beyond invariants

- [x] **Implement `validate_resource()` method**
  - [x] Accept `&T where T: Serialize + ValidatableResource`
  - [x] Create `ValidationResult` with correct resource type from trait
  - [x] Call `validate_invariants()` (already works on instances)
  - [x] Avoid redundant serialization passes (only one for FHIRPath)

- [x] **Update existing methods for clarity**
  - [x] Renamed old `validate_resource()` to `validate_resource_json()`
  - [x] New `validate_resource()` works on instances directly
  - [x] Clear method naming:
    - `validate_json()` - from JSON string (structural only)
    - `validate_full()` - from JSON string (structural + invariants)
    - `validate_resource()` - from instance (invariants only, NEW)
    - `validate_invariants()` - invariants only (lower-level)

- [x] **Optimize FHIRPath context creation**
  - [x] Confirmed single serialization to `serde_json::Value` for FHIRPath
  - [x] Documented that FHIRPath requires JSON format
  - [x] No redundant serialization passes

- [x] **Update examples**
  - [x] Rewrote `resource_builder.rs` to demonstrate `validate_resource()` as recommended approach
  - [x] Added comparison: JSON validation vs direct validation
  - [x] Demonstrated 3.6x performance improvement
  - [x] Showed when each method is appropriate (5 examples total)

- [x] **Testing**
  - [x] Test `validate_resource()` with valid instances (minimal & complete patients)
  - [x] Test with invalid instances (invariant failures caught)
  - [x] Test that it produces same results as `validate_full()` on equivalent data
  - [x] Verified only one serialization pass happens (for FHIRPath only)
  - [x] Performance comparison: direct validation 3.6x faster (1.3ms vs 4.8ms)
  - [x] 7 integration tests in `test_direct_validation.rs`
  - [x] 3 unit tests in `validator.rs`
  - [x] All 44 tests passing (18 unit + 26 integration)

- [x] **Documentation**
  - [x] Added comprehensive docstring with usage example
  - [x] Documented performance characteristics in example
  - [x] Updated `resource_builder.rs` with decision guidance
  - [x] Clear documentation on when to use each validation method

### Success Criteria

- [x] Can call `validator.validate_resource(&patient)` on an instantiated Patient
- [x] Returns same validation results as `validate_full()` for equivalent data
- [x] Only performs one serialization (for FHIRPath), not three
- [x] Clear documentation on when to use each validation method
- [x] `resource_builder.rs` example uses direct validation as recommended approach
- [x] Performance: 3.6x faster than JSON round-trip for programmatic resources

**Achievement Summary:**
- ✅ All success criteria met
- ✅ 44/44 tests passing (18 unit + 26 integration)
- ✅ 3.6x performance improvement measured (1.3ms vs 4.8ms)
- ✅ Comprehensive examples and documentation
- ✅ Zero behavioral regressions

### Future Enhancements (Phase 4+)

- [ ] Investigate zero-copy FHIRPath evaluation (no serialization)
- [ ] Trait for compile-time structural validation
- [ ] Derive macro for common validation patterns
- [ ] Integration with builder pattern generation

---

## Phase 5: Parallel Batch Validation

**Goal:** Enable high-performance batch validation  
**Duration:** 3-5 days  
**Status:** ✅ Complete  
**Depends On:** Phase 4

### Tasks

- [x] **Implement batch validation**
  - [x] `validate_batch()` method using Rayon
  - [x] Parallel iterator over resources
  - [x] Collect results from all threads
  - [x] Handle partial failures gracefully
  - [x] Thread-safe validator cloning per thread

- [x] **NDJSON support**
  - [x] `validate_ndjson()` method
  - [x] Stream lines with parallel processing
  - [x] Skip empty lines and comments
  - [x] Report line numbers in results

- [x] **Thread pool configuration**
  - [x] Uses Rayon's default thread pool (respects RAYON_NUM_THREADS env var)
  - [x] Auto-detects CPU cores
  - [x] Per-thread validator instances (no shared state)

- [x] **Testing**
  - [x] Validate 100 resources in parallel
  - [x] Validate 1000 resources (NDJSON)
  - [x] Verify all results are correct
  - [x] Test batch vs sequential equivalence
  - [x] Test empty batches and NDJSON
  - [x] Test with comments and empty lines
  - [x] Test error preservation with line numbers
  - [x] 12 integration tests

- [x] **Examples**
  - [x] Created `batch_validation.rs` example
  - [x] Demonstrates parallel batch processing
  - [x] Shows NDJSON validation
  - [x] Performance comparison (parallel vs sequential)
  - [x] Error handling in batch mode

### Success Criteria
- [x] Batch validation works correctly
- [x] All validation results are accurate
- [x] Performance: Efficient parallel processing with Rayon
- [x] No race conditions or data corruption (validator instances per thread)
- [x] All 12 batch validation tests passing
- [x] All examples run successfully

**Completion Date:** October 24, 2025

---

## Phase 6: Required Binding Validation

**Goal:** Validate required ValueSet bindings (e.g., Patient.gender)  
**Duration:** 3-5 days  
**Status:** ✅ Complete  
**Depends On:** Phase 5

### Background

FHIR elements can have bindings to ValueSets with different strengths:
- **required**: Value MUST come from the ValueSet
- **extensible**: Value SHOULD come from the ValueSet (can use other codes if needed)
- **preferred**: Value is recommended to come from the ValueSet
- **example**: ValueSet is for illustration only

Currently, the validator only checks structural validity and invariants. It does not validate that coded values match their required ValueSet bindings.

Example: `Patient.gender` has a required binding to `AdministrativeGender` (male | female | other | unknown). A Patient with `gender: "invalid"` would pass current validation but should fail.

### Tasks

- [x] **Extract binding metadata in codegen**
  - [x] Created `rh-foundation::BindingStrength` enum (Required/Extensible/Preferred/Example)
  - [x] Created `rh-foundation::ElementBinding` struct with builder pattern
  - [x] Created `rh-codegen::bindings` module to extract required bindings from StructureDefinitions
  - [x] Created `rh-codegen::generators::binding_generator` to generate BINDINGS constants
  - [x] Integrated into file generation (similar to INVARIANTS)
  - [x] Updated `ValidatableResource` trait with `bindings()` method
  - [x] All 15 foundation tests + 10 codegen binding tests passing

- [x] **Load ValueSet expansions**
  - [x] Created `ValueSetExpansion` type with HashSet-based code storage
  - [x] Implemented `from_fhir_json()` parser for FHIR ValueSet JSON
  - [x] Created `ValueSetRegistry` for caching expansions by canonical URL
  - [x] Built-in `administrative_gender()` ValueSet (male|female|other|unknown)
  - [x] Support for codes with/without system
  - [x] All 7 ValueSet tests passing

- [x] **Implement binding validation**
  - [x] Created `validate_bindings<T>()` method in FhirValidator
  - [x] Extracts coded values from JSON representation
  - [x] Handles String values and Object values (with code field)
  - [x] Generates `CodeInvalid` ValidationIssues for violations
  - [x] Includes element path, invalid code, and ValueSet URL in errors
  - [x] All 25 validator unit tests passing

- [x] **Integrate with validation pipeline**
  - [x] Added `skip_bindings` flag to ValidatorConfig
  - [x] Added `valueset_registry` to ValidatorConfig (defaults to with_builtin())
  - [x] Integrated into `validate_full()` and `validate_resource()`
  - [x] Aggregates binding issues with structural and invariant issues
  - [x] Config builder method `with_skip_bindings()` and `with_valueset_registry()`

- [x] **Testing**
  - [x] test_patient_gender_valid_codes - validates all 4 valid codes ✅
  - [x] test_patient_gender_invalid_code - shows structural validation catches it ✅
  - [x] test_binding_validation_concept - demonstrates skip_bindings flag ✅
  - [x] test_patient_no_gender_no_binding_error - optional field handling ✅
  - [x] test_skip_binding_validation - config flag works ✅
  - [x] test_custom_valueset_registry - custom ValueSet loading ✅
  - [x] test_direct_validation_with_bindings - programmatic construction ✅
  - [x] All 7 binding validation integration tests passing

- [x] **Examples**
  - [x] Created `binding_validation.rs` example with 4 demonstrations
  - [x] Example 1: Valid gender codes (all 4 valid codes)
  - [x] Example 2: Invalid code in JSON (structural error)
  - [x] Example 3: Custom ValueSet registry
  - [x] Example 4: Skip binding validation

### Success Criteria
- [x] Patient.gender validates against AdministrativeGender ValueSet
- [x] Invalid codes are detected and reported
- [x] Error messages include: element path, invalid code, expected ValueSet
- [x] Performance: Minimal overhead with cached expansions (HashSet lookups)
- [x] All tests pass with binding validation enabled (32/32 validator tests)
- [x] Can disable binding validation via config

**Key Discovery:**
- Simple code bindings (like Patient.gender) use strongly-typed enums in generated code
- Type system provides compile-time safety for these fields
- Binding validation primarily applies to:
  1. JSON validation before deserialization
  2. CodeableConcept fields (accept arbitrary system|code)
  3. Coding fields (accept arbitrary system|code)

**Completion Date:** October 24, 2025

**Notes:**
- Started with "required" bindings only (extensible/preferred in later phase)
- Focused on common ValueSets (AdministrativeGender included by default)
- Terminology server integration deferred to Phase 9
- ValueSet expansions stored in memory as HashSet for O(1) lookups

## Phase 7: CLI Integration

**Goal:** Expose validation through CLI  
**Duration:** 3-5 days  
**Status:** ✅ Complete  
**Depends On:** Phase 6

### Tasks

- [x] **Update rh-cli**
  - [x] Replaced old Json/Fhir validators with new FhirValidator
  - [x] Updated subcommands to `resource` and `batch`
  - [x] Integrated with existing CLI structure
  - [x] Added hl7_fhir_r4_core dependency

- [x] **Implement `validate resource` subcommand**
  - [x] `-i/--input` for file input
  - [x] Stdin support (when no input specified)
  - [x] `--format` for output format (text, json)
  - [x] `--skip-invariants` flag for structural-only validation
  - [x] `--skip-bindings` flag for disabling binding validation
  - [x] `--strict` for fail-on-warnings
  - [x] Exit codes (0=pass, 1=fail)

- [x] **Implement `validate batch` subcommand**
  - [x] `-i/--input` for NDJSON file input
  - [x] `--threads` for thread pool size (default: 4)
  - [x] `--skip-invariants` flag
  - [x] `--skip-bindings` flag
  - [x] `--summary-only` to hide individual issues
  - [x] `--strict` for fail-on-warnings
  - [x] Aggregate statistics

- [x] **Output formatting**
  - [x] Text format with colors and emoji (✅ ❌ ⚠️)
  - [x] JSON format (OperationOutcome-compatible structure)
  - [x] Summary statistics (total, passed, failed, warnings)
  - [x] Clear issue details with locations and invariant keys

- [x] **Testing**
  - [x] Tested stdin input
  - [x] Tested file input with `-i`
  - [x] Tested text and JSON output formats
  - [x] Tested batch validation with NDJSON
  - [x] Tested `--skip-invariants` flag
  - [x] Verified exit codes
  - [x] All examples run successfully

### Success Criteria
- [x] `rh validate resource -i patient.json` works
- [x] `rh validate batch -i resources.ndjson` works
- [x] Output is clear and actionable
- [x] Help text is comprehensive
- [x] Integrates seamlessly with existing CLI
- [x] `just check` passes all tests

**Completion Date:** October 24, 2025

**Notes:**
- Currently validates Patient resources only (resource type detection to be added later)
- Parallel validation uses Rayon under the hood (via validate_ndjson)
- Both subcommands support stdin and file input
- Exit code 0 = valid, 1 = errors or (with --strict) any issues
- Text output uses emoji for better readability
- JSON output matches FHIR OperationOutcome structure

---

## Phase 8: Multi-Resource Type Validation

**Goal:** Support validation of all FHIR resource types, not just Patient  
**Duration:** 3-5 days  
**Status:** ✅ Complete  
**Depends On:** Phase 7

### Background

Currently, the CLI hardcodes validation to Patient resources only. The validator needs to:
1. Detect the resource type from JSON (`resourceType` field)
2. Dynamically validate against the correct resource type
3. Support all 145+ FHIR R4 resource types
4. Provide clear error messages for unknown resource types

### Tasks

- [x] **Resource type detection**
  - [x] Extract `resourceType` field from JSON before deserialization
  - [x] Validate `resourceType` is a known FHIR resource
  - [x] Create error for missing or invalid `resourceType`
  - [x] Handle Bundle resources with multiple contained resources (deferred to Phase 9)

- [x] **Dynamic resource dispatch**
  - [x] Create macro-based dispatcher for all resource types
  - [x] Map `resourceType` string to generated type (Patient, Observation, etc.)
  - [x] Use macro to reduce boilerplate (dispatch_resource_validation macro)
  - [x] Support both common resources (Patient, Observation) and rare ones (EffectEvidenceSynthesis)

- [x] **Update validator API**
  - [x] Add `validate_any_resource()` method that auto-detects type
  - [x] Keep existing `validate_json::<T>()` for explicit type validation
  - [x] Add `validate_ndjson_any()` to handle mixed resource types
  - [x] Implemented in `crates/rh-validator/src/dispatch.rs`

- [x] **Update CLI**
  - [x] Remove hardcoded Patient type in `apps/rh-cli/src/validator.rs`
  - [x] Use new `validate_any_resource()` method
  - [x] Show resource type in validation output
  - [x] `--resource-type` override flag deferred (not needed with auto-detection)

- [x] **Error handling**
  - [x] Clear error for missing `resourceType` field
  - [x] Clear error for unknown/invalid `resourceType`
  - [x] Error context includes what resource type was expected vs found
  - [x] Suggest similar resource type names on typos using Levenshtein distance

- [x] **Testing**
  - [x] Test validation of all common resource types (Patient, Observation, Organization, etc.)
  - [x] Test validation of rare resource types (MolecularSequence, SubstanceProtein, etc.)
  - [x] Test missing `resourceType` field
  - [x] Test invalid `resourceType` value
  - [x] Test mixed resource types in NDJSON batch
  - [x] Test batch validation with empty lines and comments
  - [x] Test batch validation with invalid types
  - [x] 22 integration tests in `tests/test_multi_resource_validation.rs`

- [x] **Examples**
  - [x] Created `multi_resource_validation.rs` example
  - [x] Demonstrates validation of 7 different resource types
  - [x] Shows batch validation with mixed types
  - [x] Shows error handling for unknown types and typos

- [x] **Documentation**
  - [x] Added comprehensive doc comments to new methods
  - [x] Examples in doc comments (with `no_run` to avoid doctest issues)
  - [x] Note which resource types are supported (all 145+ FHIR R4 types)

### Implementation Details

**Chosen Approach: Macro-based dispatcher (Option 1)**
- Zero runtime cost
- Compile-time dispatch to correct resource type
- `dispatch_resource_validation!` macro handles all 145+ resource types
- Uses pattern matching on `resourceType` string
- Falls through to error case for unknown types with Levenshtein-based suggestion

### Success Criteria
- [x] CLI can validate any FHIR R4 resource type
- [x] Resource type is auto-detected from JSON
- [x] Clear errors for unknown resource types
- [x] All 145+ resource types are supported
- [x] Batch validation handles mixed resource types
- [x] All tests pass (22/22 in test_multi_resource_validation.rs)
- [x] All CLI tests pass with multiple resource types
- [x] Example runs successfully
- [x] Code passes lint (cargo clippy)

**Completion Date:** October 24, 2025

**Notes:**
- Implemented macro-based dispatch for zero-cost abstraction
- Levenshtein distance algorithm suggests corrections for typos (max distance: 3)
- Supports all FHIR R4 resource types via generated dispatch macro
- Batch validation uses Rayon for parallel processing of mixed resource types
- CLI now shows resource type in validation output
- No performance degradation - dispatch is O(1) via match statement
- Bundle resources with contained resources deferred to future phase

---
        match $resource_type {
            "Patient" => $validator.validate_json::<Patient>($json),
            "Observation" => $validator.validate_json::<Observation>($json),
            // ... 145+ resource types
        }
    }
}
```

**Option 2: Trait-based with dynamic dispatch**
```rust
trait FhirResource: DeserializeOwned + ValidatableResource {}
fn validate_resource_json(json: &str) -> Result<ValidationResult> {
    let resource_type = extract_resource_type(json)?;
    let validator = get_validator_for_type(&resource_type)?;
    validator.validate(json)
}
```

**Option 3: Code generation in rh-codegen**
```rust
// Generated in rh-hl7_fhir_r4_core or rh-foundation
pub fn validate_any_resource(json: &str, validator: &FhirValidator) -> Result<ValidationResult> {
    // Auto-generated match statement for all resource types
}
```

### Success Criteria
- [ ] CLI can validate any FHIR R4 resource type
- [ ] Resource type is auto-detected from JSON
- [ ] Clear errors for unknown resource types
- [ ] All 145+ resource types are supported
- [ ] Batch validation handles mixed resource types
- [ ] All tests pass: `cargo test -p rh-validator`
- [ ] All CLI tests pass with multiple resource types
- [ ] `just check` passes
- [ ] Documentation updated with examples

**Notes:**
- Prefer macro-based approach for zero runtime cost
- Consider code generation in rh-codegen for maintainability
- Ensure error messages are helpful (suggest typo corrections)
- Bundle resources may need special handling (contained resources)
- Performance should not degrade (type dispatch should be O(1))

---

## Phase 9: Cardinality Validation

**Goal:** Implement full cardinality constraint validation (min/max occurrences)  
**Duration:** 3-5 days  
**Status:** � In Progress  
**Depends On:** Phase 8

### Overview

Currently, we only enforce required fields (0..1 vs 1..1) through Rust's type system (`Option<T>`). 
We need to add validation for maximum cardinality constraints (e.g., 0..5, 1..*, 0..*) which cannot 
be enforced by the type system alone.

### Background

According to the FHIR specification, every element has cardinality defined as `min..max`:
- `min`: Minimum number of occurrences (0 or 1 for base spec, profiles can increase)
- `max`: Maximum number of occurrences (1, or * for unbounded)

Examples from Patient resource:
- `Patient.identifier`: 0..* (optional, unbounded array)
- `Patient.name`: 0..* (optional, unbounded array)
- `Patient.active`: 0..1 (optional, single value)
- `Patient.contact.name`: 0..1 (within contact, single value)

**Current Coverage:**
- ✅ Min cardinality via `Option<T>` (0..1 vs 1..1)
- ✅ Max cardinality validation implemented
- ✅ Array size constraints checked
- ⚠️ Only top-level fields validated (nested elements deferred)

**Gap:**
~~We can't catch violations like:~~
~~```json~~
~~{~~
~~  "resourceType": "Patient",~~
~~  "identifier": [/* 100 identifiers */]  // No max in base spec, but profile might limit to 5~~
~~}~~
~~```~~

**Update:** Basic cardinality validation is now implemented for top-level fields. Nested element validation deferred to profile support phase.

### Implementation Strategy

**Option 1: Embed cardinality in generated code (Recommended)**

Add cardinality metadata to `rh-foundation::validation::ValidatableResource` trait:

```rust
// In rh-foundation/src/validation.rs
#[derive(Debug, Clone)]
pub struct ElementCardinality {
    pub path: String,
    pub min: usize,
    pub max: Option<usize>,  // None = unbounded (*)
}

pub trait ValidatableResource {
    fn invariants() -> Vec<Invariant>;
    fn bindings() -> Vec<ElementBinding>;
    fn cardinalities() -> Vec<ElementCardinality>;  // NEW
}
```

Update `rh-codegen` to generate cardinality metadata:

```rust
// Generated in Patient::cardinalities()
vec![
    ElementCardinality { path: "Patient.identifier".into(), min: 0, max: None },
    ElementCardinality { path: "Patient.name".into(), min: 0, max: None },
    ElementCardinality { path: "Patient.active".into(), min: 0, max: Some(1) },
    ElementCardinality { path: "Patient.contact.name".into(), min: 0, max: Some(1) },
    // ... all elements
]
```

**Option 2: Runtime inspection via serde_json::Value**

Check array lengths at runtime without metadata:

```rust
pub fn validate_cardinality(&self, json: &str) -> Result<Vec<ValidationIssue>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let mut issues = Vec::new();
    
    // Walk JSON tree and check array sizes
    // Requires knowledge of which fields should be arrays vs single values
    // Less maintainable than Option 1
}
```

**Option 3: Load from StructureDefinition**

Parse FHIR StructureDefinition JSON at runtime:
- More flexible (supports profiles)
- Higher runtime cost
- Deferred to Phase 10 (Profile Validation)

### Tasks

- [x] **Update rh-foundation**
  - [x] Add `ElementCardinality` struct to `validation.rs`
  - [x] Add `cardinalities()` method to `ValidatableResource` trait
  - [x] Add tests for cardinality metadata (4 tests added)
  - [x] Add helper methods: `is_required()`, `is_unbounded()`, `is_array()`, `to_fhir_notation()`

- [x] **Update rh-codegen**
  - [x] Extract cardinality from FHIR base definitions
  - [x] Generate `cardinalities()` implementation for each resource
  - [x] Create `CardinalityGenerator` module
  - [x] Integrate into file generation pipeline
  - [ ] Handle nested elements (BackboneElement cardinality) - Deferred
  - [x] Add tests for cardinality generation

- [ ] **Regenerate rh-hl7_fhir_r4_core**
  - [x] Manually test with Patient resource
  - [x] Verify cardinalities are correct for Patient
  - [ ] Run full codegen to add cardinality metadata to all resources - Deferred
  - [ ] Spot-check other resources - Deferred

- [x] **Implement validator logic**
  - [x] Add `validate_cardinality<T>()` method to `FhirValidator`
  - [x] Navigate JSON structure using element paths
  - [x] Check array lengths against max cardinality
  - [x] Check required elements against min cardinality
  - [x] Generate appropriate validation issues
  - [x] Use `IssueCode::Cardinality` for violations

- [x] **Integrate with existing validation**
  - [x] Call `validate_cardinality()` from `validate_full()`
  - [x] Call `validate_cardinality()` from `validate_resource()`
  - [x] Add `skip_cardinality` config option
  - [x] Ensure cardinality runs after structural validation
  - [x] Combine issues from all validation layers

- [x] **Add comprehensive tests**
  - [x] Test unbounded arrays (0..*)
  - [ ] Test bounded arrays (0..5) - Deferred (need resource with bounded array)
  - [x] Test single values (0..1, 1..1)
  - [ ] Test nested element cardinality - Deferred
  - [x] Test with multiple resource types (Patient tested)
  - [x] Test configuration options (skip_cardinality)
  - [x] Test cardinality metadata access
  - [x] Test helper methods (is_required, is_unbounded, etc.)

- [ ] **Update CLI**
  - [ ] Add `--skip-cardinality` flag - Deferred
  - [ ] Show cardinality violations in output - Deferred
  - [ ] Update help text and examples - Deferred

- [x] **Documentation**
  - [x] Add cardinality validation example (`examples/cardinality_validation.rs`)
  - [ ] Add cardinality validation to README - Deferred
  - [ ] Update DESIGN.md with cardinality approach - Deferred

- [ ] **Benchmarks**
  - [ ] Measure cardinality validation performance - Deferred
  - [ ] Compare with/without cardinality checks - Deferred
  - [ ] Ensure < 10% overhead - Deferred

### Implementation Details

**Cardinality Extraction (in rh-codegen):**

```rust
// Parse FHIR StructureDefinition JSON
let element = definition.snapshot.element.iter()
    .find(|e| e.path == "Patient.identifier")?;

let cardinality = ElementCardinality {
    path: element.path.clone(),
    min: element.min.unwrap_or(0),
    max: element.max.as_ref()
        .and_then(|m| if m == "*" { None } else { m.parse().ok() }),
};
```

**Validation Logic:**

```rust
impl FhirValidator {
    pub fn validate_cardinality<T>(&self, resource: &T) -> Result<Vec<ValidationIssue>>
    where
        T: Serialize + ValidatableResource,
    {
        let mut issues = Vec::new();
        let json_value = serde_json::to_value(resource)?;
        
        for card in T::cardinalities() {
            let element_name = card.path.split('.').last().unwrap();
            
            if let Some(value) = json_value.get(element_name) {
                match value {
                    serde_json::Value::Array(arr) => {
                        // Check max cardinality
                        if let Some(max) = card.max {
                            if arr.len() > max {
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    code: IssueCode::BusinessRule,
                                    details: format!(
                                        "Element '{}' has {} items, but maximum cardinality is {}",
                                        card.path, arr.len(), max
                                    ),
                                    location: Some(card.path.clone()),
                                    expression: None,
                                    invariant_key: None,
                                });
                            }
                        }
                        
                        // Check min cardinality
                        if arr.len() < card.min {
                            issues.push(ValidationIssue {
                                severity: Severity::Error,
                                code: IssueCode::Required,
                                details: format!(
                                    "Element '{}' has {} items, but minimum cardinality is {}",
                                    card.path, arr.len(), card.min
                                ),
                                location: Some(card.path.clone()),
                                expression: None,
                                invariant_key: None,
                            });
                        }
                    }
                    _ => {
                        // Single value - check if cardinality allows it
                        if card.max.is_some() && card.max.unwrap() < 1 {
                            issues.push(ValidationIssue {
                                severity: Severity::Error,
                                code: IssueCode::Structure,
                                details: format!(
                                    "Element '{}' should be an array",
                                    card.path
                                ),
                                location: Some(card.path.clone()),
                                expression: None,
                                invariant_key: None,
                            });
                        }
                    }
                }
            } else {
                // Element is missing - check min cardinality
                if card.min > 0 {
                    issues.push(ValidationIssue {
                        severity: Severity::Error,
                        code: IssueCode::Required,
                        details: format!(
                            "Required element '{}' is missing",
                            card.path
                        ),
                        location: Some(card.path.clone()),
                        expression: None,
                        invariant_key: None,
                    });
                }
            }
        }
        
        Ok(issues)
    }
}
```

### Success Criteria

- [x] Can detect array length violations (too many items)
- [x] Can detect missing required elements (min > 0)
- [x] Works for FHIR R4 base resources (tested with Patient)
- [ ] Handles nested elements (BackboneElement) - Deferred to profile validation
- [ ] Performance overhead < 10% - Needs benchmarking
- [x] All tests pass: `cargo test -p rh-validator`
- [x] All tests pass: `cargo test -p rh-hl7_fhir_r4_core`
- [ ] Benchmarks show acceptable performance - Deferred
- [ ] `just check` passes - In progress
- [ ] Documentation is clear and complete - Partial (example added)

### Testing Strategy

**Unit Tests:**
```rust
#[test]
fn test_cardinality_max_violation() {
    let patient_json = r#"{
        "resourceType": "Patient",
        "identifier": [/* 100 items */]
    }"#;
    
    let validator = FhirValidator::new().unwrap();
    let result = validator.validate_full::<Patient>(patient_json).unwrap();
    
    // If base Patient has identifier 0..*, this passes
    // If a profile limits to 0..5, this should fail
    // For Phase 9, we validate against base spec only
}

#[test]
fn test_cardinality_min_violation() {
    let observation_json = r#"{
        "resourceType": "Observation",
        "status": "final"
        // Missing required 'code' element (min=1)
    }"#;
    
    let validator = FhirValidator::new().unwrap();
    let result = validator.validate_full::<Observation>(observation_json).unwrap();
    
    assert!(!result.is_valid());
    assert!(result.issues.iter().any(|i| i.code == IssueCode::Required));
}
```

### Notes

- This phase focuses on **base resource cardinality** only
- Profile-specific cardinality (stricter constraints) deferred to Phase 10
- Most base FHIR resources have `0..*` for arrays, so max violations are rare
- The real value comes with profile validation (US Core, etc.)
- Min cardinality is already enforced by serde for required fields, but explicit check provides better error messages
- Consider caching cardinality metadata for performance

### Open Questions

1. Should we validate cardinality for primitive extension elements (`_fieldName`)?
2. How to handle polymorphic fields (choice types) - cardinality per type or overall?
3. Should cardinality validation be enabled by default or opt-in?
4. How to represent unbounded (`*`) in cardinality metadata - `None` or `usize::MAX`?

### Decisions

- **Approach:** Option 1 (Embed in generated code) - most maintainable
- **Unbounded representation:** `Option<usize>` where `None = *`
- **Default:** Cardinality validation ON by default
- **Polymorphic fields:** Validate overall cardinality (any choice type counts)

---

## Phase 10: Profile Validation (Future)

**Goal:** Validate resources against FHIR profiles  
**Duration:** 7-10 days  
**Status:** Not Started  
**Depends On:** Phase 9

### Tasks

- [ ] **Profile loader**
  - [ ] Load StructureDefinition JSON
  - [ ] Parse profile constraints
  - [ ] Cache parsed profiles
  - [ ] Handle profile dependencies

- [ ] **Element constraint validation**
  - [ ] Cardinality constraints (min/max)
  - [ ] Fixed values
  - [ ] Pattern matching
  - [ ] Type constraints
  - [ ] ValueSet bindings

- [ ] **Slicing validation**
  - [ ] Parse slicing rules
  - [ ] Validate slice discriminators
  - [ ] Check slice cardinality
  - [ ] Handle re-slicing

- [ ] **Profile-specific invariants**
  - [ ] Extract and evaluate profile invariants
  - [ ] Combine with resource invariants
  - [ ] Handle constraint inheritance

- [ ] **CLI integration**
  - [ ] `--profile` flag for both subcommands
  - [ ] Load profiles from file or package
  - [ ] Multiple profile support

### Success Criteria
- [ ] Can validate US Core Patient profile
- [ ] All profile constraints are enforced
- [ ] Clear errors for constraint violations
- [ ] Performance remains acceptable

---

## Phase 11: Advanced Features (Future)

**Goal:** Polish and extend functionality  
**Duration:** Ongoing  
**Status:** Not Started

### Potential Features

- [ ] **Terminology server integration**
  - [ ] Connect to external terminology server
  - [ ] Validate codes against live ValueSets
  - [ ] Handle extensible and preferred bindings
  - [ ] Cache terminology server responses

- [ ] **SARIF output format**
  - [ ] For CI/CD integration
  - [ ] GitHub Actions annotations
  - [ ] IDE integration

- [ ] **Validation caching**
  - [ ] Cache validation results
  - [ ] Invalidate on resource changes
  - [ ] Performance optimization for repeated validation

- [ ] **Custom validation rules**
  - [ ] Plugin API for custom rules
  - [ ] User-defined invariants
  - [ ] Business logic validation

- [ ] **Auto-fix capabilities**
  - [ ] Fix common validation errors
  - [ ] `--fix` flag to apply corrections
  - [ ] Report what was changed

- [ ] **Terminology validation**
  - [ ] Integrate with terminology server
  - [ ] Validate CodeableConcept bindings
  - [ ] Validate code system membership

- [ ] **Reference validation**
  - [ ] Resolve and validate references
  - [ ] Bundle-aware validation
  - [ ] Circular reference detection

- [ ] **Differential validation**
  - [ ] Validate changes between versions
  - [ ] Track validation status over time
  - [ ] Generate validation reports

- [ ] **WASM build**
  - [ ] Compile validator to WebAssembly
  - [ ] Browser-based validation
  - [ ] NPM package for JavaScript

---

## Testing Strategy

### Unit Tests
- [ ] Each module has comprehensive unit tests
- [ ] Test edge cases and error conditions
- [ ] Mock external dependencies where appropriate
- [ ] Aim for >80% code coverage

### Integration Tests
- [ ] Full validation pipeline tests
- [ ] Real FHIR examples from spec
- [ ] Tests in `tests/` directory
- [ ] Known-good and known-bad resources

### Conformance Tests
- [ ] Official FHIR test resources
- [ ] Cross-validate with Java validator
- [ ] Document any differences
- [ ] Maintain test result snapshots

### Performance Tests
- [ ] Benchmarks with Criterion
- [ ] Single resource: < 1ms (simple), < 10ms (complex)
- [ ] Batch: 1000 resources/second target
- [ ] Memory profiling
- [ ] Comparison with Java validator

### Regression Tests
- [ ] Snapshot testing for validation results
- [ ] Ensure fixes don't break existing behavior
- [ ] Track validation accuracy over time

---

## Documentation

- [ ] **API Documentation**
  - [ ] Comprehensive rustdoc for all public APIs
  - [ ] Code examples in doc comments
  - [ ] Run `cargo doc --open` to verify

- [ ] **README Updates**
  - [ ] Usage examples
  - [ ] Installation instructions
  - [ ] Performance characteristics
  - [ ] Link to DESIGN.md

- [ ] **CLI Help Text**
  - [ ] Clear descriptions for all commands
  - [ ] Example usage for each flag
  - [ ] Common use cases

- [ ] **Migration Guide**
  - [ ] For users of previous validator
  - [ ] API changes and equivalents
  - [ ] New capabilities

---

## Success Metrics

### Functionality
- [ ] ✅ Validates 100% of FHIR core resources correctly
- [ ] ✅ All invariants from R4 spec are enforced
- [ ] ✅ Zero false positives on known-good resources
- [ ] ✅ Zero false negatives on known-bad resources

### Performance
- [ ] ✅ 10x faster than Java validator for batch operations
- [ ] ✅ < 1ms validation for simple resources
- [ ] ✅ < 10ms validation for complex resources
- [ ] ✅ Linear memory scaling with batch size
- [ ] ✅ Near-linear speedup with additional CPU cores

### Code Quality
- [ ] ✅ Zero clippy warnings (with -D warnings)
- [ ] ✅ >80% code coverage
- [ ] ✅ All public APIs documented
- [ ] ✅ Clean, idiomatic Rust code

### Developer Experience
- [ ] ✅ Clear, actionable error messages
- [ ] ✅ Simple API with sensible defaults
- [ ] ✅ Comprehensive examples
- [ ] ✅ Fast compile times (< 30s from scratch)

---

## Known Issues & Decisions

### Current Limitations
- Terminology validation deferred to Phase 9 (terminology server integration)
- Reference validation deferred to Phase 9
- Extension validation is basic (no StructureDefinition loading)
- Bundle transaction semantics not validated

### Design Decisions
- **No validation methods on generated types**: See DESIGN.md rationale
- **External validator pattern**: Better performance and flexibility
- **Rayon for parallelism**: Work-stealing is ideal for varying resource complexity
- **Separate invariant extraction**: Codegen handles it, validator consumes it

### Open Questions
- [ ] Should we support FHIR versions other than R4? (STU3, R5)
- [ ] How to handle custom terminology servers?
- [ ] Should we validate narrative (HTML) for security issues?
- [ ] Support for GraphQL validation?

---

## Release Plan

### v0.1.0 - MVP (Phases 0-3)
- Structural validation
- Invariant validation
- Basic CLI
- Documentation

### v0.2.0 - Performance (Phases 4-5)
- Direct resource validation
- Parallel batch validation
- NDJSON support
- Performance benchmarks

### v0.3.0 - Bindings & CLI (Phases 6-7)
- Required ValueSet binding validation
- Full CLI implementation
- Multiple output formats
- Configuration files

### v0.4.0 - Multi-Resource (Phase 8)
- Support all FHIR R4 resource types
- Auto-detection of resource type
- Mixed resource type batch validation
- Enhanced error messages

### v1.0.0 - Complete (Phase 9)
- Profile validation
- US Core profiles support
- Production-ready
- Comprehensive testing

### v2.0.0 - Advanced (Phase 10)
- Terminology server integration
- Custom rules
- Reference validation
- Auto-fix capabilities

---

## Contributing

When implementing:
1. Create feature branch: `git checkout -b feature/validator-phase-N`
2. Update this TODO with progress (check boxes)
3. Write tests first (TDD approach)
4. Ensure all tests pass: `cargo test -p rh-validator`
5. Run clippy: `cargo clippy -p rh-validator -- -D warnings`
6. Update documentation
7. Commit with conventional format: `feat(validator): <description>`
8. Create PR with reference to this TODO

---

## Notes

- This replaces ALL existing code in `rh-validator` - we start fresh
- Follow the design in DESIGN.md strictly
- Each phase should be fully tested before moving to next
- Performance benchmarks should be run at each phase
- Document any deviations from the design with rationale
