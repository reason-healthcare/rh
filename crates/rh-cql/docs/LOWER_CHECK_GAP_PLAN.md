# Plan: close the `cql lower-check` gap for the HypertensionManagement library

Last updated: 2026-07-28

This plan was saved as part of a working session to address a `cql lower-check`
run on a realistic hypertension decision-support CQL library. The run reported
`supported: false` with 14 unsupported ELM node kinds. Investigation showed the
report reflects the **first-pass relational lowerer allow-list**, not missing CQL
language support — every flagged kind already has a parser→ELM→eval path.

The fixture lives at
`conformance/corpus/generated/lowerer/HypertensionManagement.cql`; the
companion regression tests live in `crates/rh-cql/src/analytics.rs`.

## Background

`lower_check` (`crates/rh-cql/src/analytics.rs`) walks every ELM node in the
compiled library and asks `is_supported_for_first_pass(node_type)` whether that
ELM kind is in the first-pass lowerer's allow-list. Unsupported kinds land in
`unsupportedNodes`.

Observed unsupported kinds for the fixture (14):

```
Add(3) As(3) ByColumn(1) Case(2) Coalesce(1) FunctionRef(17) If(6)
In(1) Instance(4) IsNull(3) List(1) NamedTypeSpecifier(3) SingletonFrom(2) Tuple(2)
```

Key findings:

- All 14 kinds already parse, emit ELM, and evaluate. They are "unsupported" only
  in the first-pass lowerer allow-list / `plan_expression`.
- `FunctionRef` (17) is the largest bucket. Most are system functions the
  emitter routes to a generic `FunctionRef` instead of a canonical ELM node:
  `Today`, `AgeInYearsAt`, `First`, and FHIRHelpers-qualified
  `ToDateTime` / `ToQuantity` / `ToConcept`. The ELM enum and evaluator already
  have dedicated nodes/arms for these; `emit/` just never routes to them. Three
  (`Observation Date`, `Systolic Value`, `Diastolic Value`) are legitimate
  user-defined functions and genuinely cannot be lowered relationally.
- `NamedTypeSpecifier` (3) and `ByColumn` (1) are false positives:
  `collect_node_counts` recurses into type-specifier fields and sort-clause
  `by` arrays and counts their `type` tags as nodes.
- `Equivalent` (6, `~`) and `IncludedIn` (1, `during`) are already supported.

## Steps

1. **Fixture + snapshot test** — add the HypertensionManagement CQL under
   `conformance/corpus/generated/lowerer/` and a `lower_check` snapshot test in
   `analytics.rs` asserting today's 14 unsupported kinds.

2. **Stop false-positive counting** — `collect_node_counts` should skip
   `*TypeSpecifier` nodes under `asTypeSpecifier` / `signature` / `elementType`
   / `type` fields, and treat `ByColumn` / `ByExpression` / `ByDirection`
   sort-discriminator leaves as opaque sort metadata. Removes the 3
   `NamedTypeSpecifier` and 1 `ByColumn` false positives.

3. **Extend the lowerer allow-list** —
   `is_supported_for_first_pass` and `plan_expression` should recognize the
   already-fully-implemented structural/value kinds: `Add`, `If`, `Case`,
   `Coalesce`, `IsNull`, `List`, `Tuple`, `Instance`, `SingletonFrom`, `First`,
   `In`, `As`, plus sort metadata.

4. **Canonical ELM emit routes** — in
   `emit/operators.rs::emit_system_function`, route `Today` →
   `Expression::Today`, `First` → `Expression::First`, and `AgeInYearsAt` /
   `AgeInYears` → `Expression::CalculateAgeAt` / `CalculateAge`. Shrinks
   `FunctionRef` to only legitimate user-defined functions and FHIRHelpers
   conversions.

5. **Fallback classification** — add a "supported via runtime fallback"
   classification to `LowerCheckReport` so user-defined / FHIRHelpers
   `FunctionRef`s are reported as fallback, not lumped with genuinely unsupported
   nodes. Document the policy.

6. **Docs** — update `rh-cql/ARCHITECTURE.md` (recognised-kinds list),
   `rh-cql/README.md` (first-pass bridge section), and `SPEC_COVERAGE.md` if
   applicable; run `just docs-sync`.

7. **Verify** — re-run the lower-check against the fixture; `supported` should
   flip to true (or true-with-fallback via the new classification). Run
   `cargo test -p rh-cql --lib analytics::tests` plus the fixture test, then
   `just check`.
