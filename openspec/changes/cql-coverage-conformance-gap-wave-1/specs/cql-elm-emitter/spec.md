## ADDED Requirements

### Requirement: Wave-1 emitter parity for system functions
For the wave-1 function set (`Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power`, `Predecessor`, `Successor`), emission SHALL produce dedicated ELM expression types and SHALL NOT emit `FunctionRef` fallbacks.

#### Scenario: Unary system function emits native node
- **WHEN** a typed expression containing `Abs(-5)` is emitted
- **THEN** the resulting ELM node SHALL be `Abs` and not `FunctionRef("Abs")`

#### Scenario: Binary system function emits native node
- **WHEN** a typed expression containing `Log(100, 10)` or `2 ^ 3` is emitted
- **THEN** the resulting ELM node SHALL be `Log` or `Power` respectively and not `FunctionRef`

### Requirement: Wave-1 canonical negative literal emission
Negative numeric literals SHALL emit as `Negate(Literal(<positive-value>))` in all emitter paths used by wave-1 conformance tests.

#### Scenario: Negative integer literal shape
- **WHEN** `-5` is emitted
- **THEN** ELM SHALL contain a `Negate` node wrapping `Literal("5")`

#### Scenario: Negative decimal literal shape
- **WHEN** `-3.14` is emitted
- **THEN** ELM SHALL contain a `Negate` node wrapping `Literal("3.14")`

### Requirement: Emitter conformance regression checks for wave-1 set
The project SHALL include emitter regression tests that fail if wave-1 system functions or negative literal shape regress to prior non-conformant emission forms.

#### Scenario: System function regression is detected
- **WHEN** a future change emits `Abs` as `FunctionRef`
- **THEN** emitter conformance tests SHALL fail with a wave-1 parity expectation failure

#### Scenario: Negative literal regression is detected
- **WHEN** a future change emits `-5` as a single negative literal
- **THEN** emitter conformance tests SHALL fail with a canonical literal-shape expectation failure
