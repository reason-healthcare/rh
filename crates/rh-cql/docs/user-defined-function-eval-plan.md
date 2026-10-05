# User-Defined Function Evaluation in rh-cql

Status reviewed 2026-10-02 against `10313d4e`. The function-dispatch changes
below are implemented; complete CMS122 evaluation remains blocked. See
[Verification](#verification) for the difference between the six integration
assertions and the full patient/population audit.

## Goal

Enable the rh-cql ELM evaluator to execute user-defined function bodies (both
same-library and cross-library), as a prerequisite for correctly evaluating the
historical CMS122 FHIR 4.0.1 example with `rh cql eval`.

## Background

The CMS122 measure (`DiabetesHemoglobinA1cHbA1cPoorControl9FHIR`) depends on
helper functions from included libraries:

- `Global."Prevalence Period"(Diabetes)` — computes onset-to-abatement interval
- `Global."Normalize Interval"(effective)` — normalizes FHIR choice types to
  `Interval<DateTime>`
- `Global."Normalize Abatement"(condition)` — computes abatement interval
- `AdultOutpatientEncounters."Qualifying Encounters"` — union of encounter
  retrieves with date/status filters (this is an ExpressionRef, not a
  FunctionRef, but it internally may call functions)

The evaluator previously had a `TODO: support evaluating user-defined functions
from included libraries` in the `FunctionRef` handler. Same-library user
functions were also not evaluated by body.

## Implemented function-support changes

### 1. Function index in Engine (`engine.rs`)

Added `func_index: HashMap<String, &'lib FunctionDef>` to the `Engine` struct,
populated during `new_with_libraries` by scanning `StatementDef::Function`
entries in `library.statements`.

### 2. `find_function` method (`engine.rs`)

```rust
fn find_function(&self, name: &str) -> Option<&'lib FunctionDef> {
    self.func_index.get(name).copied()
}
```

### 3. `eval_user_function` method (`engine.rs`)

Binds operand names to argument values, pushes a scope, evaluates the body,
pops the scope:

```rust
fn eval_user_function(&mut self, fd: &FunctionDef, args: Vec<Value>) -> Result<Value, EvalError> {
    let body = fd.expression.as_ref().ok_or(...)?;
    let mut scope = BTreeMap::new();
    for (i, operand) in fd.operand.iter().enumerate() {
        let name = operand.name.clone().unwrap_or_else(|| format!("arg{i}"));
        scope.insert(name, args.get(i).cloned().unwrap_or(Value::Null));
    }
    self.push_scope(scope);
    let result = self.eval_expr(body);
    self.pop_scope();
    result
}
```

### 4. `ParameterRef` scope lookup (`engine.rs`)

Function bodies reference their operands via `ParameterRef`. The
`ParameterRef` handler previously only checked `ctx.parameters` and library
parameter declarations. Added scope stack lookup so function operand bindings
are found:

```rust
Expression::ParameterRef(r) => {
    let name = r.name.as_deref().unwrap_or("");
    if let Some(v) = self.lookup_binding(name) {
        return Ok(v.clone());
    }
    // ... existing ctx.parameters / is_library_parameter checks ...
}
```

### 5. `FunctionRef` handler update (`engine.rs`)

Updated the `FunctionRef` handler to try user-defined functions (both
same-library and cross-library) after builtin dispatch fails:

- **Cross-library**: Find the `FunctionDef` in the included library, create a
  sub-engine, and call `eval_user_function`.
- **Same-library**: Look up in `func_index` and call `eval_user_function`.

### 6. FHIR CodeableConcept ~ Code equivalent (`value.rs`)

The CMS122 measure uses `clinicalStatus ~ Global."active"` where
`clinicalStatus` is a FHIR CodeableConcept (a `Value::Tuple` with a `coding`
array) and `Global."active"` is a `Value::Code`. The `cql_equivalent` function
previously only handled `Code == Code` and fell through to `a == b` for
Tuple vs Code (always false).

Added explicit handling in `cql_equivalent` for `Tuple ~ Code` and
`Code ~ Tuple` that extracts the `coding` array from the Tuple and checks each
coding's `system` + `code` against the Code.

### 7. FHIR CodeableConcept in retrieve filter (`engine.rs`)

Added FHIR CodeableConcept Tuple handling to `filter_resources_by_code` so
that retrieve `[Condition: "Diabetes"]` correctly filters FHIR resources
by checking the `code.coding` array against value set members.

### 8. `--valuesets` flag on `rh cql eval` (`cql.rs` in rh-cli)

Added a `--valuesets <FILE>` CLI flag that loads value-set expansions from a
JSON file into an `InMemoryTerminologyProvider` and attaches it to the
`EvalContextBuilder`. The JSON format is:
```json
{"<valueset-url>": {"codes": [{"system": "...", "code": "..."}, ...]}}
```

### 9. Cross-library CodeRef resolution (`engine.rs`)

Added cross-library `CodeRef` resolution so `Global."active"` resolves from
included libraries (the code definition and its code system are looked up in
the included library).

## Testing

### Unit tests
The original implementation/refactor validation passed 859 `rh-cql` library
tests. This historical count is not evidence of full CMS122 population coverage.

### Integration test: CMS122 patient-numer
The patient-numer bundle has:
- Patient born 1965-06-30 (age 53 at MP start — in 18-75 range)
- Condition with clinicalStatus "active", code E10.10 (diabetes), onsetPeriod 2009
- Encounter with type code 99202 (Office Visit), period 2019-01-16 to 2019-01-20
- Two HbA1c Observations: 7.1% (2019-01-17) and 9.1% (2019-10-17)

Intended fixture results (not all asserted by the integration suite):
- Initial Population: true
- Denominator: true
- Numerator: true (elevated HbA1c 9.1% OR no HbA1c)
- Has Most Recent Elevated HbA1c: true (9.1% > 9%)
- Has No Record Of HbA1c: false (patient has HbA1c observations)

### Verification

At the reviewed revision, `tests/cms122_integration_test.rs` has six assertions:
five Initial Population checks and one Denominator check. It does not assert
Numerator or Denominator Exclusions, read the complete `expected-results.json`,
or fail when the sibling fixture is absent. A passing suite does not establish
complete measure evaluation.

The 2026-10-02 audit in the sibling `reasonhealth-analytics` repository evaluates
all four raw population expressions for all seven patients using isolated
patient input and an explicitly bound historical measurement period. With `rh`
built at `10313d4e`, it records 18 boolean results and 10 errors:

- All seven Initial Population and seven raw Denominator results match the
  intended membership.
- Three Numerator calls fail with interval errors.
- All seven Denominator Exclusions calls fail: six on missing terminology and
  the hospice case on an interval error.

The measurement-period boundary probe passes for the audit's millisecond/UTC
fixture scope. General CLI open/closed interval binding, unbound ELM parameter
defaults, FHIR choice/Period normalization, and terminology completeness remain
follow-up work. The local terminology file is a partial synthetic code list,
not a complete versioned expansion. Errors must remain distinct from false.

Reproduce from a sibling analytics checkout with `RH_BIN=/path/to/rh just
audit-cms122`. Its `docs/cms122-measure-validation-plan.md` owns the ordered MVP
gates; `examples/cms122-diabetes-hba1c/evaluate.py` records the raw evidence.

## Files Changed

All changes are in the `rh` repository:

- `crates/rh-cql/src/eval/engine.rs` — function index, find_function,
  eval_user_function, ParameterRef scope lookup, FunctionRef handler,
  CodeRef cross-library, FHIR CodeableConcept in filter
- `crates/rh-cql/src/eval/value.rs` — FHIR CodeableConcept ~ Code equivalent
- `apps/rh-cli/src/cql.rs` — `--valuesets` flag on `rh cql eval`
- `crates/rh-cql/tests/parser_cms122_gaps.rs` — parser gap tests (from Phase 0)

## Follow-up

1. Close the native audit errors with generic evaluator/CLI fixes and reviewed
   terminology; assert all 28 raw results and boundary cases. Establish an
   independent semantic reference before treating native output as the oracle.
2. Preserve included libraries, typed expressions, parameters, and terminology
   through relational lowering. Generate all clinical population predicates
   and result mappings, and reject required unresolved semantics.
3. Compare generated SQL membership with independently reviewed expected
   results and the complete native evaluation. Keep the raw denominator,
   exclusions, effective denominator, and eligible numerator distinct.

The [relational architecture](../ARCHITECTURE.md#experimental-relational-algebra)
documents why `lower-check` support labels and artifact emission alone do not
complete these gates.
