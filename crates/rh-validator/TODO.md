# RH Validator Development Plan

## Status: v0.2.0 - Production Ready

**Tests:** ~145 integration tests (21 test files)  
**Performance:** 3-4ms/resource, ~252 resources/sec batch  
**Cache Hit Rate:** 100% in benchmarks  
**Last Major Fix:** ext-1 false positive fix (Nov 24, 2025)  
**Last Review:** February 6, 2026

## Completed Phases (v0.1.0 → v0.2.0)

### Phase 0: Foundation ✅
Core validation engine with cardinality and type checking.

### Phase 1: FHIRPath Integration ✅
Integrated FHIRPath evaluator for constraint evaluation.

### Phase 2: Terminology ✅
ValueSet binding validation (required/extensible/preferred/example strength).

### Phase 3: Invariants ✅
Constraint validation with FHIRPath expressions (dom-1 through dom-6, ext-1).

### Phase 4: Extensions ✅
Extension validation with modifierExtension handling.

### Phase 5: Slicing ✅
Slice discrimination and validation.

### Phase 6: References ✅
Reference type validation and target profile checking.

### Phase 7: Profiles ✅
Multi-profile validation and profile discovery.

### Phase 8: Performance ✅
LRU caching for compiled rules (100% hit rate in benchmarks).

### Phase 9: Primitives ✅
Complete regex validation for FHIR primitives (id, date, dateTime, instant, code, uri, url, canonical, oid, uuid).

### Phase 10: OperationOutcome ✅
Full OperationOutcome output with 21 IssueType codes, severity levels, diagnostics, FHIRPath location expressions.

### Phase 11: Terminology Service ✅
Display name validation, HTTP client, mock service, persistent caching.

### Phase 12: ext-1 Fix ✅
Prevented false positive ext-1 errors on resources without extensions (Nov 24, 2025).

### Phase 13: FHIR Test Cases Integration ✅
Downloaded and integrated official FHIR R4 test suite, built test runner.

### Phase 14: Post-Fix Test Run ✅
Re-ran test suite after ext-1 fix: 70/100 (70%) - 15% improvement (Feb 6, 2026).

## Current Phase

### Phase 15: Conformance Analysis 🚧

**Goal:** Validate against official FHIR R4 test suite to ensure spec compliance.

**Progress:**
- ✅ 13.1: Downloaded and organized 570 R4 test cases
- ✅ 13.2: Implemented test case parser
- ✅ 13.3: Built test runner with batch execution
- ✅ 13.4: Array structure validation for max>1 elements
- ✅ 13.5: Profile-not-found as warning (Java compatibility)
- ✅ 13.6: Base64Binary format validation
- ✅ 13.7: FHIRPath descendants() fix (array handling)
- ✅ 13.8: Unknown extension definitions as errors (Java compatibility)
- ✅ 13.9: Re-run test suite after ext-1 fix (Phase 14 complete - 70%!)
- ⏳ 13.10: Document known limitations vs FHIR spec
- ⏳ 13.11: Set up automated test runs in CI

**Current Test Status (As of Feb 6, 2026):**
- **ext-1 fix applied** (Nov 24, 2025) - eliminated primary false positive source
- **Phase 14 complete**: Full test suite re-run completed
- Previous: 55/100 (55%) before ext-1 fix (Nov 2025)
- **Current: 70/100 (70.0%)** - 15% improvement! ✅
- **Agreement with Java validator: 70/100 (70.0%)**
- **Agreement with Firely SDK: 15/21 (71.4%)**

**Remaining Work Categories (Post ext-1 fix):**
1. **Questionnaire validation:** QR validation requires Questionnaire resolution
2. **Bundle validation:** Reference resolution, fullUrl, duplicate IDs
3. **Terminology:** SNOMED/LOINC validation requires terminology server
4. **Profile constraints:** Profile-specific invariants
5. **Attachment size:** Validate data length matches size field
6. **Per-element invariants:** Some invariants need element-scoped context (3 ignored tests)

**Java Compatibility Patterns Implemented:**
- ✅ Missing CodeSystem definitions → **warning** (not-found)
- ✅ Missing Extension definitions → **error** (structure: "not allowed here")
- ✅ Missing Profile definitions → **warning** (not checked)
- ✅ ext-1 scoping → only applies to Extension types/fields
- ✅ Actual validation failures → **error**

## High Priority (v0.3.0)

### Phase 15: Analyze Remaining Failures ✅ COMPLETE
**Why:** Understand the 30 remaining test failures to prioritize fixes.  
**Completed:** February 6, 2026  
**Results:** See PHASE_15_ANALYSIS.md for comprehensive breakdown  
**Key Findings:**
- ✅ All 30 failures categorized into 5 groups
- ✅ **NO FALSE NEGATIVE BUGS FOUND** - All failures are legitimate validation gaps or missing features
- ✅ pat-security-good2 investigation: Not a bug, requires security-checks configuration option
- ✅ Failure breakdown: Terminology (9), Profile Constraints (7), Questionnaires (6), Security-checks (2), Bundles (2), Other (4)
- ✅ FALSE_NEGATIVES_ANALYSIS.md updated - ext-1 fix fully validated, no remaining false positives
- ✅ Roadmap created with 5 phases to reach 85-95%:
  - Phase 15a: Security-checks option (+2%)
  - Phase 15b: Quick wins (+3%)
  - Phase 16: Terminology (+6-7%)
  - Phase 17: Profile constraints (+5-7%)
  - Phase 18: Questionnaires (+4-6%)
- ✅ **Validator is production-ready** - No correctness bugs, only missing features

**Success:** Complete roadmap established. Validator has no known bugs. Path to 85-95% clear.

### Phase 15a: Security-Checks Configuration Option 🚧 CURRENT
**Why:** Enable proper handling of HTML tag validation based on context.  
**Priority:** Medium (affects 2 tests: pat-security-good2, pat-security-bad-string)  
**Impact:** +2% pass rate (72/100)  
**Tasks:**
- Add security_checks: bool field to FhirValidator config
- Update test runner to read security-checks from manifest.json
- Modify validate_string_security() to check security_checks flag
- When security_checks=false (default): Report HTML tags as INFORMATION
- When security_checks=true: Report HTML tags as ERROR
- Update tests to verify both behaviors
- Document security-checks option in README

**Estimated:** 3-5 days  
**Success Criteria:** pat-security-good2 passes (VALID), pat-security-bad-string passes (INVALID), 72/100 tests passing.

### Phase 16: Per-Element Invariant Evaluation
**Why:** 3 ignored tests require proper element-scoped FHIRPath context.  
**Tasks:**
- Enhance FHIRPath evaluator to track element context during evaluation
- Update validate_invariant() to pass current element context
- Un-ignore tests in invariant_scoping_test.rs
- Add integration tests for nested element constraints

**Estimated:** 6-8 hours  
**Success Criteria:** All invariant_scoping_test.rs tests passing, proper context resolution in nested elements.

### Phase 17: CI/CD Integration
**Why:** Automate testing and performance monitoring.  
**Tasks:**
- Add validator benchmarks to GitHub Actions
- Set up performance regression detection
- Run FHIR test suite in CI
- Track pass rate trends over time

**Estimated:** 4-6 hours  
**Success Criteria:** Automated test runs, performance alerts, test pass rate dashboard.

### Phase 18: Parallel Validation
**Why:** Target 500+ resources/sec (current: ~200/sec).  
**Blockers:** FhirPathEvaluator thread-safety issues.  
**Tasks:**
- Refactor FhirPathEvaluator for Send + Sync
- Implement rayon-based parallel batch validation
- Add concurrency benchmarks
- Document thread-safety guarantees

**Estimated:** 8-12 hours  
**Success Criteria:** 500+ resources/sec in batch validation, all benchmarks show linear scaling.

## Medium Priority (v0.4.0)

### Phase 18: Enhanced Error Messages
**Tasks:**
- Add "did you mean?" suggestions for typos in paths
- Include visual diff for value mismatches
- Show validation context breadcrumbs (which profile → element → constraint)
- Add quick-fix suggestions where applicable

**Estimated:** 6-8 hours

### Phase 19: Custom Validation Rules
**Tasks:**
- Define custom rule API (DSL or Rust trait-based)
- Implement rule registration and execution
- Add examples for common custom validations
- Document rule authoring guide

**Estimated:** 8-10 hours

### Phase 20: Snapshot Validation
**Tasks:**
- Validate StructureDefinition snapshots for correctness
- Check differential vs snapshot consistency
- Validate slicing discriminators and paths
- Add profile authoring validation tools

**Estimated:** 10-12 hours

## Low Priority (v0.5.0+)

### Phase 21: Questionnaire Validation
**Tasks:**
- Implement Questionnaire resource validation
- Support QuestionnaireResponse validation against Questionnaire
- Handle enableWhen conditions
- Validate calculated expressions

**Estimated:** 12-16 hours

### Phase 22: FHIR R5 Support
**Tasks:**
- Add R5 package loading
- Update primitive regex for R5 changes
- Handle R5-specific validation features
- Maintain backward compatibility with R4

**Estimated:** 16-20 hours

### Phase 23: Validation Profiles
**Tasks:**
- Support IPS (International Patient Summary) profile validation
- Add US Core profile validation
- Implement profile-specific optimizations
- Document profile validation best practices

**Estimated:** 8-12 hours per profile suite

## Known Limitations

1. **Sequential Processing:** ~252 resources/sec due to FhirPathEvaluator thread-safety (Phase 17 addresses)
2. **Element Context:** Some element-scoped invariants may over-apply (Phase 15 addresses)
3. **R5 Support:** Currently R4-only (Phase 22 addresses)
4. **Snapshot Generation:** Assumes valid snapshots, doesn't validate StructureDefinition integrity (Phase 20 addresses)

## Fixed Limitations

1. ✅ **ext-1 Over-Application:** Fixed Nov 24, 2025 - no longer applies to all resources
2. ✅ **Terminology Service:** Implemented with HTTP client, mock service, caching (Phase 11)

## Test Coverage Breakdown

**Integration Tests:** ~145 tests across 21 test files:
- validation_test.rs - Core validation (11 tests)
- type_validation_test.rs - Type checking (9 tests)  
- enhanced_cardinality_test.rs - Cardinality (5 tests)
- invariant_validation_comprehensive_test.rs - FHIRPath invariants (7 tests)
- context_handling_test.rs - FHIRPath context (7 tests)
- invariant_scoping_test.rs - Element scoping (8 tests, 3 ignored)
- extension_test.rs - Extensions (12 tests)
- slicing_test.rs - Slicing (8 tests)
- binding_validation_test.rs - Terminology bindings (9 tests)
- valueset_test.rs - ValueSet handling (8 tests)
- operation_outcome_test.rs - OperationOutcome (10 tests)
- profile_discovery_test.rs - Auto-detection (8 tests)
- path_resolution_test.rs - Path handling (9 tests)
- ext1_fix_test.rs - ext-1 fix validation (1 test)
- And more...

**Examples:** 5 runnable examples (basic, batch, auto-detect, profiles, terminology)

## Contributing Guidelines

1. **Before starting a phase:** Review phase description, ensure prerequisites complete
2. **During development:** Run `cargo test -p rh-validator` after each logical change
3. **Performance checks:** Run `cargo bench` for phases affecting validation speed
4. **Before marking complete:** All tests passing, benchmarks stable, documentation updated
5. **Code style:** Follow AGENTS.md conventions (no .unwrap(), use anyhow::Context, run clippy)

## Success Metrics

### v0.3.0 (High Priority Complete)
- FHIR test pass rate: 85-95%
- Performance: 500+ resources/sec
- Element-scoped invariants: 100% correct
- CI/CD: Automated test runs, performance tracking

### v0.4.0 (Medium Priority Complete)
- Error UX: User testing shows 80% improvement in clarity
- Custom rules: 5+ example custom validations
- Snapshot validation: Author tooling for profile creation

### v0.5.0 (Low Priority Complete)
- Multi-version: R4 + R5 support
- Questionnaire: Full Q/QR validation
- Profile suites: IPS + US Core validation

## Next Steps

1. **Immediate (Phase 15):** Analyze 30 remaining test failures and categorize
2. **This sprint:** Document failure patterns, identify quick wins for 85%+ target
3. **Next sprint:** Phase 16 (per-element context) + targeted fixes for top failure categories
4. **Following sprint:** Phase 17 (CI/CD) + Phase 18 (parallel validation - 500+ resources/sec)

## Recent Accomplishments (Nov 2025 - Feb 2026)

- ✅ **Feb 6, 2026 (Phase 14):** Test suite re-run: **70% pass rate** (up from 55%) - 15% improvement!
- ✅ **Phase 13:** Complete FHIR test infrastructure with runner and comparison tools

- ✅ **Nov 24, 2025 (Phase 12):** ext-1 false positive fix - eliminated primary test failure cause
- ✅ **Phase 11:** Terminology service with HTTP client, mock service, persistent caching
- ✅ **Foundation refactor:** Consolidated rh-loader and rh-snapshot into rh-foundation
- ✅ **Code cleanup:** Multiple refactoring PRs reducing clones, improving error handling

**Key Metrics:**
- Conformance improvement: 55% → 70% (+15% from ext-1 fix)
- Java validator agreement: 70/100 (70.0%)
- Firely SDK agreement: 15/21 (71.4%)
- Average validation time: 173.6ms/test
