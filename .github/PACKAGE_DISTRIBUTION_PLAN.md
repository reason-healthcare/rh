# Package Distribution Pipeline for `rh` CLI

## Current State

- **Release workflow** (`.github/workflows/release.yml`): builds 4 binaries on `v*` tag, creates draft GitHub Release, pushes Docker to GHCR, attaches install script
- **Homebrew tap** (`reason-healthcare/homebrew-rh` repo): auto-updated by `publish-homebrew.yml` on release publish
- **Chocolatey** (`choco/`): auto-published by `publish-choco.yml` on release publish
- **Linux install script** (`scripts/install-rh.sh`): curl|sh installer, attached to releases

## Architecture Decision

- **Homebrew**: Separate `reason-healthcare/homebrew-rh` tap repo, auto-updated by workflow on release publish. Template formula kept at `homebrew-tap/Formula/rh.rb`.
- **Chocolatey**: In-repo `choco/` directory with nuspec + tools, auto-published to chocolatey.org on release publish.
- **Linux install script**: In-repo `scripts/install-rh.sh`, attached to releases and callable via raw GitHub URL.

---

## Phase 1: Homebrew Tap (macOS + Linux) — DONE

| Step | Task | Status |
|------|------|--------|
| 1a | Create `reason-healthcare/homebrew-rh` GitHub repo | Done |
| 1b | Write `homebrew-tap/Formula/rh.rb` template in this repo | Done |
| 1c | Add `publish-homebrew` workflow (`.github/workflows/publish-homebrew.yml`) | Done |
| 1d | Add `HOMEBREW_TAP_TOKEN` secret to main repo | Pending (manual) |
| 1e | Test: `brew tap reason-healthcare/rh && brew install rh` | Done |

---

## Phase 2: Chocolatey (Windows) — DONE

| Step | Task | Status |
|------|------|--------|
| 2a | Add `choco/` directory with `rh.nuspec` template and `tools/chocolateyinstall.ps1` | Done |
| 2b | Add `publish-choco` workflow (`.github/workflows/publish-choco.yml`) | Done |
| 2c | Store `CHOCOLATEY_API_KEY` as GitHub secret | Pending (manual) |
| 2d | Test: `choco install rh` | Pending (post-first-stable-release) |

The `publish-choco` workflow:
- Triggers on GitHub Release `published` event (not draft)
- Skips pre-release tags containing `-beta` or `-alpha`
- Downloads Windows binary, computes SHA256
- Updates nuspec version and install script checksum
- Packs and pushes `.nupkg` to chocolatey.org

---

## Phase 3: Linux Install Script — DONE

| Step | Task | Status |
|------|------|--------|
| 3a | Write `scripts/install-rh.sh` | Done |
| 3b | Attach install script as release asset in `release.yml` | Done |
| 3c | Update README with curl|sh one-liner | Done |
| 3d | Test on clean Ubuntu container | Pending |

The install script:
- Auto-detects OS (Linux/macOS) and architecture (x86_64/aarch64)
- Resolves latest release via GitHub API (or installs a specific version with `-v`)
- Downloads correct binary, installs to `/usr/local/bin`
- Supports custom install directory with `-d`

---

## Phase 4: Documentation & Release Flow Updates — DONE

| Step | Task | Status |
|------|------|--------|
| 4a | Update `README.md` Install section with Homebrew, Choco, and script methods | Done |
| 4b | Update release body template in `release.yml` with all install methods | Done |
| 4c | Update `RELEASING.md` with Choco and install script sections | Done |

---

## Dependency Graph

```
v* tag push
  └─ build (4 platform binaries)
      ├─ github-release (upload binaries + install script, create release)
      └─ docker (push to GHCR)

release published event:
  ├─ publish-homebrew (commit formula to tap)    [Phase 1 - DONE]
  └─ publish-choco (push to chocolatey.org)       [Phase 2 - DONE]

manual:
  └─ publish crates.io                             [existing]
```

## Secrets Required

| Secret | Purpose | Status |
|--------|---------|--------|
| `HOMEBREW_TAP_TOKEN` | Push to `reason-healthcare/homebrew-rh` tap repo | Pending (manual) |
| `CHOCOLATEY_API_KEY` | Push packages to chocolatey.org | Pending (manual) |
| `GITHUB_TOKEN` | Already used (automatic) | Active |

## Files Created/Modified

| File | Action | Status |
|------|--------|--------|
| `homebrew-tap/Formula/rh.rb` | **Created** — Homebrew formula template | Done |
| `.github/workflows/publish-homebrew.yml` | **Created** — Homebrew tap publish workflow | Done |
| `choco/rh.nuspec` | **Created** — Chocolatey package spec | Done |
| `choco/tools/chocolateyinstall.ps1` | **Created** — Chocolatey install script | Done |
| `choco/tools/chocolateyuninstall.ps1` | **Created** — Chocolatey uninstall script | Done |
| `choco/tools/chocolateybeforemodify.ps1` | **Created** — Chocolatey before-modify script | Done |
| `scripts/install-rh.sh` | **Created** — Linux/macOS install script | Done |
| `.github/workflows/publish-choco.yml` | **Created** — Chocolatey publish workflow | Done |
| `.github/workflows/release.yml` | **Modified** — install script asset, release body, job summary | Done |
| `RELEASING.md` | **Modified** — added Homebrew, Choco, and install script sections | Done |
| `README.md` | **Modified** — full install section | Done |

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Pre-release versions (0.x-beta) may confuse package managers | Homebrew/Choco workflows skip `-beta`/`-alpha` tags |
| Chocolatey automated review may reject | Start manual, add auto-push after first approved package |
| SHA256 mismatch on binary changes | Compute in CI from actual downloaded artifact, never hardcode |
| Install script `curl \| sh` security concerns | Script pinned to main branch; binary downloaded from signed release; displays version on success |
| Homebrew tap requires separate repo | Standard Homebrew convention; only holds `Formula/rh.rb` (~2KB) |