# VCL WASM Build Tasks

This Makefile provides convenient tasks for building WebAssembly packages from the rh-vcl crate.

## Usage

```bash
# Show available commands
make help

# Build all WASM targets
make wasm

# Build specific targets
make wasm-web       # ES6 modules for browsers
make wasm-node      # CommonJS for Node.js
make wasm-bundler   # For webpack/rollup/etc

# Test Node.js build
make test-wasm

# Clean build artifacts
make clean-wasm

# Development: build and serve web target
make dev-web
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

This Makefile handles different Rust installations:
- Works with rustup-managed toolchains
- Works with asdf-managed Rust
- Automatically uses appropriate PATH for wasm-pack