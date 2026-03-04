# conformance-quick-wins Specification

## Purpose
TBD - created by archiving change validator-conformance-wave-1. Update Purpose after archive.

## Requirements
### Requirement: UCUM and unit quick-win checks depend on terminology service availability
The validator SHALL perform UCUM/unit conformance quick-win checks only when an optional terminology service is configured and available for the run.

#### Scenario: Terminology service configured enables UCUM/unit checks
- **WHEN** a validation run has a configured terminology service
- **THEN** UCUM/unit quick-win checks execute and report conformance findings

#### Scenario: Terminology service not configured skips UCUM/unit checks
- **WHEN** a validation run does not configure a terminology service
- **THEN** UCUM/unit quick-win checks are skipped and a non-error informational note indicates they were not evaluated

### Requirement: Quick-win bundle uniqueness checks are deterministic
The validator MUST enforce deterministic bundle uniqueness checks for targeted conformance cases in this wave.

#### Scenario: Duplicate fullUrl in Bundle is reported
- **WHEN** a Bundle contains repeated fullUrl values where uniqueness is required
- **THEN** validation reports a deterministic issue at the bundle entry path

#### Scenario: Duplicate logical identity in Bundle is reported
- **WHEN** a Bundle contains duplicate resource identity combinations targeted by this wave
- **THEN** validation reports a deterministic issue without introducing suite-specific exceptions

### Requirement: Quick-win scope is explicitly bounded to identified failure categories
The validator SHALL implement only the documented conformance quick-win checks for this wave and MUST not expand to broad terminology parity work.

#### Scenario: Out-of-scope terminology parity behavior is not introduced
- **WHEN** a failing case requires full terminology parity not listed as a quick win
- **THEN** the run reports existing behavior and does not claim the unsupported check was implemented
