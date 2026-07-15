# crates-io-packaging

## Purpose

Defines the packaging, metadata, naming, publishing, and documentation requirements for
publishing the RH workspace to crates.io.

## Requirements

### Requirement: Workspace package metadata is complete and accurate

The workspace `[workspace.package]` table in the root `Cargo.toml` SHALL contain real values
for `authors`, `repository`, and `homepage`, and SHALL NOT contain placeholder text.

#### Scenario: Authors field is populated
- **WHEN** `cargo metadata` is run on the workspace
- **THEN** each crate that inherits authors SHALL contain at least one non-placeholder author entry

#### Scenario: Repository and homepage are accurate
- **WHEN** `cargo metadata` is run on the workspace
- **THEN** `repository` SHALL be `https://github.com/reason-healthcare/rh` and `homepage` SHALL be non-empty

### Requirement: Publishable crates carry required crates.io fields

Each library crate and CLI binary SHALL set `description`, `license`, `readme`, `keywords`, and
`categories`, directly or through workspace inheritance.

#### Scenario: Publish metadata passes dry-run validation
- **WHEN** `cargo publish --dry-run` is run for a publishable crate
- **THEN** it SHALL not fail for a missing description, README, keywords, or categories field

#### Scenario: Metadata references are valid
- **WHEN** publishable crate metadata is inspected
- **THEN** its README SHALL exist and its keywords and categories SHALL be non-empty valid values

### Requirement: Crates are versioned for the beta release

All publishable crates SHALL use a `0.x.0-beta.1` version before first crates.io publication.

#### Scenario: Standard crates use the beta version
- **WHEN** `cargo metadata` is run
- **THEN** standard publishable crates SHALL report `0.1.0-beta.1`

#### Scenario: rh-validator uses its beta version
- **WHEN** `cargo metadata` is run
- **THEN** `rh-validator` SHALL report `0.2.0-beta.1`

### Requirement: Workspace-internal path dependencies include versions

Every workspace-internal `path` dependency SHALL also specify a version matching the referenced
crate's published version.

#### Scenario: Path dependency publishes successfully
- **WHEN** `cargo publish --dry-run` is run for a crate with an internal path dependency
- **THEN** it SHALL not fail because that dependency lacks a version

#### Scenario: Path dependency version matches
- **WHEN** workspace `Cargo.toml` files are inspected
- **THEN** each internal path dependency version SHALL match its referenced crate version

### Requirement: Crate names follow the rh prefix convention

Published project crates SHALL use the `rh-` prefix and SHALL not imply standards-body provenance.

#### Scenario: Generated FHIR crate has the RH name
- **WHEN** `cargo metadata` is run
- **THEN** the FHIR R4 core package SHALL be named `rh-hl7-fhir-r4-core`

#### Scenario: CLI package and binary names are stable
- **WHEN** `cargo metadata` is run
- **THEN** the CLI package SHALL be `rh-cli` and its binary SHALL be `rh`

### Requirement: rh codegen accepts a crate-name override

The `rh codegen` command SHALL accept optional `--crate-name <NAME>` and use it for generated
package and library names, normalizing hyphens to underscores for `[lib] name`.

#### Scenario: Custom crate name is generated
- **WHEN** codegen runs with `--crate-name rh-hl7-fhir-r4-core`
- **THEN** generated `Cargo.toml` SHALL contain package name `rh-hl7-fhir-r4-core` and library name `rh_hl7_fhir_r4_core`

#### Scenario: Default crate name is preserved
- **WHEN** codegen runs without `--crate-name`
- **THEN** it SHALL retain the established auto-derived crate name

### Requirement: Publish ordering respects the dependency graph

Documentation and publish automation SHALL order publication as `rh-foundation`, then the
independent core crates, then dependent crates, followed by `rh-validator` and `rh-cli`.

#### Scenario: Publishing in dependency order succeeds
- **WHEN** crates are published in the documented dependency order
- **THEN** no crate SHALL fail because an internal dependency is unpublished

#### Scenario: Dry-run passes for all crates
- **WHEN** `cargo publish --dry-run` is run in documented dependency order
- **THEN** all invocations SHALL succeed

### Requirement: Publication documentation is accurate and undecorated

Documentation introduced or modified for publication SHALL match the current crate names,
feature flags, and commands, and SHALL not use decorative emoji in headings or prose.

#### Scenario: WASM documentation matches recipes
- **WHEN** a WASM build document is read
- **THEN** every listed command SHALL correspond to an actual justfile recipe

#### Scenario: Documentation has no decorative emoji
- **WHEN** documentation changed for publication is reviewed
- **THEN** headings, bullets, and prose SHALL not contain decorative emoji

### Requirement: rh-cli README links to per-command documentation

The CLI README SHALL index dedicated documentation pages for `codegen`, `download`, `fhirpath`,
`vcl`, `cql`, and `validate` instead of duplicating each full reference inline.

#### Scenario: Documentation index and pages exist
- **WHEN** the CLI README and docs directory are inspected
- **THEN** each major command SHALL have an indexed, dedicated documentation page

#### Scenario: Command pages are accurate
- **WHEN** a command documentation page is read
- **THEN** its examples SHALL use `rh` and reflect current flags, crate names, and subcommands
