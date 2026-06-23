# CQL Remediation Plan

Last updated: 2026-06-23

This plan is based on `conformance/results/summaries/latest-summary.md`.
It addresses current `rh-cql` gaps while avoiding the assumption that every
external corpus file is specification-correct.

## Current Signal

### HL7 Expression Matrix

The HL7 expression matrix remains the strongest specification-facing signal:

| Metric | Count |
|---|---:|
| Parsed HL7 cases | 1 426 |
| `rh-cql` pass | 1 241 |
| Wrong-answer fail | 0 |
| Skip | 45 |
| Compile error | 89 |
| Eval error | 23 |
| Invalid input unexpectedly accepted | 0 |
| Unimplemented total | 112 |

Failure concentration:

| Category | Main suites |
|---|---|
| Compile errors | intervals 35, date/time 33, value literals/selectors 17 |
| Eval errors | date/time 12, arithmetic 6, intervals 2, value literals/selectors 2, type operators 1 |
| Invalid input enforcement | none remaining |

### Three-Engine Matrix

| Implementation | Pass | Compile Err | Eval Err | Fail | Skip | Unimplemented |
|---|---:|---:|---:|---:|---:|---:|
| `rh-cql` | 1 241 | 89 | 23 | 0 | 45 | 0 |
| Java ELM | 1 410 | 16 | 0 | 0 | 0 | 0 |
| JavaScript `cql-execution` | 594 | 118 | 467 | 81 | 166 | 0 |

Interpretation:

- Java ELM passes are high-confidence translation targets.
- Java ELM failures are not automatically `rh-cql` bugs.
- JavaScript failures are interoperability findings against `rh-cql` ELM and
  `cql-execution`, not independent CQL source validation.

### Expanded Corpus

| Corpus | Files | RH Pass | RH Compile Err | Java Pass | Java Non-Pass |
|---|---:|---:|---:|---:|---:|
| Generated fixtures | 8 | 4 | 4 | 5 | 3 |
| CQFramework jvmTest | 358 | 185 | 173 | 136 | 222 |
| CQFramework examples | 34 | 1 | 33 | 6 | 28 |
| Cooking with CQL | 732 | 132 | 600 | 175 | 557 |
| CMS 2025 eCQM | 116 | 15 | 101 | 51 | 64 + 1 timeout |
| **Total** | **1 248** | **337** | **911** | **373** | **874 + 1 timeout** |

Java-pass/RH-fail rows are high-value remediation candidates:

| Corpus | Java-pass/RH-fail |
|---|---:|
| Generated fixtures | 4 |
| CQFramework jvmTest | 65 |
| CQFramework examples | 6 |
| Cooking with CQL | 87 |
| CMS 2025 eCQM | 47 |

## Validity Policy

Do not blindly assume every external file is valid CQL. Use this weighting:

| Source status | Weight | Action |
|---|---:|---|
| Java translator passes | High | Treat as a likely valid CQL source and prioritize RH parser/semantic support |
| Java translator fails | Low | Quarantine as source-invalid or reference-ambiguous until manually reviewed |
| Java translator times out | Unknown | Rerun with a higher timeout or isolate before using as a requirement |
| HL7 XML test | High | Treat as specification-facing even when informative rather than normative |
| Generated local fixture | High for intent, medium for syntax | Validate with Java before treating as a conformance target |
| JavaScript pass/fail only | Medium | Use as ELM interoperability signal, not CQL source validity proof |

Every new remediation issue should record:

- Source file and corpus.
- Java translator status.
- `rh-cql` status and diagnostic.
- Whether the CQL construct is directly supported by the CQL specification.
- Minimal reduced CQL reproducer.
- Expected ELM shape or evaluation result.

## Workstream 1: Corpus Triage And Reporting

Goal: make corpus failures actionable instead of large undifferentiated counts.

Status:

- [x] 2026-06-22: Added corpus diagnostic classes, Java-pass/RH-fail and
  Java-non-pass output views, and summary-level diagnostic counts.
- [x] 2026-06-22: Full Java-inclusive corpus rerun completed with hardened
  Java status detection. Current Java-pass/RH-fail total is 209: parser 150,
  semantic 59. Java non-pass quarantine total is 875: include-resolution 270,
  semantic 183, parser 101, model-info 7, unknown 313, timeout 1.
- [x] 2026-06-22: Reduction workflow documented in `CQL_TEST_CORPUS.md`.

Tasks:

1. [x] Add diagnostic classification to `run_corpus_audit.py`.
2. [x] Emit grouped counts for parser, semantic, model-info, include-resolution, and timeout failures.
3. [x] Emit a `java_pass_rh_fail.csv` view sorted by corpus and diagnostic class.
4. [x] Emit a `java_non_pass.csv` quarantine view.
5. [x] Add a reduction workflow for turning corpus failures into small committed fixtures.

Acceptance criteria:

- A summary shows Java-pass/RH-fail counts by diagnostic class.
- Java-non-pass rows are visible but excluded from remediation totals by default.
- The generated local fixtures all have explicit Java status in the summary.

## Workstream 2: Parser Coverage For Java-Passing Corpus

Goal: reduce broad parse failures that Java accepts.

Status:

- [x] 2026-06-22: Added parser support for `define public` and
  `define private` expression/function forms. Generated
  `library-boundaries/HelperLibrary.cql` now advances from parser failure to a
  semantic function-parameter resolution failure.
- [x] 2026-06-22: Classified omitted-colon define bodies as source-invalid for
  remediation purposes. The pinned Java translator reports `Syntax error at 42`
  and writes no ELM for the reduced case in
  `conformance/corpus/invalid-or-ambiguous.md`.
- [x] 2026-06-22: Hardened corpus Java status detection so translator output
  containing `Translation failed` is classified as `compile_error` even when
  the Java CLI process exits `0`.
- [x] 2026-06-22: Verified measure-style retrieve queries such as
  `[Encounter: "Value Set"] E where ...` parse, and fixed explicit retrieve
  code paths so `[Encounter: reason in "Asthma"] E where ...` emits
  `codeProperty: "reason"` instead of falling back to the model primary code
  path.
- [x] 2026-06-23: Expanded terminology parsing for qualified code system
  references in code/value set/concept declarations, and added `Code '...'
  from ... display ...` expression selector parsing. Representative Java-pass
  files now advance: `LibraryTests/Issue641.cql` reaches qualified-library
  semantic errors, `CumulativeMedicationDuration.cql` reaches a later interval
  aggregate parse gap, and `OperatorTests/TypeOperators.cql` compiles.
- [x] 2026-06-23: `just corpus-audit-rh` improved the RH-only expanded corpus
  baseline to 351 pass / 897 compile errors, from the prior 337 pass / 911
  compile errors.
- [x] 2026-06-23: Added parser coverage for hour-precision time literals
  (`@T10`) and prefix-sensitive interval operators (`overlaps before`,
  `meets after`). Generated `DateTimePrecision.cql` and
  `IntervalBoundaries.cql` now compile, and `just corpus-audit-rh` improves the
  RH-only expanded corpus baseline to 355 pass / 893 compile errors.

Observed patterns:

- Missing support for some real-world define forms and access modifiers.
- Retrieve/query syntax used in measure content.
- Date/time and interval phrase variants.
- Terminology declaration variants, including code display/codesystem forms.
- Library-level declaration ordering found in examples.

Initial targets:

1. [x] Support `define public` / `define private` forms in addition to current access parsing.
2. [x] Support omitted-colon define bodies where accepted by the Java translator, or explicitly classify source-invalid if not spec-valid.
3. [x] Expand retrieve parsing for measure-style `[Type: "Value Set"] Alias where ...`.
4. [x] Expand code, codesystem, valueset, and display parsing to match Java-passing corpus forms.
5. [x] Add date/time phrase parser coverage for `same or before`, `same or after`, `before`, `after`, and interval phrase variants.

Acceptance criteria:

- Each parser fix includes a minimized Java-passing fixture.
- `just corpus-audit-rh` shows a measurable drop in Java-pass/RH parse errors.
- No regression in `just audit-strict`.

## Workstream 3: Semantic And Library Resolution

Goal: handle Java-passing files that parse but fail semantic analysis.

Status:

- [x] 2026-06-23: Added function-parameter scoping during semantic analysis,
  so helper library functions such as `AddOne(value Integer): value + 1`
  compile without unresolved-identifier diagnostics.
- [x] 2026-06-23: Made CLI compile/validate use the include-aware library
  provider path, added `cql compile --lib-path`, and updated the corpus audit
  to pass each file's directory as a library search path.
- [x] 2026-06-23: Preserved library qualifiers on typed function invocations
  so included calls such as `Helper.AddOne(4)` emit `FunctionRef` with
  `libraryName: "Helper"`. The generated `library-boundaries/MainLibrary.cql`
  fixture now compiles and validates through the CLI; generated RH-only audit
  improves to 7 pass / 1 compile error.
- [x] 2026-06-23: Registered included-library parameters, code systems, value
  sets, codes, and concepts in addition to expressions/functions, and made
  qualified references emit the correct ELM ref kind with the include alias.
  Representative CQFramework files `ReferencingLibrary.cql`, `Issue641.cql`,
  and `AccessModifierReferencing.cql` now compile; an RH-only jvmTest audit
  reports 0 remaining rows with `Could not resolve qualified identifier`.
- [x] 2026-06-23: Added package-provider namespace fallback for unqualified
  standard FHIR support library includes. `FHIRHelpers`, `FHIRCommon`,
  `FHIRModelInfo`, and `FHIR-ModelInfo` now resolve from the corresponding
  `fhir.cqf.common` FHIR package when included without a CQL namespace.
- [x] 2026-06-23: Improved modelinfo-backed property resolution for FHIR fields.
  Member analysis now resolves class elements through modelinfo type specifiers,
  inherited base classes, list-valued properties, and choice-valued properties.
  The fallback FHIR R4 model now includes common complex datatypes and fields
  such as `Patient.name`, `DomainResource.text`, `Observation.value`, and
  `MedicationRequest.dosageInstruction`.

Observed patterns:

- Qualified identifiers such as `Helper.PublicValue` and measure library aliases.
- Included libraries not available to the compiler during corpus audit.
- FHIR/QI-Core property paths such as `.period`, `.value`, and choice-type fields.
- Query aliases not always available in nested contexts.

Initial targets:

1. [x] Teach corpus audit to provide include search paths for files in the same corpus directory.
2. [x] Improve compile API/library provider use for multi-library corpora.
3. [x] Resolve qualified identifiers across included libraries consistently.
4. [x] Improve modelinfo-backed property resolution for FHIR/QI-Core fields.
5. [x] Add choice-type property handling for common eCQM patterns.

Acceptance criteria:

- `MainLibrary.cql` in generated fixtures resolves included `HelperLibrary.cql`.
- FHIRHelpers/QICoreCommon compile results improve for Java-passing CMS files.
- Qualified identifier failures fall in corpus summary.

## Workstream 4: HL7 Runtime Evaluation Burn-Down

Goal: reduce the 606 HL7 unimplemented outcomes without introducing wrong answers.

Status:

- [x] 2026-06-23: Started the date/time runtime slice. Added engine
  dispatch for `Date`, `DateTime`, `Time`, `Today`, `Now`, and `TimeOfDay`
  FunctionRef calls; preserved DateTime component precision and timing/binary
  temporal precision in emitted ELM; validated date/time constructor and
  arithmetic bounds; and normalized day-precision DateTime interval edge cases
  exposed by the constructor support. `just audit-strict` now gates at
  1 109 pass / 0 wrong-answer fail / 56 skip / 94 compile errors /
  144 eval errors / 16 invalid failures, for 254 unimplemented outcomes.
- [x] 2026-06-23: Continued the interval runtime slice. Added interval-aware
  `before`/`after` evaluation for interval-vs-interval and interval-vs-point
  forms, routed scalar proper inclusion through interval point containment,
  and made interval proper containment/inclusion honor temporal precision.
  The HL7 interval suite now reports 308 pass / 0 wrong-answer fail /
  35 compile errors / 2 eval errors / 2 invalid failures. `just audit-strict`
  now gates at 1 166 pass / 0 wrong-answer fail / 56 skip / 94 compile errors /
  87 eval errors / 16 invalid failures, for 197 unimplemented outcomes.
- [x] 2026-06-23: Completed the list runtime slice. Added FunctionRef dispatch
  for `First`, `Last`, list/string `Length`, and list `Except`; added list
  length semantics that count null elements; and routed scalar/list
  `includes` and `included in` through list membership. The HL7 list suite now
  reports 197 pass / 0 wrong-answer fail / 0 compile errors / 0 eval errors /
  1 invalid failure. `just audit-strict` now gates at 1 193 pass /
  0 wrong-answer fail / 58 skip / 94 compile errors / 58 eval errors /
  16 invalid failures, for 168 unimplemented outcomes.
- [x] 2026-06-23: Completed the string runtime slice. Added FunctionRef
  dispatch for `Concatenate`, `Upper`, `Lower`, `StartsWith`, `EndsWith`,
  `Matches`, and string/list `Indexer`, sharing the direct ELM operator
  implementations. The HL7 string suite now reports 81 pass /
  0 wrong-answer fail / 0 compile errors / 0 eval errors. `just audit-strict`
  now gates at 1 230 pass / 0 wrong-answer fail / 58 skip /
  94 compile errors / 21 eval errors / 16 invalid failures, for
  131 unimplemented outcomes.
- [x] 2026-06-23: Completed the comparison/equivalence slice. Added
  model-local UCUM normalization for compatible quantity comparisons and made
  quantity equality/equivalence share the same normalization, then removed the
  HL7 cross-unit comparison skip. The HL7 comparison suite now reports
  183 pass / 0 wrong-answer fail / 0 skip / 0 compile errors / 0 eval errors.
  `just audit-strict` now gates at 1 241 pass / 0 wrong-answer fail /
  47 skip / 94 compile errors / 21 eval errors / 16 invalid failures, for
  131 unimplemented outcomes.
- [x] 2026-06-23: Completed the invalid-enforcement slice. Added parser
  enforcement for CQL Integer bounds, runtime enforcement for CQL Decimal
  scale/range, runtime validation for time literal component bounds and
  invalid interval bounds, and explicit errors for malformed `ToInteger`,
  `ToDateTime`, and `ToTime` conversions. The HL7 audit now reports 0 invalid
  failures. `just audit-strict` now gates at 1 241 pass / 0 wrong-answer fail
  / 45 skip / 89 compile errors / 23 eval errors / 0 invalid failures, for
  112 unimplemented outcomes.
- [x] 2026-06-23: Refactored the Workstream 4 invalid-enforcement helpers by
  centralizing CQL Decimal bounds and Time component bounds in the operator
  module, then reused the same time component helpers from conversion parsing.

Priority order:

1. Date/time evaluation and precision semantics.
2. Interval operators and boundary normalization.
3. List operators and null-list behavior.
4. String operators.
5. Comparison/equivalence edge cases.
6. Invalid literal and invalid cast enforcement.

Acceptance criteria:

- Lower `RH_CQL_HL7_MAX_*` thresholds after each completed slice.
- Preserve `fail=0` wrong-answer behavior in `just audit-strict`.
- Add focused unit tests for every new operator semantic.

## Workstream 5: JavaScript ELM Interoperability

Goal: reduce JavaScript `cql-execution` failures where RH-generated ELM is
valid CQL but not accepted or not interpreted equivalently by the JS runtime.

Initial targets:

1. Separate JS failures into unsupported expected-output, RH compile failure,
   JS runtime error, and value mismatch classes.
2. For JavaScript value mismatches, diff RH ELM against Java ELM for the same expression.
3. Prioritize ELM shape issues where Java ELM passes JavaScript and RH ELM does not.

Acceptance criteria:

- JS failure categories are visible in `implementation_matrix_summary.json`.
- At least one representative date/time or interval JS mismatch has a reduced fixture.

## Workstream 6: Source Validity Review

Goal: avoid enshrining invalid external CQL as requirements.

Tasks:

1. For each Java-non-pass external file, record the Java diagnostic and mark it quarantined.
2. For Java-pass but suspicious syntax, check the CQL grammar/spec section before implementation.
3. Keep a small `conformance/corpus/invalid-or-ambiguous.md` register.
4. Prefer reduced fixtures over large external files in unit tests.

Acceptance criteria:

- Java-non-pass rows are not counted as RH failures.
- Ambiguous external examples have documented disposition.

## Suggested Sequence

1. Improve corpus reporting and classification first.
2. Fix parser gaps with highest Java-pass/RH-fail counts.
3. Add include/library resolution for generated and CMS corpora.
4. Burn down HL7 date/time and interval eval errors.
5. Re-run `just audit-full`, `just corpus-audit-rh`, and `just audit-summary`.
6. Update thresholds and this plan with the new baseline.

## Commands

```bash
cd crates/rh-cql
just audit-full
just corpus-audit-rh
just audit-summary
```

Use the Java-inclusive corpus pass selectively because it is slower:

```bash
cd crates/rh-cql
just corpus-audit
```
