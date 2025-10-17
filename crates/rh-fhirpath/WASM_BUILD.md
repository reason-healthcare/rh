# FHIRPath WASM Build Tasks

This Justfile provides convenient tasks for building WebAssembly packages from the rh-fhirpath crate.

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

# Build and serve demo locally
just demo
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
- `build/` - Demo application files (gitignored)

## Compatibility

This Justfile handles different Rust installations:
- Works with rustup-managed toolchains
- Works with asdf-managed Rust
- Automatically uses appropriate PATH for wasm-pack

## Demo Application

The demo application provides an interactive web interface for:
- Parsing FHIRPath expressions
- Evaluating FHIRPath against FHIR resources
- Real-time syntax validation
- Example FHIR Patient resources
- JSON output visualization

Build and run with:
```bash
just demo
```

Then open http://localhost:8000 in your browser.
