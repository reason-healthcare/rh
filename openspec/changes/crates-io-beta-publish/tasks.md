## 1. Add `--crate-name` to `rh codegen`

- [ ] 1.1 Add `crate_name: Option<&'a str>` field to `CrateGenerationParams` in `crates/rh-codegen/src/generators/crate_generator.rs`
- [ ] 1.2 Update `generate_crate_structure` to pass `crate_name` through to `generate_cargo_toml` and `generate_readme_md`
- [ ] 1.3 Update `generate_cargo_toml` to use `crate_name` when `Some`, falling back to the auto-derived name; normalize hyphens to underscores for the `[lib] name` field
- [ ] 1.4 Update hardcoded `hl7_fhir_r4_core` string literals in `mutator_trait_generator.rs` format strings (lines ~192–193) and `crate_generator.rs` template strings to use the resolved crate name
- [ ] 1.5 Add `--crate-name <NAME>` optional argument to `CodegenArgs` in `apps/rh-cli/src/codegen.rs`
- [ ] 1.6 Wire `args.crate_name` into `CrateGenerationParams` in `handle_command`
- [ ] 1.7 Update the `command_invoked` string in `handle_command` to include `--crate-name` when provided

## 2. Fix workspace-root Cargo.toml metadata

- [ ] 2.1 Replace placeholder `authors` in `[workspace.package]` with `["Reason Healthcare <engineering@reason.health>"]`
- [ ] 2.2 Replace placeholder `repository` with `"https://github.com/reason-healthcare/rh"`
- [ ] 2.3 Add `homepage = "https://github.com/reason-healthcare/rh"` to `[workspace.package]`

## 3. Normalize per-crate metadata

- [ ] 3.1 `rh-foundation/Cargo.toml`: add `description`, `keywords`, `categories`, `readme = "README.md"`; inherit `authors`, `repository`, `homepage` via workspace
- [ ] 3.2 `rh-codegen/Cargo.toml`: add `keywords`, `categories`, `readme = "README.md"`; inherit `homepage` via workspace
- [ ] 3.3 `rh-fhirpath/Cargo.toml`: add `description`, `keywords`, `categories`, `readme = "README.md"`; switch `authors` and `repository` to workspace inheritance; add `homepage.workspace = true`
- [ ] 3.4 `rh-cql/Cargo.toml`: add `readme = "README.md"`; switch `repository` to workspace inheritance; add `authors.workspace = true`, `homepage.workspace = true`
- [ ] 3.5 `rh-validator/Cargo.toml`: add `readme = "README.md"`, `homepage.workspace = true`; inherit `authors`, `repository` via workspace where not already set
- [ ] 3.6 `rh-vcl/Cargo.toml`: add `description`, `keywords`, `categories`, `readme = "README.md"`; switch `authors` and `repository` to workspace inheritance; add `homepage.workspace = true`
- [ ] 3.7 `apps/rh-cli/Cargo.toml`: add `keywords`, `categories`, `readme = "README.md"`, `homepage.workspace = true`

## 4. Set beta versions

- [ ] 4.1 Set `version = "0.1.0-beta.1"` in `[workspace.package]` in root `Cargo.toml` (propagates to crates using `version.workspace = true`)
- [ ] 4.2 For crates that declare their own version (not workspace-inherited): update `rh-cql`, `rh-fhirpath`, `rh-validator`, `rh-vcl`, `rh-hl7_fhir_r4_core` to their respective beta versions
- [ ] 4.3 Set `rh-validator` to `version = "0.2.0-beta.1"` (does not inherit from workspace)

## 5. Add version specifiers to workspace-internal path dependencies

- [ ] 5.1 `rh-codegen/Cargo.toml`: add `version = "0.1.0-beta.1"` to `rh-foundation` path dep
- [ ] 5.2 `rh-fhirpath/Cargo.toml`: add version to `rh-foundation` and `hl7_fhir_r4_core` path deps (use `rh-hl7-fhir-r4-core` name after rename)
- [ ] 5.3 `rh-validator/Cargo.toml`: add version to `rh-foundation`, `rh-fhirpath`, `hl7_fhir_r4_core` path deps
- [ ] 5.4 `apps/rh-cli/Cargo.toml`: add version to all workspace crate path deps (`rh-foundation`, `rh-codegen`, `rh-cql`, `rh-fhirpath`, `rh-validator`, `rh-vcl`, and the renamed FHIR crate)

## 6. Gate `cdylib` behind the `wasm` feature

- [ ] 6.1 `rh-fhirpath/Cargo.toml`: change `[lib] crate-type` to `["rlib"]` only
- [ ] 6.2 `rh-fhirpath/Cargo.toml`: add `[features] wasm = ["dep:wasm-bindgen", "dep:js-sys", "dep:console_error_panic_hook"]`; mark those three deps as `optional = true`
- [ ] 6.3 Create `crates/rh-fhirpath/build.rs` that emits `cargo:rustc-cfg=wasm_feature` when the `CARGO_FEATURE_WASM` env var is set; verify `cdylib` activation approach (see Open Questions in design)
- [ ] 6.4 `rh-vcl/Cargo.toml`: change `[lib] crate-type` to `["rlib"]` only
- [ ] 6.5 `rh-vcl/Cargo.toml`: add `[features] wasm = ["dep:wasm-bindgen", "dep:js-sys", "dep:console_error_panic_hook"]`; mark those three deps as `optional = true`
- [ ] 6.6 Create `crates/rh-vcl/build.rs` matching the `rh-fhirpath` pattern
- [ ] 6.7 Add `[package.metadata.wasm-pack.profile.release] wasm-opt = ["-O4"]` to `rh-vcl/Cargo.toml` (already present in `rh-fhirpath`)
- [ ] 6.8 Verify `cargo build -p rh-fhirpath` and `cargo build -p rh-vcl` succeed without `--features wasm`
- [ ] 6.9 Verify `cargo build -p rh-fhirpath --features wasm --target wasm32-unknown-unknown` produces a `cdylib` artifact

## 7. Standardize WASM build tooling

- [ ] 7.1 Update all `wasm-pack build` calls in `crates/rh-fhirpath/justfile` to include `--features wasm`
- [ ] 7.2 Update all `wasm-pack build` calls in `crates/rh-vcl/justfile` to include `--features wasm`
- [ ] 7.3 Verify `rh-vcl/justfile` has the same recipe set as `rh-fhirpath/justfile` (`wasm`, `wasm-web`, `wasm-node`, `wasm-bundler`, `test-wasm`, `clean-wasm`, `dev-web`); add any missing recipes
- [ ] 7.4 Rewrite `crates/rh-vcl/WASM_BUILD.md` to reference `just` commands (replace all `make` references)
- [ ] 7.5 Add a `wasm` recipe to the root `justfile` that delegates to `cd crates/rh-fhirpath && just wasm` and `cd crates/rh-vcl && just wasm`; update the existing `build-wasm`/`build-wasm-node`/`build-wasm-bundler`/`test-wasm` recipes in root `justfile` to remove the stale `make test-wasm` call and align with new `wasm` recipe

## 8. Regenerate FHIR crate and update downstream references

- [ ] 8.1 Run `rh codegen hl7.fhir.r4.core 4.0.1 --output crates/rh-hl7_fhir_r4_core --crate-name rh-hl7-fhir-r4-core --force` to regenerate with the new crate name
- [ ] 8.2 Verify `crates/rh-hl7_fhir_r4_core/Cargo.toml` shows `name = "rh-hl7-fhir-r4-core"` and `[lib] name = "rh_hl7_fhir_r4_core"`
- [ ] 8.3 Update dependency key in `rh-fhirpath/Cargo.toml`: `hl7_fhir_r4_core` → `rh-hl7-fhir-r4-core`
- [ ] 8.4 Update dependency key in `rh-validator/Cargo.toml`: `hl7_fhir_r4_core` → `rh-hl7-fhir-r4-core`
- [ ] 8.5 Update dependency key in `apps/rh-cli/Cargo.toml`: `hl7_fhir_r4_core` → `rh-hl7-fhir-r4-core`
- [ ] 8.6 Update `use hl7_fhir_r4_core::` → `use rh_hl7_fhir_r4_core::` in `crates/rh-fhirpath/src/evaluator/metadata.rs` (2 occurrences)
- [ ] 8.7 Run `cargo check --workspace` and fix any remaining `hl7_fhir_r4_core` reference errors (live source only; `.archive/` files are excluded from workspace build)

## 9. Audit and update documentation

- [ ] 9.1 Review `crates/rh-fhirpath/WASM_BUILD.md` for accuracy against the updated justfile; remove any section-level emoji decoration (keep only check/x markers in status lists)
- [ ] 9.2 Review `crates/rh-vcl/WASM_BUILD.md` for accuracy (will be rewritten in 7.4 — verify the result has no section-level emoji)
- [ ] 9.3 Review `crates/rh-fhirpath/README.md` for install/usage instructions that mention crate name or feature flags; update to reflect `wasm` feature and new import path `rh_hl7_fhir_r4_core` if referenced
- [ ] 9.4 Review `crates/rh-vcl/README.md` for accuracy against justfile recipes; remove section-level emoji
- [ ] 9.5 Review `apps/rh-cli/README.md` for references to old crate name `hl7_fhir_r4_core` or package name `rh` (non-binary context); update as needed
- [ ] 9.6 Review root `README.md` for any crate name references or install instructions that need updating for beta versions
- [ ] 9.7 Remove over-use of emoji from any `.md` file touched in this change — section headers, bullet prefixes, and inline decorations SHALL use plain text; status indicators (e.g., `[x]`, checkboxes) are acceptable
- [ ] 9.8 Create `apps/rh-cli/docs/CODEGEN.md` covering `rh codegen` subcommands; move detailed codegen reference content out of README into this file; examples SHALL use `rh` binary name (not `cargo run -p rh`)
- [ ] 9.9 Create `apps/rh-cli/docs/DOWNLOAD.md` covering `rh download` subcommands; move detailed download reference content out of README into this file
- [ ] 9.10 Create `apps/rh-cli/docs/FHIRPATH.md` covering `rh fhirpath` subcommands
- [ ] 9.11 Create `apps/rh-cli/docs/VCL.md` covering `rh vcl` subcommands
- [ ] 9.12 Create `apps/rh-cli/docs/CQL.md` covering `rh cql` subcommands
- [ ] 9.13 Review and update existing `apps/rh-cli/docs/VALIDATOR.md` for accuracy (emoji, binary name, correct flags)
- [ ] 9.14 Rewrite `apps/rh-cli/README.md` to include a documentation index section linking to each `docs/<COMMAND>.md`; replace inline full-reference sections with brief descriptions + links; remove `cargo run -p rh` invocation style in favour of `rh`

## 10. Validation

- [ ] 10.1 Run `just check` from workspace root and confirm all format, lint, test, and audit checks pass
- [ ] 10.2 Run `cargo publish --dry-run -p rh-foundation` and confirm success
- [ ] 10.3 Run `cargo publish --dry-run -p rh-hl7-fhir-r4-core` and confirm success
- [ ] 10.4 Run `cargo publish --dry-run -p rh-codegen` and confirm success
- [ ] 10.5 Run `cargo publish --dry-run -p rh-cql` and confirm success
- [ ] 10.6 Run `cargo publish --dry-run -p rh-fhirpath` and confirm success
- [ ] 10.7 Run `cargo publish --dry-run -p rh-vcl` and confirm success
- [ ] 10.8 Run `cargo publish --dry-run -p rh-validator` and confirm success
- [ ] 10.9 Run `cargo publish --dry-run -p rh-cli` and confirm success
- [ ] 10.10 Run `just wasm` from workspace root and confirm both WASM crates build successfully
