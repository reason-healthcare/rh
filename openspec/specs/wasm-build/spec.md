# wasm-build

## Purpose

Defines the opt-in WASM build and packaging contract for `rh-fhirpath` and `rh-vcl`.

## Requirements

### Requirement: WASM crates gate cdylib behind a wasm feature

`rh-fhirpath` and `rh-vcl` SHALL omit `cdylib` from their default crate types and activate it
only through the `wasm` feature, so crates.io publication produces pure `rlib` artifacts.

#### Scenario: Default builds produce only rlib
- **WHEN** either WASM crate is built without `--features wasm`
- **THEN** no `cdylib` artifact SHALL be produced

#### Scenario: WASM feature activates cdylib
- **WHEN** `rh-fhirpath` is built with `--features wasm --target wasm32-unknown-unknown`
- **THEN** a `cdylib` artifact SHALL be produced

#### Scenario: Dry-run publish has no crate-type error
- **WHEN** `cargo publish --dry-run -p rh-fhirpath` is run
- **THEN** it SHALL succeed without a crate-type-related error

### Requirement: WASM-only dependencies are optional

`wasm-bindgen`, `js-sys`, and `console_error_panic_hook` SHALL be optional dependencies
activated by the `wasm` feature in both WASM crates.

#### Scenario: Non-WASM build omits wasm-bindgen
- **WHEN** `cargo tree -p rh-fhirpath` runs without the WASM feature
- **THEN** `wasm-bindgen` SHALL not appear in the dependency tree

#### Scenario: WASM feature activates wasm-bindgen
- **WHEN** the WASM feature is enabled
- **THEN** `wasm-bindgen` SHALL appear in the dependency tree

### Requirement: WASM crates share release profile and recipes

Both WASM crates SHALL set the wasm-pack release profile to `wasm-opt = ["-O4"]` and expose
the same justfile recipes: `wasm`, `wasm-web`, `wasm-node`, `wasm-bundler`, `test-wasm`,
`clean-wasm`, and `dev-web`.

#### Scenario: wasm-pack invocations enable the feature
- **WHEN** either WASM justfile is inspected
- **THEN** every `wasm-pack build` command SHALL include `--features wasm`

#### Scenario: Recipe sets match
- **WHEN** `just --list` is run in either WASM crate
- **THEN** the documented shared recipe set SHALL be listed

### Requirement: rh-vcl WASM documentation follows the justfile workflow

`rh-vcl/WASM_BUILD.md` SHALL use `just` commands and SHALL not contain `make` invocations.

#### Scenario: Documentation uses just commands
- **WHEN** `rh-vcl/WASM_BUILD.md` is read
- **THEN** it SHALL show matching `just` recipes and no `make` commands

### Requirement: Root justfile provides a workspace WASM recipe

The root `justfile` SHALL provide a `wasm` recipe that builds WASM packages for both capable
crates in sequence.

#### Scenario: Root recipe delegates to both crates
- **WHEN** `just wasm` is run from the workspace root
- **THEN** it SHALL invoke WASM builds for `rh-fhirpath` and `rh-vcl`

#### Scenario: Root recipe is discoverable
- **WHEN** `just --list` is run from the workspace root
- **THEN** a `wasm` recipe SHALL be listed
