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

# Build and serve the shared playground locally
just demo
```

## Requirements

- `wasm-pack` (automatically installed if missing)
- Rust with `wasm32-unknown-unknown` target
- Node.js (for testing)
- pnpm (for the shared playground)

## Output Directories

- `pkg/web/` - Web target (ES6 modules)
- `pkg/node/` - Node.js target (CommonJS)  
- `pkg/bundler/` - Bundler target
- `../../examples/playground/dist/` - Shared playground build output (gitignored)

## Compatibility

This Justfile handles different Rust installations:
- Works with rustup-managed toolchains
- Works with asdf-managed Rust
- Automatically uses appropriate PATH for wasm-pack

## Shared Playground

The former standalone demo is now the shared workspace playground at
`examples/playground`. It provides an interactive web interface for:
- Parsing FHIRPath expressions
- Evaluating FHIRPath against FHIR resources
- Translating and explaining VCL
- Compiling CQL to ELM JSON

Build and run with:
```bash
just demo
```

The Vite server prints the local URL.
