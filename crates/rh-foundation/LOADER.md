# FHIR Package Loader

The FHIR Package Loader provides a robust, standards-compliant implementation for downloading and managing FHIR packages from npm-style registries.

## Overview

The loader enables applications to:
- Download FHIR packages from npm-style registries (e.g., packages.fhir.org)
- Cache packages locally in a standardized directory structure
- Support authentication for private registries
- Maintain compatibility with HL7 tooling ecosystem

## Design Philosophy

### Compatibility with HL7 Tooling

The loader is designed to be fully compatible with the HL7 FHIR tooling ecosystem:

- **Standard Package Directory**: Uses `~/.fhir/packages/` by default, matching the Java validator, HAPI FHIR, and other HL7 tools
- **Package Naming Convention**: Follows the `{package-id}#{version}` format for directory names
- **Package Manifest**: Reads and writes standard `package.json` manifests
- **Registry Protocol**: Implements the npm-style registry protocol used by packages.fhir.org

This ensures that:
- Packages downloaded by rh can be used by other HL7 tools
- Packages cached by other tools can be discovered and used by rh
- Teams can share a single package cache across multiple tools

### Private Registry Support

The loader supports private FHIR package registries through flexible authentication:

```rust
use rh_foundation::loader::{PackageLoader, LoaderConfig};

let config = LoaderConfig {
    registry_url: "https://private-registry.example.com".to_string(),
    auth_token: Some("your-token-here".to_string()),
    ..Default::default()
};

let loader = PackageLoader::new(config);
```

**Authentication Methods:**
- Bearer token authentication for private registries
- Configurable per-registry credentials
- Secure token handling (never logged or displayed)

**Use Cases:**
- Enterprise-specific FHIR profiles
- Pre-release package versions
- Proprietary implementation guides
- Organization-specific extensions

### Caching Strategy

The loader implements an efficient multi-layer caching strategy:

#### 1. Package Directory Cache

Downloaded packages are cached in the filesystem:
```
~/.fhir/packages/
├── hl7.fhir.r4.core#4.0.1/
│   ├── package.json
│   ├── package/
│   │   ├── StructureDefinition-Patient.json
│   │   ├── ValueSet-administrative-gender.json
│   │   └── ...
├── hl7.fhir.us.core#5.0.1/
│   └── ...
└── custom.organization.profiles#1.0.0/
    └── ...
```

**Benefits:**
- Shared cache across all rh applications
- Persistent storage survives application restarts
- Compatible with other HL7 tools
- Reduces network bandwidth usage

#### 2. Registry Metadata Cache

Package manifests are cached to avoid repeated registry lookups:

```rust
// First lookup: queries registry
let manifest1 = loader.get_package_manifest("hl7.fhir.r4.core", "4.0.1").await?;

// Second lookup: uses cached manifest
let manifest2 = loader.get_package_manifest("hl7.fhir.r4.core", "4.0.1").await?;
```

**Cached Information:**
- Package metadata (name, version, description)
- Dependency lists
- File checksums
- Download URLs

#### 3. In-Memory Cache

Frequently accessed packages are kept in memory for immediate access:
- Reduces disk I/O
- Speeds up repeated validations
- Configurable memory limits
- LRU eviction policy

### Network Optimization

The loader is optimized for network efficiency:

**Parallel Downloads:**
```rust
// Downloads dependencies in parallel
loader.download_package_with_dependencies("hl7.fhir.us.core", "5.0.1").await?;
```

**Resumable Downloads:**
- Supports HTTP range requests
- Can resume interrupted downloads
- Validates checksums on completion

**Bandwidth Management:**
- Configurable timeouts
- Connection pooling
- Retry with exponential backoff

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────┐
│                    PackageLoader                         │
│  - Registry communication                                │
│  - Authentication handling                               │
│  - Cache coordination                                    │
└─────────────┬───────────────────────────────────────────┘
              │
              ├──► RegistryClient
              │    - HTTP requests to registry
              │    - Version resolution
              │    - Manifest retrieval
              │
              ├──► PackageCache
              │    - Filesystem operations
              │    - Package extraction
              │    - Directory management
              │
              └──► DownloadManager
                   - Tarball retrieval
                   - Checksum verification
                   - Parallel downloads
```

### Data Flow

```
┌──────────┐
│  Request │  download_package("hl7.fhir.r4.core", "4.0.1")
└────┬─────┘
     │
     ▼
┌────────────────┐
│ Check Cache    │ ~/.fhir/packages/hl7.fhir.r4.core#4.0.1/
└────┬───────────┘
     │
     ├─── Found ──────────► Return cached package
     │
     └─── Not Found
          │
          ▼
     ┌────────────────┐
     │ Query Registry │ GET https://packages.fhir.org/hl7.fhir.r4.core/4.0.1
     └────┬───────────┘
          │
          ▼
     ┌────────────────┐
     │ Download       │ GET tarball URL
     │ & Extract      │ Verify checksum
     └────┬───────────┘
          │
          ▼
     ┌────────────────┐
     │ Cache Package  │ Save to ~/.fhir/packages/
     └────┬───────────┘
          │
          ▼
     Return package
```

## Configuration

### Default Configuration

```rust
use rh_foundation::loader::{PackageLoader, LoaderConfig};

// Uses default settings
let loader = PackageLoader::default();
```

**Defaults:**
- Registry: `https://packages.fhir.org`
- Package directory: `~/.fhir/packages`
- Timeout: 30 seconds
- No authentication

### Custom Configuration

```rust
let config = LoaderConfig {
    registry_url: "https://custom-registry.example.com".to_string(),
    packages_dir: Some("/custom/path/packages".into()),
    auth_token: Some("token123".to_string()),
    timeout_secs: Some(60),
    user_agent: Some("MyApp/1.0".to_string()),
};

let loader = PackageLoader::new(config);
```

### Environment Variables

The loader respects standard environment variables:

- `FHIR_PACKAGES_DIR`: Override default package directory
- `FHIR_REGISTRY_URL`: Override default registry URL
- `FHIR_AUTH_TOKEN`: Provide authentication token

## Usage Examples

### Basic Package Download

```rust
use rh_foundation::loader::PackageLoader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let loader = PackageLoader::default();
    
    // Download a specific version
    let package_dir = loader
        .download_package("hl7.fhir.r4.core", "4.0.1")
        .await?;
    
    println!("Package downloaded to: {}", package_dir.display());
    
    Ok(())
}
```

### Working with Package Manifests

```rust
use rh_foundation::loader::PackageLoader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let loader = PackageLoader::default();
    
    // Get package metadata
    let manifest = loader
        .get_package_manifest("hl7.fhir.us.core", "5.0.1")
        .await?;
    
    println!("Package: {}", manifest.name);
    println!("Version: {}", manifest.version);
    println!("Description: {}", manifest.description.unwrap_or_default());
    
    // List dependencies
    if let Some(deps) = manifest.dependencies {
        println!("\nDependencies:");
        for (name, version) in deps {
            println!("  {} @ {}", name, version);
        }
    }
    
    Ok(())
}
```

### Download with Dependencies

```rust
use rh_foundation::loader::PackageLoader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let loader = PackageLoader::default();
    
    // Downloads package and all its dependencies
    loader
        .download_package_with_dependencies("hl7.fhir.us.core", "5.0.1")
        .await?;
    
    println!("Package and dependencies downloaded!");
    
    Ok(())
}
```

### Private Registry with Authentication

```rust
use rh_foundation::loader::{PackageLoader, LoaderConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = LoaderConfig {
        registry_url: "https://private.example.com".to_string(),
        auth_token: Some(std::env::var("PRIVATE_REGISTRY_TOKEN")?),
        ..Default::default()
    };
    
    let loader = PackageLoader::new(config);
    
    let package_dir = loader
        .download_package("my.org.profiles", "1.0.0")
        .await?;
    
    Ok(())
}
```

### Listing Available Versions

```rust
use rh_foundation::loader::PackageLoader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let loader = PackageLoader::default();
    
    // Query registry for available versions
    let versions = loader
        .list_package_versions("hl7.fhir.r4.core")
        .await?;
    
    println!("Available versions:");
    for version in versions {
        println!("  - {}", version);
    }
    
    Ok(())
}
```

## Error Handling

The loader provides detailed error types for different failure scenarios:

```rust
use rh_foundation::loader::{PackageLoader, LoaderError};

#[tokio::main]
async fn main() {
    let loader = PackageLoader::default();
    
    match loader.download_package("invalid.package", "1.0.0").await {
        Ok(path) => println!("Downloaded to: {}", path.display()),
        Err(LoaderError::PackageNotFound { package, version }) => {
            eprintln!("Package {}#{} not found in registry", package, version);
        }
        Err(LoaderError::NetworkError(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(LoaderError::AuthenticationFailed) => {
            eprintln!("Authentication failed - check your token");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Performance Characteristics

### Benchmark Results

Typical performance on a modern system with good network connectivity:

| Operation | First Run | Cached |
|-----------|-----------|--------|
| Download hl7.fhir.r4.core#4.0.1 | ~15s | <1ms |
| Download with dependencies | ~45s | <10ms |
| List package versions | ~500ms | ~50ms |
| Get package manifest | ~300ms | <1ms |

### Memory Usage

- Base loader: ~2 MB
- Per cached package manifest: ~5-10 KB
- In-memory package cache: Configurable (default: 100 MB limit)

### Disk Usage

- Core FHIR packages: ~50-100 MB each
- US Core: ~5-10 MB
- Typical workspace with 5-10 packages: ~500 MB

## Testing

The loader includes comprehensive tests:

```bash
# Run loader tests
cargo test -p rh-foundation --lib loader

# Run with HTTP feature
cargo test -p rh-foundation --features http loader
```

### Test Coverage

- ✅ Package download and extraction
- ✅ Cache hit/miss scenarios
- ✅ Authentication handling
- ✅ Network error recovery
- ✅ Concurrent downloads
- ✅ Manifest parsing
- ✅ Directory creation and cleanup

## Integration with Other Tools

### HAPI FHIR Validator

The loader is compatible with the HAPI FHIR Validator's package cache:

```bash
# Packages downloaded by rh are available to HAPI
rh download-package hl7.fhir.us.core 5.0.1

# HAPI can now use the cached package
java -jar validator_cli.jar -ig hl7.fhir.us.core#5.0.1 patient.json
```

### FHIR Shorthand (FSH) Sushi

Works seamlessly with Sushi's package management:

```yaml
# sushi-config.yaml
dependencies:
  hl7.fhir.us.core: 5.0.1
```

Both Sushi and rh will use the same cached packages in `~/.fhir/packages/`.

### VS Code FHIR Extension

The VS Code FHIR extension can discover and use packages downloaded by rh.

## Troubleshooting

### Package Not Found

**Error:** Package not found in registry

**Solutions:**
- Verify package name and version
- Check network connectivity
- Ensure registry URL is correct
- Try listing available versions

### Authentication Issues

**Error:** Authentication failed

**Solutions:**
- Verify auth token is valid and not expired
- Check token has correct permissions
- Ensure token is properly formatted (no extra whitespace)
- Test with curl: `curl -H "Authorization: Bearer TOKEN" REGISTRY_URL`

### Cache Corruption

**Error:** Invalid package in cache

**Solutions:**
```bash
# Clear cache for specific package
rm -rf ~/.fhir/packages/hl7.fhir.r4.core#4.0.1

# Clear entire cache
rm -rf ~/.fhir/packages/*
```

### Permission Errors

**Error:** Cannot write to package directory

**Solutions:**
- Check directory permissions: `ls -la ~/.fhir/packages`
- Fix permissions: `chmod 755 ~/.fhir/packages`
- Override directory with `FHIR_PACKAGES_DIR` environment variable

## Future Enhancements

Planned improvements:

- [ ] Offline mode with stale cache tolerance
- [ ] Delta downloads for package updates
- [ ] Package integrity verification with signatures
- [ ] Peer-to-peer package sharing
- [ ] Package pruning and cleanup utilities
- [ ] Mirror/proxy server support
- [ ] Package dependency visualization
- [ ] Version constraint resolution (^, ~, >=)

## References

- [FHIR Package Registry Specification](https://confluence.hl7.org/display/FHIR/NPM+Package+Specification)
- [packages.fhir.org](https://packages.fhir.org)
- [HL7 Package Registry](https://registry.fhir.org)
