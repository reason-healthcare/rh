# VCL WASM Build Tasks

This justfile provides convenient tasks for building WebAssembly packages from the rh-vcl crate.

## Usage

```bash
# Show available commands
just --list

# Build all WASM targets
just wasm

# Build specific targets
just wasm-web       # ES6 modules for browsers
just wasm-node      # CommonJS for Node.js
just wasm-bundler   # For webpack/rollup/etc

# Test Node.js build
just test-wasm

# Clean build artifacts
just clean-wasm

# Development: build and serve web target
just dev-web
```

## Requirements

- `wasm-pack` (automatically installed if missing)
- Rust with `wasm32-unknown-unknown` target
- Node.js (for testing)
- Python 3 (for development server)

## Output Directories

- `pkg/web/` - Web target (ES6 modules)
- `pkg/node/` - Node.js target (CommonJS)  
- `pkg/bundler/` - Bundler target

## Compatibility

The justfile handles different Rust installations:
- Works with rustup-managed toolchains
- Works with asdf-managed Rust
- Automatically uses appropriate PATH for wasm-pack