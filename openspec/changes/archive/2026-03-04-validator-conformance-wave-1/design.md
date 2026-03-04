## Context

`rh-validator` is at approximately 70% alignment on the official FHIR test subset, with known feature gaps documented in `crates/rh-validator/TODO.md` and `crates/rh-validator/PHASE_15_ANALYSIS.md`. This change targets a bounded conformance wave focused on known missing capabilities rather than broad refactors or bug-hunt work.

The proposal defines four capability areas to progress in lockstep:
- `security-checks-config`
- `conformance-quick-wins`
- `conformance-ci-gating`
- `validator-cli-behavior`

Primary stakeholders are validator maintainers and CLI users running conformance suites in CI.

## Goals / Non-Goals

**Goals:**
- Add configurable behavior for security-sensitive content checks so conformance manifests can treat them as informational or error-level findings.
- Implement targeted, documented quick-win validators for known failing categories (terminology/unit and bundle uniqueness cases).
- Add repeatable CI conformance execution with per-run pass/fail summaries and artifacts (without trend tooling in this wave).
- Keep runtime behavior explicit through CLI options and consistent reporting.

**Non-Goals:**
- Full conformance parity with all official FHIR tests in this wave.
- Re-architecting the validator pipeline or rule engine.
- Large-scale performance optimization unrelated to targeted conformance tasks.
- Expanding scope into unrelated validator roadmap items.

## Decisions

1. **Introduce explicit security-check policy at validator configuration boundaries**
   - Decision: Represent security-check severity as a runtime configuration that flows from CLI/test runner into validator execution context.
   - Rationale: Conformance harnesses require deterministic treatment of HTML/script checks; embedding this as config avoids hardcoded behavior.
   - Alternatives considered:
     - Build-time feature flags: rejected because conformance behavior must be switchable per run.
     - Environment-variable-only controls: rejected for poor discoverability and reproducibility in CI logs.

2. **Deliver quick wins as focused rule additions in existing validation domains**
  - Decision: Implement narrow checks in existing terminology and bundle validation paths (e.g., ValueSet/code checks, UCUM/unit checks only when optional terminology service is configured, bundle uniqueness constraints).
   - Rationale: Uses existing architecture and minimizes integration risk while addressing high-impact known failures.
   - Alternatives considered:
     - Broad standards-engine rewrite: rejected as high-risk and out of scope.
     - Suite-specific ad-hoc exceptions: rejected because they reduce correctness and maintainability.

3. **Standardize conformance execution/reporting through CI job contract**
   - Decision: Add CI workflow(s) that run selected conformance subsets and persist outputs as artifacts/summary metrics.
  - Rationale: Enables regression detection with per-run visibility while keeping this wave simple and operationally stable.
   - Alternatives considered:
     - Local-only scripts: rejected because they do not enforce consistency on merges.
     - Blocking on full-suite pass immediately: rejected to preserve iterative delivery.

4. **Keep CLI behavior additive and backward compatible**
   - Decision: Add or extend CLI flags for conformance/runtime options without breaking existing defaults.
   - Rationale: Existing users should retain current behavior unless options are explicitly enabled.
   - Alternatives considered:
     - Changing default severity globally: rejected due to risk of unexpected downstream behavior changes.

## Risks / Trade-offs

- **[Risk] Conformance-focused logic diverges from generic validator behavior** → **Mitigation:** Implement as parameterized rules in shared paths; avoid test-suite-specific branching.
- **[Risk] Terminology/UCUM checks require external assumptions or incomplete data** → **Mitigation:** Bound initial implementation to deterministic local checks and document unsupported edge cases.
- **[Risk] CI runtime and flakiness increase with conformance runs** → **Mitigation:** Start with selected stable subsets, cache dependencies, and use staged non-blocking → blocking gates.
- **[Risk] New CLI options confuse existing users** → **Mitigation:** Keep defaults unchanged, document examples in CLI validator docs, and print explicit run configuration in conformance output.

## Migration Plan

1. Add validator config model updates for security-check severity and thread through CLI/test runner interfaces.
2. Implement quick-win checks in small, separately testable increments (terminology/unit and bundle uniqueness).
3. Add/extend conformance integration tests and golden expectations for each increment.
4. Introduce CI workflow for selected conformance suite execution and summary artifact publication.
5. Keep per-run conformance summaries only (explicitly no trend tooling in this wave).
6. Promote selected thresholds to gating once baseline stability is established.

**Rollback strategy:**
- Revert or disable new runtime options to restore prior behavior.
- Mark CI conformance job as non-blocking or remove new gate thresholds if unstable.
- Keep quick-win checks behind clear configuration paths where necessary for staged rollout.

## Open Questions

- Which exact conformance test subsets should be in the first CI run to maximize signal-to-runtime ratio?
- Should UCUM/unit validation in this wave use terminology service validation only, or include additional deterministic fallback checks when terminology is unavailable?
- What pass-rate delta (absolute or relative) should trigger an initial CI warning versus merge-blocking gate?
