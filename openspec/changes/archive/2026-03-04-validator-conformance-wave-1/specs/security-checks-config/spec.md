## ADDED Requirements

### Requirement: Security checks are runtime-configurable
The validator SHALL expose a runtime security-checks mode that controls the severity assigned to HTML/script-like content findings during validation.

#### Scenario: Security checks disabled uses informational severity
- **WHEN** validation runs with security checks disabled
- **THEN** HTML/script-like content findings are emitted as informational issues and do not invalidate the resource by themselves

#### Scenario: Security checks enabled uses error severity
- **WHEN** validation runs with security checks enabled
- **THEN** HTML/script-like content findings are emitted as error-level issues using the existing security-check diagnostics

### Requirement: Security-checks mode defaults to compatibility-safe behavior
The runtime security-checks mode MUST default to disabled unless explicitly enabled by the caller.

#### Scenario: Default run does not change existing strictness
- **WHEN** a caller does not provide any security-checks runtime option
- **THEN** validation behavior matches disabled security-checks mode

### Requirement: Security-checks mode is visible in run output
Validation tooling SHALL report the effective security-checks mode for each run.

#### Scenario: Run summary includes effective mode
- **WHEN** validation starts with any explicit or default security-checks mode
- **THEN** the command output includes whether security checks are enabled or disabled
