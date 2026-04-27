# rh — Unified CLI for FHIR Tools

The `rh` CLI is a unified command-line interface for working with FHIR data. It provides FHIR code generation, FHIRPath evaluation, CQL compilation, validation, and package management in a single tool.

## Installation

### Pre-built binaries (recommended)

Download the latest release binary for your platform from the [GitHub Releases page](https://github.com/reason-healthcare/rh/releases).

**macOS (Apple Silicon)**
```bash
curl -L https://github.com/reason-healthcare/rh/releases/latest/download/rh-aarch64-apple-darwin.tar.gz | tar xz
sudo mv rh /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -L https://github.com/reason-healthcare/rh/releases/latest/download/rh-x86_64-apple-darwin.tar.gz | tar xz
sudo mv rh /usr/local/bin/
```

**Linux x86_64 (static binary)**
```bash
curl -L https://github.com/reason-healthcare/rh/releases/latest/download/rh-x86_64-unknown-linux-musl.tar.gz | tar xz
sudo mv rh /usr/local/bin/
```

**Windows x86_64**

Download [`rh-x86_64-pc-windows-msvc.zip`](https://github.com/reason-healthcare/rh/releases/latest/download/rh-x86_64-pc-windows-msvc.zip), extract it, and place `rh.exe` somewhere on your `PATH`.

### Build from source

```bash
cargo install --path apps/rh-cli
```

### From crates.io

```bash
cargo install rh-cli
```

### Docker

A pre-built Docker image is available from the GitHub Container Registry:

```bash
# Pull the latest image
docker pull ghcr.io/reason-healthcare/rh:latest

# Show available commands
docker run --rm ghcr.io/reason-healthcare/rh:latest --help

# Run rh against files in the current directory
docker run --rm \
  -v "$(pwd)":/work \
  -w /work \
  ghcr.io/reason-healthcare/rh:latest \
  validate resource --input patient.json
```

You can also pin to a specific version:

```bash
docker run --rm ghcr.io/reason-healthcare/rh:v0.1.0-beta.1 --version
```

## Quick Start

```bash
# See all available commands
rh --help

# Download a FHIR package
rh download package hl7.fhir.r4.core 4.0.1

# Generate a Rust crate from a FHIR package
rh codegen hl7.fhir.r4.core 4.0.1 --output crates/rh-hl7_fhir_r4_core

# Evaluate a FHIRPath expression
rh fhirpath eval "Patient.name.family" --data patient.json

# Validate a FHIR resource
rh validate resource --input patient.json

# Compile CQL to ELM
rh cql compile library.cql --output library.elm.json
```

## Command Reference

| Command | Description | Docs |
|---------|-------------|------|
| `rh codegen` | Generate Rust types from FHIR packages | [CODEGEN.md](docs/CODEGEN.md) |
| `rh download` | Download FHIR packages from registries | [DOWNLOAD.md](docs/DOWNLOAD.md) |
| `rh fhirpath` | Parse and evaluate FHIRPath expressions | [FHIRPATH.md](docs/FHIRPATH.md) |
| `rh vcl` | Parse and translate VCL expressions | [VCL.md](docs/VCL.md) |
| `rh cql` | Compile CQL to ELM | [CQL.md](docs/CQL.md) |
| `rh validate` | Validate FHIR resources | [VALIDATOR.md](docs/VALIDATOR.md) |
| `rh snapshot` | Generate and manage StructureDefinition snapshots | — |
| `rh publish` | Build and publish FHIR Packages from a source directory | — |

### `rh publish` subcommands

```
rh publish build <dir>   Build a FHIR Package from a source directory
rh publish lock  <dir>   Resolve canonical references and write fhir-lock.json
rh publish check <dir>   Validate source (no output written)
rh publish pack  <dir>   Pack an already-built output directory into a .tgz

Options:
  -o, --out <path>   Output directory (build) or output file path (pack)
```

See [`crates/rh-publisher/README.md`](../../crates/rh-publisher/README.md) for full documentation including `publisher.toml` configuration reference.

## Global Options

- `-v, --verbose` — Enable verbose logging
- `--help` — Print help for any command or subcommand

## Environment Variables

- `RH_REGISTRY_TOKEN` — Authentication token for private FHIR package registries
- `RUST_LOG` — Logging level (e.g., `info`, `debug`, `trace`)

## Related Documentation

- [Code Generation Library](../../crates/rh-codegen/README.md)
- [FHIRPath Library](../../crates/rh-fhirpath/README.md)
- [CQL Library](../../crates/rh-cql/README.md)
- [Validator Library](../../crates/rh-validator/README.md)
- [VCL Library](../../crates/rh-vcl/README.md)
- [Foundation Library](../../crates/rh-foundation/README.md)
- [Workspace Overview](../../README.md)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development guidelines.
