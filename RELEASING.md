# Releasing Crates to crates.io

This document describes the end-to-end process for versioning, validating, and publishing crates from this workspace to [crates.io](https://crates.io).

## Prerequisites

- Rust 1.91 or later (`rustup update stable`)
- `cargo-audit` installed (`cargo install cargo-audit`)
- A crates.io API token configured locally (`cargo login`)
- All changes merged to `main` and CI passing

## Crate Inventory

| Crate | Version field | Notes |
|---|---|---|
| `rh-foundation` | `version.workspace` | Base layer |
| `rh-hl7-fhir-r4-core` | `version.workspace` | Generated FHIR types |
| `rh-codegen` | `version.workspace` | Code generation |
| `rh-cql` | `version.workspace` | CQL library |
| `rh-fhirpath` | `version.workspace` | FHIRPath library |
| `rh-vcl` | `version.workspace` | VCL library |
| `rh-validator` | standalone | Has its own version independent of the workspace |
| `rh-cli` | `version.workspace` | CLI binary (`rh`) |

## Publish Order

Crates must be published in dependency order. Crates on the same line have no dependency on each other and can be published in parallel.

```
rh-foundation
    ↓
rh-hl7-fhir-r4-core  rh-codegen  rh-cql
    ↓
rh-fhirpath  rh-vcl
    ↓
rh-validator
    ↓
rh-cli
```

## Step-by-Step Release Process

### 1. Determine the new version

Most crates inherit their version from `[workspace.package]` in the root `Cargo.toml`. `rh-validator` is versioned independently.

Version format prior to a stable release: `0.x.0-beta.N`

Check current versions at any time:

```bash
just show-versions
```

### 2. Update version numbers

Use the version management script — it updates the workspace version and all internal path dependency version specifiers in one pass:

```bash
# Bump all workspace crates (everything except rh-validator)
just bump-version 0.2.0-beta.1

# Bump rh-validator's standalone version
just bump-validator-version 0.3.0-beta.1

# Or bump both at once
just bump-all-versions 0.2.0-beta.1 0.3.0-beta.1
```

The script updates:
- `version` in `[workspace.package]` in the root `Cargo.toml`
- All `{ path = "...", version = "..." }` internal dependency specifiers across every crate
- `rh-validator`'s own `version` field and its entry in `apps/rh-cli/Cargo.toml`

See [`scripts/bump-version`](scripts/bump-version) for the full implementation.

### 3. Run the full quality gate

```bash
just check
```

This runs formatting check, clippy, all tests, example tests, and `cargo audit` in sequence. All must pass before proceeding.

```bash
# If you need to fix formatting first
just fmt

# Re-run the gate
just check
```

### 4. Run dry-run publishes for every crate (in order)

Dry-run catches missing fields, broken path deps without versions, and packaging errors without touching the registry.

```bash
cargo publish --dry-run -p rh-foundation
cargo publish --dry-run -p rh-hl7-fhir-r4-core
cargo publish --dry-run -p rh-codegen
cargo publish --dry-run -p rh-cql
cargo publish --dry-run -p rh-fhirpath
cargo publish --dry-run -p rh-vcl
cargo publish --dry-run -p rh-validator
cargo publish --dry-run -p rh-cli
```

All invocations must exit 0 before proceeding.

### 5. Commit and tag the release

```bash
git add Cargo.toml crates/*/Cargo.toml apps/*/Cargo.toml
git commit -m "chore: bump versions to 0.x.0-beta.N"
git tag v0.x.0-beta.N
git push origin main --tags
```

### 6. Publish to crates.io

Publish each crate in dependency order. Wait for the previous crate to appear in the registry before publishing dependents (usually takes 10–30 seconds).

```bash
cargo publish -p rh-foundation
sleep 30

cargo publish -p rh-hl7-fhir-r4-core
cargo publish -p rh-codegen
cargo publish -p rh-cql
sleep 30

cargo publish -p rh-fhirpath
cargo publish -p rh-vcl
sleep 30

cargo publish -p rh-validator
sleep 30

cargo publish -p rh-cli
```

If a publish fails with "crate version not found" for a dependency, wait a bit longer and retry — the registry index can take a minute to update.

### 7. Verify the release

Confirm each crate is visible on crates.io:

```
https://crates.io/crates/rh-foundation
https://crates.io/crates/rh-hl7-fhir-r4-core
https://crates.io/crates/rh-codegen
https://crates.io/crates/rh-cql
https://crates.io/crates/rh-fhirpath
https://crates.io/crates/rh-vcl
https://crates.io/crates/rh-validator
https://crates.io/crates/rh-cli
```

## WASM Builds

`rh-fhirpath` and `rh-vcl` also ship as WebAssembly packages. Build them after the Rust crates are published:

```bash
# Build WASM for both crates
just wasm

# Or individually
cd crates/rh-fhirpath && just wasm
cd crates/rh-vcl && just wasm
```

See [crates/rh-fhirpath/WASM_BUILD.md](crates/rh-fhirpath/WASM_BUILD.md) and [crates/rh-vcl/WASM_BUILD.md](crates/rh-vcl/WASM_BUILD.md) for full WASM build and deployment details.

## Troubleshooting

**`cargo publish` fails with "already uploaded"**
The version was already published. Bump the version and start from step 2.

**`cargo publish` fails with "dependency ... does not have a version"**
A workspace-internal `path` dependency is missing its `version` field. Add `version = "x.y.z"` matching the referenced crate's version and retry.

**Dry-run passes but live publish fails with "crate version not found"**
A dependency was just published and the index hasn't propagated yet. Wait 30–60 seconds and retry.

**`cargo audit` finds vulnerabilities**
Address the advisory before releasing. Run `cargo update` to pull in patched versions, or add an exception to `audit.toml` if the advisory is not applicable and document the rationale.
