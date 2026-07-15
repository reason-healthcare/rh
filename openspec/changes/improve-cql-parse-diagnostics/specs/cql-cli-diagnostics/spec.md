## ADDED Requirements

### Requirement: CQL syntax errors use the documented CLI exit code

The `rh cql` commands SHALL exit with code `4` when compilation fails because the input contains a
CQL syntax error.

#### Scenario: Human-readable syntax error exit

- **WHEN** a CQL command processes malformed source in the default output format
- **THEN** the process exits with code `4` and reports the parser failure

#### Scenario: JSON syntax error exit

- **WHEN** a CQL command processes malformed source with global `--format json`
- **THEN** the process exits with code `4`

### Requirement: CQL syntax errors have a stable JSON envelope

With global `--format json`, a CQL syntax error SHALL be emitted as a valid agent-consumable error
envelope containing the established error fields and the useful source location.

#### Scenario: JSON error is machine-readable

- **WHEN** malformed CQL is processed with global `--format json`
- **THEN** standard output or standard error, according to the documented CLI contract, contains
  one valid JSON error envelope and no unstructured diagnostic mixed into that stream

#### Scenario: JSON error locates the failing construct

- **WHEN** the malformed input contains a recognized operator with an invalid right operand
- **THEN** the JSON error location identifies that operand rather than unrelated trailing content

#### Scenario: Successful CLI behavior is unchanged

- **WHEN** valid CQL is processed in human-readable or JSON format
- **THEN** the command retains its existing success exit code and output envelope
