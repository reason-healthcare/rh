# Idiomatic Directory Structure for FHIR Crate Generation

This document describes the new idiomatic directory structure feature added to the RH codegen tool. This feature organizes generated FHIR types into logical modules following Rust best practices.

## Overview

The new `--organized` flag enables the generation of FHIR crates with an idiomatic directory structure that separates different types of FHIR elements into logical modules:

```
fhir-model/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── bindings/
│   │   ├── mod.rs
│   │   ├── observation_status.rs
│   │   ├── allergy_category.rs
│   │   ├── encounter_class.rs
│   │   ├── administrative_gender.rs
│   │   ├── publication_status.rs
│   │   └── ...
│   ├── datatypes/
│   │   ├── mod.rs
│   │   ├── narrative.rs
│   │   ├── extension.rs
│   │   ├── coding.rs
│   │   └── codeable_concept.rs
│   │   └── ...
│   ├── primitives/
│   │   ├── mod.rs
│   │   ├── string.rs
│   │   ├── boolean.rs
│   │   └── date_time.rs
│   │   └── ...
│   ├── resource/
│   │   ├── mod.rs
│   │   ├── observation.rs
│   │   ├── patient.rs
│   │   └── bundle.rs
│   │   └── ...
│   ├── traits/
│   │   ├── mod.rs
│   │   ├── resource.rs
│   │   ├── domain_resource.rs
│   │   ├── has_extensions.rs
│   │   └── has_coding.rs
│   │   └── ...
│   └── value.rs
```

## Usage

### CLI Command

To use the organized directory structure, add the `--organized` flag to your codegen commands:

```bash
# Install a FHIR package with organized structure
rh codegen install hl7.fhir.r4.core 4.0.1 --generate-crate --organized --output ./my-fhir-crate

# Generate from existing StructureDefinitions with organized structure  
rh codegen batch --input-dir ./fhir-definitions --output-dir ./my-fhir-types --organized
```

### Programmatic Usage

You can also use the organized directory generation programmatically:

```rust
use rh_codegen::{CodeGenerator, CodegenConfig};

let mut generator = CodeGenerator::new(CodegenConfig::default());

// Generate to organized directories
generator.generate_to_organized_directories(&structure_def, &src_dir)?;

// Generate traits to organized directories
generator.generate_trait_to_organized_directory(&structure_def, &src_dir)?;
```

## Directory Classification

The codegen tool automatically classifies FHIR types into the appropriate directories:

### `src/bindings/`
Contains FHIR ValueSet enums and code bindings such as:
- **ObservationStatus** - Status of the observation (final, preliminary, etc.)
- **AllergyIntoleranceCategory** - Category of allergy or intolerance
- **AdministrativeGender** - Gender for administrative purposes
- **PublicationStatus** - Publication status of FHIR resources
- **EncounterStatus** - Current state of the encounter
- All other FHIR ValueSet enumerations and code system bindings

### `src/resource/`
Contains FHIR resource types such as:
- **Patient** - Demographics and administrative information
- **Observation** - Measurements and assertions  
- **Bundle** - Collection of resources
- **DiagnosticReport** - Clinical reports
- All other FHIR resources with `kind: "resource"`

### `src/datatypes/`
Contains FHIR data types such as:
- **Narrative** - Human-readable text
- **Extension** - Additional data elements
- **Coding** - Code from a terminology system
- **CodeableConcept** - Concept represented by codes
- **Identifier** - Identifier for objects
- **HumanName** - Person name information
- **Address** - Physical location
- All complex types with `kind: "complex-type"` that extend Element

### `src/primitives/`
Contains FHIR primitive types such as:
- **boolean** - True/false values
- **string** - Text data
- **integer** - Whole numbers
- **decimal** - Decimal numbers
- **dateTime** - Date and time
- **uri** - Uniform resource identifiers
- All types with `kind: "primitive-type"`

### `src/traits/`
Contains trait definitions for common FHIR functionality:
- **Resource** - Base resource interface
- **DomainResource** - Resources with narrative
- **HasExtensions** - Types that can have extensions
- **HasCoding** - Types that use coding systems
- Generated traits based on StructureDefinitions

## Module Structure

Each directory contains a `mod.rs` file that:
- Documents the module purpose
- Declares all sub-modules
- Re-exports all public types for convenience

### Example `src/lib.rs`

```rust
//! Generated FHIR types
//!
//! This crate contains generated Rust types for FHIR resources organized by category.
//! All types include serde serialization support.

pub use serde::{Deserialize, Serialize};

/// FHIR ValueSet enums and code system bindings
pub mod bindings;

/// FHIR resource types (Patient, Observation, etc.)
pub mod resource;

/// FHIR data types (Narrative, Extension, etc.)
pub mod datatypes;

/// FHIR primitive types (string, boolean, etc.)
pub mod primitives;

/// FHIR traits for common functionality
pub mod traits;

/// Re-export everything for convenience
pub use bindings::*;
pub use resource::*;
pub use datatypes::*;
pub use primitives::*;
pub use traits::*;
```

### Example `src/resource/mod.rs`

```rust
//! FHIR resource types

pub mod patient;
pub mod observation;
pub mod bundle;

pub use patient::*;
pub use observation::*;
pub use bundle::*;
```

## Benefits

### 1. **Better Organization**
- Logical separation of concerns
- Easier to find specific types
- Reduced cognitive load when working with large FHIR packages

### 2. **Improved Compile Times**
- Parallel compilation of separate modules
- Reduced dependencies between unrelated types
- Better incremental compilation

### 3. **Enhanced Developer Experience**
- IDE auto-completion works better with smaller modules
- Easier code navigation and exploration
- More maintainable code organization

### 4. **Rust Best Practices**
- Follows standard Rust module conventions
- Enables selective imports (`use my_fhir::resource::Patient`)
- Better encapsulation and namespace management

### 5. **Scalability**
- Handles large FHIR packages (like R4 core) more efficiently
- Easier to extend and modify specific type categories
- Better support for partial package usage

## Comparison with Flat Structure

### Traditional Flat Structure
```
src/
├── lib.rs
├── patient.rs
├── observation.rs
├── bundle.rs
├── narrative.rs
├── extension.rs
├── coding.rs
├── boolean.rs
├── string.rs
└── ... (hundreds of files)
```

### New Organized Structure
```
src/
├── lib.rs
├── bindings/
│   ├── mod.rs
│   ├── observation_status.rs
│   ├── allergy_category.rs
│   └── administrative_gender.rs
├── resource/
│   ├── mod.rs
│   ├── patient.rs
│   ├── observation.rs
│   └── bundle.rs
├── datatypes/
│   ├── mod.rs
│   ├── narrative.rs
│   ├── extension.rs
│   └── coding.rs
├── primitives/
│   ├── mod.rs
│   ├── boolean.rs
│   └── string.rs
└── traits/
    ├── mod.rs
    ├── resource.rs
    └── domain_resource.rs
```

## Examples

### Generated Patient Resource

```rust
// src/resource/patient.rs
use crate::datatypes::{Extension, HumanName, Identifier};
use crate::primitives::boolean;
use crate::bindings::AdministrativeGender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: Option<String>,
    pub active: Option<boolean>,
    pub name: Vec<HumanName>,
    pub identifier: Vec<Identifier>,
    pub gender: Option<AdministrativeGender>,
    // ... other fields
}
```

### Generated ValueSet Enum

```rust
// src/bindings/administrative_gender.rs
use serde::{Deserialize, Serialize};

/// The gender of a person used for administrative purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdministrativeGender {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "unknown")]
    Unknown,
}
```

### Generated Extension Data Type

```rust
// src/datatypes/extension.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub url: String,
    pub value: Option<serde_json::Value>,
    // ... other fields
}
```

### Generated Resource Trait

```rust
// src/traits/resource.rs
use crate::datatypes::Meta;

pub trait Resource {
    fn resource_type(&self) -> &'static str;
    fn id(&self) -> Option<&str>;
    fn meta(&self) -> Option<&Meta>;
}
```

## Migration Guide

If you have existing code that uses the flat structure, migrating to the organized structure is straightforward:

### Before (Flat Structure)
```rust
use my_fhir_crate::{Patient, Observation, Extension, AdministrativeGender};
```

### After (Organized Structure)
```rust
// Option 1: Use re-exports (no change needed)
use my_fhir_crate::{Patient, Observation, Extension, AdministrativeGender};

// Option 2: Use specific modules (more explicit)
use my_fhir_crate::resource::{Patient, Observation};
use my_fhir_crate::datatypes::Extension;
use my_fhir_crate::bindings::AdministrativeGender;

// Option 3: Mixed approach
use my_fhir_crate::resource::*;
use my_fhir_crate::datatypes::Extension;
use my_fhir_crate::bindings::*;
```

## Implementation Details

The organized directory generation is implemented through several key components:

1. **FhirTypeCategory Enum** - Classifies FHIR types into categories
2. **generate_to_organized_directories()** - Generates structs to appropriate directories
3. **generate_trait_to_organized_directory()** - Generates traits to the traits directory
4. **organize_generated_files()** - Moves existing generated files into organized structure
5. **generate_module_files()** - Creates mod.rs files for each directory

The classification logic examines:
- FHIR kind (`resource`, `complex-type`, `primitive-type`)
- Base type (`DomainResource`, `Element`, etc.)
- Type name patterns and conventions
- File content analysis for edge cases

## Future Enhancements

Potential future improvements include:

1. **Custom Classification Rules** - Allow users to define custom type categorization
2. **Selective Module Generation** - Generate only specific modules (e.g., only resources)
3. **Cross-Module Dependencies** - Better handling of dependencies between modules
4. **Documentation Generation** - Automatic generation of module documentation
5. **Benchmark Comparisons** - Performance comparisons between flat and organized structures

## Contributing

To contribute to the organized directory structure feature:

1. Check the implementation in `crates/rh-codegen/src/generator.rs`
2. Update classification logic in `classify_fhir_structure_def()`
3. Add new directory categories if needed
4. Update the CLI in `apps/rh-cli/src/codegen.rs`
5. Add tests and examples

## Conclusion

The idiomatic directory structure feature brings modern Rust module organization to FHIR code generation, making generated crates more maintainable, performant, and developer-friendly. It follows Rust conventions while respecting FHIR's logical type hierarchy, providing the best of both worlds.
