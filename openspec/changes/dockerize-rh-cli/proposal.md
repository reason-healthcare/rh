## Why

The `rh` CLI is currently distributed as platform-specific binary archives. A Docker image would make it immediately usable in CI pipelines, containerized environments, and cloud-based workflows without requiring a local Rust toolchain or manual binary installation.

## What Changes

- Add a multi-stage `Dockerfile` that builds the `rh` CLI from source and produces a minimal runtime image
- Add a `.dockerignore` to keep build context lean
- Extend the release workflow to build and push a Docker image to GitHub Container Registry (GHCR) on version tags
- Document Docker usage in `apps/rh-cli/README.md`

## Capabilities

### New Capabilities
- `docker-image`: A Docker image (`ghcr.io/reason-healthcare/rh`) that packages the `rh` CLI binary, built via multi-stage Dockerfile using the existing musl static build target, enabling use in containers and CI environments

### Modified Capabilities
- `validator-cli-behavior`: No requirement changes — Docker is a new distribution channel that surfaces the same CLI behavior

## Impact

- New file: `Dockerfile` (repo root)
- New file: `.dockerignore` (repo root)
- Modified: `.github/workflows/release.yml` — add Docker build/push job after binaries are built
- Modified: `apps/rh-cli/README.md` — add Docker usage section
- No changes to Rust source code or existing crate behavior
- Depends on GHCR (`ghcr.io`) — requires `packages: write` permission in release workflow
