# Developer Guide

This guide covers building, testing, and working with the **rh** monorepo. For contribution guidelines see [CONTRIBUTING.md](CONTRIBUTING.md). For architecture and design decisions see [.github/ARCHITECTURE.md](.github/ARCHITECTURE.md) and [.github/CODING_STANDARDS.md](.github/CODING_STANDARDS.md).

## Workspace Structure

```
.
├── Cargo.toml              # Workspace root
├── crates/                 # Library crates
│   ├── rh-codegen/            # FHIR code generation from StructureDefinitions
│   ├── rh-cql/                # CQL compiler and evaluator
│   ├── rh-foundation/         # Foundation utilities (errors, HTTP, I/O, loader, snapshot)
│   ├── rh-fhirpath/           # FHIRPath expression parser and evaluator
│   ├── rh-fsh/                # FHIR Shorthand (FSH) to FHIR JSON compiler
│   ├── rh-packager/           # FHIR package assembler
│   ├── rh-validator/          # Hybrid FHIR R4 validator
│   ├── rh-vcl/                # ValueSet Compose Language (VCL) parser and translator
│   ├── rh-hl7_fhir_r4_core/   # Generated R4 FHIR types
│   └── rh-hl7_fhir_r5_core/   # Generated R5 FHIR types
├── apps/
│   └── rh-cli/                # Unified CLI application
├── packages/                  # WebAssembly-backed npm packages
│   ├── fhirpath/              # @reasonhealth/fhirpath
│   ├── vcl/                   # @reasonhealth/vcl
│   └── cql/                   # @reasonhealth/cql
├── examples/
│   └── playground/            # Vite playground for the npm packages
├── justfile                   # Task runner commands
└── setup.sh                   # Development setup script
```

## Prerequisites

- Rust 1.91 or later (`rustup` recommended)
- [just](https://github.com/casey/just) task runner (optional but recommended)

## Setup

```bash
./setup.sh  # or: just setup
```

This will:
- Verify the Rust toolchain (≥ 1.91) and active `rustup` toolchain
- Install `clippy`, `rustfmt`, `cargo-audit`, `cargo-watch`, and `cargo-nextest`
- Run initial formatting and checks

Skip optional tool installs (useful for CI / offline):
```bash
SKIP_INSTALL=1 ./setup.sh
```

After setup:
```bash
just check   # full gate: fmt-check, lint, test, audit
rh --help
```

## Building

```bash
cargo build                      # all workspace crates
cargo build -p <package-name>    # single crate
```

## Testing

```bash
cargo test                       # all workspace tests
cargo test -p <package-name>     # single crate
just test-fhir                   # quick FHIR validation suite (5 cases)
just test-fhir-50                # extended (50 cases)
just test-fhir-all               # full suite (~570 cases)
```

## Linting & Formatting

```bash
cargo fmt --all                  # apply formatting
cargo fmt --all -- --check       # check only (used in CI)
cargo clippy --workspace --all-targets --all-features -- -D warnings
just check                       # runs fmt-check, lint, test, audit — run before committing
```

## Running the CLI from source

```bash
cargo run -p rh-cli -- --help
cargo run -p rh-cli -- fhirpath eval 'Patient.name.given' -d patient.json
cargo run -p rh-cli -- validate resource --input patient.json
```

## WASM and npm Packages

The WebAssembly packages live in the pnpm workspace under `packages/`:

```bash
pnpm install
pnpm -r build
pnpm -r test
```

Rust compile-only checks (no wasm-pack required):
```bash
just wasm-check
```

Build a specific package for a specific target:
```bash
just wasm-build fhirpath all
just wasm-build vcl all
just wasm-build cql all
```

The interactive playground is documented in [examples/playground/README.md](examples/playground/README.md). The npm publish process is documented in [packages/RELEASE.md](packages/RELEASE.md).

## Dependency Management

### Workspace Dependencies

Prefer adding shared dependencies to the root `Cargo.toml`:

```toml
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"
```

Reference them in individual crates:

```toml
[dependencies]
serde = { workspace = true }
```

### Crate-specific Dependencies

Add directly to the crate's own `Cargo.toml`:

```toml
[dependencies]
regex = "1.0"
```

## Docs Drift Checks

After changes to CLI commands, workspace dependencies, or architecture docs, run:

```bash
just docs-sync
```

This verifies that the architecture dependency diagram and CLI help text in the repository docs stay in sync with `cargo metadata` and `rh --help`.

## Generated FHIR Crates

```bash
just regen-r4       # regenerate rh-hl7_fhir_r4_core
just regen-r5       # regenerate rh-hl7_fhir_r5_core
just regen-check    # drift-check committed generated crates
```

## Release Management

See [RELEASING.md](RELEASING.md) for the full release workflow, version bump commands, and publish order.
