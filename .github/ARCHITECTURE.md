# Architecture

## Crate Dependency Graph

```
Layer 0 (leaf): rh-foundation, rh-hl7_fhir_r4_core, rh-hl7_fhir_r5_core
Layer 1: rh-codegen   (→ foundation)
         rh-fsh       (→ foundation)
         rh-vcl       (→ foundation)
         rh-fhirpath  (→ foundation, r4_core)
         rh-cql       (→ foundation)
Layer 2: rh-validator (→ foundation[http], fhirpath)
Layer 3: rh-packager  (→ foundation, fhirpath, vcl, validator)
Layer 4: rh-cli       (→ all)
```

**Dependency Layers:**
- **Layer 0** (no internal dependencies): `rh-foundation`, `rh-hl7_fhir_r4_core`, `rh-hl7_fhir_r5_core`
- **Layer 1** (foundation, generated types): `rh-codegen`, `rh-vcl`, `rh-fhirpath`, `rh-fsh`, `rh-cql`
- **Layer 2**: `rh-validator`
- **Layer 3**: `rh-packager`
- **Layer 4** (binary): `rh-cli`

## Key Crates

- **rh-foundation**: Base utilities, error types, HTTP client wrappers, package loader, snapshot generation, in-memory caching
- **rh-codegen**: Generates Rust types from FHIR StructureDefinitions
- **rh-fhirpath**: Parser and evaluator for FHIRPath expressions (WASM-capable)
- **rh-cql**: CQL-to-ELM compiler, evaluator, explain mode, and source maps
- **rh-fsh**: nom-based FSH parser and FHIR JSON exporter with rayon parallel export
- **rh-validator**: Profile-based FHIR validation with LRU caching
- **rh-vcl**: ValueSet Compose Language parser, translator, and explainer (WASM-capable)
- **rh-packager**: FHIR Package assembler with built-in processors (snapshot, validate, CQL, FSH)
- **rh-hl7_fhir_r4_core**: Pre-generated R4 FHIR types
- **rh-hl7_fhir_r5_core**: Pre-generated R5 FHIR types
