# RH Validator Development Plan

## Status: v0.2.0 - Production Ready

**Tests:** 1,287 passing (132 validator-specific)  
**Performance:** 3-4ms/resource, ~200 resources/sec batch  
**Cache Hit Rate:** 100% in benchmarks  
**Last Major Fix:** ext-1 invariant scoping (Nov 24, 2025)

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

### Phase 11: Documentation ✅
README, examples (basic, batch, auto-detect, profiles, output), and API docs.

## Current Phase

### Phase 12: FHIR Test Cases Integration 🚧

**Goal:** Validate against official FHIR R4 test suite to ensure spec compliance.

**Progress:**
- ✅ 12.1: Downloaded and organized 570 R4 test cases
- ✅ 12.2: Implemented test case parser
- ✅ 12.3: Built test runner with batch execution
- ✅ 12.4: Array structure validation for max>1 elements
- ✅ 12.5: Profile-not-found as warning (Java compatibility)
- ✅ 12.6: Base64Binary format validation
- 🚧 12.7: 100-test suite: 53/100 (53%) agreement with Java
- ⏳ 12.8: Document known limitations vs FHIR spec
- ⏳ 12.9: Set up automated test runs in CI

**Current Test Status (Nov 26, 2025):**
- Agreement with Java: 53/100 (53%)
- False Positives: 1 (contained - FHIRPath descendants() bug)
- False Negatives: 46 (mostly missing validations)

**Remaining False Negatives Categories:**
1. **External profiles:** Tests require profiles we don't have
2. **Bundle validation:** Reference resolution, duplicate IDs
3. **XHTML/narrative:** txt-1 invariant, security checks
4. **Terminology:** SNOMED/LOINC validation requires terminology server
5. **Attachment size:** Validate data length matches size field

**Next Action:** Fix FHIRPath descendants() bug in rh-fhirpath to resolve last false positive.

## High Priority (v0.3.0)

### Phase 13: Per-Element Invariant Evaluation
**Why:** 3 ignored tests require proper element-scoped FHIRPath context.  
**Tasks:**
- Enhance FHIRPath evaluator to track element context during evaluation
- Update validate_invariant() to pass current element context
- Un-ignore tests in invariant_scoping_test.rs
- Add integration tests for nested element constraints

**Estimated:** 6-8 hours  
**Success Criteria:** All invariant_scoping_test.rs tests passing, proper context resolution in nested elements.

### Phase 14: CI/CD Integration
**Why:** Automate testing and performance monitoring.  
**Tasks:**
- Add validator benchmarks to GitHub Actions
- Set up performance regression detection
- Run FHIR test suite in CI
- Track pass rate trends over time

**Estimated:** 4-6 hours  
**Success Criteria:** Automated test runs, performance alerts, test pass rate dashboard.

### Phase 15: Parallel Validation
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

### Phase 16: Enhanced Error Messages
**Tasks:**
- Add "did you mean?" suggestions for typos in paths
- Include visual diff for value mismatches
- Show validation context breadcrumbs (which profile → element → constraint)
- Add quick-fix suggestions where applicable

**Estimated:** 6-8 hours

### Phase 17: Custom Validation Rules
**Tasks:**
- Define custom rule API (DSL or Rust trait-based)
- Implement rule registration and execution
- Add examples for common custom validations
- Document rule authoring guide

**Estimated:** 8-10 hours

### Phase 18: Snapshot Validation
**Tasks:**
- Validate StructureDefinition snapshots for correctness
- Check differential vs snapshot consistency
- Validate slicing discriminators and paths
- Add profile authoring validation tools

**Estimated:** 10-12 hours

## Low Priority (v0.5.0+)

### Phase 19: Questionnaire Validation
**Tasks:**
- Implement Questionnaire resource validation
- Support QuestionnaireResponse validation against Questionnaire
- Handle enableWhen conditions
- Validate calculated expressions

**Estimated:** 12-16 hours

### Phase 20: FHIR R5 Support
**Tasks:**
- Add R5 package loading
- Update primitive regex for R5 changes
- Handle R5-specific validation features
- Maintain backward compatibility with R4

**Estimated:** 16-20 hours

### Phase 21: Validation Profiles
**Tasks:**
- Support IPS (International Patient Summary) profile validation
- Add US Core profile validation
- Implement profile-specific optimizations
- Document profile validation best practices

**Estimated:** 8-12 hours per profile suite

## Known Limitations

1. **Sequential Processing:** ~200 resources/sec due to FhirPathEvaluator thread-safety (Phase 15 addresses)
2. **Element Context:** Some element-scoped invariants may over-apply (Phase 13 addresses)
3. **Custom Terminology:** External terminology server integration not yet implemented
4. **R5 Support:** Currently R4-only (Phase 20 addresses)
5. **Snapshot Generation:** Assumes valid snapshots, doesn't validate StructureDefinition integrity (Phase 18 addresses)

## Test Coverage Breakdown

- **Core validation:** 47 tests (cardinality, types, primitives)
- **FHIRPath constraints:** 28 tests (invariants, dom-* rules)
- **Terminology:** 18 tests (ValueSet bindings, all strengths)
- **Extensions:** 12 tests (modifierExtension, profiles)
- **Slicing:** 15 tests (discrimination, ordering)
- **References:** 9 tests (type checking, target profiles)
- **OperationOutcome:** 17 tests (all IssueType codes, severity)
- **Integration:** 5 examples (basic, batch, auto-detect, profiles, output)
- **Ignored (pending Phase 13):** 3 tests (element-scoped invariants)

**Total:** 132 validator tests, 1,287 workspace tests

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

1. **Immediate:** Re-run 570 FHIR test cases after ext-1 fix
2. **This week:** Complete Phase 12 analysis and reporting
3. **Next sprint:** Phase 13 (per-element context) + Phase 14 (CI/CD)
4. **Following sprint:** Phase 15 (parallel validation)
