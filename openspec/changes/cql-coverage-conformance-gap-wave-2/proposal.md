## Why

After wave-1, `rh-cql` still has substantial unimplemented CQL behavior in the documented baseline (`Fail=0`, `Compile err=149`, `Eval err=572` on 2026-03-09). The next highest-leverage step is a bounded wave-2 focused on non-clinical operators that remain incomplete in `SPEC_COVERAGE.md` and continue to drive conformance error volume in `CONFORMANCE.md`.

## What Changes

- Implement a wave-2 closure set for remaining non-clinical high/medium-priority gaps from `SPEC_COVERAGE.md`.
- Add end-to-end support for `TimeOfDay`, `Precision`, `LowBoundary`, and `HighBoundary` (semantic registration, emission mapping, eval dispatch/logic, tests).
- Complete medium-priority operator gaps: full `Repeat` behavior (fixpoint semantics), `Size`, `Product`, and `GeometricMean`.
- Close high-volume eval-dispatch gaps still called out in `CONFORMANCE.md` for nullological and aggregate function families (including `IsNull`, `IsTrue`, `IsFalse`, `Coalesce`, and aggregate function dispatch parity).
- Add targeted regression and HL7 conformance tests tied to each closed gap, and update `SPEC_COVERAGE.md` / `CONFORMANCE.md` with before/after wave-2 deltas.
- Keep clinical-context and FHIR-navigation features (`AgeIn*`, `Children`, `Descendents`) explicitly deferred for a later wave.

## Capabilities

### New Capabilities
- `cql-conformance-gap-wave-2`: Delivers the wave-2 operator gap-closure set selected from coverage and conformance documents, with auditable metric improvements and regression protection.

### Modified Capabilities
- `cql-eval-engine`: Extend required eval coverage and dispatch for wave-2 nullological, aggregate, date/time uncertainty, and list/interval operators.
- `cql-semantic-analysis`: Add required operator/function signatures and overload-resolution behavior for wave-2 functions and operators.
- `cql-elm-emitter`: Ensure wave-2 operators emit canonical/native ELM forms needed for parity and downstream eval dispatch.
- `cql-parser-conformance`: Add parser support for any wave-2 operators that remain parse-stage gaps (notably `Size` if still missing).
- `cql-spec-coverage`: Require post-wave-2 metric refresh and traceability from closed gaps to implementation and tests.

## Impact

- Affected crate: `crates/rh-cql`
- Primary modules: `src/parser/`, `src/operators.rs`, `src/semantic/`, `src/emit/`, `src/eval/`
- Affected docs: `crates/rh-cql/SPEC_COVERAGE.md`, `crates/rh-cql/CONFORMANCE.md`
- Affected tests: targeted unit/integration tests and `tests/hl7_eval_tests.rs`
- No external service or dependency changes required; scope is internal language/runtime closure and conformance documentation
