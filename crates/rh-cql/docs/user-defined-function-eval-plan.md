# User-Defined Function Evaluation in rh-cql

## Goal

Enable the rh-cql ELM evaluator to execute user-defined function bodies (both
same-library and cross-library), so that the published CMS122 FHIR 4.0.1
measure compiles AND evaluates correctly with `rh cql eval`.

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

## Changes Made (complete)

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
All 859 existing `rh-cql` lib tests pass after each change.

### Integration test: CMS122 patient-numer
The patient-numer bundle has:
- Patient born 1965-06-30 (age 53 at MP start — in 18-75 range)
- Condition with clinicalStatus "active", code E10.10 (diabetes), onsetPeriod 2009
- Encounter with type code 99202 (Office Visit), period 2019-01-16 to 2019-01-20
- Two HbA1c Observations: 7.1% (2019-01-17) and 9.1% (2019-10-17)

Expected results:
- Initial Population: true
- Denominator: true
- Numerator: true (elevated HbA1c 9.1% OR no HbA1c)
- Has Most Recent Elevated HbA1c: true (9.1% > 9%)
- Has No Record Of HbA1c: false (patient has HbA1c observations)

### Verification
All evaluator issues described in earlier drafts are resolved. The current
CMS122 integration suite passes for the relevant population paths, including
patient-numer, patient-denom, patient-no-encounter, patient-no-diabetes,
patient-no-hba1c, and patient-too-young fixtures.

## Files Changed

All changes are in the `rh` repository:

- `crates/rh-cql/src/eval/engine.rs` — function index, find_function,
  eval_user_function, ParameterRef scope lookup, FunctionRef handler,
  CodeRef cross-library, FHIR CodeableConcept in filter
- `crates/rh-cql/src/eval/value.rs` — FHIR CodeableConcept ~ Code equivalent
- `apps/rh-cli/src/cql.rs` — `--valuesets` flag on `rh cql eval`
- `crates/rh-cql/tests/parser_cms122_gaps.rs` — parser gap tests (from Phase 0)

## Follow-up
The native evaluator work is complete. Follow-up work is now limited to the
SQL lowering gaps: generate SQL that preserves age, encounter, diabetes,
HbA1c, and exclusion predicates so `rh-analytics` can reproduce the expected
CMS122 population membership.
