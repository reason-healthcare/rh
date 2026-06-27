# Java Agreement Improvement Plan

This plan tracks work to improve full R4 validator agreement with the Java
validator while keeping exact conformance logs as the audit baseline.

## Baseline

Latest full R4 log:

```text
target/conformance-logs/r4-full-20260627-065927-reference-bundle-contained.log
```

Current agreement from that run:

- No terminology: 324/399 (81.2%)
- With terminology: 328/399 (82.2%)

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
5. Generate a per-run Java mismatch triage artifact with test name, category,
   expected result, RH result, Java result, error count, and warning count. Use
   this artifact to pick subsequent validator behavior work by largest
   mismatch cluster rather than by intuition. The exact full R4 log for this
   iteration is
   `target/conformance-logs/r4-full-20260626-074737-java-triage-final.log`;
   agreement remained 310/399 without terminology and 314/399 with terminology.
   Triage artifacts:
   `target/conformance-triage/r4-java-mismatches-1782474630-no-terminology.csv`
   and
   `target/conformance-triage/r4-java-mismatches-1782474794-with-terminology.csv`.
   The largest with-terminology clusters are validation-resource (22),
   reference-bundle-contained (19), invariant (11), and
   questionnaire-response (9).
6. Improve validation-resource checks with a focused first slice: enforce
   `CodeSystem.supplements` content, validate parameterized ValueSet
   expressions, check known SearchParameter `derivedFrom` type/base
   consistency, check known CapabilityStatement search parameter definition
   types, and preserve Java-compatible `system|code` Coding filter values as
   non-fatal. The exact full R4 log for this iteration is
   `target/conformance-logs/r4-full-20260626-082311-validation-resource.log`;
   agreement improved to 318/399 without terminology and 322/399 with
   terminology. Triage artifacts:
   `target/conformance-triage/r4-java-mismatches-1782476812-no-terminology.csv`
   and
   `target/conformance-triage/r4-java-mismatches-1782476977-with-terminology.csv`.
   The with-terminology validation-resource cluster dropped from 22 to 14.
7. Improve reference, Bundle, contained-resource, signature, html-reference, and
   UCUM-display agreement. Fix document bundle duplicate handling for versioned
   references, accept `uin:uuid:` fullUrls, make contained resource IDs align
   with Java's leniency, enforce selected Bundle/reference/html/signature false
   negatives, and stop treating UCUM display-name differences as fatal in the
   mock terminology pass. The exact full R4 log for this iteration is
   `target/conformance-logs/r4-full-20260627-065927-reference-bundle-contained.log`;
   agreement improved to 324/399 without terminology and 328/399 with
   terminology. Triage artifacts:
   `target/conformance-triage/r4-java-mismatches-1782558183-no-terminology.csv`
   and
   `target/conformance-triage/r4-java-mismatches-1782558347-with-terminology.csv`.
   The reference-bundle-contained cluster dropped to 14 without terminology and
   13 with terminology. A follow-up targeted `dsig` module run fixed the
   `signatures-example-1` false positive while preserving
   `signatures-example-2` invalid agreement; a full R4 rerun is still needed to
   snapshot that follow-up.

Next:

8. Continue the remaining `reference-bundle-contained` mismatches. Based on
   `r4-java-mismatches-1782558347-with-terminology.csv`, prioritize:
   - False positives: `vs-canonical-good`, `contained-resource-bad-id-ignore`.
   - False negatives: `practitioner-role-example`, `dr-example-org`,
     `mr-covid-bnd1`, `dr-eh`, `obs-temp-bad`, `obs-hgvs-bad`,
     `bundle-conformsto`, `bundle-ea-testcase`, `obs-vs-1`, `obs-vs-2`.
   - Deliverable: full alignment on all `reference-bundle-contained` mismatches in
     a new full-R4 iteration and updated triage file.
9. Continue `validation-resource` false negatives (14 currently): fix remaining
   SNOMED/ValueSet concept/filter/ECL issues, fix conformance resource status
   and jurisdiction constraints, and enforce invariant execution where currently
   warning-only or no-op.
10. Fix `invariant` and `extension` false positives by loading/recognizing core
   extension definitions in canonical order and treating unresolved canonical
   extension references consistently with Java warning/error semantics.
11. Continue `questionnaire-response` alignment (9 mismatches): tighten
    async-terminology behavior, answer value-set resolution, and quantity/min-max
    unit validation.
12. Add a conformance-only lenient JSON parser mode for fixtures that Java
    accepts but strict JSON rejects (`json-parser` mismatch cluster), without
    affecting normal CLI validation mode.
13. Re-run full-R4 conformance with one-threaded execution, snapshot the full
    logs and triage CSV, then report delta agreement per category and decide the
    next highest-priority cluster.

Create a commit for each step. After implementation work is done, run
`just check`. Keep exact full R4 conformance logs for each validator behavior
iteration. Keep full triage CSV for each iteration and include cluster direction
(expected invalid/invalid vs valid/valid) in the commit summary.
