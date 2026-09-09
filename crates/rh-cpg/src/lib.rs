//! Clinical Practice Guidelines (CPG) support for FHIR resources.

pub mod error;
pub mod measure;
pub mod questionnaire;
pub mod resolver;

pub mod apply;

pub mod context;
pub mod expression;
pub mod fhir_to_cql;

// The wasm module requires the optional wasm-bindgen dependency, so gate
// it on the `wasm` feature as well; downstream wasm32 builds without the
// feature then skip the module instead of failing to resolve wasm_bindgen.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub mod wasm;
