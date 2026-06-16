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
| `rh-hl7-fhir-r4-core` | `version.workspace` | Generated FHIR R4 types |
| `rh-hl7-fhir-r5-core` | `version.workspace` | Generated FHIR R5 types |
| `rh-codegen` | `version.workspace` | Code generation |
| `rh-cql` | `version.workspace` | CQL library |
| `rh-fsh` | `version.workspace` | FSH compiler |
| `rh-fhirpath` | `version.workspace` | FHIRPath library |
| `rh-vcl` | `version.workspace` | VCL library |
| `rh-packager` | `version.workspace` | Package tooling |
| `rh-validator` | standalone | Standalone version; normally bumped alongside all other crates |
| `rh-cli` | `version.workspace` | CLI binary (`rh`) |

## Publish Order

Crates must be published in dependency order. Crates on the same line have no dependency on each other and can be published in parallel.

```
rh-foundation
    ↓
rh-hl7-fhir-r4-core  rh-hl7-fhir-r5-core  rh-codegen  rh-cql  rh-fsh
    ↓
rh-fhirpath  rh-vcl
    ↓
rh-validator
    ↓
rh-packager
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

Bump all crates to the same version in one command:

```bash
just bump-version 0.3.0-beta.1
```

This updates the workspace version, `rh-validator`'s standalone version, and all internal path dependency version specifiers across every crate.

See [`scripts/bump-version`](scripts/bump-version) for the full implementation.

> **Releasing a single crate?** See [Releasing a single crate](#releasing-a-single-crate) below.

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
cargo publish --dry-run -p rh-hl7-fhir-r5-core
cargo publish --dry-run -p rh-codegen
cargo publish --dry-run -p rh-cql
cargo publish --dry-run -p rh-fsh
cargo publish --dry-run -p rh-fhirpath
cargo publish --dry-run -p rh-vcl
cargo publish --dry-run -p rh-validator
cargo publish --dry-run -p rh-packager
cargo publish --dry-run -p rh-cli
```

All invocations must exit 0 before proceeding.

### 5. Commit and tag the release

```bash
git add Cargo.toml Cargo.lock crates/*/Cargo.toml apps/*/Cargo.toml
git commit -m "chore: bump versions to 0.x.0-beta.N"
git tag v0.x.0-beta.N
git push origin main --tags
```

Pushing the tag triggers the `release.yml` workflow, which builds binaries for all platforms and creates a **draft** GitHub Release. Once all build jobs succeed, review and publish the draft at:
```
https://github.com/reason-healthcare/rh/releases
```

### 6. Publish to crates.io

Publish each crate in dependency order. Wait for the previous crate to appear in the registry before publishing dependents (usually takes 10–30 seconds).

```bash
cargo publish -p rh-foundation
sleep 30

cargo publish -p rh-hl7-fhir-r4-core
cargo publish -p rh-hl7-fhir-r5-core
cargo publish -p rh-codegen
cargo publish -p rh-cql
cargo publish -p rh-fsh
sleep 30

cargo publish -p rh-fhirpath
cargo publish -p rh-vcl
sleep 30

cargo publish -p rh-validator
sleep 30

cargo publish -p rh-packager
sleep 30

cargo publish -p rh-cli
```

If a publish fails with "crate version not found" for a dependency, wait a bit longer and retry — the registry index can take a minute to update.

### 7. Verify the release

Confirm each crate is visible on crates.io:

```
https://crates.io/crates/rh-foundation
https://crates.io/crates/rh-hl7-fhir-r4-core
https://crates.io/crates/rh-hl7-fhir-r5-core
https://crates.io/crates/rh-codegen
https://crates.io/crates/rh-cql
https://crates.io/crates/rh-fsh
https://crates.io/crates/rh-fhirpath
https://crates.io/crates/rh-vcl
https://crates.io/crates/rh-packager
https://crates.io/crates/rh-validator
https://crates.io/crates/rh-cli
```

## Homebrew Tap

The Homebrew formula lives in the separate `reason-healthcare/homebrew-rh` tap repository and automatically updates when a GitHub Release is published. The `publish-homebrew` workflow:

1. Downloads the macOS and Linux release assets
2. Computes SHA256 checksums for each binary
3. Generates an updated `Formula/rh.rb` with version, URLs, and checksums
4. Commits and pushes to the `reason-healthcare/homebrew-rh` repository

The workflow only runs for stable releases (skips `-beta` and `-alpha` tags).

**Secrets required:**
- `HOMEBREW_TAP_TOKEN` — a PAT with `repo` scope, stored in the main repo's GitHub Actions secrets

**First-time setup:**
1. Create the `reason-healthcare/homebrew-rh` repository on GitHub (empty, with a `Formula/` directory)
2. Generate a PAT with `repo` scope for pushing to the tap
3. Add it as `HOMEBREW_TAP_TOKEN` in the main repo's GitHub Actions secrets
4. Copy the template formula from `homebrew-tap/Formula/rh.rb` as the initial formula

**Manual formula update** (if needed):
```bash
git clone https://github.com/reason-healthcare/homebrew-rh
cd homebrew-rh
# Edit Formula/rh.rb with version, URLs, and SHA256 checksums
git add Formula/rh.rb
git commit -m "rh x.y.z"
git push
```

**User install:**
```bash
brew tap reason-healthcare/rh
brew install rh
```

## Chocolatey Package

The Chocolatey package auto-publishes to the community feed when a GitHub Release is published. The `publish-choco` workflow:

1. Downloads the Windows release asset
2. Computes SHA256 checksum
3. Updates `choco/rh.nuspec` version and `choco/tools/chocolateyinstall.ps1` checksum
4. Packs and pushes the `.nupkg` to chocolatey.org

The workflow only runs for stable releases (skips `-beta` and `-alpha` tags).

**Secrets required:**
- `CHOCOLATEY_API_KEY` — stored in the main repo's GitHub Actions secrets

**Manual package update** (if needed):
```powershell
cd choco
# Update version in rh.nuspec and checksum in tools/chocolateyinstall.ps1
choco pack
choco push rh.X.Y.Z.nupkg --source https://push.chocolatey.org/ --api-key <API_KEY>
```

**User install:**
```powershell
choco install rh
```

## Linux Install Script

The `scripts/install-rh.sh` script provides a `curl | sh` install experience for Linux and macOS. It auto-detects OS and architecture, downloads the correct binary from the latest GitHub Release, and installs it to `/usr/local/bin`.

The script is attached to each GitHub Release as a release asset.

**Usage:**
```bash
# Install latest version
curl -fsSL https://raw.githubusercontent.com/reason-healthcare/rh/main/scripts/install-rh.sh | sh

# Install specific version
curl -fsSL https://raw.githubusercontent.com/reason-healthcare/rh/main/scripts/install-rh.sh | sh -s -- -v 1.0.0

# Install to custom directory
curl -fsSL https://raw.githubusercontent.com/reason-healthcare/rh/main/scripts/install-rh.sh | sh -s -- -d ~/.local/bin
```

## WASM Builds

`rh-fhirpath`, `rh-vcl`, and `rh-cql` also ship as WebAssembly-backed NPM packages.
The local npm publish process is documented in [`packages/RELEASE.md`](packages/RELEASE.md).

Build them after the Rust crates are published:

```bash
# Build WASM for all npm-backed crates
just wasm

# Or individually
cd crates/rh-fhirpath && just wasm
cd crates/rh-vcl && just wasm
cd crates/rh-cql && just wasm
```

Before publishing NPM packages, run:

```bash
pnpm -r build
pnpm -r test
pnpm -r pack:dry-run
```

Then follow [`packages/RELEASE.md`](packages/RELEASE.md) to publish
`@reasonhealth/fhirpath`, `@reasonhealth/vcl`, and
`@reasonhealth/cql` locally.

## Generated FHIR Crate Release Policy

The R4 and R5 crates are generated artifacts and share the workspace version.
Any codegen change that alters committed generated output must be treated as a
generated-crate release change:

1. Regenerate both committed crates:
   ```bash
   just regen-r4
   just regen-r5
   ```
2. Run the drift gate:
   ```bash
   just regen-check
   ```
3. Build and test generated crates:
   ```bash
   cargo test -p rh-hl7-fhir-r4-core
   cargo test -p rh-hl7-fhir-r5-core
   ```
4. Include the generated output diff in the release commit and bump the
   workspace version before publishing.

If a codegen change is internal-only and `just regen-check` reports no diff,
the generated crates do not need special release treatment beyond the normal
workspace publish order.

## Releasing a single crate

Occasionally you may need to release one crate at a different version — for example, a patch fix in `rh-validator` without touching the rest of the workspace. Use the targeted recipes:

```bash
# Bump only rh-validator (leaves all workspace crates unchanged)
just bump-validator-version 0.2.1

# Bump only workspace crates (leaves rh-validator unchanged)
just bump-workspace-version 0.1.1
```

Then publish only the affected crate(s) and their dependents in dependency order. Dependents that weren't re-versioned do **not** need to be re-published unless their `version` specifier for the changed crate no longer satisfies the new version.

---

## Troubleshooting

**`cargo publish` fails with "already uploaded"**
The version was already published. Bump the version and start from step 2.

**`cargo publish` fails with "dependency ... does not have a version"**
A workspace-internal `path` dependency is missing its `version` field. Add `version = "x.y.z"` matching the referenced crate's version and retry.

**Dry-run passes but live publish fails with "crate version not found"**
A dependency was just published and the index hasn't propagated yet. Wait 30–60 seconds and retry.

**`cargo audit` finds vulnerabilities**
Address the advisory before releasing. Run `cargo update` to pull in patched versions, or add an exception to `audit.toml` if the advisory is not applicable and document the rationale.
