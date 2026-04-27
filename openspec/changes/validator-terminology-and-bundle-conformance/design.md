## Context

After conformance wave-1 (`2026-03-04-validator-conformance-wave-1`), the validator pass rate on the official FHIR test subset stands at approximately 72% (72/100). The post-wave analysis (`PHASE_15_ANALYSIS.md`) categorized the remaining 30 failures. This change targets the two categories with the clearest scope and highest combined test impact:

### Terminology/Vocabulary Failures (9 tests)

| Test | Root Cause |
|------|------------|
| `vs-bad-code` | ValueSet membership check not enforced for required bindings |
| `cvx` | CVX CodeSystem not available in terminology service |
| `supplement-1`, `supplement-1a`, `supplement-1b`, `supplement-2` | CodeSystem supplement not supported |
| `obs-temp-bad` | UCUM unit validation not enforced |
| `patient-ig-bad` | Profile-specific binding strength not enforced |
| `hakan-se*` (3 tests) | Swedish national CodeSystem not available |

### Bundle Validation Failures (2 tests)

| Test | Root Cause |
|------|------------|
| `bundle-id-5` | Bundle entry ID uniqueness not enforced |
| `mr-covid-bnd1` | Bundle entry reference resolution / `fullUrl` consistency |

## Goals / Non-Goals

**Goals:**
- Implement CodeSystem supplement support so supplement-defined codes are recognized during validation.
- Add ValueSet expansion membership enforcement for `required`-strength bindings.
- Add UCUM unit validation for unit-bearing elements with required UCUM bindings.
- Extend local terminology service stubs to include CVX and UCUM for deterministic test coverage.
- Enforce `Bundle.entry` ID uniqueness.
- Enforce `Bundle.entry.fullUrl` reference consistency.

**Non-Goals:**
- Full terminology server integration (VSAC, remote SNOMED/LOINC expansion) — out of scope for this wave.
- Profile-constraint framework work (separate roadmap item).
- Questionnaire/QuestionnaireResponse validation.
- Changes to `preferred`- or `example`-strength binding behavior.

## Decisions

1. **CodeSystem supplement support scoped to local/bundled supplements**
   - Decision: Implement supplement code recognition for CodeSystem resources that are locally available (loaded with the resource package). Supplements referencing remote systems that are not locally available are treated the same as unknown CodeSystems (warning, not error).
   - Rationale: The failing tests (`supplement-1` through `supplement-2`) use supplements embedded in the test package. Remote supplement resolution is a larger infrastructure concern.
   - Alternatives considered:
     - Full remote supplement resolution: rejected as out of scope and requiring network/caching infrastructure.
     - Skip supplement tests entirely: rejected because they represent a real validation gap.

2. **ValueSet membership enforcement at required binding strength only**
   - Decision: Add explicit ValueSet expansion membership checks for `required`-strength bindings. For `extensible`, `preferred`, and `example` bindings, retain current behavior (no membership error, only warnings where applicable).
   - Rationale: Required bindings are the only conformance-breaking case; tightening weaker strengths risks false positives.
   - Alternatives considered:
     - Uniform enforcement across all binding strengths: rejected due to false positive risk.

3. **UCUM unit validation via deterministic local check**
   - Decision: Validate UCUM units for Quantity and similar element types using the locally available UCUM code set. Do not require a terminology server for unit validation.
   - Rationale: UCUM is a closed, stable code set that can be embedded; network dependence for unit validation is unnecessary overhead.
   - Alternatives considered:
     - Terminology-service-only path: rejected because it makes unit validation unavailable in offline mode.

4. **Bundle entry uniqueness as a structural rule, not an invariant expression**
   - Decision: Implement bundle entry ID uniqueness as a dedicated structural rule in the bundle validator (not as an evaluated FHIRPath invariant), to avoid dependency on element-scoped FHIRPath context (which is addressed in the companion `validator-per-element-invariant-scoping` change).
   - Rationale: Structural rule is simpler, testable in isolation, and does not block on context-passing work.

5. **Local CodeSystem stubs for CVX and Swedish CodeSystems**
   - Decision: Add minimal local stubs for CVX vaccine codes and Swedish CodeSystem codes (sufficient to cover the failing tests) to the mock/test terminology service. Do not attempt full coverage.
   - Rationale: These are test-suite-specific CodeSystems; minimal stubs enable deterministic coverage without large data dependencies.
   - Alternatives considered:
     - Download and embed full external CodeSystems: rejected as maintenance burden and licensing concern.

## Architecture Notes

### Terminology Service Extensions

The `MockTerminologyService` was extended to strip version suffixes (e.g. `|4.0.1`) from
ValueSet URLs before lookup, allowing callers using versioned canonical URLs to match
unversioned registrations.

Supplement resolution was already implemented (`validate_coding_displays` checks `is_supplement`
and `register_codesystem` auto-registers supplement CodeSystems). No new interface methods
were required.

### ValueSet Validation for Required Bindings

In `validate_binding_at_path`, when a ValueSet has no pre-computed expansion (`is_extensional`
returns false) and the binding strength is `required`, the validator now calls
`ts.validate_code_in_valueset(valueset_url, system, code, None)` if a terminology service is
configured. A `TerminologyError::NotFound` (ValueSet unknown to the service) is treated as a
graceful skip. Without a terminology service, intensional ValueSets continue to be skipped.

### Bundle Validation

Bundle entry fullUrl uniqueness, resource identity uniqueness, and `fullUrl` reference
resolution were already fully implemented in `validate_bundle()`. No new work was needed.

### Binding Validation Pipeline Change

In the existing binding validation path, after determining binding strength:
- For `required` strength with no pre-computed expansion: delegate to terminology service if
  configured. Report an error if `result.result == false`.
- For all other strengths, and for `required` strength without a terminology service: no change.

## Risks / Trade-offs

- **[Risk] CodeSystem supplement support introduces false positives for resources that use non-supplemented codes** → **Mitigation:** Supplement checks are additive (they add valid codes, not invalidate existing ones); only codes explicitly defined in supplements become valid.
- **[Risk] ValueSet expansion at validation time adds latency** → **Mitigation:** The existing terminology service already caches expansions; no additional expansion cost for previously-seen ValueSets.
- **[Risk] Bundle uniqueness rule is stricter than current behavior and may affect passing tests** → **Mitigation:** Cross-check against full conformance suite; bundle-id-5 is the only failing test in this category.

## Resolved Questions

**Q: Should the bundle `fullUrl` reference check cover only `urn:uuid:` references, or also relative URL resolution?**

The FHIR R4 bundle reference resolution specification requires resolving all internal reference types: `urn:uuid:`, relative URLs (e.g. `Patient/123`), and absolute URLs — all matched against `entry.fullUrl`. The implementation should follow the full resolution algorithm, not only the `urn:uuid:` case. The `mr-covid-bnd1` failure most likely involves a relative-URL reference that fails to resolve because the matching `fullUrl` entry is absent or mismatched; the full algorithm is required to detect this correctly.

**Q: Is CVX code data available under an appropriate license for embedding in the test service?**

CVX vaccine codes are published by the CDC as a public-domain code system freely available for use. Embedding a representative stub set of CVX codes in the mock terminology service is acceptable from a licensing standpoint. The stub should cover the specific CVX codes used in the failing conformance tests, not attempt full coverage of the ~250+ active CVX codes.
