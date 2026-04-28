## ADDED Requirements

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
