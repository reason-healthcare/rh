# rh-foundation

Foundation crate providing common utilities, error handling, and shared functionality for the RH workspace.

## Overview

`rh-foundation` serves as the foundational layer for all other crates in the RH workspace, providing:

- **Error Handling**: Common error types and utilities
- **Configuration**: Traits and utilities for configuration management
- **I/O Operations**: File reading/writing with JSON support
- **HTTP Client**: Optional HTTP utilities with async support
- **JSON Utilities**: Convenient JSON parsing and serialization

## Features

- **Default**: Core functionality (error handling, config, I/O, JSON)
- **http**: Enables HTTP client utilities (requires `reqwest` and `tokio`)

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

## Module Structure

```
rh-foundation/
├── error.rs       - Error types and utilities
├── config.rs      - Configuration traits
├── io.rs          - File I/O operations
├── json.rs        - JSON utilities
└── http.rs        - HTTP client (feature: http)
```

## Migration from rh-common

This crate replaces the old `rh-common` and `rh-core` crates. To migrate:

1. Update `Cargo.toml`: Replace `rh-common` with `rh-foundation`
2. Update imports: Change `use rh_common::*` to `use rh_foundation::*`
3. Update error types: `CommonError` → `FoundationError`

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
