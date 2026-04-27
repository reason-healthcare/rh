## ADDED Requirements

### Requirement: Docker image contains the rh binary
The project SHALL provide a Docker image (`ghcr.io/reason-healthcare/rh`) that contains the `rh` CLI binary and allows users to run any `rh` subcommand via `docker run`.

#### Scenario: Run rh version in container
- **WHEN** a user executes `docker run --rm ghcr.io/reason-healthcare/rh:latest rh --version`
- **THEN** the container exits 0 and prints the `rh` version string to stdout

#### Scenario: Run rh with a mounted volume
- **WHEN** a user executes `docker run --rm -v $(pwd):/work -w /work ghcr.io/reason-healthcare/rh:latest rh validate input.json`
- **THEN** the container accesses files from the host directory and produces the same output as the native binary

### Requirement: Docker image is minimal and contains no build artifacts
The Docker image runtime layer SHALL contain only the `rh` binary and the CA certificate bundle required for TLS. It SHALL NOT contain the Rust toolchain, source code, Cargo registry, or any other build-time files.

#### Scenario: Image size is not bloated by build toolchain
- **WHEN** a user inspects the image with `docker image inspect`
- **THEN** the compressed image size is under 50 MB

### Requirement: Dockerfile is buildable locally
A `Dockerfile` at the repository root SHALL allow any user with Docker installed to build the image locally without additional setup.

#### Scenario: Local build succeeds from repository root
- **WHEN** a user runs `docker build -t rh .` from the repository root
- **THEN** the build completes successfully and produces a runnable image

### Requirement: Docker image is published to GHCR on release
The release CI workflow SHALL build and push the Docker image to `ghcr.io/reason-healthcare/rh` whenever a version tag (`v*`) is pushed.

#### Scenario: Image is tagged with version and latest
- **WHEN** the tag `v0.1.0-beta.1` is pushed
- **THEN** the image is available as both `ghcr.io/reason-healthcare/rh:v0.1.0-beta.1` and `ghcr.io/reason-healthcare/rh:latest`

#### Scenario: Binary release artifacts are unaffected by Docker job failure
- **WHEN** the Docker push job fails (e.g., registry outage)
- **THEN** the binary release artifacts and GitHub Release draft are still created successfully

### Requirement: Docker usage is documented
The `apps/rh-cli/README.md` SHALL include a Docker usage section explaining how to pull and run the image.

#### Scenario: Pull and run documented example works
- **WHEN** a user follows the documented `docker run` example in the README
- **THEN** the command executes successfully and produces expected output
