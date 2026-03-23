## ADDED Requirements

### Requirement: Wave-2 temporal and uncertainty functions emit canonical ELM nodes
The emitter SHALL map wave-2 temporal and uncertainty functions (`TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`) to canonical/native ELM expression forms and SHALL NOT fall back to unresolved `FunctionRef` output when a native form exists.

#### Scenario: TimeOfDay emits canonical ELM node
- **WHEN** a typed expression containing `TimeOfDay()` is emitted
- **THEN** the resulting ELM uses the canonical `TimeOfDay` expression form

#### Scenario: Precision emits canonical ELM node
- **WHEN** a typed expression containing `Precision(x)` is emitted
- **THEN** the resulting ELM uses the canonical `Precision` expression form

### Requirement: Wave-2 aggregate and list-control functions emit executable forms
The emitter SHALL produce executable ELM for `Product`, `GeometricMean`, `Repeat`, and interval `Size` expressions consistent with eval dispatch expectations.

#### Scenario: Product and GeometricMean emit executable nodes
- **WHEN** typed AST nodes for `Product` or `GeometricMean` are emitted
- **THEN** ELM output contains executable aggregate expression nodes expected by runtime dispatch

#### Scenario: Repeat and interval Size emit executable nodes
- **WHEN** typed AST nodes for `Repeat` or interval `Size` are emitted
- **THEN** ELM output contains expression forms that route to dedicated eval handlers
