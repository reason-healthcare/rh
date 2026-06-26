# Java Agreement Improvement Plan

This plan tracks work to improve full R4 validator agreement with the Java
validator while keeping exact conformance logs as the audit baseline.

## Baseline

Latest full R4 log:

```text
target/conformance-logs/r4-full-20260625-224327.log
```

Current agreement from that run:

- No terminology: 299/386 (77.5%)
- With terminology: 291/386 (75.4%)

## Steps

1. Make full R4 conformance runs deterministic by forcing a single test thread
   for `test-fhir-all`, so no-terminology and terminology summaries do not
   interleave in logs.
2. Fix result accounting for cases reported as `Expected INVALID, got INVALID
   (0E/0W)`, so parse/load failures and invalid outcomes are represented with
   actionable issue details.
3. Improve package-versioned profile selection before relaxing validation,
   covering both `*-good` false positives and `*-bad` false negatives.
4. Tighten derived extension and profile enforcement for invalid cases that
   Java catches but RH currently reports valid.
5. Fix reference, Bundle, and contained-resource resolution semantics as a
   cluster, including versioned references and internal references.
6. Address deterministic local terminology and binding gaps after structural
   profile/reference behavior is stable.
7. Generate a per-run triage artifact with test name, category, expected
   result, RH result, Java result, error count, and warning count to track
   agreement deltas by subsystem.

After each implementation step, run `just check`, create a commit, and keep the
full R4 conformance log for before/after comparison.
