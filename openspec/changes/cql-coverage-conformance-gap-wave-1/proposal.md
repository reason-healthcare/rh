## Why

`rh-cql` currently reports 82% CQL 1.5.3 operator coverage (143/175) in `SPEC_COVERAGE.md` and, while preserving zero wrong-answer regressions, still has substantial unimplemented behavior in `CONFORMANCE.md` (149 compile errors, 628 eval errors as of 2026-03-09). The highest-leverage next step is a focused gap-closure wave that prioritizes operators already partially implemented but not fully wired across Parse/Semantic/Emit/Eval.

## What Changes

- Implement a bounded `rh-cql` conformance wave focused on high-impact operator gaps identified in `SPEC_COVERAGE.md` and `CONFORMANCE.md`.
- Close emitter parity gaps by ensuring system functions emit as native ELM expression nodes and negative numeric literals emit as `Negate(Literal(...))`.
- Add semantic registration and eval dispatch for currently disconnected operators/functions that already have partial support in code (for example string/date extraction functions).
- Implement missing high-priority operators that materially reduce current compile/eval error volume.
- Add targeted conformance and regression tests tied to the selected operator set.
- Update `SPEC_COVERAGE.md` and `CONFORMANCE.md` with post-change coverage/error metrics and explicit evidence links to tests.

## Capabilities

### New Capabilities
- `cql-conformance-gap-wave-1`: Delivers a scoped set of operator-level parser/semantic/emitter/eval closures that improve measured CQL conformance while preserving zero wrong-answer behavior.

### Modified Capabilities
- `cql-elm-emitter`: Tighten emitter conformance requirements and regression coverage for native system-function emission and negative literal shape.
- `cql-eval-engine`: Extend required operator coverage and dispatch behavior for selected high-priority string/list/date/time/nullological gaps.
- `cql-spec-coverage`: Add requirements for post-change metric refresh and auditable traceability from documented gaps to implemented tests.

## Impact

- Affected crate: `crates/rh-cql`
- Primary modules: `src/emit/`, `src/eval/`, `src/operators.rs`, `src/semantics/`, `src/parser/` (where needed)
- Affected docs: `crates/rh-cql/SPEC_COVERAGE.md`, `crates/rh-cql/CONFORMANCE.md`
- Affected tests: `crates/rh-cql/tests/hl7_eval_tests.rs`, `crates/rh-cql/tests/emit_conformance_tests.rs`, plus new/expanded focused eval tests
- No external dependency additions expected; scope is implementation and test coverage within existing architecture
