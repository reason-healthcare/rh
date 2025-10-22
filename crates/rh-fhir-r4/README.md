# hl7_fhir_r4_core

**Generated FHIR Types for hl7.fhir.r4.core**

This crate contains automatically generated Rust types for FHIR (Fast Healthcare Interoperability Resources) based on the `hl7.fhir.r4.core` package.

## ⚠️ Important Notice

**This crate was automatically generated using the RH codegen CLI tool.**

- **Generator command**:
```bash
rh codegen hl7.fhir.r4.core 4.0.1 --output ../generated/rh-fhir-r4 --force
```

- **Generation timestamp**: 2025-10-22 00:39:50.485653 -04:00

## 📦 Package Information

* **Package Name** hl7.fhir.r4.core
* **Package Author** HL7 Inc
* **Version** 4.0.1
* **Canonical URL** `http://hl7.org/fhir`

**📊 There are 0 structs, 0 enums, with a total of 0 types**

## 📚 Description

Definitions (API, structures and terminologies) for the R4 version of the FHIR standard

## 🚀 Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
hl7_fhir_r4_core = "0.1.0"
```

Example usage:

### Deserializing from JSON

```rust
use hl7_fhir_r4_core::*;
use serde_json;

// All types include serde serialization support
let json_data = r#"{\"resourceType\": \"Patient\", \"id\": \"example\"}"#;
let patient: Patient = serde_json::from_str(json_data)?;

println!("Patient ID: {}", patient.id.unwrap_or_default());
```

### Creating resources procedurally

```rust
use hl7_fhir_r4_core::*;
use serde_json;

// Create a new Patient resource
let patient = Patient {
    id: Some("patient-123".to_string()),
    meta: None,
    implicit_rules: None,
    language: None,
    text: None,
    contained: vec![],
    extension: vec![],
    modifier_extension: vec![],
    active: Some(true),
    name: vec![HumanName {
        family: Some("Doe".to_string()),
        given: vec!["John".to_string()],
        ..Default::default()
    }],
    gender: Some(AdministrativeGender::Male),
    ..Default::default()
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&patient)?;
println!("Patient JSON: {}", json);
```

## 🏗️ Structure

This crate uses an idiomatic Rust module structure organized by FHIR type category:

- **`src/resources/`** - FHIR resource types (Patient, Observation, Bundle, etc.)
- **`src/profiles/`** - FHIR profiles derived from core resources (vital signs, BMI, etc.)
- **`src/datatypes/`** - FHIR data types (Narrative, Extension, Coding, etc.)
- **`src/primitives/`** - FHIR primitive types (string, boolean, dateTime, etc.)
- **`src/traits/`** - Common trait interfaces for FHIR functionality
- **`src/lib.rs`** - Main library file that re-exports all modules

## 🔄 Regeneration

To regenerate this crate with updated FHIR definitions, run:

```bash
rh codegen hl7.fhir.r4.core 4.0.1 --output ../generated/rh-fhir-r4 --force
```

## ⚖️ License

This generated crate is provided under MIT OR Apache-2.0 license.

## 🔗 Related Links

- [FHIR Specification](https://hl7.org/fhir/)
- [RH Codegen Tool](https://github.com/reason-healthcare/rh)
