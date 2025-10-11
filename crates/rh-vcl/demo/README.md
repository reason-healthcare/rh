# VCL Parser Demo# VCL Parser Demo



This is a minimal web demo of the ValueSet Compose Language (VCL) parser and FHIR translation capabilities.This is a minimal web demo of the ValueSet Compose Language (VCL) parser and FHIR translation capabilities.



## Quick Start## Quick Start



### Build and Run Demo### Build and Run Demo



```bash```bash

# From the rh-vcl crate directory# From the rh-vcl crate directory

just build-demojust build-demo



# Serve locally# Serve locally

just demojust demo

# Or manually: cd build && python3 -m http.server 8000# Or manually: cd build && python3 -m http.server 8000

``````



### Docker### Docker



```bash```bash

# Build Docker image# Build Docker image

just build-dockerjust build-docker



# Run container# Run container

docker run -p 8080:80 rh-vcl-demodocker run -p 8080:80 rh-vcl-demo

``````



Then open http://localhost:8000 (or 8080 for Docker) in your browser.Then open http://localhost:8000 (or 8080 for Docker) in your browser.



## Features## Features



- **Parse VCL expressions** - See the Abstract Syntax Tree (AST) structure- **Parse VCL expressions** - See the Abstract Syntax Tree (AST) structure

- **Translate to FHIR** - Convert VCL to FHIR ValueSet.compose format- **Translate to FHIR** - Convert VCL to FHIR ValueSet.compose format

- **Configurable default system** - Set default system URI for codes without explicit system- **Configurable default system** - Set default system URI for codes without explicit system

- **Interactive examples** - Click example buttons to try different VCL patterns- **Interactive examples** - Click example buttons to try different VCL patterns

- **Real-time feedback** - Immediate parsing and translation results- **Real-time feedback** - Immediate parsing and translation results

- **Simple interface** - Clean, streamlined UI with working functionality- **Simple interface** - Clean, streamlined UI with working functionality

- **Bookmarkable URLs** - Expression and system parameters saved in URL for easy sharing

## Example VCL Expressions

## URL Parameters

- **Simple code**: `123456`

The demo supports URL parameters for bookmarking and sharing specific configurations:- **Code with system**: `(http://snomed.info/sct)123456`

- **Property filter**: `status = "active"`

- `expr` or `expression` - The VCL expression to parse/translate- **Is-a relationship**: `category << 123456`

- `system` or `default_system` - The default system URI- **Conjunction**: `(123456, 789012)`

- **Disjunction**: `123456; 789012`

### Examples- **Filter list**: `{status = "active", category << 123456}`



```## Technical Details

# Simple code with SNOMED system

http://localhost:8000?expr=123456&system=http://snomed.info/sct- **Frontend**: Vanilla HTML/CSS/JavaScript (ES6 modules)

- **WASM**: Built with wasm-pack targeting web browsers

# Property filter- **No bundler required** - Uses native ES6 module imports

http://localhost:8000?expr=status%20%3D%20%22active%22- **Containerized**: Nginx Alpine container for easy deployment



# Complex filter list## Files

http://localhost:8000?expr=%7Bstatus%20%3D%20%22active%22%2C%20category%20%3C%3C%20123456%7D

```- `demo/index.html` - Main demo page source

- `demo/Dockerfile` - Container configuration

## Example VCL Expressions- `build/` - Generated static files (gitignored)

  - `index.html` - Demo page

- **Simple code**: `123456`  - `rh_vcl.js` - WASM JavaScript bindings

- **Code with system**: `(http://snomed.info/sct)123456`  - `rh_vcl_bg.wasm` - WebAssembly binary

- **Property filter**: `status = "active"`

- **Is-a relationship**: `category << 123456`## API Reference

- **Conjunction**: `(123456, 789012)`

- **Disjunction**: `123456; 789012`### Available Functions (Used in Demo)

- **Filter list**: `{status = "active", category << 123456}`

- **Simple functions**: `parse_vcl_simple()`, `translate_vcl_simple()`, `translate_vcl_with_system()`

## Technical Details- **Utilities**: `get_version()`



- **Frontend**: Vanilla HTML/CSS/JavaScript (ES6 modules)All functions return `WasmResult` objects with `success`, `data`, and `error` properties.

- **WASM**: Built with wasm-pack targeting web browsers

- **No bundler required** - Uses native ES6 module imports**Usage Examples:**

- **Containerized**: Nginx Alpine container for easy deployment```javascript

- **URL state management** - Automatic parameter synchronization with browser history// Simple parsing

const parseResult = parse_vcl_simple('123456');

## API Referenceif (parseResult.success) {

    const ast = JSON.parse(parseResult.data);

### Available Functions (Used in Demo)}



- **Simple functions**: `parse_vcl_simple()`, `translate_vcl_simple()`, `translate_vcl_with_system()`// Translation with default system

- **Utilities**: `get_version()`const translateResult = translate_vcl_with_system('123456', 'http://snomed.info/sct');



All functions return `WasmResult` objects with `success`, `data`, and `error` properties.// Translation without system (will use default if provided in UI)

const simpleResult = translate_vcl_simple('(http://snomed.info/sct)123456');

**Usage Examples:**```

```javascript

// Simple parsing## Development

const parseResult = parse_vcl_simple('123456');

if (parseResult.success) {The demo is built from the main VCL parser crate and uses the web-targeted WASM build.

    const ast = JSON.parse(parseResult.data);All VCL parsing and translation functionality is exposed through WASM bindings with

}the same parameter options as the CLI tool.

// Translation with default system
const translateResult = translate_vcl_with_system('123456', 'http://snomed.info/sct');

// Translation without system (will use default if provided in UI)
const simpleResult = translate_vcl_simple('(http://snomed.info/sct)123456');
```

## Files

- `demo/index.html` - Main demo page source with URL parameter support
- `demo/Dockerfile` - Container configuration
- `build/` - Generated static files (gitignored)
  - `index.html` - Demo page
  - `rh_vcl.js` - WASM JavaScript bindings
  - `rh_vcl_bg.wasm` - WebAssembly binary

## Development

The demo is built from the main VCL parser crate and uses the web-targeted WASM build.
All VCL parsing and translation functionality is exposed through WASM bindings with
the same parameter options as the CLI tool.

### URL State Management

The demo automatically:
- Loads expression and system from URL parameters on page load
- Updates URL parameters when inputs change (with debouncing)
- Synchronizes URL state when using example buttons or parsing/translating
- Preserves bookmark-friendly URLs by excluding default values