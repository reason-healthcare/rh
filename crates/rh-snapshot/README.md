# rh-snapshot

FHIR StructureDefinition snapshot generation library.

## Overview

This crate provides high-performance snapshot generation from FHIR StructureDefinition resources by merging base definitions with profile differentials. It handles complex FHIR features including multi-level inheritance, slicing, extensions, and comprehensive property merging with automatic caching for optimal performance.

## Status

✅ **Feature Complete** - Phases 1-9 implemented with 103 passing tests

## Features

- ✅ Load StructureDefinition resources from JSON, files, directories, or FHIR packages
- ✅ Generate snapshots from base + differential with full FHIR compliance
- ✅ Handle multi-level inheritance chains (e.g., QICorePatient → USCorePatient → Patient → DomainResource → Resource)
- ✅ Merge element properties: cardinality, types, bindings, constraints, text properties, flags
- ✅ Full slicing support with discriminators, rules, ordering, and automatic slice child expansion
- ✅ Extension handling as specialized slicing (e.g., US Core race, ethnicity, birthsex)
- ✅ Circular dependency detection with clear error messages
- ✅ Validation of StructureDefinition consistency (cardinality, binding strength, constraints, types)
- ✅ Automatic snapshot caching with O(1) lookup for repeated calls
- ✅ CLI integration with `rh snapshot generate` and `rh snapshot info` commands

## Quick Start

```rust
use rh_snapshot::types::{Differential, ElementDefinition, Snapshot, StructureDefinition};
use rh_snapshot::SnapshotGenerator;

fn main() -> anyhow::Result<()> {
    let mut generator = SnapshotGenerator::new();

    // Load base Patient resource
    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot { element: vec![/* ... */] }),
        differential: None,
    };
    generator.load_structure_definition(patient);

    // Load US Core Patient profile
    let us_core = StructureDefinition {
        url: "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
        name: "USCorePatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        differential: Some(Differential { element: vec![/* ... */] }),
        snapshot: None,
    };
    generator.load_structure_definition(us_core);

    // Generate snapshot (automatically handles inheritance, merging, caching)
    let snapshot = generator.generate_snapshot(
        "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
    )?;

    println!("Generated {} elements", snapshot.element.len());
    Ok(())
}
```

## Examples

Run comprehensive examples demonstrating all features:

```bash
# Basic usage tutorial
cargo run --example basic_usage

# Multi-level inheritance (5 levels deep)
cargo run --example multi_level_inheritance

# Array slicing (Patient.identifier → MRN, SSN, DL)
cargo run --example slicing_example

# Extensions (US Core race, ethnicity, birthsex)
cargo run --example extension_example
```

## CLI Usage

```bash
# Generate snapshot from FHIR packages
rh snapshot generate \
  --package hl7.fhir.r4.core@4.0.1 \
  --package hl7.fhir.us.core@6.1.0 \
  --output us-core-patient-snapshot.json \
  http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient

# Display snapshot info
rh snapshot info \
  --package hl7.fhir.r4.core@4.0.1 \
  --package hl7.fhir.us.core@6.1.0 \
  http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient
```

## Performance

- **First generation**: O(n) where n = elements in inheritance chain
- **Cached generation**: O(1) hash lookup
- **Memory**: O(m) where m = unique profiles loaded
- **Inheritance optimization**: Base profiles cached once, shared by all derived profiles
- **Example**: QICorePatient → USCorePatient → Patient
  - First call: Generates all 3, caches all 3
  - Subsequent calls: Instant cache hits

## Implementation Status

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1 | ✅ | Core infrastructure and types |
| Phase 2 | ✅ | Basic snapshot generation |
| Phase 3 | ✅ | Property merging (cardinality, types, bindings, constraints) |
| Phase 4 | ✅ | Path handling and nested elements |
| Phase 5 | ✅ | Slicing support with discriminators |
| Phase 6 | ✅ | Extension handling |
| Phase 7 | ✅ | Validation and edge cases |
| Phase 8 | ✅ | Optimization and caching |
| Phase 9 | ✅ | Integration and examples |
| Phase 10 | 🔄 | Documentation and production readiness |

**Test Coverage**: 103 passing tests across 12 test files

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SnapshotGenerator                        │
├─────────────────────────────────────────────────────────────┤
│  • Registry: HashMap<URL, StructureDefinition>              │
│  • Cache: RefCell<HashMap<URL, Snapshot>>                   │
│  • generate_snapshot(url) → Snapshot                        │
│    ├─ Check cache (O(1) if cached)                          │
│    ├─ Resolve base recursively                              │
│    ├─ Get base snapshot (recursive, cached)                 │
│    ├─ Merge base + differential                             │
│    └─ Cache result                                          │
└─────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                      ElementMerger                          │
├─────────────────────────────────────────────────────────────┤
│  • merge_elements(base[], diff[]) → merged[]                │
│    ├─ Index by (path, slice_name)                           │
│    ├─ Merge properties with validation                      │
│    ├─ Handle slicing & extensions                           │
│    ├─ Auto-expand slice children                            │
│    └─ Validate constraints                                  │
└─────────────────────────────────────────────────────────────┘
```

See [TODO.md](TODO.md) for detailed implementation plan and phase descriptions.

## Future Enhancements

The following features are deferred for future releases:

### Advanced Features
- **Reslicing**: Support for reslicing already-sliced arrays (advanced FHIR feature)
- **Fixed/Pattern Values**: Handle fixed[x] and pattern[x] element constraints (rarely used in practice)

### CLI Commands
- **`rh snapshot diff <url1> <url2>`**: Compare two profile snapshots
  - Show element differences side-by-side
  - Highlight added/removed/modified elements
  - Useful for analyzing profile differences

- **`rh snapshot validate <file>`**: Validate snapshot completeness
  - Check for missing required elements
  - Verify cardinality constraints are satisfied
  - Note: Full validation is handled by rh-validator

### Integration Features
- **Auto-generate on load**: Generate snapshots automatically when loading packages
  - Current: On-demand generation (more flexible)
  - Future: Configurable auto-generation option

- **Persistent caching**: Write generated snapshots back to FHIR packages
  - Current: In-memory cache (sufficient for most use cases)
  - Future: Disk-based cache for very large package sets

### rh-validator Integration
- Extract validation rules from snapshots
- Generate validation metadata automatically
- Provide terminology bindings for code validation
- Expose FHIRPath constraints for invariant evaluation

These features are not critical for the core use case (snapshot generation for validation) but may be added based on user feedback and requirements.
