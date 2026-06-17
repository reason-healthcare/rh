# Release Guide: Rust Health (rh)

This is the **single source of truth** for releasing Rust Health across all distribution channels:
- **Crates.io** (Rust libraries and CLI)
- **NPM** (WebAssembly packages)
- **Chocolatey** (Windows)
- **Homebrew** (macOS/Linux)
- **Docker** (Containers)
- **Nix** (NixOS packages)
- **GitHub Releases** (Binaries and artifacts)

---

## Release Overview

A typical release follows this sequence:

```
1. Prepare and validate locally
2. Bump versions
3. Run quality gates
4. Tag release on GitHub
   ↓ (GitHub Actions triggered)
   ├→ Publish Rust crates to crates.io
   ├→ Build and upload binaries
   ├→ Create GitHub Release (draft)
   ├→ Update Homebrew tap
   ├→ Update Chocolatey package
   ├→ Update Docker image
   └→ Publish npm packages (manual)
5. Auto-update Nix (no action needed)
```

---

## Prerequisites

Before starting a release:

```bash
# Rust toolchain
rustup update stable

# Tools
cargo install cargo-audit
npm install -g pnpm

# Authentication
cargo login                          # crates.io API token
gh auth login                        # GitHub CLI (auto-configured)

# Verify
cargo --version && npm --version && gh --version
```

### Required Secrets (in GitHub Actions)
- `HOMEBREW_TAP_TOKEN` — For publishing to homebrew-rh tap
- `CHOCOLATEY_API_KEY` — For publishing to chocolatey.org
- Docker automatically uses GHCR

---

## Step-by-Step Release Process

### 1. Prepare Your Code

Ensure all changes are merged to `main` and CI is passing:

```bash
git fetch origin
git checkout main
git pull origin main

# Verify CI status
gh run list --limit 1 --status completed
```

### 2. Determine the New Version

Check current versions:

```bash
just show-versions
```

Version format:
- **Stable**: `0.2.1`, `1.0.0`, `2.3.4`
- **Pre-release**: `0.3.0-beta.1`, `0.3.0-rc.1`, `0.3.0-alpha.1`

### 3. Bump All Versions

Update workspace version and all dependencies in one command:

```bash
just bump-version 0.3.0
```

This updates:
- `Cargo.toml` workspace version
- `rh-validator`'s standalone version
- All internal `path` dependency versions
- `flake.nix` version

Verify:

```bash
git diff Cargo.toml flake.nix
just show-versions
```

### 4. Run Quality Gates

```bash
# Format check and fix
just fmt
just fmt-check

# Run full gate (format, clippy, tests, audit)
just check
```

All checks must pass. If `cargo audit` reports issues:
- Fix dependencies with `cargo update`
- Or document exceptions in `audit.toml` with rationale

### 5. Test Dry-Run Publishes

Test crate.io publishing without uploading:

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

All must exit 0. If any fail:
- Fix the issue
- Re-run `just check`
- Re-run dry-runs

### 6. Commit and Tag

```bash
# Stage all version changes
git add Cargo.toml Cargo.lock crates/*/Cargo.toml apps/*/Cargo.toml flake.nix

# Commit
git commit -m "chore: bump versions to 0.3.0"

# Tag release
git tag v0.3.0

# Push both
git push origin main --tags
```

This triggers the `release.yml` GitHub Actions workflow.

### 7. Publish to Crates.io

Wait for the tag push and monitor the workflow. Once tag CI completes:

```bash
# Publish in dependency order, waiting between each
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

If a publish fails with "dependency not found", wait longer and retry (registry index sync).

### 8. Verify Crates.io

All crates should be visible within minutes:

```bash
# Check each
curl -s https://crates.io/api/v1/crates/rh-foundation | jq '.crate.newest_version'
curl -s https://crates.io/api/v1/crates/rh-cli | jq '.crate.newest_version'
# ... etc
```

### 9. Monitor GitHub Release Build

The `release.yml` workflow creates a **draft** GitHub Release:

1. **Wait for all jobs to complete** (15-30 minutes typically)
2. **Review the draft** at: https://github.com/reason-healthcare/rh/releases
3. **Publish the draft** when ready

The release draft contains:
- Pre-built binaries (Linux, macOS, Windows, musl)
- Install script
- Build artifacts

### 10. Verify Binaries Work

Once published:

```bash
# Download and test from release page
curl -L https://github.com/reason-healthcare/rh/releases/download/v0.3.0/rh-x86_64-unknown-linux-gnu -o /tmp/rh
chmod +x /tmp/rh
/tmp/rh --version
```

---

## Automatic Distribution Updates

After tagging the release, the following happen **automatically** (no action needed):

### Homebrew (macOS/Linux)
- `publish-homebrew` workflow runs
- Updates `reason-healthcare/homebrew-rh` tap
- Downloads release binaries
- Computes SHA256 checksums
- Updates `Formula/rh.rb`
- Users can then: `brew tap reason-healthcare/rh && brew install rh`

**Status**: Automatic ✅

### Chocolatey (Windows)
- `publish-choco` workflow runs
- Downloads Windows binary
- Updates `choco/rh.nuspec` version
- Updates `choco/tools/chocolateyinstall.ps1` checksum
- Pushes to chocolatey.org
- Users can then: `choco install rh`

**Status**: Automatic ✅

### Docker
- GitHub Actions automatically builds and pushes to ghcr.io
- Tagged as: `ghcr.io/reason-healthcare/rh:v0.3.0`
- Also tagged as `latest`
- Users can then: `docker pull ghcr.io/reason-healthcare/rh:latest`

**Status**: Automatic ✅

### Nix (NixOS packages)
- Nixpkgs bot detects new GitHub tag
- Creates PR to nixpkgs updating version
- PR auto-merges or is reviewed by maintainers
- Users can then: `nix shell nixpkgs#rh`

**Status**: Automatic ✅ (after initial PR #532525 merges)

### Install Script
- `scripts/install-rh.sh` is attached to release
- Users can then: `curl -fsSL https://raw.githubusercontent.com/reason-healthcare/rh/main/scripts/install-rh.sh | sh`

**Status**: Automatic ✅

---

## Manual Step: NPM Packages

The WebAssembly packages must be published manually. **Do this after crates.io publishes:**

### Build WASM

```bash
# Build all WASM packages
just wasm

# Or individually
cd crates/rh-fhirpath && just wasm
cd crates/rh-vcl && just wasm
cd crates/rh-cql && just wasm
```

### Test Packages

```bash
cd packages
pnpm -r build
pnpm -r test
pnpm -r pack:dry-run
```

### Publish to NPM

Follow [`packages/RELEASE.md`](packages/RELEASE.md) to publish:
- `@reasonhealth/fhirpath`
- `@reasonhealth/vcl`
- `@reasonhealth/cql`

---

## Verify the Complete Release

Once everything is published:

```bash
# Crates.io
cargo search rh-cli | head -5

# NPM
npm view @reasonhealth/fhirpath versions | tail -5

# GitHub Release
gh release view v0.3.0 --repo reason-healthcare/rh

# Docker
docker pull ghcr.io/reason-healthcare/rh:v0.3.0 && docker run ghcr.io/reason-healthcare/rh:v0.3.0 --version

# Nix (after PR merges)
nix shell github:reason-healthcare/rh -- rh --version
# or (after nixpkgs merge)
nix shell nixpkgs#rh -- rh --version
```

---

## Generated FHIR Crate Release Policy

The R4 and R5 crates are generated artifacts. If a codegen change alters their output:

```bash
# 1. Regenerate
just regen-r4
just regen-r5

# 2. Verify no drift
just regen-check

# 3. Test
cargo test -p rh-hl7-fhir-r4-core
cargo test -p rh-hl7-fhir-r5-core

# 4. Include generated diff in release commit
git add crates/rh-hl7-fhir-*/src/

# 5. Proceed with normal release
```

---

## Releasing a Single Crate

To release only one crate at a different version (e.g., `rh-validator` patch):

```bash
# Bump only that crate
just bump-validator-version 0.2.1

# Test and commit
cargo test -p rh-validator
git add crates/rh-validator/Cargo.toml apps/rh-cli/Cargo.toml
git commit -m "rh-validator: bump to 0.2.1"
git tag rh-validator-0.2.1

# Publish only that crate and dependents
cargo publish -p rh-validator
sleep 30
cargo publish -p rh-packager
sleep 30
cargo publish -p rh-cli
```

---

## Troubleshooting

### `cargo publish` fails with "already uploaded"
- The version was already published
- Bump the version and start from Step 3

### `cargo publish` fails with "dependency ... does not have a version"
- A path dependency is missing its `version` field
- Add `version = "x.y.z"` to the dependency and retry

### `cargo publish` fails with "crate version not found"
- A dependency was just published and registry index hasn't synced
- Wait 30-60 seconds and retry

### `cargo audit` finds vulnerabilities
- Fix by updating dependencies: `cargo update`
- Or add exception to `audit.toml` with rationale
- Cannot proceed without resolving or documenting

### Dry-run passes but live publish fails
- Usually a timing issue with registry sync
- Wait and retry
- Contact crates.io support if persists

### GitHub Release draft doesn't appear
- Check the `release.yml` workflow status
- Verify binaries built successfully in CI
- Common issue: binary not stripped (check build logs)

---

## Distribution Timeline

| Step | Time | Auto? | Required? |
|------|------|-------|-----------|
| Code preparation | Minutes | No | Yes |
| Version bumping | Seconds | No | Yes |
| Quality gates | 5-10 min | No | Yes |
| Dry-run publishes | 2-3 min | No | Yes |
| Commit and tag | Seconds | No | Yes |
| Binary builds | 15-30 min | Yes | Yes |
| crates.io publish | 5-15 min | No | Yes |
| Homebrew tap | 5-10 min | Yes | Yes |
| Chocolatey | 5-10 min | Yes | Yes |
| Docker push | 5-10 min | Yes | Yes |
| Nix update | 1-2 weeks | Yes | Yes (after PR #532525 merges) |
| NPM packages | 5-10 min | No | Optional |

**Total manual effort**: ~30-45 minutes (mostly waiting)
**Post-merge effort**: ~15 minutes (NPM packages only)

---

## Quick Checklist

```bash
# Before releasing
☐ git fetch && git checkout main && git pull
☐ All PRs merged and CI passing
☐ Decide new version

# Release
☐ just bump-version X.Y.Z
☐ just check (all pass)
☐ cargo publish --dry-run (all pass)
☐ git add && git commit && git tag && git push --tags
☐ Wait for release.yml to complete
☐ Verify GitHub Release draft
☐ Publish release draft

# Post-release
☐ Verify crates.io (all visible)
☐ Verify Docker (image pulled successfully)
☐ Test binaries from release page
☐ Publish NPM packages (follow packages/RELEASE.md)
☐ Monitor Nix PR auto-update (if nixpkgs PR #532525 merged)
```

---

## Documentation

- **Crates**: See [Cargo.toml](Cargo.toml) and [apps/rh-cli/README.md](apps/rh-cli/README.md)
- **NPM**: See [packages/RELEASE.md](packages/RELEASE.md)
- **Homebrew**: [reason-healthcare/homebrew-rh](https://github.com/reason-healthcare/homebrew-rh)
- **Nix**: PR #532525 to [NixOS/nixpkgs](https://github.com/NixOS/nixpkgs)
- **Docker**: [Dockerfile](Dockerfile)

---

## Summary

This is a **fully automated** release pipeline. Your responsibility is:

1. **Prepare** code and decide version
2. **Bump** versions with one command
3. **Tag** the release
4. **Publish** NPM packages (manual, but simple)

Everything else happens automatically:
- ✅ crates.io publish
- ✅ GitHub Releases
- ✅ Homebrew tap update
- ✅ Chocolatey publish
- ✅ Docker push
- ✅ Nix update (after initial PR merges)

**Total time per release**: 30-45 minutes (mostly CI waiting)
**Ongoing maintenance**: Essentially zero
