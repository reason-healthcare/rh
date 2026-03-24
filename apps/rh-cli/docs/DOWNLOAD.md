# rh CLI - Download Commands

## Overview

The `rh download` command downloads and installs FHIR packages from npm-style registries into the local package cache (`~/.fhir/packages` by default). Downloaded packages can then be used by other `rh` commands such as `codegen` and `validate`.

## Commands

### `rh download package`

Download a specific FHIR package version from a registry.

**Usage:**
```bash
rh download package [OPTIONS] <PACKAGE> <VERSION>
```

**Arguments:**
- `<PACKAGE>` - Package name (e.g., `hl7.fhir.r4.core`)
- `<VERSION>` - Package version (e.g., `4.0.1`)

**Options:**
- `-o, --output <OUTPUT>` - Output directory for the downloaded package (defaults to `~/.fhir/packages`)
- `--registry <REGISTRY>` - Registry URL (default: `https://packages.fhir.org`)
- `--overwrite` - Overwrite package if it already exists in the cache
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Download HL7 FHIR R4 Core to default cache
rh download package hl7.fhir.r4.core 4.0.1

# Download to a custom directory
rh download package hl7.fhir.r4.core 4.0.1 --output ./fhir-packages

# Force re-download if already cached
rh download package hl7.fhir.r4.core 4.0.1 --overwrite

# Use a custom registry
rh download package hl7.fhir.r4.core 4.0.1 --registry https://packages.simplifier.net
```

---

### `rh download list`

List available versions for a FHIR package from a registry.

**Usage:**
```bash
rh download list [OPTIONS] <PACKAGE>
```

**Arguments:**
- `<PACKAGE>` - Package name (e.g., `hl7.fhir.r4.core`)

**Options:**
- `--registry <REGISTRY>` - Registry URL (default: `https://packages.fhir.org`)
- `--latest` - Show only the latest version
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# List all versions of HL7 FHIR R4 Core
rh download list hl7.fhir.r4.core

# Show only the latest version
rh download list hl7.fhir.r4.core --latest

# Query a custom registry
rh download list hl7.fhir.r4.core --registry https://packages.simplifier.net
```

## Package Cache

By default, packages are stored in `~/.fhir/packages`. The directory layout follows the standard FHIR package convention:

```
~/.fhir/packages/
  hl7.fhir.r4.core#4.0.1/
    package.json
    ...
```

Other `rh` commands automatically discover packages in this cache directory, so you typically only need to `download` a package once.
