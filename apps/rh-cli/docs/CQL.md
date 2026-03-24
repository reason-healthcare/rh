# rh CLI - CQL Commands

## Overview

The `rh cql` command provides tools for working with CQL (Clinical Quality Language), compiling CQL source files to ELM (Expression Logical Model) JSON, evaluating expressions, and inspecting compilation details.

See the [CQL crate README](../../../crates/rh-cql/README.md) for library-level documentation.

## Commands

### `rh cql compile`

Compile a CQL source file to ELM JSON.

**Usage:**
```bash
rh cql compile [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-o, --output <OUTPUT>` - Output file path (defaults to stdout)
- `--compact` - Output compact JSON (no pretty-printing)
- `--debug` - Enable debug mode (includes annotations, locators, and result types in output)
- `--strict` - Enable strict mode (disable implicit conversions)
- `--signatures` - Include all signatures in output
- `--source-map` - Also emit a source-map sidecar file alongside the ELM output
- `--source-map-output <PATH>` - Path for source-map output (defaults to `<output>.sourcemap.json` or stderr)
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Compile a CQL file to ELM JSON
rh cql compile library.cql

# Write output to a file
rh cql compile library.cql --output library.elm.json

# Compile with debug annotations
rh cql compile library.cql --debug

# Compile in strict mode
rh cql compile library.cql --strict

# Emit compact JSON with a source map
rh cql compile library.cql --compact --source-map --output library.elm.json

# Compile from stdin
echo 'library Test version '"'"'1.0'"'"' ...' | rh cql compile -
```

---

### `rh cql validate`

Validate CQL source without generating ELM output.

**Usage:**
```bash
rh cql validate [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Validate a CQL file
rh cql validate library.cql

# Validate from stdin
cat library.cql | rh cql validate -
```

---

### `rh cql info`

Parse a CQL file and display library information.

**Usage:**
```bash
rh cql info [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Show library information
rh cql info library.cql
```

---

### `rh cql eval`

Evaluate a named expression definition in a compiled CQL library.

**Usage:**
```bash
rh cql eval [OPTIONS] --expression <EXPRESSION> <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-e, --expression <EXPRESSION>` - Name of the expression definition to evaluate (required)
- `--trace` - Output a step-by-step evaluation trace
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Evaluate a named expression
rh cql eval --expression "InDemographic" library.cql

# Evaluate with a trace
rh cql eval --expression "InDemographic" --trace library.cql
```

---

### `rh cql explain`

Explain a CQL parse tree or compilation details.

**Usage:**
```bash
rh cql explain [OPTIONS] <FILE>
```

**Arguments:**
- `<FILE>` - Path to a CQL file, or `-` to read from stdin

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Explain CQL parse tree
rh cql explain library.cql
```

---

### `rh cql repl`

Start an interactive REPL for CQL compilation and exploration.

**Usage:**
```bash
rh cql repl [OPTIONS]
```

**Options:**
- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Print help

**Examples:**

```bash
# Start REPL
rh cql repl
```

## CQL Resources

- [CQL Specification (HL7)](https://cql.hl7.org/)
- [ELM Specification](https://cql.hl7.org/elm.html)
