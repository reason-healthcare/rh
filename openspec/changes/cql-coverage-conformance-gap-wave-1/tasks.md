## 1. Scope And Baseline

- [x] 1.1 Capture the 2026-03-09 conformance baseline in working notes (Fail, compile-error, eval-error totals) for comparison
- [x] 1.2 Define the wave-1 operator/function closure set from SPEC_COVERAGE and CONFORMANCE (including deferred items list)
- [x] 1.3 Map each wave-1 item to the owning pipeline stage(s): semantic registration, emitter mapping, eval dispatch/implementation

## 2. Emitter Conformance Parity

- [x] 2.1 Ensure wave-1 system functions (`Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power`, `Predecessor`, `Successor`) emit as dedicated ELM nodes, not `FunctionRef`
- [x] 2.2 Ensure negative numeric literals emit as `Negate(Literal(<positive-value>))` across relevant emitter paths
- [x] 2.3 Add or update emitter conformance tests to fail on regressions for system-function emission and negative literal shape

## 3. Semantic Registration And Eval Dispatch

- [x] 3.1 Register wave-1 function-syntax operators in semantic/operator resolution to eliminate unknown-function compile failures
- [x] 3.2 Wire eval dispatch for wave-1 string/date extraction operators (`Substring`, `PositionOf`, `LastPositionOf`, `SplitOnMatches`, `ReplaceMatches`, string `IndexOf`, `DateFrom`, `TimeFrom`, `TimezoneOffsetFrom`)
- [x] 3.3 Implement and wire list slice operators (`Tail`, `Skip`, `Take`, `Slice`) with deterministic null and bounds semantics
- [x] 3.4 Add focused eval tests for all newly wired operators, including null-propagation and edge-case coverage

## 4. Conformance Verification

- [x] 4.1 Run targeted `rh-cql` test binaries covering emitter/eval changes and fix regressions
- [x] 4.2 Run HL7 conformance evaluation suite and record post-wave metrics
- [x] 4.3 Verify wrong-answer `Fail` remains `0` and compile/eval error outcomes improve from the documented baseline

## 5. Coverage And Conformance Documentation

- [x] 5.1 Update `crates/rh-cql/SPEC_COVERAGE.md` rows/statuses/notes for all wave-1 changes
- [x] 5.2 Update `crates/rh-cql/CONFORMANCE.md` with dated post-wave metrics, command evidence, and before/after deltas
- [x] 5.3 Ensure each closed wave-1 gap references implementation and test evidence; keep deferred items explicitly listed
