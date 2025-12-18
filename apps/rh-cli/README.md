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

# Download FHIR packages from registries
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1

# Evaluate a FHIRPath expression
cargo run -p rh -- fhirpath eval "Patient.name.family" -d examples/patient.json

# Parse and translate VCL expressions
cargo run -p rh -- vcl parse "(http://snomed.info/sct)status = \"active\""
cargo run -p rh -- vcl translate "(http://snomed.info/sct)123456" --default-system http://snomed.info/sct

# Validate JSON syntax
cargo run -p rh -- validate json -i examples/patient.json

# Start an interactive FHIRPath REPL
cargo run -p rh -- fhirpath repl
```

## Environment Variables

The RH CLI uses environment variables for configuration:

- **`RH_REGISTRY_TOKEN`** - Authentication token for private FHIR package registries
- **`RUST_LOG`** - Controls logging level (e.g., `info`, `debug`, `trace`)

### Authentication Example

```bash
# Set authentication token for private registries
export RH_REGISTRY_TOKEN="your-bearer-token-here"

# Download from private registry
cargo run -p rh -- download package my.private.package 1.0.0 --registry https://private-registry.com
```

## Command Overview

The RH CLI is organized into subcommands, each providing specialized functionality:

```
rh
├── codegen     # FHIR code generation
│   ├── init    # Initialize configuration
│   ├── generate # Generate from single file
│   ├── batch   # Batch processing
│   └── install # Install and generate
├── download    # FHIR package management
│   ├── package # Download packages from registries
│   └── list    # List available package versions
├── fhirpath    # FHIRPath expression operations
│   ├── parse   # Parse expressions
│   ├── eval    # Evaluate expressions
│   ├── repl    # Interactive shell
│   └── test    # Run test suites
├── vcl         # ValueSet Compose Language (VCL) operations
│   ├── parse   # Parse VCL expressions
│   ├── translate # Translate to FHIR ValueSet.compose
│   ├── explain # Explain expressions in plain English
│   └── repl    # Interactive VCL shell
└── validate    # JSON and FHIR validation
    └── json    # JSON syntax validation
```

## Code Generation (`rh codegen`)

Generate type-safe Rust code from FHIR StructureDefinitions.

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

## Package Management (`rh download`)

Download and manage FHIR packages from npm-style registries using the loader module from rh-foundation.

### Download Packages

Download FHIR packages from registries:

```bash
# Download a package (to default ~/.fhir/packages directory)
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1

# Download to custom location
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1 -o ./custom-packages/

# Overwrite existing package
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1 --overwrite

# Use custom registry with authentication (set token via environment variable)
export RH_REGISTRY_TOKEN="your-auth-token"
cargo run -p rh -- download package my.custom.package 1.0.0 \
  --registry https://my-fhir-registry.com
```

### List Package Versions

Query available versions for FHIR packages:

```bash
# List available versions for a package
cargo run -p rh -- download list hl7.fhir.r4.core

# Get latest version only
cargo run -p rh -- download list hl7.fhir.r4.core --latest
```

### Integration with Code Generation

The package manager integrates with code generation for streamlined workflows:

```bash
# Install package and generate types in one step
cargo run -p rh -- codegen install hl7.fhir.r4.core 4.0.1 -o ./generated/
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

## ValueSet Compose Language (`rh vcl`)

Parse, translate, and explain ValueSet Compose Language (VCL) expressions with comprehensive support for FHIR ValueSet.compose generation and hierarchical component explanations.

### Parse VCL Expressions

Parse VCL expressions and view the Abstract Syntax Tree (AST):

```bash
# Parse and show pretty output
cargo run -p rh -- vcl parse "123456"

# Parse with system URI
cargo run -p rh -- vcl parse "(http://snomed.info/sct)status = \"active\""

# Show AST in JSON format
cargo run -p rh -- vcl parse "category << 123456" --format json

# Debug output with full AST details
cargo run -p rh -- vcl parse "* - inactive" --format debug
```

### Translate to FHIR ValueSet.compose

Translate VCL expressions to FHIR ValueSet.compose structures:

```bash
# Translate simple code
cargo run -p rh -- vcl translate "123456" --default-system http://snomed.info/sct

# Translate with explicit system URI
cargo run -p rh -- vcl translate "(http://snomed.info/sct)status = \"active\""

# Complex expression with multiple systems
cargo run -p rh -- vcl translate "(http://loinc.org)12345-6; concept <<123455" --default-system http://snomed.info/sct

# Save output to file
cargo run -p rh -- vcl translate "category << 123456, status = \"active\"" -o valueset-compose.json

# Pretty format for human reading
cargo run -p rh -- vcl translate "* - {inactive, retired}" --format pretty
```

### Explain VCL Expressions

Get detailed plain-English explanations of VCL expressions with hierarchical component breakdown:

```bash
# Basic explanation
cargo run -p rh -- vcl explain "status = \"active\""

# Complex expression explanation
cargo run -p rh -- vcl explain "(http://snomed.info/sct)category << 123456, status = \"active\""

# JSON format for programmatic use
cargo run -p rh -- vcl explain "* - inactive" --format json

# Save explanation to file
cargo run -p rh -- vcl explain "(code1, code2); code3" -o explanation.txt
```

### Interactive VCL REPL

Start an interactive shell for VCL expression experimentation:

```bash
# Basic REPL
cargo run -p rh -- vcl repl

# REPL with translation mode
cargo run -p rh -- vcl repl --translate

# REPL with explanation mode
cargo run -p rh -- vcl repl --explain

# REPL with both translation and explanation
cargo run -p rh -- vcl repl --translate --explain --default-system http://snomed.info/sct
```

#### REPL Commands

The VCL REPL supports special commands:

- `.help` - Show available commands and VCL syntax examples
- `.exit` or `.quit` - Exit the REPL
- `exit` or `quit` - Alternative exit commands

#### REPL Example Session

```
vcl> status = "active"
✅ Parsed successfully:
  AST: Filter expression with property 'status' equals 'active'

✅ Translation successful:
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "status",
          "op": "=",
          "value": "active"
        }
      ]
    }
  ]
}

✅ Explanation:
Expression Type: Filter - defines criteria for selecting codes
Overall Explanation: codes where status equals "active"

Components:
  • status (Property): The 'status' property of codes
  • = (Operator): equals
  • "active" (Value): The target value "active"
```

### VCL Language Features

The RH CLI supports all VCL language constructs:

#### Basic Expressions
- **Wildcards**: `*` (matches all codes)
- **Simple codes**: `123456` or `"quoted code"`  
- **System URIs**: `(http://snomed.info/sct)123456`

#### Operators
- **Conjunction** (AND): `code1, code2, code3`
- **Disjunction** (OR): `code1; code2; code3`
- **Exclusion** (NOT): `* - inactive`

#### Filters
- **Equality**: `status = "active"`
- **Is-a relationships**: `category << 123456`
- **Descendant-of**: `type < 456789`
- **Regular expressions**: `code / "^[A-Z]+$"`
- **Set operations**: `status ^ {active, inactive}`
- **Of operations**: `*.category`, `{code1, code2}.property`

#### Advanced Features
- **Nested expressions**: `(code1, code2); code3`
- **ValueSet inclusions**: `^http://example.org/valueset`
- **Filter lists**: `{status = "active", category << 123456}`
- **Mixed systems**: `(http://loinc.org)123-4; (http://snomed.info/sct)567890`

## Validation (`rh validate`)

Validate FHIR resources with comprehensive error reporting, including structural validation, FHIRPath invariants, and ValueSet binding checks.

### Resource Validation

Validate a single FHIR resource:

```bash
# Validate from file
cargo run -p rh -- validate resource --input patient.json

# Validate from stdin
cat patient.json | cargo run -p rh -- validate resource

# JSON output format
cargo run -p rh -- validate resource --input patient.json --format json

# Skip invariant validation (structural only)
cargo run -p rh -- validate resource --input patient.json --skip-invariants

# Skip binding validation
cargo run -p rh -- validate resource --input patient.json --skip-bindings

# Strict mode (exit with error code on any issues, including warnings)
cargo run -p rh -- validate resource --input patient.json --strict
```

### Batch Validation

Validate multiple FHIR resources from NDJSON:

```bash
# Validate NDJSON file
cargo run -p rh -- validate batch --input patients.ndjson

# Summary only (hide individual issues)
cargo run -p rh -- validate batch --input patients.ndjson --summary-only

# Custom thread count
cargo run -p rh -- validate batch --input patients.ndjson --threads 8

# Skip invariants and bindings
cargo run -p rh -- validate batch --input patients.ndjson --skip-invariants --skip-bindings
```

### Validation Options

**Common Options:**
- `--input, -i`: Input file path (reads from stdin if not provided)
- `--format, -f`: Output format (`text` or `json`)
- `--skip-invariants`: Skip FHIRPath invariant validation (structural only)
- `--skip-bindings`: Skip ValueSet binding validation
- `--strict`: Exit with non-zero code on any issues (including warnings)

**Batch-Specific Options:**
- `--threads`: Number of threads for parallel validation (default: 4)
- `--summary-only`: Show summary only (hide individual issues)

### Output Examples

**Text Format (Default):**
```
✅ FHIR resource is valid
⚠️  3 warning(s)

Issues:
  1. ⚠️  Failed to parse invariant dom-6 expression: ...
     Location: 
     Invariant: dom-6
```

**Error Reporting:**
```
❌ FHIR validation failed
  Errors: 2
  Warnings: 1

Issues:
  1. ❌ Invariant pat-1 failed: SHALL at least contain a contact's details or a reference to an organization
     Location: Patient
     Invariant: pat-1
  2. ⚠️  Failed to parse invariant dom-6 expression: ...
     Invariant: dom-6
```

**Batch Validation Summary:**
```
📋 Batch Validation Summary:
  Total resources: 10
  ✅ Valid: 8
  ❌ Invalid: 2
  Total errors: 3
  Total warnings: 5

❌ Invalid resources:
  Line 4 (Patient): 1 error(s), 2 warning(s)
    ❌ Invariant ext-1 failed: Must have either extensions or value[x], not both
       at 
```

**JSON Format:**
```json
{
  "resourceType": "Patient",
  "valid": true,
  "issues": []
}
```

### Validation Examples

**Basic Validation:**
```bash
# Valid Patient resource
cat > patient.json << EOF
{
  "resourceType": "Patient",
  "id": "example",
  "text": {
    "status": "generated",
    "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">Example</div>"
  },
  "gender": "male",
  "name": [{
    "family": "Doe",
    "given": ["John"]
  }]
}
EOF

cargo run -p rh -- validate resource --input patient.json --skip-invariants
# Output: ✅ FHIR resource is valid
```

**Batch Processing:**
```bash
# Create test NDJSON file
cat > patients.ndjson << EOF
{"resourceType":"Patient","id":"p1","text":{"status":"generated","div":"<div xmlns=\"http://www.w3.org/1999/xhtml\">P1</div>"},"gender":"male","name":[{"family":"Doe"}]}
{"resourceType":"Patient","id":"p2","text":{"status":"generated","div":"<div xmlns=\"http://www.w3.org/1999/xhtml\">P2</div>"},"gender":"female","name":[{"family":"Smith"}]}
{"resourceType":"Patient","id":"p3","text":{"status":"generated","div":"<div xmlns=\"http://www.w3.org/1999/xhtml\">P3</div>"},"gender":"other","name":[{"family":"Jones"}]}
EOF

# Validate with summary
cargo run -p rh -- validate batch --input patients.ndjson --skip-invariants
# Output:
# 📋 Batch Validation Summary:
#   Total resources: 3
#   ✅ Valid: 3
#   ❌ Invalid: 0
#   Total errors: 0
#   Total warnings: 0
```

**Structured Output:**
```bash
# JSON format for integration with other tools
cargo run -p rh -- validate resource --input patient.json --format json --skip-invariants
# Output:
# {
#   "resourceType": "Patient",
#   "valid": true,
#   "issues": []
# }
```

**Configuration Options:**
```bash
# Structural validation only (no invariants or bindings)
cargo run -p rh -- validate resource --input patient.json --skip-invariants --skip-bindings

# Strict mode (warnings cause non-zero exit)
cargo run -p rh -- validate resource --input patient.json --strict

# Parallel batch validation with 8 threads
cargo run -p rh -- validate batch --input large-dataset.ndjson --threads 8 --summary-only
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
- **`rh-validator`** - FHIR resource validation
- **`rh-vcl`** - ValueSet Compose Language parsing and translation
- **`rh-foundation`** - Foundation utilities (errors, I/O, HTTP, loader, snapshot)

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

# 7. Work with ValueSet Compose Language (VCL)
cargo run -p rh -- vcl translate "(http://snomed.info/sct)123456 OR (http://loinc.org)LA6113-0"

# 8. Start interactive exploration
cargo run -p rh -- fhirpath repl --data ./examples/patient.json
```

### VCL Workflow

```bash
# Parse and validate VCL expressions
cargo run -p rh -- vcl parse "(http://snomed.info/sct)active-status = \"active\""

# Convert VCL to FHIR ValueSet.compose
cargo run -p rh -- vcl translate "123456 OR 789012" \
  --default-system http://snomed.info/sct \
  --format json

# Get detailed explanations of VCL expressions
cargo run -p rh -- vcl explain "(http://snomed.info/sct)status = \"active\" AND severity != \"mild\"" \
  --hierarchical --max-depth 3

# Interactive VCL development
cargo run -p rh -- vcl repl
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
export RH_REGISTRY_TOKEN="your-auth-token"

cargo run -p rh -- download package my.org.custom.fhir 2.1.0 \
  --registry https://fhir-packages.my-org.com \
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
- **[VCL Parser](../../crates/rh-vcl/README.md)** - Library documentation for ValueSet Compose Language
- **[Foundation](../../crates/rh-foundation/README.md)** - Foundation utilities including loader and snapshot
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
