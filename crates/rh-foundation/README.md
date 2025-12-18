# rh-foundation

Foundation crate providing common utilities, error handling, and shared functionality for the RH workspace.

## Overview

`rh-foundation` serves as the foundational layer for all other crates in the RH workspace, providing:

- **Error Handling**: Common error types with context support
- **Configuration**: Traits and utilities for configuration management
- **I/O Operations**: File reading/writing with JSON support
- **HTTP Client**: Optional HTTP utilities with async support (feature: `http`)
- **Package Loader**: FHIR package downloading from npm-style registries (feature: `http`, module: `loader`) - [📖 Documentation](LOADER.md)
- **Snapshot Generator**: StructureDefinition snapshot generation with differential merging (module: `snapshot`) - [📖 Documentation](SNAPSHOT.md)
- **JSON Utilities**: Convenient JSON parsing and serialization
- **CLI Utilities**: Common CLI patterns for input/output and formatting
- **WASM Utilities**: WebAssembly helpers (feature: `wasm`)

## Features

- **Default**: Core functionality (error handling, config, I/O, JSON, CLI, snapshot generation)
- **http**: Enables HTTP client, FHIR package loader, and enhanced snapshot loading (requires `reqwest`, `tokio`, `url`, `tar`, `flate2`)
- **wasm**: Enables WebAssembly utilities (requires `wasm-bindgen`)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rh-foundation = { path = "../rh-foundation" }

# Or with HTTP support
rh-foundation = { path = "../rh-foundation", features = ["http"] }
```

### Error Handling

```rust
use rh_foundation::{FoundationError, Result};

fn example() -> Result<()> {
    // Use the foundation error type
    Err(FoundationError::InvalidInput("bad data".to_string()))
}
```

### Configuration

```rust
use rh_foundation::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    port: u16,
}

impl Config for MyConfig {
    fn validate(&self) -> rh_foundation::Result<()> {
        if self.port == 0 {
            return Err(rh_foundation::FoundationError::InvalidInput(
                "Port cannot be 0".to_string()
            ));
        }
        Ok(())
    }
}
```

### I/O Operations

```rust
use rh_foundation::io;

// Load configuration from file
let config: MyConfig = io::load_config_from_file("config.json")?;

// Save configuration to file
io::save_config_to_file(&config, "config.json")?;

// Read/write JSON
let data: SomeType = io::read_json("data.json")?;
io::write_json("output.json", &data, true)?; // true = pretty print
```

### HTTP Client (with `http` feature)

```rust
use rh_foundation::http::HttpClient;

#[tokio::main]
async fn main() -> rh_foundation::Result<()> {
    let client = HttpClient::new()?;
    
    // Download as bytes
    let data = client.download("https://example.com/data.json").await?;
    
    // Download as text
    let text = client.download_text("https://example.com/file.txt").await?;
    
    // Download and parse JSON
    let json: SomeType = client.download_json("https://example.com/api").await?;
    
    // Download to file
    client.download_to_file("https://example.com/file.zip", "file.zip").await?;
    
    Ok(())
}
```

### JSON Utilities

```rust
use rh_foundation::json;

// Parse JSON string
let data: MyType = json::parse(json_str)?;

// Serialize to JSON string
let json = json::stringify(&data, true)?; // true = pretty print

// Parse from bytes
let data: MyType = json::from_bytes(&bytes)?;

// Serialize to bytes
let bytes = json::to_bytes(&data, false)?; // false = compact
```

### CLI Utilities (NEW!)

```rust
use rh_foundation::cli;
use std::path::PathBuf;

// Read input from file, inline string, or stdin
let content = cli::read_input(Some("input.txt"), None).await?;
let content = cli::read_input(None, Some("inline data".to_string())).await?;
let content = cli::read_input(None, None).await?; // from stdin

// Read from PathBuf or stdin (common in clap CLIs)
let path: Option<PathBuf> = Some(PathBuf::from("data.json"));
let content = cli::read_input_from_path(&path)?;

// Read and parse JSON
let data: MyType = cli::read_json("config.json")?;

// Write output to file or stdout
cli::write_output(Some(Path::new("output.txt")), "content")?;
cli::write_output(None, "content")?; // to stdout

// Format output with different styles
use rh_foundation::cli::OutputFormat;
cli::print_with_format(&data, OutputFormat::Json)?;
cli::print_with_format(&data, OutputFormat::JsonCompact)?;
cli::print_with_format(&data, OutputFormat::DebugPretty)?;

// Print results with error handling
let success = cli::print_result(some_result);

// Conditional exit (useful for --strict flags)
cli::exit_if(has_errors && strict, 1);
```

### Package Loader (with `http` feature)

Download and manage FHIR packages from npm-style registries.

📖 **[Full Documentation: LOADER.md](LOADER.md)** - Complete guide including HL7 tooling compatibility, private registries, caching strategies, and performance optimization.

```rust
use rh_foundation::loader::{PackageLoader, LoaderConfig};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the loader
    let config = LoaderConfig {
        registry_url: "https://packages.fhir.org".to_string(),
        auth_token: None,
        timeout_seconds: 30,
        max_retries: 3,
        verify_checksums: false,
        overwrite_existing: false,
    };
    
    let loader = PackageLoader::new(config)?;
    
    // Get default packages directory (~/.fhir/packages)
    let packages_dir = PackageLoader::get_default_packages_dir()?;
    
    // Download a FHIR package
    let manifest = loader.download_package(
        "hl7.fhir.r4.core",
        "4.0.1",
        &packages_dir
    ).await?;
    
    println!("Downloaded: {} v{}", manifest.name, manifest.version);
    
    // Check if package is already downloaded
    let is_downloaded = loader.is_package_downloaded(
        "hl7.fhir.r4.core",
        "4.0.1",
        &packages_dir
    )?;
    
    // List available versions
    let versions = loader.list_versions("hl7.fhir.r4.core").await?;
    
    // Get latest version
    let latest = loader.get_latest_version("hl7.fhir.r4.core").await?;
    
    Ok(())
}
```

### Snapshot Generator

Generate complete snapshots from FHIR StructureDefinitions with differential elements.

📖 **[Full Documentation: SNAPSHOT.md](SNAPSHOT.md)** - Complete guide including design philosophy, architecture, performance optimization, and advanced usage patterns.

```rust
use rh_foundation::snapshot::{
    SnapshotGenerator, StructureDefinitionLoader, StructureDefinition
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = SnapshotGenerator::new();
    
    // Load base FHIR definitions
    let base_definitions = StructureDefinitionLoader::load_from_package(
        "hl7.fhir.r4.core",
        "4.0.1",
        &PackageLoader::get_default_packages_dir()?
    )?;
    
    generator.load_structure_definitions(base_definitions);
    
    // Load your custom profile
    let my_profile = StructureDefinitionLoader::load_from_file(
        Path::new("MyPatientProfile.json")
    )?;
    
    generator.load_structure_definition(my_profile);
    
    // Generate snapshot with full element tree
    let snapshot = generator.generate_snapshot(
        "http://example.org/StructureDefinition/MyPatient"
    )?;
    
    println!("Generated snapshot with {} elements", snapshot.element.len());
    
    // Cache management
    println!("Cache size: {}", generator.cache_size());
    generator.clear_cache();
    
    Ok(())
}
```

**Snapshot Generation Features:**

- **Differential Merging**: Automatically merges base and differential elements
- **Cardinality Validation**: Ensures differential constraints are stricter than base
- **Type Restriction**: Validates differential types are subsets of base types
- **Binding Validation**: Verifies binding strength hierarchy
- **Slice Support**: Handles slicing and reslicing with proper element expansion
- **Circular Dependency Detection**: Prevents infinite loops in inheritance chains
- **Caching**: Caches generated snapshots for performance

### Snapshot Loader

Load StructureDefinitions from various sources:

```rust
use rh_foundation::snapshot::StructureDefinitionLoader;
use std::path::Path;

// Load from a single file
let sd = StructureDefinitionLoader::load_from_file(
    Path::new("MyProfile.json")
)?;

// Load all StructureDefinitions from a directory
let definitions = StructureDefinitionLoader::load_from_directory(
    Path::new("profiles/")
)?;

// Load from a FHIR package
let package_definitions = StructureDefinitionLoader::load_from_package(
    "hl7.fhir.us.core",
    "5.0.1",
    &PackageLoader::get_default_packages_dir()?
)?;

println!("Loaded {} StructureDefinitions", definitions.len());
```

## Module Structure

```
rh-foundation/
├── error.rs       - Error types and utilities
├── config.rs      - Configuration traits
├── io.rs          - File I/O operations
├── json.rs        - JSON utilities
├── cli.rs         - CLI utilities
├── validation.rs  - FHIR validation types
├── http.rs        - HTTP client (feature: http)
├── loader.rs      - FHIR package loader (feature: http)
├── wasm.rs        - WASM utilities (feature: wasm)
└── snapshot/      - Snapshot generation
    ├── error.rs         - Snapshot-specific errors
    ├── generator.rs     - Snapshot generator
    ├── sd_loader.rs     - StructureDefinition loader
    ├── merger.rs        - Element merging logic
    ├── path.rs          - ElementPath utilities
    └── types.rs         - FHIR types
```

## Migration from rh-common

This crate replaces the old `rh-common` and `rh-core` crates. To migrate:

1. Update `Cargo.toml`: Replace `rh-common` with `rh-foundation`
2. Update imports: Change `use rh_common::*` to `use rh_foundation::*`
3. Update error types: `CommonError` → `FoundationError`

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
