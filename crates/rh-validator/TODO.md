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
**Status:** Not Started  
**Depends On:** Phase 1, Phase 2

### Tasks

- [ ] **Integrate rh-fhirpath**
  - [ ] Add FHIRPath engine to `FhirValidator`
  - [ ] Handle engine initialization and configuration
  - [ ] Implement resource context for evaluation
  - [ ] Handle FHIRPath evaluation errors gracefully

- [ ] **Implement invariant validation**
  - [ ] `validate_invariants<T: ValidatableResource>()` method
  - [ ] Iterate through all invariants for resource type
  - [ ] Evaluate each FHIRPath expression
  - [ ] Collect results and failures
  - [ ] Add invariant key and expression to ValidationIssue

- [ ] **Handle FHIRPath evaluation errors**
  - [ ] Distinguish between:
    - Invariant failed (returns false)
    - Evaluation error (invalid expression, wrong context)
    - Runtime error (null reference, type mismatch in FHIRPath)
  - [ ] Generate appropriate ValidationIssue for each case
  - [ ] Log evaluation errors for debugging

- [ ] **Combine structural + invariant validation**
  - [ ] Update `validate_json()` to run both stages
  - [ ] Structural validation first (fast fail)
  - [ ] Invariant validation second (if structural passes)
  - [ ] Option to skip invariants (`--no-invariants` flag)
  - [ ] Aggregate all issues in ValidationResult

- [ ] **Testing**
  - [ ] Test each invariant for Patient (pat-1, pat-2, etc.)
  - [ ] Test resources that pass all invariants
  - [ ] Test resources that fail specific invariants
  - [ ] Test resources that trigger FHIRPath errors
  - [ ] Integration tests with complex resources
  - [ ] Compare results with official FHIR validator

### Success Criteria
- [ ] All Patient invariants are evaluated correctly
- [ ] Can identify which invariant failed
- [ ] Clear error messages with FHIRPath expression
- [ ] Evaluation errors don't crash validator
- [ ] Performance: < 10ms for complex resources
- [ ] 100% agreement with official validator on test cases

---

## Phase 4: Parallel Batch Validation

**Goal:** Enable high-performance batch validation  
**Duration:** 3-5 days  
**Status:** Not Started  
**Depends On:** Phase 3

### Tasks

- [ ] **Implement batch validation**
  - [ ] `validate_batch()` method using Rayon
  - [ ] Parallel iterator over resources
  - [ ] Collect results from all threads
  - [ ] Handle partial failures gracefully

- [ ] **NDJSON support**
  - [ ] `validate_ndjson()` method
  - [ ] Stream lines with `par_bridge()`
  - [ ] Skip empty lines and comments
  - [ ] Report line numbers in errors

- [ ] **Thread pool configuration**
  - [ ] Respect `ValidatorConfig.max_threads`
  - [ ] Auto-detect CPU cores if not specified
  - [ ] Rayon thread pool initialization
  - [ ] Shared FHIRPath engine pool (thread-local or Arc)

- [ ] **Progress reporting** (optional)
  - [ ] Add `indicatif` for progress bars
  - [ ] Report validation progress for large batches
  - [ ] Estimated time remaining
  - [ ] Configurable (can disable for CI/CD)

- [ ] **Memory management**
  - [ ] Chunked processing for very large files
  - [ ] Streaming mode for NDJSON
  - [ ] Configurable batch size
  - [ ] Monitor memory usage in tests

- [ ] **Testing**
  - [ ] Validate 100 resources in parallel
  - [ ] Validate 10,000 resources (NDJSON)
  - [ ] Verify all results are correct
  - [ ] Test with different thread counts (1, 4, 8, 16)
  - [ ] Memory profiling with large batches
  - [ ] Benchmark against single-threaded

### Success Criteria
- [ ] Batch validation works correctly
- [ ] Linear or better speedup with more threads
- [ ] Memory usage is reasonable (< 2x single resource)
- [ ] Performance target: 1000 resources/second on 8 cores
- [ ] No race conditions or data corruption

---

## Phase 5: CLI Integration

**Goal:** Expose validation through CLI  
**Duration:** 3-5 days  
**Status:** Not Started  
**Depends On:** Phase 4

### Tasks

- [ ] **Update rh-cli**
  - [ ] Add `validate` subcommand module to `apps/rh-cli/src/`
  - [ ] Use `rh-foundation::cli` utilities for I/O
  - [ ] Integrate with existing CLI structure

- [ ] **Implement `validate resource` subcommand**
  - [ ] `-i/--input` for file input
  - [ ] Stdin support (when no input specified)
  - [ ] `--format` for output format (text, json)
  - [ ] `--profile` for profile validation (future)
  - [ ] `--strict` for fail-on-warnings
  - [ ] Exit codes (0=pass, 1=fail)

- [ ] **Implement `validate batch` subcommand**
  - [ ] `-i/--input` for NDJSON file or directory
  - [ ] `--threads` for thread pool size
  - [ ] `--progress` for progress reporting
  - [ ] `--summary-only` to hide individual issues
  - [ ] Aggregate statistics

- [ ] **Output formatting**
  - [ ] Text format with colors and emoji (✅ ❌ ⚠️)
  - [ ] JSON format (OperationOutcome compatible)
  - [ ] Summary statistics (total, passed, failed, warnings)
  - [ ] Use `rh-foundation::cli::OutputFormat`

- [ ] **Configuration file support**
  - [ ] `--config` flag for validator.toml
  - [ ] Define configuration schema
  - [ ] Load and merge with CLI flags
  - [ ] Document configuration options

- [ ] **Testing**
  - [ ] Test all CLI flags and combinations
  - [ ] Test stdin/stdout piping
  - [ ] Test exit codes
  - [ ] Test output formats
  - [ ] Integration tests with real files
  - [ ] CLI help text is clear

### Success Criteria
- [ ] `rh validate resource -i patient.json` works
- [ ] `rh validate batch -i resources.ndjson` works
- [ ] Output is clear and actionable
- [ ] Help text is comprehensive
- [ ] Integrates seamlessly with existing CLI
- [ ] Examples in README work correctly

---

## Phase 6: Profile Validation (Future)

**Goal:** Validate resources against FHIR profiles  
**Duration:** 7-10 days  
**Status:** Not Started  
**Depends On:** Phase 5

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

## Phase 7: Advanced Features (Future)

**Goal:** Polish and extend functionality  
**Duration:** Ongoing  
**Status:** Not Started

### Potential Features

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
- Terminology validation deferred to Phase 7
- Reference validation deferred to Phase 7
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

### v0.2.0 - Performance (Phase 4)
- Parallel batch validation
- NDJSON support
- Performance benchmarks

### v0.3.0 - Usability (Phase 5)
- Full CLI implementation
- Multiple output formats
- Configuration files

### v1.0.0 - Complete (Phase 6)
- Profile validation
- US Core profiles support
- Production-ready
- Comprehensive testing

### v2.0.0 - Advanced (Phase 7)
- Custom rules
- Terminology validation
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
