# rh CLI - VCL Commands

## Overview

The `rh vcl` command provides tools for working with VCL (ValueSet Compose Language), a concise expression language for defining FHIR `ValueSet.compose` structures. VCL expressions can be translated directly into FHIR-conformant `ValueSet.compose` JSON.

See the [VCL crate README](../../../crates/rh-vcl/README.md) and [EXAMPLES.md](../../../crates/rh-vcl/EXAMPLES.md) for the full language reference.

## Commands

### `rh vcl parse`

Parse a VCL expression and display its abstract syntax tree (AST).

**Usage:**
```bash
rh vcl parse [OPTIONS] <EXPRESSION>
```

**Arguments:**
- `<EXPRESSION>` - VCL expression to parse

**Options:**
- `-f, --format <FORMAT>` - Output format: `pretty` (default), `json`, `debug`
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Parse a VCL expression
rh vcl parse "http://snomed.info/sct|73211009"

# Output as JSON
rh vcl parse --format json "http://snomed.info/sct|73211009"
```

---

### `rh vcl translate`

Translate a VCL expression into a FHIR `ValueSet.compose` JSON structure.

**Usage:**
```bash
rh vcl translate [OPTIONS] <EXPRESSION>
```

**Arguments:**
- `<EXPRESSION>` - VCL expression to translate

**Options:**
- `-f, --format <FORMAT>` - Output format: `json` (default), `pretty`
- `-o, --output <OUTPUT>` - Output file path (prints to stdout if not provided)
- `-s, --default-system <DEFAULT_SYSTEM>` - Default code system URI for codes without an explicit system
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Translate to FHIR ValueSet.compose JSON
rh vcl translate "http://snomed.info/sct|73211009"

# Write output to a file
rh vcl translate "http://snomed.info/sct|73211009" --output compose.json

# Use a default system for bare codes
rh vcl translate "73211009" --default-system http://snomed.info/sct

# Pretty-print the output
rh vcl translate "http://snomed.info/sct|73211009" --format pretty
```

---

### `rh vcl explain`

Explain a VCL expression in plain English.

**Usage:**
```bash
rh vcl explain [OPTIONS] <EXPRESSION>
```

**Arguments:**
- `<EXPRESSION>` - VCL expression to explain

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Explain what a VCL expression means
rh vcl explain "http://snomed.info/sct|73211009"
```

---

### `rh vcl repl`

Start an interactive REPL for VCL expressions.

**Usage:**
```bash
rh vcl repl [OPTIONS]
```

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Start REPL
rh vcl repl
```

## VCL Language

VCL is a compact notation for FHIR `ValueSet.compose` that supports:

- Single codes: `http://snomed.info/sct|73211009`
- Code sets: `http://snomed.info/sct|(73211009 OR 44054006)`
- Filters: `http://snomed.info/sct|concept is-a 73211009`
- Unions and intersections of code sets

See [EXAMPLES.md](../../../crates/rh-vcl/EXAMPLES.md) for a full set of examples.

## WebAssembly

The VCL crate also ships a WASM build for browser/Node.js use. See [WASM_BUILD.md](../../../crates/rh-vcl/WASM_BUILD.md) for build instructions.
