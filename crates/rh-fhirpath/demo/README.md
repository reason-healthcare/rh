# FHIRPath WASM Demo

This directory contains a demo application for the FHIRPath WASM engine.

## Files

- `index.html` - Interactive web demo for FHIRPath parsing and evaluation

## Building

From the `crates/rh-fhirpath` directory:

```bash
just build-demo
```

This will:
1. Build the WASM package for web target
2. Copy necessary files to `build/` directory
3. Create a ready-to-use demo application

## Running

### Option 1: Using Just
```bash
just demo
```

### Option 2: Manual
```bash
just build-demo
cd build && python3 -m http.server 8000
```

Then open http://localhost:8000 in your browser.

## Features

- **Parse FHIRPath Expressions**: Test FHIRPath expression syntax
- **Evaluate Against Resources**: Run FHIRPath queries on FHIR resources
- **Real-time Validation**: Instant syntax checking
- **Example Resources**: Pre-loaded FHIR Patient examples
- **JSON Visualization**: Pretty-printed output

## Usage

1. Enter a FHIRPath expression (e.g., `Patient.name.given`)
2. Provide a FHIR resource as JSON (or use examples)
3. Click "Parse" to validate syntax
4. Click "Evaluate" to run the expression against the resource
5. View results in formatted JSON

## Examples

The demo includes example FHIRPath expressions:
- `Patient.name.given` - Extract given names
- `Patient.name.family` - Extract family names
- `Patient.birthDate` - Get birth date
- `Patient.telecom.where(system='email').value` - Filter email addresses
- `Patient.address.city` - Get city from addresses
