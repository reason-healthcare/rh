## 1. Workspace Scaffold

- [x] 1.1 Create `crates/rh-publisher/` with `Cargo.toml`, `src/lib.rs`, `src/error.rs`
- [x] 1.2 Add `pulldown-cmark` to workspace dependencies in root `Cargo.toml`
- [x] 1.3 Add `rh-publisher` as a dependency of `rh-cli` in `apps/rh-cli/Cargo.toml`
- [x] 1.4 Add `toml` crate to workspace dependencies for `publisher.toml` parsing
- [x] 1.5 Wire `rh publish` stub subcommand into `apps/rh-cli/src/main.rs` (skeleton only, exits with "not yet implemented")

## 2. Core Data Models

- [x] 2.1 Implement `PackageJson` struct (de/serialize `package.json` — name, version, fhirVersions, dependencies, url, description)
- [x] 2.2 Implement `PublisherConfig` struct (de/serialize `publisher.toml` — `[hooks]`, `[validate]`, `[cql]` blocks)
- [x] 2.3 Implement `PublishContext` struct (source dir, output dir, loaded `PackageJson`, mutable resource map `HashMap<String, serde_json::Value>`, config)
- [x] 2.4 Write unit tests for `PackageJson` round-trip serialization
- [x] 2.5 Write unit tests for `PublisherConfig` TOML parsing including missing-block defaults

## 3. Source Directory Loading

- [x] 3.1 Implement source dir scanner — find all `*.json` FHIR resources and load into resource map
- [x] 3.2 Implement `*.md` file discovery — partition into resource-matched and standalone lists
- [x] 3.3 Fail with clear error if `package.json` is absent
- [x] 3.4 Fail with clear error if no `ImplementationGuide` resource is found
- [x] 3.5 Write unit tests for source directory scanning with fixture directories

## 4. ImplementationGuide Sync Check

- [x] 4.1 Implement sync validator — compare `name`/`packageId`, `version`, `url`, `fhirVersions` between `package.json` and the IG resource
- [x] 4.2 Return descriptive errors identifying each mismatched field
- [x] 4.3 Write unit tests for each sync mismatch case and the passing case

## 5. .index.json Generation

- [x] 5.1 Implement `.index.json` builder — iterate resource map, extract `resourceType`, `id`, `url`, `version` per resource
- [x] 5.2 Write `"index-version": 2` per FHIR Package Spec
- [x] 5.3 Write unit tests for index generation with mixed resource types

## 6. Output Directory Assembly

- [x] 6.1 Implement output writer — materialise resource map to `<output>/package/*.json`
- [x] 6.2 Copy standalone markdown files to `<output>/package/other/`
- [x] 6.3 Write `.index.json` to `<output>/package/.index.json`
- [x] 6.4 Write unit tests for output directory structure

## 7. Tarball Packing

- [x] 7.1 Implement tarball writer — compress `<output>/package/` into `<name>-<version>.tgz` with `package/` path prefix using `tar` + `flate2`
- [x] 7.2 Write unit test verifying tarball entry path prefix and file presence

## 8. `rh publish` CLI Subcommands

- [x] 8.1 Implement `rh publish build <dir>` — orchestrate load → sync check → hooks → assemble → write
- [x] 8.2 Implement `rh publish pack <dir>` — run before_pack hooks → pack → after_pack hooks
- [x] 8.3 Implement `rh publish check <dir>` — run sync check and hook dry-run without writing output
- [x] 8.4 Implement `rh publish lock <dir>` — delegate to lock module (stub until section 10)
- [x] 8.5 Add `--output <path>` flag to `build` and `pack` subcommands

## 9. Lifecycle Hook Runner

- [x] 9.1 Define `HookProcessor` trait (`name() -> &'static str`, `async fn run(&self, ctx: &mut PublishContext) -> Result<()>`)
- [x] 9.2 Implement processor registry — map of name → boxed `HookProcessor`
- [x] 9.3 Implement hook runner — iterate stage processor list, abort on first error
- [x] 9.4 Fail at startup with named error for unknown processor names in `publisher.toml`
- [x] 9.5 Write unit tests for sequential execution, abort-on-first-failure, and unknown-name detection

## 10. Narrative Processor

- [x] 10.1 Implement markdown-to-XHTML converter using `pulldown-cmark`, wrapping output in `<div xmlns="http://www.w3.org/1999/xhtml">`
- [x] 10.2 Implement resource-narrative matcher — pair `.md` files with resource map entries by filename stem
- [x] 10.3 Embed converted XHTML into resource `text` (`status: "generated"`, `div: "..."`)
- [x] 10.4 Route unmatched `.md` files to `package/other/` in the output
- [x] 10.5 Write unit tests for matching, embedding, and unmatched routing

## 11. Canonical Lock (`fhir-lock.json`)

- [x] 11.1 Implement canonical reference scanner — walk resource JSON values looking for canonical-type fields
- [x] 11.2 Build exclusion list for well-known external code system URLs (SNOMED, LOINC, RxNorm, ICD-10, ICD-11)
- [x] 11.3 Implement resolver — load dependency packages from `~/.fhir/packages/` and look up canonical URLs
- [x] 11.4 Implement `fhir-lock.json` writer — serialize `generated`, `pinMode`, `canonicals[]`
- [x] 11.5 Implement lock reader + build-time pinning — append `|version` to unversioned canonicals in output resources
- [x] 11.6 Emit warning (non-fatal) when a canonical cannot be resolved
- [x] 11.7 Fail with named error when a required dependency package is not present
- [x] 11.8 Wire `rh publish lock` subcommand to lock implementation
- [x] 11.9 Write unit tests for canonical scanning, pinning application, and exclusion list

## 12. `snapshot` Hook Processor

- [x] 12.1 Implement `SnapshotProcessor` — find StructureDefinition entries in resource map lacking `snapshot.element`
- [x] 12.2 Load dependency packages and initialise `SnapshotGenerator` from `rh-foundation`
- [x] 12.3 Run generator per StructureDefinition and update resource in map
- [x] 12.4 Fail with clear error identifying unresolvable `baseDefinition` URLs
- [x] 12.5 Register `SnapshotProcessor` in processor registry under name `"snapshot"`
- [x] 12.6 Write unit tests for already-snapshotted resources being skipped, and successful expansion

## 13. `validate` Hook Processor

- [x] 13.1 Implement `ValidateProcessor` — iterate resource map, run `rh-validator` `FhirValidator` per resource
- [x] 13.2 Load dependency packages for validation context
- [x] 13.3 Abort on first resource with ERROR-severity issues; collect and print WARNING issues non-fatally
- [x] 13.4 Wire `[validate]` config block (`terminology_server`, `skip_invariants`, `skip_bindings`)
- [x] 13.5 Register `ValidateProcessor` in processor registry under name `"validate"`
- [ ] 13.6 Write unit tests for ERROR-abort, WARNING-pass-through, and missing-dependency error

## 14. `cql` Hook Processor

- [x] 14.1 Implement CQL file discoverer — find all `*.cql` files in source directory
- [x] 14.2 Implement Library resource matcher — locate `Library-<name>.json` in resource map by CQL library name
- [x] 14.3 Implement auto-create for missing Library resources (minimal conformant resource with `logic-library` type)
- [x] 14.4 Compile each CQL file to ELM using `rh-cql` with `FileLibrarySourceProvider` + `PackageLibrarySourceProvider`
- [x] 14.5 Base64-encode CQL source and ELM JSON; embed as `Library.content[]` (`text/cql`, `application/elm+json`)
- [x] 14.6 Set `Library.type` to `logic-library` when absent
- [x] 14.7 Emit informational message when a Library is auto-created
- [x] 14.8 Wire `[cql]` config block (`packages_dir`)
- [x] 14.9 Register `CqlProcessor` in processor registry under name `"cql"`
- [x] 14.10 Write unit tests for CQL compile+embed, auto-create, type preservation, and compile-error abort

## 15. Integration Tests

- [ ] 15.1 Create fixture source directory with `package.json`, `ImplementationGuide.json`, one StructureDefinition, one ValueSet, one `.cql` file, and `.md` narrative files
- [ ] 15.2 Integration test: `rh publish build` on fixture — verify output structure, `.index.json`, narrative embed
- [ ] 15.3 Integration test: `rh publish pack` on fixture — verify `.tgz` is produced with `package/` prefix
- [ ] 15.4 Integration test: `rh publish lock` on fixture — verify `fhir-lock.json` structure
- [ ] 15.5 Integration test: `rh publish build` with `before_build = ["snapshot", "validate"]` in `publisher.toml`
- [ ] 15.6 Integration test: `rh publish check` detects IG sync mismatch without writing output

## 16. Documentation

- [ ] 16.1 Write `crates/rh-publisher/README.md` with usage examples and source directory layout
- [ ] 16.2 Add `publisher.toml` reference (all supported fields) to README
- [ ] 16.3 Add `fhir-lock.json` format documentation to README
- [ ] 16.4 Update `apps/rh-cli/README.md` with `rh publish` subcommand reference
