# False negatives analysis

This document tracks whether `rh-validator` is incorrectly rejecting valid resources.

## Current conclusion

No confirmed false-negative validator bugs are tracked in the current conformance notes.

The earlier `pat-security-good2` concern was resolved as a configuration mismatch, not a correctness bug:

- the test expectation depends on the `security-checks` runtime option
- `rh-validator` now exposes that behavior through `ValidationOptions { security_checks }`
- the CLI exposes the same switch through `rh validate ... --security-checks`

## What remains

Current gaps are feature-completeness and conformance breadth rather than known false-negative defects. The active planning artifacts are:

- [`openspec/planning/rh-validator/TODO.md`](../../openspec/planning/rh-validator/TODO.md)
- [`openspec/planning/rh-validator/PHASE_15_ANALYSIS.md`](../../openspec/planning/rh-validator/PHASE_15_ANALYSIS.md)
- [`openspec/planning/rh-validator/PHASE_15_COMPLETE.md`](../../openspec/planning/rh-validator/PHASE_15_COMPLETE.md)

## Historical note

The `ext-1` false-positive work is complete and remains an important regression area, but it is no longer an open false-negative investigation.
