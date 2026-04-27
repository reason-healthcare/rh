## Context

The `rh` CLI is a Rust binary that currently ships as platform-specific binary archives (`.tar.gz` / `.zip`) attached to GitHub Releases. Users must download, extract, and place the binary on their `$PATH` manually.

For CI/CD pipelines, cloud environments, and containerized workflows, a Docker image removes that friction — no Rust toolchain, no manual install, just `docker run`. The existing release workflow already builds a fully-static `x86_64-unknown-linux-musl` binary (vendored OpenSSL), making it ideal as the foundation for a minimal Docker image.

There is no existing Docker infrastructure in the repository.

## Goals / Non-Goals

**Goals:**
- Produce a minimal Docker image containing only the `rh` binary
- Build the image in CI on every version tag and push to GHCR (`ghcr.io/reason-healthcare/rh`)
- Tag the image with the version (e.g., `v0.1.0-beta.1`) and `latest`
- Support `linux/amd64` initially; leave the door open for `linux/arm64`
- Keep the Dockerfile self-contained and buildable locally with `docker build .`

**Non-Goals:**
- Publishing to Docker Hub (GHCR is sufficient for open-source projects on GitHub)
- Multi-arch builds in the first iteration (ARM cross-compilation complexity is deferred)
- Bundling FHIR package caches, terminology data, or other runtime assets inside the image
- Changing any Rust source code or crate behavior

## Decisions

### Use a multi-stage Dockerfile with distroless/scratch runtime

**Decision**: Build stage uses `rust:1.91-slim` (Debian-based, matches `rust-version` in `Cargo.toml`); runtime stage uses `gcr.io/distroless/static-debian12` (or `scratch`).

**Rationale**: The musl static binary has no dynamic library dependencies, so `scratch` or distroless static images are valid. `distroless/static` is preferred over bare `scratch` because it includes a valid `/etc/ssl/certs` CA bundle (needed for HTTPS requests made by `rh`) and a minimal `/etc/passwd`. This avoids runtime TLS verification failures when `rh` fetches remote FHIR packages.

**Alternative considered**: Use the pre-built musl release binary from GitHub Releases. Rejected — requires fetching artifacts in CI, creates ordering dependency, and makes local `docker build` harder.

### Target `x86_64-unknown-linux-musl` in the build stage

**Decision**: The Dockerfile uses `--target x86_64-unknown-linux-musl` and `OPENSSL_VENDORED=1`, matching the existing release workflow.

**Rationale**: Produces a fully-static binary with no shared library dependencies, which is required for the distroless/scratch runtime stage. This also mirrors what users already receive from the binary release artifacts.

### GHCR as the registry

**Decision**: Push to `ghcr.io/reason-healthcare/rh`.

**Rationale**: GHCR is co-located with the GitHub repository, uses the same `GITHUB_TOKEN` for auth (no additional secrets), and is free for public repositories. Avoids Docker Hub rate limits and credentials management.

### Release workflow: add a new `docker` job

**Decision**: Add a `docker` job to `release.yml` that runs after the `build` matrix completes. It does not depend on the `release` job (GitHub Release creation), so it can proceed in parallel.

**Rationale**: Keeps Docker build independent of the release notes draft. If the Docker push fails, the binary release is unaffected.

## Risks / Trade-offs

- **Build time**: Full source compilation in the Docker build stage adds ~5–10 min to the release CI. → Mitigation: The job is independent and runs in parallel with no downstream blocking.
- **Layer caching**: Docker layer caching in GitHub Actions is not automatic. → Mitigation: Use `cache-from`/`cache-to` with `type=gha` (GitHub Actions cache) via `docker/build-push-action`.
- **CA certificates**: Using `scratch` would break any `rh` subcommand that performs HTTPS calls. → Mitigation: `distroless/static-debian12` bundles CA certificates.
- **ARM users**: `linux/arm64` is not built in the first iteration. → Mitigation: Document the limitation; add multi-arch with `docker buildx` in a follow-on change.

## Migration Plan

1. Add `Dockerfile` and `.dockerignore` to repo root
2. Add `docker` job to `.github/workflows/release.yml`
3. Update `apps/rh-cli/README.md` with Docker usage instructions
4. No rollback required — image publication is additive; existing binary artifacts are unchanged

## Open Questions

- None — all key decisions resolved above.
