# Java Agreement Improvement Plan

This plan tracks work to improve full R4 validator agreement with the Java
validator while keeping exact conformance logs as the audit baseline.

## Baseline

Latest full R4 run:

```text
just test-fhir-all
```

Current agreement from that run:

- No terminology: 331/399 (83.0%)
- With terminology: 336/399 (84.2%)

Current triage artifacts:

- No terminology:
  `target/conformance-triage/r4-java-mismatches-1782570430-no-terminology.csv`
- With terminology:
  `target/conformance-triage/r4-java-mismatches-1782570600-with-terminology.csv`

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
8. Apply focused invariant and extension increments:
   - Apply the Period datatype invariant (`per-1`) so `encounter-period` aligns
     with Java.
   - Enforce the `humanname-mothers-family` extension context on
     `HumanName.family`, fixing `maiden-name-extension`.
   - Validate `ValueSet.compose.include.concept` codes against supported
     terminology in terminology-enabled runs, fixing `vs-bad-code`.
9. Continue reference-bundle-contained with a transaction/batch reference rule:
   relative references inside a Bundle entry missing `fullUrl` are invalid even
   when the bundle type is `transaction` or `batch`. This fixed
   `bundle-ea-testcase` and improved the `bundle` module to 20/23 Java
   agreement before later full-suite rerun.
10. Continue questionnaire-response with the base Questionnaire `qst-2`
    invariant: multiple `enableWhen` conditions require `enableBehavior`. This
    fixed `q-enablewhen-me-wrong` and improved the `questionnaire` module to
    75/82 Java agreement.
11. Continue profile-slicing with same-div XHTML narrative fragment validation:
    `href="#fragment"` links in `Narrative.div` must resolve to a local `id` or
    `name` anchor. This fixed `ab-list-slicing` and improved the `profile`
    module to 42/49 Java agreement.
12. Re-run full R4 conformance after the increments above. The current full
    suite agreement is 331/399 without terminology and 336/399 with
    terminology. Current mismatch counts are:
    - No terminology (68): validation-resource 14, invariant 14,
      reference-bundle-contained 11, questionnaire-response 8, other 7,
      json-parser 6, extension 5, profile-slicing 2, terminology 1.
    - With terminology (63): validation-resource 13, invariant 12,
      reference-bundle-contained 10, questionnaire-response 8, other 7,
      json-parser 6, extension 5, profile-slicing 2.

Next:

13. `validation-resource` false negatives (13 with terminology, 14 without).
    This is now the largest with-terminology cluster and all current
    with-terminology rows are Java-invalid/RH-valid. Keep these as small
    sub-steps:
    - Fixed/pattern quantity binding: `fixed-quantity-binding-observation`.
    - Measure/MeasureReport resource rules: `mr-covid-m5`,
      `measure-report-ihe`.
    - StructureDefinition invariants or differential/snapshot rules:
      `obs-mz`, `ai5`, `ai6`, `StructureDefinition-Slice23`, `ext-derived`,
      `ext-derived-circle`.
    - ValueSet/ECL/property rules: `vs-bad-props`, `vs-bad-ecl`,
      `vs-bad-ecl-us`.
    - CodeSystem metadata/status rule: `cs-narrative-status-pub`.
    - Match-type pattern rule: `matchetype-pattern-2`.
    Suggested first task: inspect Java outcome for
    `fixed-quantity-binding-observation` and implement one fixed Quantity
    profile check; verify only the `profile` module before committing.
14. `invariant` false negatives (12 with terminology). These are all
    Java-invalid/RH-valid and should be split by resource family:
    - Measure/MeasureReport invariants: `mr-covid-m`, `mr-covid-m2`,
      `mr-covid-m3`, `mr-covid-mr1`, `measure-report-ihe`.
    - Narrative/comment/id/security-adjacent invariants:
      `resource-invalid-id-3`, `bad-markdown-no-html`, `comments-4`, `ai3`,
      `ai4`, `obs-temp-code2`.
    - Observation supplement invariant: `supplement-1a`.
    Suggested first task: inspect the Java outcome for `resource-invalid-id-3`
    and decide whether this belongs in base resource ID validation or a
    profile-specific invariant.
15. `reference-bundle-contained` remaining mismatches (10 with terminology).
    Current failure directions:
    - False positive: `contained-resource-bad-id-ignore` (contained dom-3
      leniency/configuration).
    - False negatives: `practitioner-role-example`, `dr-example-org`,
      `mr-covid-bnd1`, `dr-eh`, `obs-temp-bad`, `obs-hgvs-bad`,
      `bundle-conformsto`, `obs-vs-1`, `obs-vs-2`.
    Suggested first task: inspect `contained-resource-bad-id-ignore` and the
    local contained-reference graph; this likely also informs the
    questionnaire-response dom-3 false positives.
16. `questionnaire-response` remaining mismatches (8). Split into two
    independent sub-clusters:
    - False positives from contained dom-3/reference detection: `contained`,
      `qr-internal-refs`, `q_val_fail`.
    - False negatives from unresolved async/value-set/quantity validation:
      `choice-async-qr`, `choice-gender-coding-async-qr`,
      `open-choice-gender-coding-async-qr`, `quantity-min-max-qr`,
      `quantity-units-not-in-value-set-qr`.
    Suggested first task: fix contained dom-3 reference detection for
    Questionnaire/QuestionnaireResponse local references, because it can reduce
    both this cluster and `reference-bundle-contained`.
17. `extension` false positives (5). Current rows are all Java-valid/RH-invalid:
    `res-inv-example-good`, `res-inv-example-bad`, `matchetype-extension-1`,
    `matchetype-extension-2`, `profile-compliesWith`. These mostly indicate
    missing core/test extension definition recognition or over-strict unknown
    extension handling. Suggested first task: whitelist or load the known core
    extensions used by `res-inv-example-*` (`organization-brand`,
    `organization-portal`, `endpoint-fhir-version`) only if Java treats them as
    acceptable in this context.
18. `profile-slicing` remaining false negatives (2): `patient-ig-bad` and
    `sdoh-type-slice`. Both are Java-invalid/RH-valid. Keep this after the
    larger validation-resource/invariant/reference work unless a targeted
    discriminator rule is obvious.
19. `json-parser` false positives (6): `shc-bundle`, `json-comments-1-yes`,
    `json-comments-2-yes`, `bad-json-close`, `bad-json-close-2`,
    `bad-json-close-3`. Add a conformance-only lenient JSON parser mode only in
    the test harness; do not relax normal CLI validation.
20. `other` false negatives (7): `pat-security-bad-string`, `shc-bad-1`,
    `shc-cvx`, `cm`, `sp-composite`, `contract-binding-test`, `scoring-test`.
    Leave these as last-resort investigation buckets unless one maps cleanly to
    an existing validator subsystem.
21. After each small task, run the relevant `just test-fhir-module <module>`,
    then `just check`, commit, and only then run `just test-fhir-all` when a
    full category slice is complete or the behavior could affect multiple
    modules.

Create a commit for each step. After implementation work is done, run
`just check`. Keep exact full R4 conformance logs for each validator behavior
iteration. Keep full triage CSV for each iteration and include cluster direction
(expected invalid/invalid vs valid/valid) in the commit summary.
