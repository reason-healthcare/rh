## Why

`rh-validator` is production-ready but still at ~70% alignment on the official FHIR test subset, with remaining gaps clearly documented in `TODO.md` and `PHASE_15_ANALYSIS.md`. The highest-leverage next step is a focused conformance wave that closes known feature gaps (not correctness bugs) and makes progress measurable in CI.

## What Changes

- Implement `security-checks` configuration behavior so HTML/script checks match test manifest expectations.
- Deliver documented quick wins from conformance analysis (terminology and bundle checks) to raise pass rate with bounded scope.
- Add automated conformance execution/reporting in CI so pass-rate changes are visible and regressions are caught early.

## Capabilities

### New Capabilities
- `security-checks-config`: Configurable severity for security-sensitive content checks (information vs error) driven by validator/test settings.
- `conformance-quick-wins`: Targeted validation additions for explicitly identified failing cases (e.g., ValueSet/code validation, UCUM/unit checks, bundle uniqueness checks).
- `conformance-ci-gating`: Repeatable CI runs for selected FHIR test suites with trendable pass-rate outputs.

### Modified Capabilities
- `validator-cli-behavior`: Extend CLI/test-runner behavior to pass through conformance-related runtime options and report outcomes consistently.

## Impact

- Affected crates: `crates/rh-validator`, `apps/rh-cli`.
- Affected docs: validator roadmap/conformance docs and CLI validator docs.
- Expected outcome: move from static roadmap items to an executable conformance increment with measurable pass-rate improvement.
