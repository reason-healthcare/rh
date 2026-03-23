## 1. Scope And Baseline

- [x] 1.1 Capture and record the wave-2 starting baseline from `CONFORMANCE.md` (2026-03-09 post-wave-1 metrics: `Fail=0`, `Compile err=149`, `Eval err=572`)
- [x] 1.2 Build a wave-2 closure matrix mapping each in-scope operator/function to required stages (parse, semantic, emit, eval, tests)
- [x] 1.3 Confirm and record deferred out-of-scope items (`AgeIn*`, `Children`, `Descendents`) in working notes and docs plan

## 2. Parser And Semantic Registration

- [x] 2.1 Add or verify parser support for interval `size of` syntax (if still a parse-stage gap) and add parser regression tests
- [x] 2.2 Register semantic signatures/overloads for wave-2 functions (`TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`, `Product`, `GeometricMean`, `Repeat`, `Size`)
- [x] 2.3 Add semantic tests validating deterministic overload selection and conversion-policy behavior for wave-2 operators

## 3. Emitter And Eval Implementation

- [x] 3.1 Ensure emitter maps wave-2 temporal/uncertainty functions to canonical executable ELM forms (`TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`)
- [x] 3.2 Add eval dispatch for nullological operators (`IsNull`, `IsTrue`, `IsFalse`, `Coalesce`) and cover null-semantics edge cases
- [x] 3.3 Implement eval support for `Product` and `GeometricMean` and validate numeric-list behavior
- [x] 3.4 Replace `Repeat` stub with deterministic fixpoint evaluation and add termination/regression tests
- [x] 3.5 Implement interval `Size` evaluation semantics and add bounded/unbounded interval tests
- [x] 3.6 Add eval coverage for `TimeOfDay`, `Precision`, `LowBoundary`, and `HighBoundary` with deterministic clock/temporal test cases

## 4. Conformance Verification

- [x] 4.1 Run targeted `rh-cql` unit/integration tests covering parser, semantic, emitter, and eval wave-2 changes
- [x] 4.2 Run HL7 conformance evaluation suite and capture post-wave-2 per-suite totals
- [x] 4.3 Verify quality gates: wrong-answer `Fail` remains `0` and compile/eval error totals improve from the recorded baseline

## 5. Coverage And Conformance Documentation

- [x] 5.1 Update `crates/rh-cql/SPEC_COVERAGE.md` rows, notes, and status summaries for all closed wave-2 gaps
- [x] 5.2 Update `crates/rh-cql/CONFORMANCE.md` with dated before/after wave-2 metrics, command evidence, and category impact notes
- [x] 5.3 Ensure every closed wave-2 gap has implementation/test evidence references and deferred items remain explicitly listed
