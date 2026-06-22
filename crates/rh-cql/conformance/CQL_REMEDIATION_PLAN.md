# CQL Remediation Plan

Last updated: 2026-06-22

This plan is based on `conformance/results/summaries/latest-summary.md`.
It addresses current `rh-cql` gaps while avoiding the assumption that every
external corpus file is specification-correct.

## Current Signal

### HL7 Expression Matrix

The HL7 expression matrix remains the strongest specification-facing signal:

| Metric | Count |
|---|---:|
| Parsed HL7 cases | 1 426 |
| `rh-cql` pass | 772 |
| Wrong-answer fail | 0 |
| Skip | 48 |
| Compile error | 123 |
| Eval error | 467 |
| Invalid input unexpectedly accepted | 16 |
| Unimplemented total | 606 |

Failure concentration:

| Category | Main suites |
|---|---|
| Compile errors | intervals 63, date/time 34, value literals/selectors 21 |
| Eval errors | date/time 211, intervals 103, lists 56, strings 44, comparisons 33 |
| Invalid input enforcement | value literals/selectors 7, type operators 4, types 3, intervals 2 |

### Three-Engine Matrix

| Implementation | Pass | Compile Err | Eval Err | Fail | Skip | Unimplemented |
|---|---:|---:|---:|---:|---:|---:|
| `rh-cql` | 772 | 123 | 467 | 0 | 48 | 16 |
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
| Generated fixtures | 8 | 4 | 4 | 8 | 0 |
| CQFramework jvmTest | 358 | 182 | 176 | 358 | 0 |
| CQFramework examples | 34 | 1 | 33 | 34 | 0 |
| Cooking with CQL | 732 | 132 | 600 | 725 | 7 |
| CMS 2025 eCQM | 116 | 15 | 101 | 115 | 1 timeout |
| **Total** | **1 248** | **334** | **914** | **1 240** | **8** |

Java-pass/RH-fail rows are high-value remediation candidates:

| Corpus | Java-pass/RH-fail |
|---|---:|
| Cooking with CQL | 593 |
| CQFramework jvmTest | 176 |
| CMS 2025 eCQM | 100 |
| CQFramework examples | 33 |
| Generated fixtures | 4 |

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
- [x] 2026-06-22: Full Java-inclusive corpus rerun completed. Current
  Java-pass/RH-fail total is 906: parser 673, semantic 233. Java non-pass
  quarantine total is 8: unknown 7, timeout 1.
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

Observed patterns:

- Missing support for some real-world define forms and access modifiers.
- Retrieve/query syntax used in measure content.
- Date/time and interval phrase variants.
- Terminology declaration variants, including code display/codesystem forms.
- Library-level declaration ordering found in examples.

Initial targets:

1. [x] Support `define public` / `define private` forms in addition to current access parsing.
2. Support omitted-colon define bodies where accepted by the Java translator, or explicitly classify source-invalid if not spec-valid.
3. Expand retrieve parsing for measure-style `[Type: "Value Set"] Alias where ...`.
4. Expand code, codesystem, valueset, and display parsing to match Java-passing corpus forms.
5. Add date/time phrase parser coverage for `same or before`, `same or after`, `before`, `after`, and interval phrase variants.

Acceptance criteria:

- Each parser fix includes a minimized Java-passing fixture.
- `just corpus-audit-rh` shows a measurable drop in Java-pass/RH parse errors.
- No regression in `just audit-strict`.

## Workstream 3: Semantic And Library Resolution

Goal: handle Java-passing files that parse but fail semantic analysis.

Observed patterns:

- Qualified identifiers such as `Helper.PublicValue` and measure library aliases.
- Included libraries not available to the compiler during corpus audit.
- FHIR/QI-Core property paths such as `.period`, `.value`, and choice-type fields.
- Query aliases not always available in nested contexts.

Initial targets:

1. Teach corpus audit to provide include search paths for files in the same corpus directory.
2. Improve compile API/library provider use for multi-library corpora.
3. Resolve qualified identifiers across included libraries consistently.
4. Improve modelinfo-backed property resolution for FHIR/QI-Core fields.
5. Add choice-type property handling for common eCQM patterns.

Acceptance criteria:

- `MainLibrary.cql` in generated fixtures resolves included `HelperLibrary.cql`.
- FHIRHelpers/QICoreCommon compile results improve for Java-passing CMS files.
- Qualified identifier failures fall in corpus summary.

## Workstream 4: HL7 Runtime Evaluation Burn-Down

Goal: reduce the 606 HL7 unimplemented outcomes without introducing wrong answers.

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
