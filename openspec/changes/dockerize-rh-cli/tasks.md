## 1. Dockerfile

- [ ] 1.1 Create `Dockerfile` at repo root with a multi-stage build: `rust:1.91-slim` build stage targeting `x86_64-unknown-linux-musl` with `OPENSSL_VENDORED=1`, and `gcr.io/distroless/static-debian12` runtime stage copying only the `rh` binary
- [ ] 1.2 Set `ENTRYPOINT ["/rh"]` in the runtime stage so `docker run ghcr.io/reason-healthcare/rh <args>` works directly
- [ ] 1.3 Verify local build: `docker build -t rh .` completes successfully
- [ ] 1.4 Verify `docker run --rm rh --version` exits 0 and prints the version string

## 2. Build Context

- [ ] 2.1 Create `.dockerignore` at repo root excluding `target/`, `.git/`, `docs/`, `assets/`, and other non-source files to keep the build context small

## 3. CI — Release Workflow

- [ ] 3.1 Add `packages: write` to the top-level `permissions` block in `.github/workflows/release.yml`
- [ ] 3.2 Add a `docker` job to `release.yml` that runs on `ubuntu-latest`, needs `build` (so it runs after binaries are built), and performs:
  - Checkout sources
  - Log in to GHCR using `docker/login-action` with `GITHUB_TOKEN`
  - Build and push using `docker/build-push-action` with `cache-from`/`cache-to` GHA caching
  - Tags: `ghcr.io/reason-healthcare/rh:${{ github.ref_name }}` and `ghcr.io/reason-healthcare/rh:latest`

## 4. Documentation

- [ ] 4.1 Add a `## Docker` section to `apps/rh-cli/README.md` showing how to pull (`docker pull ghcr.io/reason-healthcare/rh:latest`) and run with a volume mount example
