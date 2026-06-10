//! Generated FHIR Rust bindings
//!
//! This crate contains Rust types and traits for FHIR resources and data types.
//! It includes macros for primitive field generation and maintains FHIR compliance.

// Allow clippy lint for derivable Default implementations
//
// TODO: Future optimization - derive Default when possible instead of manual impl
//
// Currently, we generate explicit Default implementations for all structs.
// Many of these could use #[derive(Default)] instead, which would be more idiomatic.
//
// Pros of deriving Default:
// - More idiomatic Rust code
// - Less generated code (no manual impl blocks)
// - Clearer intent (all fields use Default::default())
//
// Cons of current approach (manual impl):
// - Clippy warns about 1,100+ derivable implementations
// - More verbose generated code
//
// Pros of current approach:
// - Explicit and predictable behavior
// - Handles mixed initialization patterns consistently
// - Simpler code generation logic
//
// To implement derive-based approach would require:
// 1. Analyze all field types to ensure they implement Default
// 2. Detect required fields with non-Default initializations (String::new(), Vec::new(), etc.)
// 3. Add "Default" to struct derives only when ALL fields can use Default::default()
// 4. Skip manual impl generation for those structs
//
#![allow(clippy::derivable_impls)]

pub mod bindings;
pub mod datatypes;
pub mod extensions;
pub mod macros;
pub mod metadata;
pub mod prelude;
pub mod primitives;
pub mod profiles;
pub mod resources;
pub mod traits;
pub mod validation;

pub use serde::{Deserialize, Serialize};
