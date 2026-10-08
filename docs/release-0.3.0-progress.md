# RH 0.3.0 release progress

Started: 2026-10-08. Published: 2026-10-08. Version: `0.3.0`.

[RH 0.3.0](https://github.com/reason-healthcare/rh/releases/tag/v0.3.0) is public.
All 12 Rust crates, four npm packages, CLI archives, Docker, Homebrew, and
Chocolatey are published and verified. The release checklist is complete.
Analytics adoption is the next separate change and has not started.

## Scope

Release from merged RH main, starting at `29b63f5a`, with all changes since
`v0.2.8`: executable bundles, CPG execution and SDC extraction, compiler and
analytics artifact documentation, and pinned terminology snapshots. Companion
Analytics terminology support is merged at `5ed0ff1`.

Preparation used the isolated `codex/release-0.3.0` branch. Publication used a
clean checkout of release commit `6654ee9df04cbcde99ca0d7b276c36369b5285b4`.
Original dirty checkouts are preserved. Analytics adoption has a separate clean worktree on
`codex/rh-0.3.0-adoption`; it will consume the released CLI and stable JSON
artifacts without linking compiler internals.

## Gates and progress

| Work item | Status | Evidence / next step |
|---|---|---|
| Merge terminology changes | Complete | RH #74 and Analytics #1 merged |
| Verify merged-main hosted CI | Complete | RH `29b63f5a`: CI run `37806843073` and CodeQL run `37806843487` succeeded |
| Inventory changes and release surfaces | Complete | 12 Rust crates and four npm packages; CPG added to release inventories and publish order |
| Align Rust/npm versions and lockfiles | Complete | All packages target `0.3.0`; external Cargo entries and pnpm resolution unchanged |
| Release notes and migration guidance | Complete | Includes terminology errors, public API changes, fixture scope, and clinical limits |
| Validate generated artifacts | Complete | R4 191 tests and R5 247 tests passed; regeneration drift check passed; only generated manifest/README versions changed |
| Local repository gates | Complete | Formatting, strict Clippy, 2,861 tests / 15 ignored, example builds, docs sync, and dependency audit passed |
| WASM/npm gates | Complete | All four packages and playground built; 32 JavaScript tests and four npm pack dry-runs passed |
| Rust package gate | Complete | Workspace publication dry-run verified all 12 packaged crates; uploads were explicitly skipped |
| Validate built CLI against Analytics | Complete | Optimized CLI was archived, extracted, and reported `rh 0.3.0`; all 78 contract checks and both examples passed |
| Prepare release PR | Complete | [PR #75](https://github.com/reason-healthcare/rh/pull/75), head `16f42895`; independent diff review and all local artifact gates passed |
| Verify release-head hosted CI | Complete | PR head `16f42895`: CI `37813572047` and CodeQL `37813568018` succeeded; no review requests or unresolved threads |
| Tag/build/distribute Rust and CLI 0.3.0 | Complete | GitHub release, all 12 crates, four CLI platforms, Docker, Homebrew, Chocolatey, and installer verified |
| Publish npm packages | Complete | User published all four packages; public `0.3.0` versions, `latest` tags, package layouts, and exact-version consumer tests verified |
| Pin released RH in Analytics | Pending | Release assets and checksums are ready; next change pins the version and validates CI/examples |

## Release boundaries

Terminology snapshots cover complete finite FHIR expansions. Native evaluation
and Analytics execution remain offline after preparation. Synthetic fixtures
require explicit opt-in. This release does not establish full CMS122 clinical
conformance or generic CQL-to-SQL equivalence; existing limitations remain
documented.

Local checks, packaged-artifact checks, hosted CI, and publication are separate
evidence. A version bump or tag does not establish successful distribution.

The release workflow now generates `SHA256SUMS`, uses locked Cargo builds, and
marks stable tags appropriately. Tag push still publishes the versioned Docker
image and `latest` before the GitHub draft is published; pre-tag validation must
therefore be complete first. Analytics adoption waits for actual release assets
and their checksums.

The first workspace gate caught four golden ELM fixtures with the old translator
version. They were regenerated through the documented golden-test command and
checked structurally: only `translatorVersion` changed from `0.2.8` to `0.3.0`.
The full gate rerun passed with those release metadata updates. Its existing R5
diagnostic remains 1,488 pass / 23 fail / 99 not implemented / 8 skipped under
`--no-baseline-check`; that diagnostic is not a full-conformance gate.

Registry preflight found no existing `0.3.0` versions for the 12 Rust crates or
four npm packages. Availability was rechecked before uploads; the completed
publication and verification results are recorded below.

The local macOS ARM64 candidate archive passed the Analytics integration checks
against merged Analytics source `5ed0ff1`. Native and SQL membership selected
exactly p1/p2 with matching snapshot provenance; the terminology-free example
also passed. The golden terminology files remain byte-identical across both
repositories. This evidence is for a locally built candidate, not downloaded
GitHub release assets. Subsequent hosted-binary and distribution verification
is complete and recorded below.

Package verification and workspace builds initially shared a Cargo target cache.
After the package dry-run passed, a repeated docs-sync build encountered mixed
packaged/workspace FHIRPath artifacts. The release guide now gives package
verification a separate target directory. Rebuilding the affected workspace
crate resolved the cache conflict, and a fresh docs-sync check passed without
source-code changes.

## Remaining release checklist — before Analytics adoption

This checklist follows [RELEASE.md](../RELEASE.md), the
[release workflow](../.github/workflows/release.yml), and the
[Homebrew](../.github/workflows/publish-homebrew.yml) and
[Chocolatey](../.github/workflows/publish-choco.yml) workflows. The numbered
release guide controls the sequence: verify crates.io before publishing the
GitHub draft, then publish npm packages. The workflow's job summary lists some
of these steps in a different order.

The preparation and release checklist below are complete. npm publication was
performed by the user and verified against the public registry. Check each item only after recording its result. This checklist does
not itself merge, tag, publish, or change Analytics. Although npm is optional
in the generic guide, all four npm packages are included in this 0.3.0 release.

### 1. Finish PR #75 and identify the release commit

- [x] Recheck PR #75 at its current head: all CI and CodeQL jobs complete
  successfully, review feedback addressed, and the PR ready and mergeable.
  Record the reviewed head and check-run links; an earlier passing head is
  insufficient if the branch changes.
- [x] Merge PR #75 using the verified head. Fetch `upstream/main` into a clean,
  isolated checkout and record the resulting main commit as the release SHA.
- [x] Verify successful main-push CI and CodeQL for that exact release SHA.
  Confirm it contains the reviewed release changes and that all 12 Rust crates,
  four npm packages, generated metadata, and changelog agree on `0.3.0`.
  Update the changelog date if publication moves to another day.
- [x] Confirm the guide's local gates cover the final release contents:
  `just check`, `just docs-sync`, generated-crate drift/tests, WASM/npm builds,
  tests and package dry-runs. Rerun affected validation after any changes.
  Verify the committed source with
  `cargo publish --workspace --dry-run --locked --target-dir target/release-packaging`;
  do not use `--allow-dirty` for this final package check.

### 2. Complete publication preflight

- [x] Verify Rust >= 1.91, Node >= 24, pnpm 10.11.0, `wasm-pack`, the WASM
  target, and authenticated GitHub/crates.io publishing identities. Access to
  all Rust crates was established by successful publication.
- [x] User: authenticate npm and confirm publish access to `@reasonhealth`.
- [x] Confirm `HOMEBREW_TAP_TOKEN` and `CHOCOLATEY_API_KEY` are configured and
  the release workflow has its required `contents: write` and `packages: write`
  permissions. Record readiness without recording credentials.
- [x] Recheck that the remote `v0.3.0` tag and release do not already exist,
  and that `0.3.0` is available for every Rust/npm package. If a partial release
  exists, inventory what was published and follow the guide's recovery rules
  before continuing; do not move a public tag or overwrite package versions.
- [x] Confirm the checkout is clean and its HEAD equals the recorded release
  SHA. Complete this gate before pushing the tag: tag push publishes both
  `ghcr.io/reason-healthcare/rh:v0.3.0` and `:latest`, even while GitHub remains
  a draft.

### 3. Tag and verify release builds

- [x] Create `v0.3.0` at the recorded release SHA and push only that tag to
  `upstream`. Verify the remote tag resolves to the same commit. The guide's
  `origin` examples refer to the canonical repository; this checkout names it
  `upstream`.
- [x] Wait for every job in the tag's `release.yml` run to succeed, including
  all four binary builds, draft creation, and Docker publication. Record the
  run URL and the tagged SHA.
- [x] Inspect the draft: tag `v0.3.0`, stable (`prerelease: false`), correct
  release notes from `CHANGELOG.md`, and these six assets:
  `rh-aarch64-apple-darwin.tar.gz`, `rh-x86_64-apple-darwin.tar.gz`,
  `rh-x86_64-unknown-linux-musl.tar.gz`, `rh-x86_64-pc-windows-msvc.zip`,
  `install-rh.sh`, and `SHA256SUMS`.
- [x] Download the actual hosted assets and verify all five payload checksums
  against `SHA256SUMS`. Extract and smoke-test the binaries on their matching
  platforms/runners (`rh --version` reports `rh 0.3.0`; `rh --help` works).
  Record archive checksums and platform results. Prior locally built candidate
  hashes are not the hosted release hashes.
- [x] Pull and run the versioned Docker image, verify `rh 0.3.0`, and record
  its digest. Verify `latest` resolves to the same published image.

### 4. Publish and verify all 12 Rust crates

Publish from the recorded release revision using `cargo publish -p <crate>`.
Use the separate packaging target directory, run Cargo operations sequentially,
and allow about 30 seconds for registry propagation between layers. If a
dependency is not visible, follow the guide's 30–60 second retry guidance.

- [x] Layer 1: `rh-foundation`.
- [x] Layer 2: `rh-hl7-fhir-r4-core`, `rh-hl7-fhir-r5-core`, `rh-codegen`,
  `rh-cql`.
- [x] Layer 3: `rh-fhirpath`, `rh-vcl`, `rh-fsh`.
- [x] Layer 4: `rh-validator`.
- [x] Layer 5: `rh-cpg`, `rh-packager`.
- [x] Layer 6: `rh-cli`.
- [x] Verify version `0.3.0` is visible and downloadable for each of the 12
  crates on crates.io; save package/version links and publish results. Do not
  infer success for all crates from `rh-cli` or a latest-version search alone.

### 5. Publish the GitHub release and npm packages

- [x] Review the draft and changelog once more, then publish the stable GitHub
  release. Verify its public page and unauthenticated asset downloads. Record
  the release URL and publication time.
- [x] Build/test/package the four WASM wrappers from the release source as
  described in the guide: `pnpm -r build`, `pnpm -r test`, and
  `pnpm -r pack:dry-run`. Confirm each package contains `dist/`, `wasm/`,
  `wasm-node/`, `wasm-bundler/`, and `README.md`, with version `0.3.0`.
- [x] User: publish `@reasonhealth/fhirpath`, `@reasonhealth/vcl`, `@reasonhealth/cql`,
  and `@reasonhealth/cpg` with public access and the stable `latest` tag.
  Do not publish the private playground. Follow the guide's local publishing
  instructions, including its restriction on local `--provenance` use.
- [x] After manual publication, verify `0.3.0` exists and `latest` points to it for all four npm packages;
  install the exact versions in a temporary consumer and smoke-test them.
  Record package/version links and results.

### 6. Verify downstream distribution

- [x] Confirm the release-published Homebrew workflow succeeds. Verify the
  tap's `Formula/rh.rb` uses `0.3.0`, correct asset URLs and SHA-256 values for
  macOS ARM64, macOS Intel, and Linux. Smoke-test installation and version on
  an available supported host; record the formula commit and workflow URL.
- [x] Confirm the release-published Chocolatey workflow succeeds. Verify the
  `0.3.0` package and Windows archive checksum, public availability, and a
  Windows installation/version smoke test. Record workflow and package links;
  a successful upload alone does not prove the package is publicly installable.
- [x] Verify the hosted `install-rh.sh` checksum, then test that script with
  `-v 0.3.0 -d <temporary-existing-directory>` and confirm the installed
  version. Keep release verification out of the normal system installation.
- [x] Resolve any failed distribution job using the guide's channel-specific
  recovery instructions and recheck the result. Record any unavailable
  platform verification or pending registry step explicitly rather than
  marking it complete.

### 7. Record the handoff gate for Analytics

- [x] Using the downloaded, checksum-verified release CLI, rerun the existing
  Analytics terminology contract (26 cases / 78 checks), value-set membership
  example, and terminology-free end-to-end example against the merged Analytics
  code. Do not change Analytics dependency pins during this verification.
- [x] Record the release SHA/tag, public release URL, Rust/CLI distribution results,
  and the exact asset URLs, platform names, and SHA-256 values Analytics will
  consume. Preserve the original CQL and the documented clinical limits.
- [x] Mark RH release/distribution complete once all preceding gates have
  evidence. This clears the release gate for the separate Analytics adoption
  change, which will pin `v0.3.0` and the verified hosted archive checksums.
  The Analytics change itself remains pending.

### Execution evidence

| Evidence | Value |
|---|---|
| Final reviewed PR head / CI URLs | `16f42895af26ea99e535a7a6a566b6f76fcf356c`; [CI](https://github.com/reason-healthcare/rh/actions/runs/37813572047), [CodeQL](https://github.com/reason-healthcare/rh/actions/runs/37813568018): successful |
| Merged release SHA / main CI URLs | `6654ee9df04cbcde99ca0d7b276c36369b5285b4`; [CI](https://github.com/reason-healthcare/rh/actions/runs/37815779258), [CodeQL](https://github.com/reason-healthcare/rh/actions/runs/37815779335): successful |
| Remote tag SHA / release workflow URL | `6654ee9df04cbcde99ca0d7b276c36369b5285b4`; [release workflow](https://github.com/reason-healthcare/rh/actions/runs/37820840016): successful |
| Public GitHub release URL / publication time | [v0.3.0](https://github.com/reason-healthcare/rh/releases/tag/v0.3.0); 2026-10-08 18:24:44 UTC; stable and latest |
| Hosted asset checksums / platform smoke results | All five payload checksums verified; [native smoke run](https://github.com/reason-healthcare/rh/actions/runs/37822588396) passed on all four platforms |
| Docker digest / version result | `sha256:611feec770c5621a2475d0de2eb9d01c3c918c28d7ef4002ff855e11634dd920`; anonymous pull and `rh 0.3.0` passed; `latest` matches |
| 12 crates.io versions | All 12 published and downloaded as `0.3.0`; registry checksums matched; every archive records clean source SHA `6654ee9d` |
| Four npm versions / consumer smoke results | All four public `0.3.0` versions and `latest` tags verified; Node API consumers and package layouts passed; CPG's eight package tests also passed |
| Homebrew formula commit / workflow / install result | [Formula `5b9413f0`](https://github.com/reason-healthcare/homebrew-rh/commit/5b9413f0eabfbc07e5ecb13ea5dca9dada22a14e); [publish](https://github.com/reason-healthcare/rh/actions/runs/37824071981) and [native install/version/help](https://github.com/reason-healthcare/rh/actions/runs/37824262138) passed |
| Chocolatey package / workflow / install result | [rh 0.3.0](https://community.chocolatey.org/packages/rh/0.3.0); [publish](https://github.com/reason-healthcare/rh/actions/runs/37824072011) and [public-feed install/shim/version/help](https://github.com/reason-healthcare/rh/actions/runs/37824262138) passed |
| Install-script result | Verified hosted script installed `rh 0.3.0` into an isolated temporary directory |
| Released CLI Analytics contract / example results | Hosted macOS ARM64 CLI: 26 cases / 78 checks, value-set membership and terminology-free end-to-end examples all passed against Analytics `5ed0ff1` |
| Analytics handoff ready | Yes; release complete, Analytics implementation remains pending |

## Final release evidence

The release tag resolves to `6654ee9df04cbcde99ca0d7b276c36369b5285b4`, whose
source tree exactly matches reviewed PR #75 head `16f42895`. Main CI, CodeQL,
and all release jobs passed. A fresh local `just check` passed with 2,861 tests,
15 ignored, strict Clippy, formatting, examples, and dependency audit; docs sync
passed. The committed-source package dry-run verified all 12 crates without
uploads before publication.

All 12 live Cargo uploads succeeded in dependency order. Each public crate
archive was downloaded, matched its registry SHA-256, and contained clean
`.cargo_vcs_info.json` metadata pointing to the release SHA. The published set is
`rh-foundation`, `rh-hl7-fhir-r4-core`, `rh-hl7-fhir-r5-core`, `rh-codegen`,
`rh-cql`, `rh-fhirpath`, `rh-vcl`, `rh-fsh`, `rh-validator`, `rh-cpg`,
`rh-packager`, and `rh-cli`, all at `0.3.0`.

All six GitHub release assets were downloaded anonymously after publication and
matched the verified SHA-256 values. Four native runner checks exercised the
archives from release run `37820840016`, pinned by checksum. Homebrew's formula
contains the three matching platform checksums; fresh Homebrew and Chocolatey
installations passed on macOS ARM64 and Windows x64 respectively. Docker's
versioned tag and `latest` have the same digest and execute `rh 0.3.0`.

The downloaded CLI passed the existing Analytics 26-case / 78-check terminology
contract, native/SQL value-set membership example with matching provenance, and
terminology-free end-to-end example against Analytics `5ed0ff1`. These checks
used the actual hosted binary. Analytics source and dependency pins remain
unchanged. All four npm packages were published by the user and independently
verified after installation from the public registry.

### Hosted payload checksums

Download assets from the [v0.3.0 release](https://github.com/reason-healthcare/rh/releases/tag/v0.3.0).
Use these hosted archive hashes for the later Analytics pin, not local candidate
build hashes.

| Hosted payload | SHA-256 |
|---|---|
| `rh-aarch64-apple-darwin.tar.gz` | `9e0e7cabe6e93f0e3cc92ae2b971e9d6033570a926c19cddba048268646d473a` |
| `rh-x86_64-apple-darwin.tar.gz` | `00213acd93d76174662056d584900e0946ce0f51873562c48f97b627092326c6` |
| `rh-x86_64-unknown-linux-musl.tar.gz` | `36db89e87d785289df11a04aa4c201a634a5603f10299559c51924e5bafeb796` |
| `rh-x86_64-pc-windows-msvc.zip` | `e998986ce4c53b6ee39f2057c877040e2141d1ddfcddb809c80b08b80a1682a5` |
| `install-rh.sh` | `d464b378aed543828c483b51526299adff668c3d60be02a15f7d65e42f659fb0` |

### npm publication verification

The user published all four npm packages. Each is available as `0.3.0` with the
`latest` tag. Exact versions were installed into a temporary consumer from the
public registry with install scripts disabled. Node API smoke tests passed for
FHIRPath evaluation, VCL translation, CQL compilation/evaluation, and CPG
PlanDefinition application. All packages include `dist/`, `wasm/`, `wasm-node/`,
`wasm-bundler/`, and `README.md`.

Published FHIRPath, VCL, and CQL compiled artifacts match the previously tested
release-preparation builds byte for byte. Published CPG artifacts match the
clean tagged publishing checkout byte for byte; its eight package tests were
rerun successfully against those artifacts. Earlier WASM/npm preparation ran
all four builds, all package dry-runs, and 32 tests including the playground.

| npm package | Version / tag | Published UTC |
|---|---|---|
| `@reasonhealth/fhirpath` | `0.3.0` / `latest` | 2026-10-08T18:29:09.406Z |
| `@reasonhealth/vcl` | `0.3.0` / `latest` | 2026-10-08T18:27:59.646Z |
| `@reasonhealth/cql` | `0.3.0` / `latest` | 2026-10-08T18:28:47.571Z |
| `@reasonhealth/cpg` | `0.3.0` / `latest` | 2026-10-08T18:28:15.321Z |

No further npm publication is needed. The next work item is the separate
Analytics adoption PR using the hosted release URLs and checksums above.
