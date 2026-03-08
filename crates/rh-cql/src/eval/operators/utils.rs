//! Shared null-propagation macros and error helper for CQL operator submodules.

use super::super::context::EvalError;

/// Early-return `Ok(Value::Null)` when the single operand is `Value::Null`.
///
/// The operand must be a local variable of type `Value`, and the enclosing
/// function must return `Result<Value, EvalError>`.
macro_rules! null1 {
    ($a:expr) => {
        if matches!($a, Value::Null) {
            return Ok(Value::Null);
        }
    };
}
pub(super) use null1;

/// Early-return `Ok(Value::Null)` when either operand is `Value::Null`.
///
/// The operands must be local variables of type `Value`, and the enclosing
/// function must return `Result<Value, EvalError>`.
macro_rules! null2 {
    ($a:expr, $b:expr) => {
        if matches!($a, Value::Null) || matches!($b, Value::Null) {
            return Ok(Value::Null);
        }
    };
}
pub(super) use null2;

/// Construct an `EvalError::General` with operator context.
pub(super) fn err(op: &str, msg: &str) -> EvalError {
    EvalError::General(format!("{op}: {msg}"))
}
