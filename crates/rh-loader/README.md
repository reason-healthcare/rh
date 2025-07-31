# RH Loader - FHIR Package Loading Library

A Rust library for downloading and loading FHIR packages from npm-style registries such as packages.fhir.org. This crate provides robust package management capabilities with support for authentication, retry logic, and comprehensive error handling.

## Features

- **📦 Package Downloads** - Download FHIR packages from npm-style registries
- **🔐 Authentication** - Support for bearer token authentication
- **🔄 Retry Logic** - Automatic retry with exponential backoff for failed downloads
- **📋 Package Management** - List versions, get latest versions, and manage dependencies
- **🗜️ Archive Handling** - Automatic extraction of gzipped tarballs
- **✅ Checksum Verification** - Optional integrity verification (planned)
- **🔍 Comprehensive Logging** - Detailed tracing for debugging and monitoring

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rh-loader = { path = "../path/to/rh-loader" }
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use rh_loader::{PackageLoader, LoaderConfig};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create loader with default configuration
    let config = LoaderConfig::default();
    let loader = PackageLoader::new(config)?;

    // Download a FHIR package to default location (~/.fhir/packages)
    let default_dir = PackageLoader::get_default_packages_dir()?;
    let manifest = loader.download_package(
        "hl7.fhir.r4.core",
        "4.0.1",
        &default_dir
    ).await?;

    println!("Downloaded: {} v{}", manifest.name, manifest.version);
    Ok(())
}
```

### Advanced Configuration

```rust
use rh_loader::{PackageLoader, LoaderConfig};

let config = LoaderConfig {
    registry_url: "https://packages.fhir.org".to_string(),
    auth_token: Some("your-bearer-token".to_string()),
    timeout_seconds: 60,
    max_retries: 5,
    verify_checksums: true,
    overwrite_existing: false,
};

let loader = PackageLoader::new(config)?;
```

## API Reference

### LoaderConfig

Configuration options for the package loader:

```rust
pub struct LoaderConfig {
    /// Registry URL (defaults to https://packages.fhir.org)
    pub registry_url: String,
    /// Optional authentication token
    pub auth_token: Option<String>,
    /// HTTP client timeout in seconds (default: 30)
    pub timeout_seconds: u64,
    /// Maximum number of retry attempts (default: 3)
    pub max_retries: u32,
    /// Whether to verify checksums when available (default: false)
    pub verify_checksums: bool,
    /// Whether to overwrite existing packages (default: false)
    pub overwrite_existing: bool,
}
```

### PackageLoader

Main interface for package operations:

#### Methods

- `new(config: LoaderConfig) -> LoaderResult<Self>`
  - Create a new package loader with the given configuration

- `download_package(package_name: &str, version: &str, extract_to: &Path) -> LoaderResult<PackageManifest>`
  - Download and extract a package to the specified directory

- `list_versions(package_name: &str) -> LoaderResult<Vec<String>>`
  - List all available versions for a package

- `get_latest_version(package_name: &str) -> LoaderResult<String>`
  - Get the latest version identifier for a package

- `get_default_packages_dir() -> LoaderResult<PathBuf>`
  - Get the default FHIR packages directory (~/.fhir/packages)

### Data Types

#### PackageManifest

Represents package metadata:

```rust
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dist: PackageDist,
    pub dependencies: Option<HashMap<String, String>>,
    pub keywords: Option<Vec<String>>,
}
```

#### PackageDist

Package distribution information:

```rust
pub struct PackageDist {
    pub tarball: String,
    pub shasum: Option<String>,
    pub integrity: Option<String>,
}
```

## Examples

### Download Multiple Packages

```rust
use rh_loader::{PackageLoader, LoaderConfig};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = PackageLoader::new(LoaderConfig::default())?;
    let default_dir = PackageLoader::get_default_packages_dir()?;
    
    let packages = vec![
        ("hl7.fhir.r4.core", "4.0.1"),
        ("hl7.fhir.us.core", "5.0.1"),
        ("hl7.terminology.r4", "5.4.0"),
    ];

    for (name, version) in packages {
        println!("Downloading {}@{}...", name, version);
        let manifest = loader.download_package(
            name, 
            version, 
            &default_dir
        ).await?;
        println!("✅ Downloaded {} v{} to {}", 
            manifest.name, 
            manifest.version,
            default_dir.join(format!("{}#{}", name, version)).display()
        );
    }

    Ok(())
}
```

### List Available Versions

```rust
use rh_loader::{PackageLoader, LoaderConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = PackageLoader::new(LoaderConfig::default())?;
    
    // List all versions
    let versions = loader.list_versions("hl7.fhir.r4.core").await?;
    println!("Available versions: {:?}", versions);
    
    // Get latest version
    let latest = loader.get_latest_version("hl7.fhir.r4.core").await?;
    println!("Latest version: {}", latest);

    Ok(())
}
```

### Custom Registry with Authentication

```rust
use rh_loader::{PackageLoader, LoaderConfig};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = LoaderConfig {
        registry_url: "https://my-private-registry.com".to_string(),
        auth_token: Some(std::env::var("FHIR_TOKEN")?),
        timeout_seconds: 120,
        max_retries: 5,
        verify_checksums: true,
    };

    let loader = PackageLoader::new(config)?;
    
    let config_with_overwrite = LoaderConfig {
        registry_url: "https://my-private-registry.com".to_string(),
        auth_token: Some(std::env::var("FHIR_TOKEN")?),
        timeout_seconds: 120,
        max_retries: 5,
        verify_checksums: true,
        overwrite_existing: true, // Allow overwriting existing packages
    };

    let loader = PackageLoader::new(config_with_overwrite)?;
    let default_dir = PackageLoader::get_default_packages_dir()?;
    
    let manifest = loader.download_package(
        "my.org.custom.fhir",
        "2.1.0",
        &default_dir
    ).await?;

    println!("Downloaded private package: {}", manifest.name);
    Ok(())
}
```

### Error Handling

```rust
use rh_loader::{PackageLoader, LoaderConfig, LoaderError};

#[tokio::main]
async fn main() {
    let loader = PackageLoader::new(LoaderConfig::default()).unwrap();
    let default_dir = PackageLoader::get_default_packages_dir().unwrap();
    
    match loader.download_package("nonexistent.package", "1.0.0", &default_dir).await {
        Ok(manifest) => println!("Downloaded: {}", manifest.name),
        Err(LoaderError::PackageNotFound { package, version }) => {
            println!("Package {}@{} not found", package, version);
        }
        Err(LoaderError::PackageExists { package, version, path }) => {
            println!("Package {}@{} already exists at {}", package, version, path);
        }
        Err(LoaderError::Http(e)) => {
            println!("Network error: {}", e);
        }
        Err(LoaderError::ArchiveError { message }) => {
            println!("Extraction failed: {}", message);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
}
```

## Error Types

The library provides comprehensive error types for different failure scenarios:

- `LoaderError::Http` - Network and HTTP errors
- `LoaderError::PackageNotFound` - Package or version not found
- `LoaderError::PackageExists` - Package already exists (when overwrite is disabled)
- `LoaderError::InvalidManifest` - Malformed package metadata
- `LoaderError::ArchiveError` - Tarball extraction failures
- `LoaderError::Authentication` - Authentication-related errors
- `LoaderError::Io` - File system operations
- `LoaderError::Json` - JSON parsing errors

## Registry Support

The loader supports npm-style package registries with the following features:

### Supported Registries

- **packages.fhir.org** - Official FHIR package registry
- **Custom registries** - Any npm-compatible registry
- **Private registries** - With bearer token authentication

### Package Storage

Packages are stored using the FHIR package naming convention:

- **Default location**: `~/.fhir/packages/`
- **Package directory format**: `{package.name}#{package.version}`
- **Example**: `hl7.fhir.r4.core#4.0.1`
- **Duplicate handling**: Packages are not overwritten by default

### Registry API Compatibility

The loader expects registries to provide:

- Package metadata at `/{package-name}`
- Version-specific manifests in the `versions` object
- Tarball URLs in the `dist.tarball` field
- Optional `dist-tags` for version aliases (latest, beta, etc.)

## Performance Considerations

### Async Operations

All network operations are fully async and non-blocking:

- Concurrent downloads supported
- Configurable timeouts prevent hanging
- Retry logic with exponential backoff
- Streaming download and extraction

### Memory Usage

- Efficient streaming for large packages
- Tarball data held in memory only during extraction
- Minimal allocation overhead
- No unnecessary data copying

### Network Optimization

- HTTP/2 support through reqwest
- Connection reuse across requests
- Configurable timeout and retry policies
- User-Agent header for identification

## Integration Examples

### CI/CD Pipeline

```bash
#!/bin/bash
# Download FHIR packages for CI builds to default location

cargo run --example download-packages << EOF
hl7.fhir.r4.core 4.0.1
hl7.fhir.us.core 5.0.1
hl7.terminology.r4 5.4.0
EOF

# Packages will be stored in ~/.fhir/packages/ with format:
# ~/.fhir/packages/hl7.fhir.r4.core#4.0.1/
# ~/.fhir/packages/hl7.fhir.us.core#5.0.1/
# ~/.fhir/packages/hl7.terminology.r4#5.4.0/
```

### Docker Integration

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && \
    mkdir -p /root/.fhir/packages
COPY --from=builder /app/target/release/my-app /usr/local/bin/
ENV FHIR_REGISTRY_URL="https://packages.fhir.org"
CMD ["my-app"]
```

### Configuration Management

```rust
// Load configuration from environment
use rh_loader::LoaderConfig;

fn load_config() -> LoaderConfig {
    LoaderConfig {
        registry_url: std::env::var("FHIR_REGISTRY_URL")
            .unwrap_or_else(|_| "https://packages.fhir.org".to_string()),
        auth_token: std::env::var("FHIR_TOKEN").ok(),
        timeout_seconds: std::env::var("FHIR_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30),
        max_retries: std::env::var("FHIR_MAX_RETRIES")
            .unwrap_or_else(|_| "3".to_string())
            .parse()
            .unwrap_or(3),
        verify_checksums: std::env::var("FHIR_VERIFY_CHECKSUMS")
            .map(|v| v == "true")
            .unwrap_or(false),
        overwrite_existing: std::env::var("FHIR_OVERWRITE_EXISTING")
            .map(|v| v == "true")
            .unwrap_or(false),
    }
}
```

## Development

### Running Tests

```bash
# Run unit tests
cargo test

# Run integration tests (requires network)
cargo test --features integration-tests

# Run with logging
RUST_LOG=debug cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Dependencies

- **reqwest** - HTTP client with async support
- **tokio** - Async runtime
- **serde** - JSON serialization/deserialization
- **tar** - Tarball extraction
- **flate2** - Gzip decompression
- **tracing** - Structured logging
- **url** - URL parsing and manipulation

## Related Projects

- **rh-codegen** - FHIR code generation from downloaded packages
- **rh-validator** - FHIR resource validation
- **rh-fhirpath** - FHIRPath expression evaluation
- **rh-cli** - Command-line interface using this loader

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Changelog

### v0.1.0

- Initial release
- Basic package downloading and extraction
- Default FHIR package directory (~/.fhir/packages)
- Package naming convention (package.name#version)
- Duplicate package protection with overwrite option
- Authentication support
- Retry logic with exponential backoff
- Comprehensive error handling
- Version listing and latest version detection