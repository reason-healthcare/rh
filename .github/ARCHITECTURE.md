# Architecture

## Workspace Layers

The Rust workspace is organized around a small foundation layer, generated FHIR
type crates, domain libraries, packaging/validation, and the CLI edge.

| Layer | Crates | Role |
|---|---|---|
| 0 | `rh-foundation` | Shared error types, package loading, snapshot generation, CLI helpers, WASM envelope utilities |
| 1 | `rh-hl7-fhir-r4-core`, `rh-hl7-fhir-r5-core` | Generated FHIR model crates built on foundation traits and metadata |
| 2 | `rh-codegen`, `rh-cql`, `rh-fhirpath`, `rh-fsh`, `rh-vcl` | Domain libraries for codegen, CQL, FHIRPath, FSH, and VCL |
| 3 | `rh-validator`, `rh-packager` | Validation and package assembly orchestration |
| 4 | `rh-cli` | Command-line application and integration edge |

## Crate Dependency Graph

```text
rh-foundation
├── rh-hl7-fhir-r4-core
│   ├── rh-fhirpath
│   │   └── rh-validator
│   │       └── rh-packager
│   └── rh-fsh
│       └── rh-packager
├── rh-hl7-fhir-r5-core
├── rh-codegen
├── rh-cql
│   └── rh-packager
└── rh-vcl

rh-cli depends on the public libraries it exposes:
rh-codegen, rh-cql, rh-fhirpath, rh-foundation, rh-fsh,
rh-hl7-fhir-r4-core, rh-packager, rh-validator, and rh-vcl.
```

The following dependency list is checked by `scripts/check-docs-sync.sh`
against `cargo metadata`.

<!-- docs-sync:crate-deps:start -->
```text
rh-cli: rh-codegen, rh-cql, rh-fhirpath, rh-foundation, rh-fsh, rh-hl7-fhir-r4-core, rh-packager, rh-validator, rh-vcl
rh-codegen: rh-foundation
rh-cql: rh-foundation
rh-fhirpath: rh-foundation, rh-hl7-fhir-r4-core
rh-foundation: -
rh-fsh: rh-foundation, rh-hl7-fhir-r4-core
rh-hl7-fhir-r4-core: rh-foundation
rh-hl7-fhir-r5-core: rh-foundation
rh-packager: rh-cql, rh-foundation, rh-fsh, rh-hl7-fhir-r4-core, rh-validator
rh-validator: rh-fhirpath, rh-foundation, rh-hl7-fhir-r4-core
rh-vcl: rh-foundation
```
<!-- docs-sync:crate-deps:end -->

## Key Crates

| Crate | Description |
|---|---|
| `rh-foundation` | Base utilities, error types, HTTP client wrappers, package loader, snapshot generation, in-memory caching, CLI helpers, and WASM result envelopes |
| `rh-codegen` | Generates Rust crates from FHIR StructureDefinitions and package archives |
| `rh-cql` | CQL-to-ELM compiler, evaluator, explain mode, source maps, and WASM facade |
| `rh-fhirpath` | Parser and evaluator for FHIRPath expressions, with R4 type metadata and WASM facade |
| `rh-fsh` | nom-based FSH parser and FHIR JSON exporter with rayon parallel export |
| `rh-validator` | Profile-based FHIR R4 validation with cached snapshots, compiled rules, FHIRPath invariant evaluation, and local terminology checks |
| `rh-vcl` | ValueSet Compose Language parser, translator, explainer, and WASM facade |
| `rh-packager` | FHIR Package assembler with built-in snapshot, validate, CQL, and FSH processors |
| `rh-hl7-fhir-r4-core` | Pre-generated R4 FHIR types, traits, bindings, extensions, and metadata |
| `rh-hl7-fhir-r5-core` | Pre-generated R5 FHIR types, traits, bindings, extensions, and metadata |
| `rh-cli` | Unified CLI for codegen, package download/build, validation, FHIRPath, CQL, FSH, VCL, and snapshots |

## WASM-Capable Matrix

| Crate | Feature | Root check | NPM package | Notes |
|---|---|---|---|---|
| `rh-foundation` | `wasm` | `cargo check -p rh-foundation --target wasm32-unknown-unknown --no-default-features --features wasm` | n/a | Shared panic hook and `WasmResult` infrastructure only |
| `rh-fhirpath` | `wasm` | `just wasm-build fhirpath <target>` | `@reasonhealth/fhirpath` | Exports parse/evaluate wrappers for node, web, and bundler targets |
| `rh-vcl` | `wasm` | `just wasm-build vcl <target>` | `@reasonhealth/vcl` | Exports parse, translate, validate, and explain wrappers |
| `rh-cql` | `wasm` | `just wasm-build cql <target>` | `@reasonhealth/cql` | Exports CQL compile and ELM evaluation wrappers |

Use `just wasm-check` for compile-only checks and `pnpm -r build && pnpm -r
test` for the npm package/build test path.

## JavaScript Packages And Playground

The `packages/` directory is a pnpm workspace containing the public
WebAssembly-backed packages:

- `packages/fhirpath` -> `@reasonhealth/fhirpath`
- `packages/vcl` -> `@reasonhealth/vcl`
- `packages/cql` -> `@reasonhealth/cql`

The `examples/playground` package is a private Vite app that exercises all
three npm packages and is deployed through the Pages workflow.

## Generated Crates

The generated R4/R5 crates are committed artifacts. Codegen changes must keep
regeneration hermetic:

- `just regen-r4` regenerates `crates/rh-hl7_fhir_r4_core`
- `just regen-r5` regenerates `crates/rh-hl7_fhir_r5_core`
- `just regen-check` regenerates both into temporary directories and fails if
  committed output is stale

Generated crate metadata is split by category under `src/metadata/` to keep
incremental builds smaller. Generated extension modules are committed for both
R4 and R5.
