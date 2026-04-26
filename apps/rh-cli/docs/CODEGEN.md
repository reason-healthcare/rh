# rh CLI - Code Generation Commands

## Overview

The `rh codegen` command generates organized Rust crates from FHIR packages. It downloads a FHIR package from a registry (or uses a locally cached copy), then produces a Rust crate with typed structs and traits for all resource types in the package.

## Command

### `rh codegen`

Generate a Rust crate from a FHIR package.

**Usage:**
```bash
rh codegen [OPTIONS] <PACKAGE> [VERSION]
```

**Arguments:**
- `<PACKAGE>` - Package name (e.g., `hl7.fhir.r4.core`)
- `[VERSION]` - Package version (e.g., `4.0.1`). If omitted, uses the latest available version.

**Options:**
- `-o, --output <OUTPUT>` - Output directory for the generated crate (default: `./generated`)
- `--crate-name <CRATE_NAME>` - Override the generated crate name (e.g., `rh-hl7-fhir-r4-core`). When not provided, the name is auto-derived from the FHIR package name.
- `--package-dir <PACKAGE_DIR>` - Package directory for downloaded packages (defaults to `~/.fhir/packages`)
- `--registry <REGISTRY>` - Registry URL (default: `https://packages.fhir.org`)
- `--overwrite` - Overwrite the downloaded package if it already exists in the cache
- `--force` - Force overwrite of a non-empty output directory
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Generate a crate from HL7 FHIR R4 Core
rh codegen hl7.fhir.r4.core 4.0.1 --output crates/rh-hl7_fhir_r4_core

# Override the generated crate name
rh codegen hl7.fhir.r4.core 4.0.1 \
  --output crates/rh-hl7_fhir_r4_core \
  --crate-name rh-hl7-fhir-r4-core

# Use latest version
rh codegen hl7.fhir.r4.core --output ./generated

# Use a custom registry
rh codegen hl7.fhir.r4.core 4.0.1 \
  --output ./generated \
  --registry https://packages.simplifier.net

# Force regeneration of an existing output directory
rh codegen hl7.fhir.r4.core 4.0.1 --output ./generated --force
```

## Generated Crate Structure

The generated crate contains:

```
generated/
  Cargo.toml        # Package manifest (name derived from FHIR package or --crate-name)
  README.md         # Crate documentation
  src/
    lib.rs          # Library root with module declarations
    types/          # Generated resource type modules
    ...
```

The `[lib]` section in `Cargo.toml` uses an underscore-based name derived from the crate's package name (hyphens replaced with underscores), allowing the crate to be used in Rust code with `use <lib_name>::...`.

## Crate Naming

By default, the crate name is auto-derived from the FHIR package name (e.g., `hl7.fhir.r4.core` → `hl7-fhir-r4-core`). Use `--crate-name` to override this — for example, to add a vendor prefix:

```bash
rh codegen hl7.fhir.r4.core 4.0.1 \
  --output crates/rh-hl7_fhir_r4_core \
  --crate-name rh-hl7-fhir-r4-core
```

The `[lib] name` is always derived from the package name by replacing hyphens with underscores.
