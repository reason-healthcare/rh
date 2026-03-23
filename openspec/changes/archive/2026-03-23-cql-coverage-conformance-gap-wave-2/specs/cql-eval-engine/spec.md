## ADDED Requirements

### Requirement: Wave-2 nullological operators evaluate through explicit dispatch
The eval engine SHALL provide explicit dispatch and evaluation behavior for `IsNull`, `IsTrue`, `IsFalse`, and `Coalesce` expressions in the main engine dispatch path.

#### Scenario: Nullological function evaluates without unsupported-operation errors
- **WHEN** `IsNull(null)`, `IsTrue(true)`, `IsFalse(false)`, or `Coalesce(null, 1)` is evaluated
- **THEN** evaluation returns a spec-consistent result and does not fail due to missing dispatch

#### Scenario: Nullological semantics preserve three-valued logic expectations
- **WHEN** nullological operators receive `null` and non-null combinations
- **THEN** results follow CQL null semantics for each operator

### Requirement: Wave-2 aggregate functions and list fixpoint behavior are executable
The eval engine SHALL evaluate `Product` and `GeometricMean`, and SHALL implement full `Repeat` fixpoint behavior with deterministic termination semantics.

#### Scenario: Product and GeometricMean evaluate on numeric lists
- **WHEN** `Product({1, 2, 3})` and `GeometricMean({1.0, 4.0})` are evaluated
- **THEN** results are computed and returned without unsupported-function errors

#### Scenario: Repeat reaches deterministic fixpoint
- **WHEN** `Repeat` is evaluated for a list expression that converges
- **THEN** evaluation iterates until stable output and returns the fixpoint result

### Requirement: Wave-2 temporal uncertainty and interval size functions evaluate end-to-end
The eval engine SHALL evaluate `TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`, and interval `Size` expressions with deterministic null and type behavior.

#### Scenario: TimeOfDay uses deterministic eval clock
- **WHEN** `TimeOfDay()` is evaluated with a fixed evaluation clock
- **THEN** the returned time value matches the configured clock time-of-day

#### Scenario: Precision and boundaries evaluate for temporal inputs
- **WHEN** `Precision`, `LowBoundary`, or `HighBoundary` is evaluated for a supported temporal value
- **THEN** the engine returns the corresponding precision/boundary result or spec-defined null outcome

#### Scenario: Interval size evaluates for bounded interval
- **WHEN** `size of Interval[1, 5]` is evaluated
- **THEN** the engine returns a deterministic size result per CQL interval semantics
