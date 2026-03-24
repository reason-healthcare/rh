## ADDED Requirements

### Requirement: Both WASM crates gate cdylib behind a wasm feature
`rh-fhirpath` and `rh-vcl` SHALL NOT include `cdylib` in their default `[lib] crate-type`. The `cdylib` crate type SHALL only be activated when the `wasm` feature is enabled, so that `cargo publish` produces a pure `rlib` artifact.

#### Scenario: Default build produces only rlib
- **WHEN** `cargo build -p rh-fhirpath` is run without `--features wasm`
- **THEN** no `cdylib` artifact is produced

#### Scenario: Default build produces only rlib for rh-vcl
- **WHEN** `cargo build -p rh-vcl` is run without `--features wasm`
- **THEN** no `cdylib` artifact is produced

#### Scenario: wasm feature activates cdylib
- **WHEN** `cargo build -p rh-fhirpath --features wasm --target wasm32-unknown-unknown` is run
- **THEN** a `cdylib` artifact is produced

#### Scenario: Dry-run publish succeeds without cdylib warning
- **WHEN** `cargo publish --dry-run -p rh-fhirpath` is run
- **THEN** the command succeeds without a crate-type related error

---

### Requirement: wasm-bindgen and WASM-only dependencies are optional under the wasm feature
`wasm-bindgen`, `js-sys`, and `console_error_panic_hook` SHALL be declared as optional dependencies activated by the `wasm` feature in both `rh-fhirpath` and `rh-vcl`. They SHALL NOT be unconditional dependencies.

#### Scenario: Non-WASM build does not pull in wasm-bindgen
- **WHEN** `cargo tree -p rh-fhirpath` is run without `--features wasm`
- **THEN** `wasm-bindgen` does not appear in the dependency tree

#### Scenario: wasm feature activates wasm-bindgen
- **WHEN** `cargo tree -p rh-fhirpath --features wasm` is run
- **THEN** `wasm-bindgen` appears in the dependency tree

---

### Requirement: Both WASM crates have identical Cargo.toml wasm-pack release profile
`rh-fhirpath` and `rh-vcl` SHALL each have a `[package.metadata.wasm-pack.profile.release]` section with `wasm-opt = ["-O4"]`.

#### Scenario: rh-fhirpath has wasm-pack release profile
- **WHEN** `rh-fhirpath/Cargo.toml` is inspected
- **THEN** it contains `[package.metadata.wasm-pack.profile.release]` with `wasm-opt = ["-O4"]`

#### Scenario: rh-vcl has wasm-pack release profile
- **WHEN** `rh-vcl/Cargo.toml` is inspected
- **THEN** it contains `[package.metadata.wasm-pack.profile.release]` with `wasm-opt = ["-O4"]`

---

### Requirement: Both WASM crates have identical justfile recipe sets
`rh-fhirpath/justfile` and `rh-vcl/justfile` SHALL provide the same named recipes: `default`, `check-wasm-pack`, `wasm`, `wasm-web`, `wasm-node`, `wasm-bundler`, `test-wasm`, `clean-wasm`, `dev-web`. All `wasm-pack build` invocations SHALL pass `--features wasm`.

#### Scenario: wasm-pack build invocations include --features wasm
- **WHEN** the justfile in either WASM crate is inspected
- **THEN** every `wasm-pack build` call includes `--features wasm`

#### Scenario: Both justfiles expose the same recipe names
- **WHEN** `just --list` is run in `rh-fhirpath/`
- **THEN** recipes `wasm`, `wasm-web`, `wasm-node`, `wasm-bundler`, `test-wasm`, `clean-wasm`, `dev-web` are listed
- **WHEN** `just --list` is run in `rh-vcl/`
- **THEN** the same set of recipes is listed

---

### Requirement: rh-vcl WASM_BUILD.md accurately reflects the justfile-based workflow
`rh-vcl/WASM_BUILD.md` SHALL reference `just` commands, not `make` commands. All examples in the file SHALL match the actual justfile recipe names.

#### Scenario: WASM_BUILD.md uses just commands
- **WHEN** `rh-vcl/WASM_BUILD.md` is read
- **THEN** it contains `just wasm`, `just wasm-web`, etc.
- **THEN** it does NOT contain `make wasm` or any `make` invocation

---

### Requirement: Root justfile provides a workspace-level wasm recipe
The root `justfile` SHALL include a `wasm` recipe that builds WASM packages for all WASM-capable crates (`rh-fhirpath` and `rh-vcl`) in sequence.

#### Scenario: Root wasm recipe delegates to both crates
- **WHEN** `just wasm` is run from the workspace root
- **THEN** `wasm-pack build` runs for `rh-fhirpath`
- **THEN** `wasm-pack build` runs for `rh-vcl`

#### Scenario: Root wasm recipe is listed
- **WHEN** `just --list` is run from the workspace root
- **THEN** a `wasm` recipe appears in the output
