# Java Agreement Improvement Plan

This plan tracks work to improve full R4 validator agreement with the Java
validator while keeping exact conformance logs as the audit baseline.

## Baseline

Latest full R4 log:

```text
target/conformance-logs/r4-full-20260626-023515-package-id-resolution.log
```

Current agreement from that run:

- No terminology: 310/399 (77.7%)
- With terminology: 314/399 (78.7%)

## Steps

Completed:

1. Make full R4 conformance runs deterministic by forcing a single test thread
   for `test-fhir-all`, so no-terminology and terminology summaries do not
   interleave in logs.
2. Fix result accounting for cases reported as `Expected INVALID, got INVALID
   (0E/0W)`, so parse/load failures and invalid outcomes are represented with
   actionable issue details.
3. Improve package-versioned profile selection before relaxing validation,
   covering both `*-good` false positives and `*-bad` false negatives.
4. Resolve manifest package IDs, not just local `.tgz` archives. Reuse installed
   package cache entries such as `~/.fhir/packages/<id>#<version>/package` when
   the R4 manifest lists package IDs in `packages` or `package-map`, while
   keeping local archive materialization for test-owned packages. The exact
   full R4 log for this iteration is
   `target/conformance-logs/r4-full-20260626-023515-package-id-resolution.log`;
   agreement remained 310/399 without terminology and 314/399 with terminology.

Next:

5. Add a conformance-only lenient JSON parser mode for validator fixtures that
   Java accepts but strict JSON rejects, without changing normal CLI validation
   behavior.
6. Improve validation-resource checks for `StructureDefinition`, `ValueSet`, and
   `CodeSystem` cases, especially property/filter/ECL/status failures currently
   reported valid by RH.
7. Tighten `QuestionnaireResponse` agreement for answer value set resolution,
   async terminology expectations, and quantity min/max or unit compatibility.
8. Reduce false positives from core invariant and extension handling after
   loading/recognizing core extension definitions more completely.
9. Fix reference, Bundle, contained-resource, signature, and html-reference
   semantics as a later structural cluster.
10. Generate a per-run triage artifact with test name, category, expected
    result, RH result, Java result, error count, and warning count to track
    agreement deltas by subsystem.

Create a commit for each step. After implementation work is done, run
`just check`. Keep exact full R4 conformance logs for each validator behavior
iteration.
