# CQL Test Corpus Strategy

Last updated: 2026-06-19

This document defines the intended source mix for `rh-cql` CQL test content. It
is a strategy, not a required directory layout. Keep concrete fixtures organized
under `crates/rh-cql/` and prefer incremental additions that can be wired into
the audit matrix.

## Source Layers

| Layer | Purpose | Primary Source | Current Status |
|---|---|---|---|
| Spec conformance | Core language behavior across operators, types, nullology, intervals, date/time, lists, strings, literals, and errors | HL7 CQL tests | Checked in under `tests/fixtures/hl7_cql_tests/`; evaluated by `tests/hl7_eval_tests.rs` |
| Translation equivalence | Compare `rh-cql` ELM output against the reference CQL-to-ELM translator | `cqframework/clinical_quality_language` | Java setup pinned to `v4.2.0`; comparison harness exists under `conformance/scripts/` |
| Clinical realism | Stress realistic FHIR/QI-Core authoring patterns, includes, retrieves, terminology, and define chains | `cqframework/ecqm-content-cms-2025` | Not yet imported; recommended next corpus |
| Engine scenarios | Evaluate patient-level behavior against known data | Synthetic FHIR R4 Bundles plus targeted CQL | Local retrieve tests exist; broader bundle corpus not yet created |
| Generated edge cases | Find parser/type/eval gaps outside realistic authoring style | Generated CQL for operators, intervals, nulls, choice types, terminology, and errors | Recommended after the first realistic corpus import |

## Verified Upstream Notes

- HL7 CQL tests are informative, not normative, but they enumerate the same sections currently used by our XML fixtures: aggregate functions/operator, arithmetic, comparison, conditionals, date/time, errors/messaging, intervals, lists, logic, nullology, strings, type operators, types, and value literals/selectors.
- The CQL 1.5.3 reference-implementations appendix identifies the CQL-to-ELM translator as the reference implementation and lists the JavaScript CQL execution framework plus FHIR data-source provider.
- `cqframework/clinical_quality_language` contains CQL documentation, examples, compiler/translator tooling, and ELM runtime code. It is the correct source for Java translator parity and broader parser/compiler corpus mining.
- `cqframework/ecqm-content-cms-2025` states that its measure content uses QI-Core 6.0.0 based on FHIR R4 v4.0.1, making it useful for realistic R4/QI-Core stress tests.
- eCQI currently lists CQL v1.5.3 as the latest official published version and says it is based on FHIR R4.

## What To Test From Each Layer

### HL7 CQL Tests

Use the HL7 XML tests as the base language matrix for:

- Parser behavior.
- Type resolution and overload selection.
- Null/three-valued logic.
- Interval semantics.
- Date/time precision and timezone-sensitive behavior.
- List, string, aggregate, and nullological operators.
- ELM shape compatibility for operator lowering.

These tests should continue to feed `implementation_matrix.json` / `.csv`.

### CQFramework Translator Corpus

Use the Java translator as the translation oracle:

1. Compile CQL with `rh-cql`.
2. Compile the same CQL with the pinned CQFramework Java translator.
3. Normalize known metadata differences.
4. Diff ELM structure and record status/notes in generated reports.

Prioritize importing or referencing:

- `Src/java/cql-to-elm/src/jvmTest` CQL files.
- Operator tests.
- Cooking with CQL examples.
- Representative examples that include retrieves, terminology, includes, and model info.

### CMS / CQFramework FHIR eCQM Content

Use CMS 2025 FHIR eCQM content for realistic integration stress:

- FHIR retrieves.
- Terminology and value set references.
- Library includes and shared libraries.
- Measurement period parameters.
- Population criteria and define chains.
- QI-Core-on-R4 authoring patterns.
- Choice-type access and casts.
- Clinical date intervals.
- `exists`, `not exists`, `with`, `without`, joins, and sorting.

Treat these primarily as compile/ELM comparison tests first. Add evaluation only when the required FHIR fixtures, terminology stubs, and model behavior are explicit.

### Generated CQL Fixtures

Generate targeted CQL for cases realistic measures tend not to cover:

- Boolean `true`/`false`/`null` combinations.
- Integer, Long, Decimal, and Quantity boundary behavior.
- Equality/equivalence and cross-type comparisons.
- `is null`, `is not null`, `Coalesce`, and null propagation.
- `if/then/else` and `case`.
- List `contains`, `in`, `union`, `intersect`, `except`, `flatten`, `distinct`, sort, and null-list edge cases.
- Interval `includes`, `during`, `overlaps`, `starts`, `ends`, `meets`, and open/closed/null boundaries.
- DateTime precision, `same as`, `same or before`, `same or after`, offsets, and date-only conversions.
- UCUM quantity comparisons and conversions.
- Tuple construction and nested tuple access.
- Function overloads, optional arguments, forward references, and include boundaries.
- `Message()`, invalid casts, invalid literals, and bad overload resolution.

## FHIR R4 Retrieve Stress Themes

Create a small set of hand-authored CQL libraries plus FHIR R4 bundles that target:

- `Observation.effective[x]`: `dateTime` vs `Period`.
- `Condition.onset[x]`: `dateTime` vs `Period` vs `Age`.
- `MedicationRequest.medication[x]`: `CodeableConcept` vs `Reference`.
- `Procedure.performed[x]`: `dateTime` vs `Period`.
- `Encounter.period`.
- Choice-type casts and failed casts.
- `FHIRHelpers.ToConcept`, `ToDateTime`, and `ToInterval`.
- Code, value set, and date filters on retrieves.
- Missing fields and null propagation through FHIR property access.

Keep plain FHIR R4 model behavior separate from QI-Core-on-R4 authoring patterns, even when both use FHIR R4 4.0.1 underneath.

## Expected Artifacts Per Corpus Item

For each committed CQL corpus item, prefer storing or generating:

- Source CQL.
- Expected diagnostics.
- Expected ELM JSON or normalized ELM diff status.
- Optional FHIR Bundle fixtures.
- Expected evaluation result by expression.
- Implementation matrix row(s), including status and notes for `rh-cql`, Java ELM, and JavaScript evaluation.

## Near-Term Import Order

1. Keep HL7 XML tests as the base matrix.
2. Add CQFramework Java translator corpus slices to the ELM comparison harness.
3. Add CMS 2025 FHIR eCQM content as compile/ELM realistic corpus.
4. Add synthetic FHIR R4 bundles for patient-level evaluation.
5. Add generated edge-case libraries for gaps revealed by the first four layers.

## Source Links

- HL7 CQL tests: https://build.fhir.org/ig/HL7/cql/tests.html
- CQL reference implementations: https://cql.hl7.org/10-c-referenceimplementations.html
- CQFramework CQL tooling: https://github.com/cqframework/clinical_quality_language
- CMS 2025 FHIR eCQM content: https://github.com/cqframework/ecqm-content-cms-2025
- eCQI CQL versions: https://ecqi.healthit.gov/cql/versions
