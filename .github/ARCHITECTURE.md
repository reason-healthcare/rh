# Architecture

## Crate Dependency Tree

- **rh-cli** (binary - unified CLI)
  - **rh-codegen** (FHIR → Rust type generator)
    - **rh-foundation** `[http]`
  - **rh-fsh** (FSH → FHIR JSON compiler)
    - **hl7_fhir_r4_core**
  - **rh-validator** (FHIR resource validator)
    - **rh-foundation** `[http]`
    - **rh-fhirpath**
      - **rh-foundation**
      - **hl7_fhir_r4_core**
  - **rh-fhirpath** (FHIRPath expression engine)
    - **rh-foundation**
    - **hl7_fhir_r4_core**
  - **rh-vcl** (ValueSet Compose Language)
    - **rh-foundation**
  - **hl7_fhir_r4_core** (generated R4 types)

**Dependency Layers:**
- **Layer 0** (no dependencies): `rh-foundation`, `hl7_fhir_r4_core`
- **Layer 1** (foundation only): `rh-codegen`, `rh-vcl`, `rh-fhirpath`, `rh-fsh`
- **Layer 2** (depends on layer 1): `rh-validator`
- **Layer 3** (binary): `rh-cli`

## Key Crates

- **rh-foundation**: Base utilities, error types, HTTP client wrappers, package loader, snapshot generation
- **rh-codegen**: Generates Rust types from FHIR StructureDefinitions
- **rh-fhirpath**: Parser and evaluator for FHIRPath expressions
- **rh-fsh**: nom-based FSH parser and FHIR JSON exporter with rayon parallel export
- **rh-validator**: Profile-based FHIR validation with LRU caching
- **rh-vcl**: ValueSet Compose Language parser (WASM-compatible)
- **hl7_fhir_r4_core**: Pre-generated R4 FHIR types
