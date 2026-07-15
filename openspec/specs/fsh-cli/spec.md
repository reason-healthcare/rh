# fsh-cli Specification

## Purpose

Define command-line behavior for compiling, parsing, and inspecting FSH.

## Requirements

### Requirement: Provide `rh fsh compile` command for FSH → FHIR JSON
The CLI SHALL provide a `rh fsh compile <files...>` subcommand that accepts one or more FSH file paths, compiles them using `FshCompiler`, and writes FHIR JSON output files to a specified or default output directory.

#### Scenario: Compile a single FSH file
- **WHEN** `rh fsh compile patient-profile.fsh --output ./fhir/` is run
- **THEN** FHIR JSON files are written to `./fhir/` and the command exits with code 0

#### Scenario: Compile multiple FSH files
- **WHEN** `rh fsh compile *.fsh --output ./fhir/` is run with multiple FSH files
- **THEN** all entities from all files are compiled and written as separate JSON files

#### Scenario: Compilation error exits non-zero
- **WHEN** a FSH file contains a parse error
- **THEN** the command prints the error with file name, line, and column, and exits with a non-zero code

### Requirement: Provide `rh fsh parse` command for AST inspection
The CLI SHALL provide a `rh fsh parse <file>` subcommand that parses a single FSH file and prints the resulting AST as JSON to stdout, for debugging purposes.

#### Scenario: Parse and dump AST
- **WHEN** `rh fsh parse my-profile.fsh` is run on a valid FSH file
- **THEN** the AST is printed as pretty-printed JSON to stdout and the command exits with code 0

### Requirement: Provide `rh fsh tank` command for resolved tank inspection
The CLI SHALL provide a `rh fsh tank <files...>` subcommand that parses and resolves the given FSH files, printing the resolved `FshTank` as JSON to stdout, for debugging purposes.

#### Scenario: Dump resolved tank
- **WHEN** `rh fsh tank *.fsh` is run on valid FSH files
- **THEN** the resolved tank (with aliases and RuleSets expanded) is printed as JSON and the command exits with code 0

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
