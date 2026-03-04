# validate-input-globs Specification

## Purpose
TBD - created by archiving change validate-input-globs. Update Purpose after archive.
## Requirements
### Requirement: Validate command accepts glob input patterns
The `rh validate` CLI SHALL accept glob patterns passed through `-i` and `--input`, including common wildcard forms such as `*` and recursive patterns such as `**`.

#### Scenario: Single glob pattern resolves to multiple files
- **WHEN** a user runs `rh validate -i 'fixtures/*.json'` and the pattern matches multiple files
- **THEN** the CLI includes all matched files in the validation input set

#### Scenario: Recursive glob pattern is accepted
- **WHEN** a user runs `rh validate -i 'fixtures/**/*.json'`
- **THEN** the CLI resolves matching files from nested directories

### Requirement: Validate command supports mixed explicit paths and globs
The `rh validate` CLI SHALL support combining explicit file paths and glob patterns in the same invocation.

#### Scenario: Mixed inputs are combined
- **WHEN** a user runs `rh validate -i patient.json -i 'fixtures/*.json'`
- **THEN** the CLI validates both the explicit file and all files matched by the glob

### Requirement: Resolved validation inputs are deterministic and unique
Before validation starts, the CLI SHALL produce a deterministic, deduplicated list of resolved files from all input tokens.

#### Scenario: Overlapping patterns do not cause duplicate validation
- **WHEN** multiple input patterns match the same file
- **THEN** that file is validated exactly once

#### Scenario: Input ordering is deterministic
- **WHEN** the same command is run repeatedly against unchanged files
- **THEN** the resolved file order is stable across runs

### Requirement: Unmatched glob patterns fail with actionable feedback
If an input token is interpreted as a glob pattern and matches no files, the CLI MUST fail before validation and identify the unmatched pattern.

#### Scenario: No-match glob returns an explicit error
- **WHEN** a user runs `rh validate -i 'missing/**/*.json'` and no files match
- **THEN** the command exits with an error message that includes the unmatched input pattern

### Requirement: Explicit file path behavior remains backward compatible
If an input token corresponds to an existing file path, the CLI SHALL treat it as an explicit path and preserve existing validation behavior.

#### Scenario: Existing explicit input works unchanged
- **WHEN** a user runs `rh validate -i patient.json` and the file exists
- **THEN** validation proceeds using that file as before without requiring glob semantics

