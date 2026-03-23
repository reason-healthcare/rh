## ADDED Requirements

### Requirement: Wave-2 semantic registration covers targeted operator/function set
Semantic analysis SHALL register and resolve wave-2 function-syntax operators and unary forms (`TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`, `Product`, `GeometricMean`, `Repeat`, and interval `Size` where applicable) so they compile through normal overload resolution.

#### Scenario: Wave-2 function resolves during semantic analysis
- **WHEN** a CQL expression uses a wave-2 function (for example `Product({1,2,3})`)
- **THEN** semantic analysis resolves the call without unknown-function diagnostics

#### Scenario: Wave-2 temporal function resolves with operand typing
- **WHEN** a CQL expression uses `Precision(x)` or `LowBoundary(x)`
- **THEN** semantic analysis resolves overloads based on operand type and records the selected signature

### Requirement: Wave-2 overload selection remains deterministic
For wave-2 functions/operators with multiple candidates, semantic resolution SHALL select deterministic overloads for equivalent typed inputs and apply existing implicit-conversion policy consistently.

#### Scenario: Equivalent inputs resolve to identical overload
- **WHEN** the same wave-2 expression is analyzed repeatedly with the same input types
- **THEN** the same overload is selected each time

#### Scenario: Conversion policy is respected
- **WHEN** an implicit conversion is required for a wave-2 operator and conversions are disabled by compiler option
- **THEN** semantic analysis emits a type error rather than silently coercing values
