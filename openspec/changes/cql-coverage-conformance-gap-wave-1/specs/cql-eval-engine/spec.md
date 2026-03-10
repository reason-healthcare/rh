## ADDED Requirements

### Requirement: Wave-1 operator dispatch closure
The eval engine SHALL include explicit dispatch coverage for the wave-1 gap set so expressions no longer fail with unsupported-operation errors solely due to missing dispatch wiring.

#### Scenario: String extraction and search operators dispatch
- **WHEN** `Substring`, `PositionOf`, `LastPositionOf`, `SplitOnMatches`, `ReplaceMatches`, and string `IndexOf` expressions are evaluated
- **THEN** evaluation SHALL execute operator-specific logic rather than returning an unsupported-operation error

#### Scenario: Date/time extraction operators dispatch
- **WHEN** `DateFrom`, `TimeFrom`, and `TimezoneOffsetFrom` expressions are evaluated
- **THEN** evaluation SHALL dispatch to implemented handlers and return typed results or spec-defined null outcomes

#### Scenario: List slice operators dispatch
- **WHEN** `Tail`, `Skip`, `Take`, or `Slice` expressions are evaluated
- **THEN** evaluation SHALL dispatch and return deterministic list outputs or spec-defined null outcomes

### Requirement: Wave-1 semantic registration for function-syntax operators
Function-syntax operators included in the wave-1 gap set SHALL be registered in semantic/operator resolution so they compile through the standard pipeline.

#### Scenario: Function compiles through semantic resolution
- **WHEN** a CQL expression uses a wave-1 function-syntax operator (for example `Substring('abc', 1, 2)`)
- **THEN** semantic analysis SHALL resolve it without an unknown-function compile failure

#### Scenario: Overload selection remains deterministic
- **WHEN** multiple overload candidates exist for a wave-1 function
- **THEN** semantic resolution SHALL select the same overload deterministically for equivalent input types

### Requirement: Wave-1 conformance metrics improve from baseline
For the 2026-03-09 HL7 full-suite baseline, wave-1 implementation SHALL reduce unimplemented-behavior outcomes while preserving zero wrong-answer failures.

#### Scenario: Compile/eval outcomes improve
- **WHEN** the HL7 conformance suite is rerun after wave-1
- **THEN** compile-error and/or eval-error totals SHALL be lower than baseline values recorded in `CONFORMANCE.md`

#### Scenario: Correctness invariant preserved
- **WHEN** post-wave conformance results are reported
- **THEN** wrong-answer `Fail` count SHALL remain `0`
