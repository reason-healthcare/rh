## ADDED Requirements

### Requirement: Workspace package metadata is complete and accurate
The workspace `[workspace.package]` table in the root `Cargo.toml` SHALL contain real values for `authors`, `repository`, and `homepage`. It SHALL NOT contain placeholder text such as `"Your Name"` or `"yourusername"`.

#### Scenario: Authors field is populated
- **WHEN** `cargo metadata` is run on the workspace
- **THEN** the `authors` field for each crate that inherits it contains at least one non-placeholder author entry

#### Scenario: Repository field points to the real repository
- **WHEN** `cargo metadata` is run on the workspace
- **THEN** the `repository` field resolves to `https://github.com/reason-healthcare/rh`

#### Scenario: Homepage field is present
- **WHEN** `cargo metadata` is run on the workspace
- **THEN** the `homepage` field is non-empty

---

### Requirement: All publishable crates carry required crates.io fields
Each library crate and the CLI binary SHALL have `description`, `license`, `readme`, `keywords`, and `categories` fields set, either directly or via workspace inheritance.

#### Scenario: Description is present
- **WHEN** `cargo publish --dry-run` is run for any crate
- **THEN** the command does not fail with a missing `description` error

#### Scenario: README field is set
- **WHEN** `cargo publish --dry-run` is run for any crate
- **THEN** the `readme` field points to an existing file in the crate directory

#### Scenario: Keywords and categories are present
- **WHEN** `cargo metadata` is inspected for any publishable crate
- **THEN** `keywords` contains at least one entry and `categories` contains at least one valid crates.io category string

---

### Requirement: All crates are versioned at beta
All publishable crates SHALL have a version in the form `0.x.0-beta.1` prior to the first crates.io publish.

#### Scenario: Standard crates are at 0.1.0-beta.1
- **WHEN** `cargo metadata` is run
- **THEN** all crates except `rh-validator` report a version of `0.1.0-beta.1`

#### Scenario: rh-validator is at 0.2.0-beta.1
- **WHEN** `cargo metadata` is run
- **THEN** `rh-validator` reports a version of `0.2.0-beta.1`

---

### Requirement: All workspace-internal path dependencies include a version specifier
Every `path` dependency on another workspace crate SHALL also specify a `version` field matching that crate's published version. This is required by crates.io.

#### Scenario: Path dependency with version passes dry-run
- **WHEN** `cargo publish --dry-run` is run for any crate with a workspace-internal `path` dependency
- **THEN** the command does not fail with a "path dependency ... does not have a version" error

#### Scenario: Version specifier matches the declared version of the dependency
- **WHEN** `Cargo.toml` files are inspected
- **THEN** the `version` in each path dependency matches the `version` field of the referenced crate's `Cargo.toml`

---

### Requirement: Crate names follow the `rh-` prefix convention
All crates published under this project SHALL use the `rh-` prefix in their package name to establish consistent namespace ownership and avoid implying official provenance from standards bodies.

#### Scenario: Generated FHIR crate is named rh-hl7-fhir-r4-core
- **WHEN** `cargo metadata` is run
- **THEN** the FHIR R4 core crate reports `name = "rh-hl7-fhir-r4-core"`

#### Scenario: CLI crate is named rh-cli with binary rh
- **WHEN** `cargo metadata` is run
- **THEN** the CLI package name is `rh-cli`
- **THEN** the binary name produced is `rh`

---

### Requirement: `rh codegen` accepts a `--crate-name` override
The `rh codegen` command SHALL accept an optional `--crate-name <NAME>` argument. When provided, the generated crate's `Cargo.toml` `name` field and `[lib] name` field SHALL use this value instead of the auto-derived name.

#### Scenario: Custom name appears in generated Cargo.toml
- **WHEN** `rh codegen hl7.fhir.r4.core 4.0.1 --output ./out --crate-name rh-hl7-fhir-r4-core` is run
- **THEN** `./out/Cargo.toml` contains `name = "rh-hl7-fhir-r4-core"`

#### Scenario: Custom name appears in generated [lib] section
- **WHEN** `rh codegen hl7.fhir.r4.core 4.0.1 --output ./out --crate-name rh-hl7-fhir-r4-core` is run
- **THEN** `./out/Cargo.toml` contains `name = "rh_hl7_fhir_r4_core"` under the `[lib]` section (hyphens normalized to underscores)

#### Scenario: Omitting --crate-name preserves existing auto-derive behavior
- **WHEN** `rh codegen hl7.fhir.r4.core 4.0.1 --output ./out` is run without `--crate-name`
- **THEN** `./out/Cargo.toml` contains `name = "hl7_fhir_r4_core"` (unchanged default behavior)

---

### Requirement: Publish ordering respects the dependency graph
Documentation and any publish automation SHALL express the crates.io publish order as: `rh-foundation` → `rh-hl7-fhir-r4-core`, `rh-codegen`, `rh-cql` (parallel) → `rh-fhirpath`, `rh-vcl` (parallel) → `rh-validator` → `rh-cli`.

#### Scenario: Publishing in order succeeds
- **WHEN** each crate is published to crates.io in the documented order
- **THEN** no crate fails due to an unpublished dependency

#### Scenario: dry-run passes for all crates
- **WHEN** `cargo publish --dry-run` is run for each crate in dependency order
- **THEN** all invocations succeed without error

---

### Requirement: Documentation files are accurate and free of decorative emoji
All `.md` documentation files modified or introduced as part of this change SHALL accurately reflect the current state of the codebase (correct crate names, feature flags, command invocations). Section headers, bullet prefixes, and inline prose in documentation files SHALL NOT use decorative emoji. Status indicators such as `[x]` checkbox markers are acceptable.

#### Scenario: WASM_BUILD.md files match the justfile
- **WHEN** `crates/rh-fhirpath/WASM_BUILD.md` or `crates/rh-vcl/WASM_BUILD.md` is read
- **THEN** every command example listed corresponds to an actual recipe in the respective justfile

#### Scenario: Documentation does not use section-level emoji
- **WHEN** any `.md` file touched in this change is reviewed
- **THEN** no emoji appears in section headings (`##`, `###`) or as bullet-point prefixes or inline decoration
- **THEN** `[x]` checkbox markers and plain-text status indicators remain permitted

---

### Requirement: rh-cli README links to per-command documentation pages
The `apps/rh-cli/README.md` SHALL include a documentation index that links to a `docs/<COMMAND>.md` file for each major CLI command. A `docs/<COMMAND>.md` file SHALL exist for each of: `codegen`, `download`, `fhirpath`, `vcl`, `cql`, `validate`. The README SHALL NOT duplicate the full command reference inline; instead it SHALL provide a brief description of each command and a link to its doc page.

#### Scenario: README contains a documentation index
- **WHEN** `apps/rh-cli/README.md` is read
- **THEN** it contains a section listing each major command with a link to `docs/<COMMAND>.md`

#### Scenario: A doc page exists for each major command
- **WHEN** the `apps/rh-cli/docs/` directory is listed
- **THEN** files `CODEGEN.md`, `DOWNLOAD.md`, `FHIRPATH.md`, `VCL.md`, `CQL.md`, and `VALIDATOR.md` are all present

#### Scenario: Each doc page covers its command accurately
- **WHEN** any `docs/<COMMAND>.md` file is read
- **THEN** all command examples use the correct binary name `rh` (not `cargo run -p rh`)
- **THEN** any feature flags, crate names, or subcommand signatures match the current implementation
