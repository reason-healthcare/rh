# rh_hl7_fhir_r5_core

**Generated FHIR Types for hl7.fhir.r5.core**

This crate contains automatically generated Rust types for FHIR (Fast Healthcare Interoperability Resources) based on the `hl7.fhir.r5.core` package.

## Important Notice

**This crate was automatically generated using the RH codegen CLI tool.**

- **Generator command**:
```bash
rh codegen hl7.fhir.r5.core 5.0.0 --output crates/rh-hl7_fhir_r5_core --force --crate-name rh-hl7-fhir-r5-core
```

- **Generation timestamp**: 2026-06-15 19:58:41.145553 -05:00

## Package Information

* **Package Name** hl7.fhir.r5.core
* **Package Author** HL7 Inc
* **Version** 5.0.0
* **Canonical URL** `http://hl7.org/fhir`

**Statistics:** 162 resources, 51 datatypes, 61 profiles, 546 extensions, and 245 required-binding enum modules.

**Version-specific notes:**

- Generated from FHIR R5 `hl7.fhir.r5.core` version `5.0.0`.
- R5-only model shapes such as `integer64` and `CodeableReference` are emitted
  from the package definitions.
- Repeating fields are emitted as `Vec<T>` with serde defaults so missing JSON
  arrays deserialize as empty vectors.
- Metadata is split under `src/metadata/` by resource, datatype, primitive,
  profile, and other categories.
- R5 extension definitions are generated under `src/extensions/`; use the
  generated `extension_by_url()` helpers from accessor traits to inspect
  extension lists.

## Description

FHIR Core package - the NPM package that contains all the definitions for the base FHIR specification (built Sun, Mar 26, 2023 15:21+1100+11:00)

## Features

- **Complete FHIR type definitions** - All resources, datatypes, and primitives
- **Serde serialization** - Built-in JSON serialization/deserialization support
- **Type metadata** - Compile-time metadata for field types and path resolution
- **Idiomatic Rust** - Clean, organized module structure with proper naming conventions
- **Zero-cost abstractions** - PHF (perfect hash function) maps for O(1) metadata lookups

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
rh_hl7_fhir_r5_core = "0.2.0"
```

### Deserializing FHIR Resources

```rust
use rh_hl7_fhir_r5_core::resources::patient::Patient;
use serde_json;

let json_data = r#"{\"resourceType\": \"Patient\", \"id\": \"example\"}"#;
let patient: Patient = serde_json::from_str(json_data)?;

println!("Patient ID: {}", patient.id.unwrap_or_default());
```

### Creating Resources Programmatically

This crate provides two idiomatic ways to work with FHIR resources using builder traits:

#### Option 1: Resource Module with Re-exported Traits (Recommended)

Each resource module re-exports its associated traits for convenience:

```rust
// Import resource with its traits - all in one place!
use rh_hl7_fhir_r5_core::resources::patient::{Patient, PatientMutators};
use rh_hl7_fhir_r5_core::prelude::*;  // Gets base traits (ResourceMutators, etc.)
use rh_hl7_fhir_r5_core::datatypes::human_name::HumanName;

// Build a patient using the builder pattern
let patient = <Patient as PatientMutators>::new()
    .set_id("patient-123".to_string())
    .set_active(true)
    .add_name(HumanName {
        family: Some("Doe".to_string()),
        given: vec!["John".to_string()],
        ..Default::default()
    })
    .set_gender(Some(AdministrativeGender::Male))
    .set_birth_date("1990-01-15".to_string());
```

#### Option 2: Prelude Module

For common base traits, use the prelude module:

```rust
use rh_hl7_fhir_r5_core::prelude::*;  // ValidatableResource, ResourceMutators, etc.
use rh_hl7_fhir_r5_core::resources::patient::{Patient, PatientMutators};

let patient = <Patient as PatientMutators>::new()
    .set_id("example".to_string());
```

The prelude includes:
- `ValidatableResource` - Access invariants and validation rules
- `ResourceMutators` - Builder methods for all resources
- `DomainResourceMutators` - Builder methods for domain resources

#### Direct Struct Construction

You can also construct resources directly:

```rust
use rh_hl7_fhir_r5_core::resources::patient::Patient;
use rh_hl7_fhir_r5_core::datatypes::human_name::HumanName;
use rh_hl7_fhir_r5_core::bindings::administrative_gender::AdministrativeGender;

let patient = Patient {
    id: Some("patient-123".to_string()),
    active: Some(true),
    name: vec![HumanName {
        family: Some("Doe".to_string()),
        given: vec!["John".to_string()],
        ..Default::default()
    }],
    gender: Some(AdministrativeGender::Male),
    birth_date: Some("1990-01-15".to_string()),
    ..Default::default()
};
```

### Using Type Metadata

This crate includes compile-time metadata for all FHIR types, enabling runtime type introspection and path resolution:

```rust
use rh_hl7_fhir_r5_core::metadata::{resolve_path, get_field_info, FhirFieldType, FhirPrimitiveType};

// Resolve nested paths to their FHIR types
if let Some(field_type) = resolve_path("Patient.birthDate") {
    match field_type {
        FhirFieldType::Primitive(FhirPrimitiveType::Date) => {
            println!("birthDate is a FHIR date type");
        }
        _ => {}
    }
}

// Resolve complex nested paths
if let Some(field_type) = resolve_path("Patient.name.given") {
    match field_type {
        FhirFieldType::Primitive(FhirPrimitiveType::String) => {
            println!("name.given is a string array");
        }
        _ => {}
    }
}

// Get field information directly
if let Some(field_info) = get_field_info("Patient", "active") {
    println!("Min cardinality: {}", field_info.min);
    println!("Max cardinality: {:?}", field_info.max);
    println!("Is choice type: {}", field_info.is_choice_type);
}
```

The metadata system enables:
- **Path resolution** - Navigate nested paths like `Patient.name.given`
- **Type introspection** - Determine field types at runtime
- **Cardinality information** - Min/max occurrence constraints
- **Choice type detection** - Identify polymorphic fields
- **Zero runtime cost** - All lookups use compile-time perfect hash maps

## Structure

This crate organizes FHIR types into logical modules:

- **resources/** - All FHIR resources (Patient, Observation, etc.)
- **profiles/** - FHIR profiles (Vitalsigns, BodyHeight, etc.)
- **datatypes/** - Complex and primitive datatypes (HumanName, Address, etc.)
- **bindings/** - ValueSet enumerations (AdministrativeGender, etc.)
- **primitives/** - Base primitive types (DateType, DateTimeType, etc.)
- **traits/** - Mutator, accessor, and existence traits for all types
- **prelude.rs** - Commonly used traits (ValidatableResource, ResourceMutators, etc.)
- **metadata/** - Type metadata split by category (resources, datatypes, primitives) for faster incremental compilation

R5-specific compatibility:

- This crate targets FHIR R5 `5.0.0`; do not use it for R4 validation or R4
  package output.
- Binding enum modules are generated for required value sets; this crate
  currently has 245 required-binding modules.

## Regenerating This Crate

To regenerate this crate with updated FHIR definitions:

```bash
rh codegen hl7.fhir.r5.core 5.0.0 --output crates/rh-hl7_fhir_r5_core --force --crate-name rh-hl7-fhir-r5-core
```

## License

This generated crate is provided under MIT OR Apache-2.0 license.

## Related Links

- [FHIR Specification](https://hl7.org/fhir/)
- [FHIR Package Registry](https://packages.fhir.org/)
- [RH Project](https://github.com/reasonhealth/rh)

---

*Generated by RH codegen tool at 2026-06-15 19:58:41.145759 -05:00*
