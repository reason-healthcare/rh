# RH - Unified CLI for FHIR Tools

The **RH CLI** is a unified command-line interface that provides comprehensive functionality for working with FHIR (Fast Healthcare Interoperability Resources) data. It combines code generation, FHIRPath expression evaluation, and package management capabilities into a single, easy-to-use tool.

## Quick Start

```bash
# Build the CLI
cargo build -p rh

# Run with help to see all available commands
cargo run -p rh -- --help

# Generate Rust types from a FHIR StructureDefinition
cargo run -p rh -- codegen generate -i examples/patient.json -o examples/patient.rs

# Evaluate a FHIRPath expression
cargo run -p rh -- fhirpath eval "Patient.name.family" -d examples/patient.json

# Validate JSON syntax
cargo run -p rh -- validate json -i examples/patient.json

# Start an interactive FHIRPath REPL
cargo run -p rh -- fhirpath repl
```

## Command Overview

The RH CLI is organized into subcommands, each providing specialized functionality:

```
rh
├── codegen     # FHIR code generation and package management
│   ├── init    # Initialize configuration
│   ├── generate # Generate from single file
│   ├── batch   # Batch processing
│   ├── download # Download packages
│   └── install # Install and generate
├── fhirpath    # FHIRPath expression operations
│   ├── parse   # Parse expressions
│   ├── eval    # Evaluate expressions
│   ├── repl    # Interactive shell
│   └── test    # Run test suites
└── validate    # JSON and FHIR validation
    └── json    # JSON syntax validation
```

## Code Generation (`rh codegen`)

Generate type-safe Rust code from FHIR StructureDefinitions with full package management support.

### Initialize Configuration

Create a new codegen configuration file:

```bash
cargo run -p rh -- codegen init
cargo run -p rh -- codegen init --output-dir ./generated --package-name my_fhir_types
```

### Generate from Single File

Generate Rust types from a single FHIR StructureDefinition:

```bash
cargo run -p rh -- codegen generate -i patient.json -o patient.rs
cargo run -p rh -- codegen generate -i observation.json -o observation.rs --package-name fhir_obs
```

### Batch Processing

Process multiple FHIR StructureDefinitions at once:

```bash
cargo run -p rh -- codegen batch -i ./fhir-definitions/ -o ./generated/
cargo run -p rh -- codegen batch -c codegen.json
```

### Package Management

Download and install FHIR packages from npm-style registries:

```bash
# Download a package (to default ~/.fhir/packages directory)
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1

# Download to custom location
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1 -o ./custom-packages/

# List available versions for a package
cargo run -p rh -- download list hl7.fhir.r4.core

# Get latest version only
cargo run -p rh -- download list hl7.fhir.r4.core --latest

# Install package and generate types
cargo run -p rh -- codegen install hl7.fhir.r4.core 4.0.1 -o ./generated/

# Overwrite existing package
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1 --overwrite

# Use custom registry with authentication
cargo run -p rh -- download package my.custom.package 1.0.0 \
  --registry https://my-fhir-registry.com \
  --token your-auth-token
```

## FHIRPath Operations (`rh fhirpath`)

Parse, evaluate, and test FHIRPath expressions with comprehensive support for the FHIRPath specification.

### Parse Expressions

Parse FHIRPath expressions and view the Abstract Syntax Tree (AST):

```bash
# Parse and show pretty output
cargo run -p rh -- fhirpath parse "Patient.name.family"

# Show AST in JSON format
cargo run -p rh -- fhirpath parse "name.where(use='official')" --format json

# Debug output with full AST details
cargo run -p rh -- fhirpath parse "telecom.where(system='email').value" --format debug
```

### Evaluate Expressions

Evaluate FHIRPath expressions against FHIR data:

```bash
# Evaluate against a JSON file
cargo run -p rh -- fhirpath eval "Patient.name.family" --data patient.json

# Mathematical expressions
cargo run -p rh -- fhirpath eval "5 + 3 * 2"

# String operations
cargo run -p rh -- fhirpath eval "'hello world'.upper().substring(0, 5)"

# Output in different formats
cargo run -p rh -- fhirpath eval "Patient.name" --data patient.json --format json
```

### Interactive REPL

Start an interactive FHIRPath shell for experimentation:

```bash
# Start REPL without data
cargo run -p rh -- fhirpath repl

# Start REPL with pre-loaded FHIR data
cargo run -p rh -- fhirpath repl --data patient.json
```

**REPL Commands:**
- `.help` - Show available commands
- `.data` - Display currently loaded data
- `.load <file>` - Load FHIR data from JSON file
- `.quit` or `.exit` - Exit the REPL

**Example REPL Session:**
```
🔍 FHIRPath Interactive REPL
Type FHIRPath expressions to evaluate them.
Commands: .help, .data, .quit

fhirpath> Patient.name.family
=> [String("Doe")]

fhirpath> 5 + 3 * 2
=> [Integer(11)]

fhirpath> 'hello'.upper()
=> [String("HELLO")]

fhirpath> .quit
Goodbye!
```

### Test Suites

Run FHIRPath test suites from JSON files:

```bash
# Run tests from a file
cargo run -p rh -- fhirpath test --file tests.json

# Run tests with specific FHIR data
cargo run -p rh -- fhirpath test --file tests.json --data patient.json
```

**Test File Format:**
```json
[
  {
    "expression": "Patient.name.family",
    "expected": ["Doe"]
  },
  {
    "expression": "5 + 3",
    "expected": 8
  },
  {
    "expression": "invalid.expression",
    "expected": null,
    "should_error": true
  }
]
```

## Validation (`rh validate`)

Validate JSON syntax and FHIR resources with comprehensive error reporting and multiple output formats.

### JSON Syntax Validation

Validate JSON documents for syntax errors and structural issues:

```bash
# Validate a single JSON file
cargo run -p rh -- validate json --input patient.json

# Validate from stdin
echo '{"resourceType": "Patient", "id": "123"}' | cargo run -p rh -- validate json

# JSON output format
cargo run -p rh -- validate json --input patient.json --format json

# Multiple JSON documents (NDJSON)
cargo run -p rh -- validate json --input bundle.ndjson --multiple

# Custom validation parameters
cargo run -p rh -- validate json --input deep.json --max-depth 50

# Strict mode (exit with error code on validation failure)
cargo run -p rh -- validate json --input data.json --strict
```

### Validation Options

- `--input, -i`: Input file path (reads from stdin if not provided)
- `--format, -f`: Output format (`text` or `json`)
- `--multiple`: Process as NDJSON (multiple JSON documents)
- `--max-depth`: Maximum allowed nesting depth (default: 100)
- `--stats`: Show detailed statistics for valid JSON
- `--strict`: Exit with non-zero code on validation failure

### Output Examples

**Text Format (Default):**
```
✅ JSON is valid
```

**Error Reporting:**
```
❌ JSON validation failed with 1 error(s):
  1. JSON syntax error: expected ',' or ']' at line 5, column 12
```

**Multiple Document Summary:**
```
📋 Validation Summary:
  Total documents: 8
  ✅ Valid: 6
  ❌ Invalid: 2

❌ Invalid documents:
  Line 4: 1 error(s)
    - JSON syntax error: key must be a string at line 1, column 2
```

**JSON Format:**
```json
{
  "valid": true,
  "errors": []
}
```

### Validation Examples

**Basic Validation:**
```bash
# Valid JSON
echo '{"resourceType": "Patient", "id": "123"}' | cargo run -p rh -- validate json
# Output: ✅ JSON is valid

# Invalid JSON
echo '{"resourceType": "Patient", "id":}' | cargo run -p rh -- validate json
# Output: ❌ JSON validation failed with 1 error(s):
#   1. JSON syntax error: expected value at line 1, column 32
```

**Multiple Document Processing:**
```bash
# Create test NDJSON file
cat > test.ndjson << EOF
{"resourceType": "Patient", "id": "1"}
{"resourceType": "Observation", "id": "2"}
{invalid json}
{"resourceType": "Organization", "id": "3"}
EOF

# Validate with summary
cargo run -p rh -- validate json --input test.ndjson --multiple
# Output:
# 📋 Validation Summary:
#   Total documents: 4
#   ✅ Valid: 3
#   ❌ Invalid: 1
```

**Structured Output:**
```bash
# JSON format for integration with other tools
echo '{"test": "value"}' | cargo run -p rh -- validate json --format json
# Output:
# {
#   "valid": true,
#   "errors": []
# }
```

**Depth Validation:**
```bash
# Test nesting limits
echo '{"a":{"b":{"c":{"d":"too deep"}}}}' | cargo run -p rh -- validate json --max-depth 2
# Output: ❌ JSON validation failed with 1 error(s):
#   1. Schema validation error: Maximum nesting depth of 2 exceeded at depth 3
```

**Batch Processing:**
```bash
# Validate multiple FHIR resources with strict error handling
cargo run -p rh -- validate json \
  --input fhir-bundle.ndjson \
  --multiple \
  --format json \
  --strict
```

## 🛠️ Global Options

All commands support global options for enhanced control:

### Verbose Logging

Enable detailed logging for debugging and development:

```bash
# Enable verbose logging for any command
cargo run -p rh -- -v codegen generate -i patient.json -o patient.rs
cargo run -p rh -- --verbose fhirpath eval "Patient.name"
```

### Version Information

```bash
cargo run -p rh -- --version
```

## Configuration

### Codegen Configuration

The code generation commands can use a configuration file (`codegen.json`) to specify default options:

```json
{
  "input_dir": "./fhir-definitions",
  "output_dir": "./generated",
  "package_name": "my_fhir_types",
  "module_name": "types",
  "generate_enums": true,
  "registry_url": "https://packages.fhir.org",
  "auth_token": null
}
```

Create this file using:
```bash
cargo run -p rh -- codegen init
```

## Architecture

The RH CLI is built using:

- **[clap](https://clap.rs/)** - Command-line argument parsing with subcommands
- **[tokio](https://tokio.rs/)** - Async runtime for network operations
- **[tracing](https://tracing.rs/)** - Structured logging and diagnostics
- **[anyhow](https://docs.rs/anyhow/)** - Error handling and context

### Code Organization

```
src/
├── main.rs       # CLI entry point and global options
├── codegen.rs    # Code generation subcommands
└── fhirpath.rs   # FHIRPath operation subcommands
```

### Dependencies

The CLI leverages these workspace crates:

- **`rh-codegen`** - FHIR code generation from StructureDefinitions
- **`rh-fhirpath`** - FHIRPath parsing and evaluation
- **`rh-validator`** - JSON syntax and FHIR resource validation
- **`rh-loader`** - FHIR package downloading from npm-style registries
- **`rh-common`** - Shared utilities and error handling

## Performance

### Async Operations

Network operations (package downloads) are fully async and support:
- Concurrent downloads
- Request timeouts
- Retry logic with exponential backoff
- Authentication with bearer tokens

### Memory Efficiency

- Streaming JSON parsing for large FHIR files
- Efficient AST representation for FHIRPath expressions
- Minimal memory footprint for CLI operations

## Examples

### End-to-End Workflow

```bash
# 1. Initialize a new project
cargo run -p rh -- codegen init --output-dir ./my-fhir-project

# 2. Download FHIR core package (to ~/.fhir/packages)
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1

# 3. Install and generate types
cargo run -p rh -- codegen install hl7.fhir.r4.core 4.0.1 -o ./my-fhir-project/

# 4. Validate FHIR resources for JSON syntax
cargo run -p rh -- validate json --input ./examples/patient.json --stats

# 5. Validate multiple resources from a bundle
cargo run -p rh -- validate json --input ./examples/bundle.ndjson --multiple --format json

# 6. Test FHIRPath expressions against generated types
cargo run -p rh -- fhirpath eval "Patient.name.family" --data ./examples/patient.json

# 7. Start interactive exploration
cargo run -p rh -- fhirpath repl --data ./examples/patient.json
```

### Batch Processing

```bash
# Process an entire directory of FHIR definitions
cargo run -p rh -- codegen batch \
  --input-dir ./fhir-r4-definitions/ \
  --output-dir ./generated/ \
  --package-name fhir_r4

# Use configuration file for complex setups
cargo run -p rh -- codegen batch --config ./codegen.json
```

### Custom Registry Usage

```bash
# Work with private FHIR package registries
export FHIR_TOKEN="your-auth-token"

cargo run -p rh -- download package my.org.custom.fhir 2.1.0 \
  --registry https://fhir-packages.my-org.com \
  --token $FHIR_TOKEN \
  --output ./custom-packages/
```

### Validation Pipeline

```bash
# Complete validation workflow
# 1. Validate JSON syntax first
cargo run -p rh -- validate json --input raw-data.ndjson --multiple --strict

# 2. Process valid JSON through other tools
if [ $? -eq 0 ]; then
    echo "JSON validation passed, processing..."
    cargo run -p rh -- fhirpath eval "Bundle.entry.resource.resourceType" --data raw-data.ndjson
else
    echo "JSON validation failed, check your data"
    exit 1
fi
```

## Testing

Run the CLI's tests:

```bash
# Test the CLI module
cargo test -p rh

# Test with verbose output
cargo test -p rh -- --nocapture

# Test specific functionality
cargo test -p rh test_codegen_commands
```

## Related Documentation

- **[FHIR Code Generation](../../crates/rh-codegen/README.md)** - Library documentation for code generation
- **[FHIRPath](../../crates/rh-fhirpath/README.md)** - Library documentation for FHIRPath operations
- **[FHIR Validator](../../crates/rh-validator/README.md)** - Library documentation for validation
- **[FHIR Loader](../../crates/rh-loader/README.md)** - Library documentation for package downloading
- **[Workspace Overview](../../README.md)** - Main project documentation

## Contributing

This CLI is part of the larger FHIR Rust monorepo. See the main [CONTRIBUTING.md](../../CONTRIBUTING.md) for development guidelines.

### Adding New Subcommands

To add new functionality:

1. Create a new module in `src/` (e.g., `src/newfeature.rs`)
2. Add the subcommand to the `Commands` enum in `main.rs`
3. Implement the command handler following the existing patterns
4. Update this README with examples and documentation

### Code Style

Follow the workspace coding standards:
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Include comprehensive error handling with `anyhow`
- Add logging with `tracing` for debugging support
