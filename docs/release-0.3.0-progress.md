# RH 0.3.0 release progress

Started: 2026-10-08. Target: `0.3.0`.

## Scope

Release from merged RH main, starting at `29b63f5a`, with all changes since
`v0.2.8`: executable bundles, CPG execution and SDC extraction, compiler and
analytics artifact documentation, and pinned terminology snapshots. Companion
Analytics terminology support is merged at `5ed0ff1`.

Preparation uses the isolated `codex/release-0.3.0` branch. Original dirty
checkouts are preserved. Analytics adoption has a separate clean worktree on
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
| Prepare release PR | Complete | `codex/release-0.3.0`; independent diff review and all local artifact gates passed |
| Verify release-head hosted CI | Pending | Starts after publishing the release preparation branch |
| Tag/build/distribute 0.3.0 | Pending | Use the validated release commit; verify binaries and distribution channels |
| Pin released RH in Analytics | Pending | Exact version and checksums; CI and example adoption after assets exist |

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
four npm packages. Publication is still pending; version availability will be
checked again before uploading.

The local macOS ARM64 candidate archive passed the Analytics integration checks
against merged Analytics source `5ed0ff1`. Native and SQL membership selected
exactly p1/p2 with matching snapshot provenance; the terminology-free example
also passed. The golden terminology files remain byte-identical across both
repositories. This evidence is for a locally built candidate, not downloaded
GitHub release assets. Hosted binaries and distribution checks remain pending.

Package verification and workspace builds initially shared a Cargo target cache.
After the package dry-run passed, a repeated docs-sync build encountered mixed
packaged/workspace FHIRPath artifacts. The release guide now gives package
verification a separate target directory. Rebuilding the affected workspace
crate resolved the cache conflict, and a fresh docs-sync check passed without
source-code changes.
