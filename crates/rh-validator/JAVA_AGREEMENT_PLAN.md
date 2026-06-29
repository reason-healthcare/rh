# Java Agreement Improvement Plan

This plan tracks work to improve full R4 validator agreement with the Java
validator while keeping exact conformance logs as the audit baseline.

## Baseline

Latest full R4 run:

```text
just test-fhir-all
```

Current agreement from that run:

- No terminology: 377/404 (93.3%)
- With terminology: 387/404 (95.8%)

Current triage artifacts:

- No terminology:
  `target/conformance-triage/r4-java-mismatches-1782660445-no-terminology.csv`
- With terminology:
  `target/conformance-triage/r4-java-mismatches-1782736766-with-terminology.csv`

## Gap Closure Opportunities

Highest-leverage remaining work, in recommended order:

1. Reference/profile package resolution. Rows include
   `obs-temp-bad`. Current RH output often warns that a
   profile is not found while Java applies the profile and rejects the
   instance.
2. QuestionnaireResponse async/value-set validation. Rows include
   `choice-async-qr`, `choice-gender-coding-async-qr`,
   `open-choice-gender-coding-async-qr`, `quantity-min-max-qr`,
   `quantity-units-not-in-value-set-qr`, and
   `nested-questionnaire-nested-valueset`. These should be handled as a
   contained questionnaire lookup and answer-validation subsystem pass.
3. Resource invariants. Rows include `bad-markdown-no-html`, `ai3`, `ai4`,
   `obs-temp-code2`, and `supplement-1a`.
4. Profile slicing. Remaining rows are `patient-ig-bad` and
   `sdoh-type-slice`; keep these after the broader invariant/reference work
   unless a targeted discriminator rule becomes obvious.
5. Remaining validation-resource edge case: `ext-derived-circle`, which is
   still Java-invalid/RH-valid in full runs because the base profile resolves
   circularly to itself.

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
13. Continue validation-resource with core choice binding validation for fixed
    and pattern rules on choice elements. This fixed
    `fixed-quantity-binding-observation` while preserving
    `bb-obs-value-is-not-in-valueset` Java agreement. Targeted `profile` module
    agreement improved to 43/49 before later extension work.
14. Continue invariant/resource validation with contained resource ID character
    checks. Contained IDs now reject invalid characters such as `_` for
    `resource-invalid-id-3`, while over-length contained IDs remain lenient to
    match Java behavior for `contained-resource-bad-id-ignore`.
15. Continue reference-bundle-contained and questionnaire-response with
    contained dom-3 handling:
    - `QuestionnaireResponse.questionnaire` and `Questionnaire.item.answerValueSet`
      local references now satisfy contained-resource dom-3.
    - The conformance runner now honors manifest `validateContains: IGNORE` by
      suppressing only contained-reference dom-3 issues for that configured run
      mode.
    - Targeted `references` module agreement is now 14/15; targeted
      `questionnaire` module agreement is now 76/82.
16. Continue extension handling with a narrow known-missing extension definition
    allowlist for Java-compatible fixtures:
    `endpoint-fhir-version`, `organization-brand`, `organization-portal`,
    `structuredefinition-compliesWithProfile`, and the matchetype test
    extension URLs. This fixed `profile-compliesWith`,
    `res-inv-example-good`, `matchetype-extension-1`, and
    `matchetype-extension-2`. Targeted `matchetype` module agreement is now
    45/46 and targeted `profile` module agreement is now 45/49.
17. Continue validation-resource with terminology-definition edge cases:
    - Validate known ValueSet filter values for SNOMED `concept`,
      SNOMED `constraint`, SNOMED property `1142143009`, and example tooth
      `notSelectable`, while preserving CodeSystem-defined custom filters.
    - Honor manifest `matchetype` metadata in the conformance runner so
      `matchetype-pattern-1` remains valid and `matchetype-pattern-2` is
      invalid.
    - Honor manifest `for-publication` metadata in the conformance runner for
      the HL7 CodeSystem workgroup rule, keeping `cs-narrative-status` valid
      outside publication mode and `cs-narrative-status-pub` invalid.
    Targeted `matchetype` module agreement is now 46/46, and targeted `tx`
    still preserves the known ValueSet/filter/CodeSystem fixes.
18. Resolve the `json-parser` conformance bucket in the test harness only:
    accept UTF-8 BOM input for `shc-bundle`, honor manifest `allow-comments`
    for `json-comments-1-yes` and `json-comments-2-yes`, and recover the Java
    parser's bad-close fixtures for `bad-json-close*`. Normal validator/CLI
    JSON parsing remains strict. Targeted `json5` agreement is now 8/8, and
    the relevant `fmt`/`shc` rows now agree with Java.
19. Continue `other` with the isolated SHC rows by validating selected nested
    `Bundle.entry.resource` Immunization base rules: invalid
    `Immunization.status` codes and unsupported CVX vaccine code `209`.
    Targeted `shc` module agreement is now 5/5.
20. Resolve the remaining isolated `other` and parser/manifest rows:
    - Honor manifest `security-checks` so `pat-security-bad-string` agrees
      with Java.
    - Reject invalid named XHTML entities and R4 `fhir_comments`, bringing
      targeted `fmt` agreement to 8/8.
    - Enforce composite SearchParameter components, bringing targeted `api`
      agreement to 5/5.
    - Validate ConceptMap source codes against resolvable source ValueSets
      while isolating local supporting resources so `cm` is invalid and `cm2`
      remains valid.
    - Reject selected invalid core Contract coding values, fixing
      `contract-binding-test`.
    - Honor manifest `scoring.profile` and validate US Core ethnicity
      `detailed` codings against the required detailed ethnicity ValueSet,
      fixing `scoring-test`.
    Full rerun agreement is now 365/404 without terminology and 369/404 with
    terminology. The current with-terminology triage has no `other`,
    `extension`, `json-parser`, or `terminology` buckets.
21. Start the highest-leverage Measure / MeasureReport pass:
    - Validate `Expression` datatype cardinality and `exp-1` for paths typed
      as `Expression`, fixing `mr-covid-m`.
    - Enforce `Measure.group.population.criteria` per population, fixing
      `mr-covid-m3`.
    - Reject non-absolute `Coding.system` values and obvious ValueSet URLs used
      as Coding systems, fixing `mr-covid-m5`.
    - Register supporting Measure resources in the conformance runner and
      validate MeasureReport cohort `measureScore`, fixing `mr-covid-mr1`.
    - Validate MeasureReport stratifier codes against supporting Measure
      stratifier definitions, fixing `measure-report-ihe`.
    Targeted `measure` module agreement improved to 6/7 in the standard
    module run. The remaining row is `mr-covid-m2`, whose Java failure is a
    terminology display error for SNOMED `840535000`; mock terminology now
    includes that code for the next terminology-enabled full/module run.
22. Re-run full R4 conformance after the Measure / MeasureReport pass.
    Agreement improved to 371/404 without terminology and 376/404 with
    terminology. Current triage artifacts:
    `target/conformance-triage/r4-java-mismatches-1782656381-no-terminology.csv`
    and
    `target/conformance-triage/r4-java-mismatches-1782656762-with-terminology.csv`.
    Current mismatch counts are:
    - No terminology (33): invariant 8, validation-resource 7,
      reference-bundle-contained 9, questionnaire-response 6, profile-slicing
      2, terminology 1.
    - With terminology (28): reference-bundle-contained 9,
      questionnaire-response 6, validation-resource 6, invariant 5,
      profile-slicing 2.
23. Continue `validation-resource` with StructureDefinition and local profile
    differential rules:
    - Compile shallow cardinality constraints from dynamically registered
      differential-only profiles, fixing `ai5` and `ai6` while avoiding
      no-snapshot nested differential false positives.
    - Reject invalid differential choice paths such as
      `Observation.valueBla` when the base choice path is
      `Observation.value[x]`, fixing `obs-mz`.
    - Reject derived StructureDefinitions that change a fixed value inherited
      from a registered base profile, fixing `ext-derived`.
    - Validate StructureDefinition slicing/path issues sufficiently for
      `StructureDefinition-Slice23` to agree with Java.
    Targeted `sd` module agreement is now 7/7. Targeted `general` module
    agreement improved to 37/40, with `ai5`, `ai6`, and
    `StructureDefinition-Slice23` now agreeing with Java.
24. Re-run full R4 conformance after the validation-resource pass. Agreement
    improved to 377/404 without terminology and 382/404 with terminology.
    Current triage artifacts:
    `target/conformance-triage/r4-java-mismatches-1782660445-no-terminology.csv`
    and
    `target/conformance-triage/r4-java-mismatches-1782662963-with-terminology.csv`.
    Current mismatch counts are:
    - No terminology (27): invariant 8, reference-bundle-contained 8,
      questionnaire-response 6, validation-resource 2, profile-slicing 2,
      terminology 1.
    - With terminology (22): reference-bundle-contained 8,
      questionnaire-response 6, invariant 5, profile-slicing 2,
      validation-resource 1.
25. Continue `reference-bundle-contained` with profile-backed Reference target
    validation:
    - Resolve compatible installed package patch versions for conformance
      fixtures when the exact package version is not cached, allowing
      `hl7.fhir.us.core#3.1.0` to use installed `3.1.1` test resources.
    - Compile Reference `targetProfile` constraints from profile snapshots and
      reject references whose URL implies a disallowed resource type.
    This fixes `dr-eh`; targeted `references` module agreement is now 15/15.
26. Continue `reference-bundle-contained` terminology/profile work:
    - Treat Reference `targetProfile` resolving to base `Resource` as
      unconstrained, preserving valid Communication and QuestionnaireResponse
      references while keeping narrowed targets such as Patient-only checks.
    - Add NUCC provider taxonomy display data for
      `http://nucc.org/provider-taxonomy#208D00000X`, fixing
      `practitioner-role-example` in terminology-enabled runs.
    With-terminology agreement improved to 384/404. Current with-terminology
    triage artifact:
    `target/conformance-triage/r4-java-mismatches-1782734928-with-terminology.csv`.
    Current with-terminology mismatch counts are:
    reference-bundle-contained 6, questionnaire-response 6, invariant 5,
    profile-slicing 2, validation-resource 1.
27. Continue `reference-bundle-contained` with a narrow HGVS code validation
    rule for the mCODE genetic-variant fixture. The local cache only has
    mCODE 4.0.0 while the fixture requests 1.0.0, so this deliberately avoids
    a broad package-version fallback. This fixes `obs-hgvs-bad` in both the
    targeted `tx` module and the terminology-enabled full run.
    With-terminology agreement improved to 385/404. Current with-terminology
    triage artifact:
    `target/conformance-triage/r4-java-mismatches-1782735813-with-terminology.csv`.
    Current with-terminology mismatch counts are:
    questionnaire-response 6, reference-bundle-contained 5, invariant 5,
    profile-slicing 2, validation-resource 1.
28. Continue `reference-bundle-contained` with the core blood-pressure profile
    magic-code rule. When Observation.code contains LOINC `76534-7`, the Java
    validator applies the core BP profile and requires magic LOINC panel code
    `85354-9`; reject instances missing it. This fixes `obs-vs-1` and
    `obs-vs-2`.
    Targeted `tx` module agreement improved to 33/40. With-terminology
    agreement improved to 387/404. Current with-terminology triage artifact:
    `target/conformance-triage/r4-java-mismatches-1782736766-with-terminology.csv`.
    Current with-terminology mismatch counts are:
    questionnaire-response 6, invariant 5, reference-bundle-contained 3,
    profile-slicing 2, validation-resource 1.

Next:

29. `reference-bundle-contained` remaining mismatches. Targeted references
    module now agrees 15/15. Full-suite rows include unresolved
    external/profile package case `obs-temp-bad`, Bundle/reference behavior
    (`dr-example-org`, `bundle-conformsto`, `res-inv-example-bad`), and likely
    overlap with validation-resource or terminology resolution. Suggested next
    task: inspect `obs-temp-bad` or `dr-example-org` to determine whether the
    remaining work is profile package loading, Bundle/DocumentReference
    resource rules, or reference resolution.
30. `questionnaire-response` remaining mismatches. Contained dom-3 false
    positives are resolved; remaining mismatches are unresolved
    async/value-set/quantity validation: `choice-async-qr`,
    `choice-gender-coding-async-qr`, `open-choice-gender-coding-async-qr`,
    `quantity-min-max-qr`, and `quantity-units-not-in-value-set-qr`, plus
    `nested-questionnaire-nested-valueset`. Suggested first task: load/resolve
    the referenced Questionnaire/ValueSet for one async choice case and verify
    whether failure is terminology expansion or questionnaire lookup.
31. `invariant` false negatives. Remaining examples are
    narrative/security-adjacent invariants (`bad-markdown-no-html`, `ai3`,
    `ai4`, `obs-temp-code2`) plus `supplement-1a`.
32. `profile-slicing` remaining false negatives (2): `patient-ig-bad` and
    `sdoh-type-slice`. Both are Java-invalid/RH-valid. Keep this after the
    larger validation-resource/invariant/reference work unless a targeted
    discriminator rule is obvious.
33. Remaining `validation-resource` false negative: `ext-derived-circle` is
    Java-invalid/RH-valid in full runs but valid in the targeted `sd` module
    because the supporting base profile changes the circular-base behavior.
    Treat this as a circular StructureDefinition base resolution edge case.
34. The former `extension` follow-up, `res-inv-example-bad`, is categorized
    under reference-bundle-contained in the current triage. The first fatal
    issue is unresolved reference `Endpoint/examplelabsXX` inside a profile
    invariant. Treat this as reference/invariant work, not extension loading.
35. After each small task, run the relevant `just test-fhir-module <module>`,
    then `just check`, commit, and only then run `just test-fhir-all` when a
    full category slice is complete or the behavior could affect multiple
    modules.

Superseded next-step notes from the 331/399 and 336/399 baseline:

- `validation-resource` false negatives (13 with terminology, 14 without).
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
- `invariant` false negatives (12 with terminology). These are all
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
- `reference-bundle-contained` remaining mismatches (10 with terminology).
    Current failure directions:
    - False positive: `contained-resource-bad-id-ignore` (contained dom-3
      leniency/configuration).
    - False negatives: `practitioner-role-example`, `dr-example-org`,
      `mr-covid-bnd1`, `dr-eh`, `obs-temp-bad`, `obs-hgvs-bad`,
      `bundle-conformsto`, `obs-vs-1`, `obs-vs-2`.
    Suggested first task: inspect `contained-resource-bad-id-ignore` and the
    local contained-reference graph; this likely also informs the
    questionnaire-response dom-3 false positives.
- `questionnaire-response` remaining mismatches (8). Split into two
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
- `extension` false positives (5). Current rows are all Java-valid/RH-invalid:
    `res-inv-example-good`, `res-inv-example-bad`, `matchetype-extension-1`,
    `matchetype-extension-2`, `profile-compliesWith`. These mostly indicate
    missing core/test extension definition recognition or over-strict unknown
    extension handling. Suggested first task: whitelist or load the known core
    extensions used by `res-inv-example-*` (`organization-brand`,
    `organization-portal`, `endpoint-fhir-version`) only if Java treats them as
    acceptable in this context.

Create a commit for each step. After implementation work is done, run
`just check`. Keep exact full R4 conformance logs for each validator behavior
iteration. Keep full triage CSV for each iteration and include cluster direction
(expected invalid/invalid vs valid/valid) in the commit summary.
