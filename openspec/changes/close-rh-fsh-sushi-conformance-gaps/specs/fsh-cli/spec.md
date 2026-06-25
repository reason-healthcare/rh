## ADDED Requirements

### Requirement: Write FSH compile output using safe deterministic file names
The `rh fsh compile --output` command SHALL write every generated resource to a deterministic file path that cannot accidentally create nested paths from resource ids or canonical URLs.

#### Scenario: Resource id contains slash
- **WHEN** a generated resource id or resourceType contains `/`
- **THEN** the CLI sanitizes the output filename while preserving the resource JSON content

#### Scenario: Multiple resources sanitize to same filename
- **WHEN** two generated resources would produce the same sanitized filename
- **THEN** the CLI disambiguates filenames deterministically and reports the mapping in verbose or JSON output

### Requirement: Provide conformance-friendly JSON compile output
The `rh fsh compile --format json` command SHALL emit all compiled resources in a stable envelope suitable for automated conformance comparison.

#### Scenario: JSON compile succeeds with output directory omitted
- **WHEN** `rh --format json fsh compile <files...>` is run
- **THEN** stdout contains an envelope with all compiled resources and stderr contains only diagnostics

#### Scenario: Compilation diagnostics are represented
- **WHEN** compilation completes with non-fatal warnings
- **THEN** the JSON envelope includes warning diagnostics without preventing comparison of generated resources
