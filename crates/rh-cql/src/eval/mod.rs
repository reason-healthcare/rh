//! # Eval Engine
//!
//! ELM evaluation engine with a runtime [`Value`] model, three-valued logic,
//! and provider traits.
//!
//! The evaluator takes [`crate::elm::Library`] as input and evaluates named
//! expressions against an [`EvalContext`].  This makes the engine compatible
//! with ELM produced by any conformant compiler, not just rh-cql.
//!
//! ## Submodules (planned)
//!
//! - [`value`]   — Runtime [`Value`] enum and CQL date/time types
//! - [`tvl`]     — Three-valued logic (And, Or, Not, Implies, Xor)
//!
//! Future phases will add:
//! - `context`   — `EvalContext`, `DataProvider`, `TerminologyProvider`
//! - `operators` — Arithmetic, comparison, string, date/time operators
//! - `intervals` — Interval operators
//! - `lists`     — List operators
//! - `queries`   — Query evaluation
//! - `engine`    — `evaluate_elm` / `evaluate_elm_with_trace` entry points

pub mod tvl;
pub mod value;

pub use tvl::{tvl_and, tvl_implies, tvl_not, tvl_or, tvl_xor};
pub use value::{CqlCode, CqlConcept, CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value};
