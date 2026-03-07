//! # Eval Engine
//!
//! ELM evaluation engine with a runtime [`Value`] model, three-valued logic,
//! and provider traits.
//!
//! The evaluator takes [`crate::elm::Library`] as input and evaluates named
//! expressions against an [`EvalContext`].  This makes the engine compatible
//! with ELM produced by any conformant compiler, not just rh-cql.
//!
//! ## Submodules
//!
//! - [`value`]   — Runtime [`Value`] enum and CQL date/time types
//! - [`tvl`]     — Three-valued logic (And, Or, Not, Implies, Xor)
//! - [`context`] — [`EvalContext`], [`DataProvider`], [`TerminologyProvider`],
//!                 [`InMemoryDataProvider`], [`InMemoryTerminologyProvider`]
//! - [`operators`] — Arithmetic and comparison operators (§12 / §5)
//!
//! Future phases will add:
//! - `intervals` — Interval operators
//! - `lists`     — List operators
//! - `queries`   — Query evaluation
//! - `engine`    — `evaluate_elm` / `evaluate_elm_with_trace` entry points

pub mod context;
pub mod operators;
pub mod tvl;
pub mod value;

pub use context::{
    Clock, DataProvider, EvalContext, EvalContextBuilder, EvalError, FixedClock,
    InMemoryDataProvider, InMemoryTerminologyProvider, TerminologyProvider,
};
pub use tvl::{tvl_and, tvl_implies, tvl_not, tvl_or, tvl_xor};
pub use value::{CqlCode, CqlConcept, CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value};
pub use operators::{
    abs, add, ceiling, divide, equal, equivalent, exp, floor, greater, greater_or_equal,
    less, less_or_equal, ln, log, max_value, min_value, modulo, multiply, negate, power,
    predecessor, round, subtract, successor, truncate,
};
