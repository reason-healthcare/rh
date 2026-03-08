//! CQL arithmetic operators (CQL §12).
//!
//! Exports: [`add`], [`subtract`], [`multiply`], [`divide`], [`modulo`],
//! [`negate`], [`abs`], [`ceiling`], [`floor`], [`truncate`], [`round`],
//! [`power`], [`ln`], [`exp`], [`log`].

use super::super::context::EvalError;
use super::super::value::{CqlQuantity, Value};

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

macro_rules! null1 {
    ($a:expr) => {
        if matches!($a, Value::Null) {
            return Ok(Value::Null);
        }
    };
}

macro_rules! null2 {
    ($a:expr, $b:expr) => {
        if matches!($a, Value::Null) || matches!($b, Value::Null) {
            return Ok(Value::Null);
        }
    };
}

fn err(op: &str, msg: &str) -> EvalError {
    EvalError::General(format!("{op}: {msg}"))
}

fn numeric_as_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Integer(x) => Some(*x as f64),
        Value::Long(x) => Some(*x as f64),
        Value::Decimal(x) => Some(*x),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Arithmetic operators (9.10)
// ---------------------------------------------------------------------------

/// CQL `Add` (`+`): Integer + Integer → Integer, Long + Long → Long,
/// Decimal + Decimal → Decimal, Quantity + Quantity (same unit) → Quantity.
pub fn add(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x + y)),
        (Value::Long(x), Value::Long(y)) => Ok(Value::Long(x + y)),
        (Value::Decimal(x), Value::Decimal(y)) => Ok(Value::Decimal(x + y)),
        (Value::Quantity(x), Value::Quantity(y)) if x.unit == y.unit => {
            Ok(Value::Quantity(CqlQuantity {
                value: x.value + y.value,
                unit: x.unit.clone(),
            }))
        }
        // CQL String concatenation: 'a' + 'b' = 'ab'
        (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{x}{y}"))),
        _ => Err(err(
            "Add",
            &format!("unsupported operand types: {a:?} + {b:?}"),
        )),
    }
}

/// CQL `Subtract` (`-`).
pub fn subtract(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x - y)),
        (Value::Long(x), Value::Long(y)) => Ok(Value::Long(x - y)),
        (Value::Decimal(x), Value::Decimal(y)) => Ok(Value::Decimal(x - y)),
        (Value::Quantity(x), Value::Quantity(y)) if x.unit == y.unit => {
            Ok(Value::Quantity(CqlQuantity {
                value: x.value - y.value,
                unit: x.unit.clone(),
            }))
        }
        _ => Err(err(
            "Subtract",
            &format!("unsupported operand types: {a:?} - {b:?}"),
        )),
    }
}

/// CQL `Multiply` (`*`).
pub fn multiply(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x * y)),
        (Value::Long(x), Value::Long(y)) => Ok(Value::Long(x * y)),
        (Value::Decimal(x), Value::Decimal(y)) => Ok(Value::Decimal(x * y)),
        (Value::Quantity(x), Value::Quantity(y)) => Ok(Value::Quantity(CqlQuantity {
            value: x.value * y.value,
            unit: format!("{}.{}", x.unit, y.unit),
        })),
        _ => Err(err(
            "Multiply",
            &format!("unsupported operand types: {a:?} * {b:?}"),
        )),
    }
}

/// CQL `Divide` (`/`): Integer operands are promoted to Decimal; result is
/// always Decimal.  Returns `null` on divide-by-zero per CQL spec.
pub fn divide(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    let (num, den) = match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => (*x as f64, *y as f64),
        (Value::Long(x), Value::Long(y)) => (*x as f64, *y as f64),
        (Value::Decimal(x), Value::Decimal(y)) => (*x, *y),
        _ => {
            return Err(err(
                "Divide",
                &format!("unsupported operand types: {a:?} / {b:?}"),
            ))
        }
    };
    if den == 0.0 {
        return Ok(Value::Null); // CQL spec: division by zero → null
    }
    Ok(Value::Decimal(num / den))
}

/// CQL `Modulo` (`mod`): remainder after integer division.  Returns `null` on
/// modulo-by-zero.
pub fn modulo(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => {
            if *y == 0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Integer(x % y))
            }
        }
        (Value::Long(x), Value::Long(y)) => {
            if *y == 0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Long(x % y))
            }
        }
        (Value::Decimal(x), Value::Decimal(y)) => {
            if *y == 0.0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Decimal(x % y))
            }
        }
        _ => Err(err(
            "Modulo",
            &format!("unsupported operand types: {a:?} mod {b:?}"),
        )),
    }
}

/// CQL `Negate` (unary `-`).
pub fn negate(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(-x)),
        Value::Long(x) => Ok(Value::Long(-x)),
        Value::Decimal(x) => Ok(Value::Decimal(-x)),
        Value::Quantity(q) => Ok(Value::Quantity(CqlQuantity {
            value: -q.value,
            unit: q.unit.clone(),
        })),
        _ => Err(err("Negate", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Abs`: absolute value.
pub fn abs(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(x.abs())),
        Value::Long(x) => Ok(Value::Long(x.abs())),
        Value::Decimal(x) => Ok(Value::Decimal(x.abs())),
        Value::Quantity(q) => Ok(Value::Quantity(CqlQuantity {
            value: q.value.abs(),
            unit: q.unit.clone(),
        })),
        _ => Err(err("Abs", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Ceiling`: least integer greater than or equal to the argument.
/// Integer/Long pass through unchanged.
pub fn ceiling(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(*x)),
        Value::Long(x) => Ok(Value::Long(*x)),
        Value::Decimal(x) => Ok(Value::Integer(x.ceil() as i64)),
        _ => Err(err("Ceiling", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Floor`: greatest integer less than or equal to the argument.
pub fn floor(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(*x)),
        Value::Long(x) => Ok(Value::Long(*x)),
        Value::Decimal(x) => Ok(Value::Integer(x.floor() as i64)),
        _ => Err(err("Floor", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Truncate`: integer part of the argument (truncate toward zero).
pub fn truncate(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(*x)),
        Value::Long(x) => Ok(Value::Long(*x)),
        Value::Decimal(x) => Ok(Value::Integer(x.trunc() as i64)),
        _ => Err(err("Truncate", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Round`: round to `precision` decimal places (default 0).
/// Uses round-half-up semantics per the CQL specification.
///
/// `precision` must be an `Integer` value or `None` (treated as 0).
pub fn round(a: &Value, precision: Option<&Value>) -> Result<Value, EvalError> {
    null1!(a);
    let prec: i32 = match precision {
        None | Some(Value::Null) => 0,
        Some(Value::Integer(p)) => *p as i32,
        Some(other) => {
            return Err(err(
                "Round",
                &format!("precision must be Integer, got {other:?}"),
            ))
        }
    };
    let x = match a {
        Value::Decimal(x) => *x,
        Value::Integer(x) => *x as f64,
        Value::Long(x) => *x as f64,
        _ => return Err(err("Round", &format!("unsupported type: {a:?}"))),
    };
    let factor = 10f64.powi(prec);
    Ok(Value::Decimal((x * factor).round() / factor))
}

/// CQL `Power` (`^`): raises `base` to the power `exp`.  Returns Decimal.
pub fn power(base: &Value, exp: &Value) -> Result<Value, EvalError> {
    null2!(base, exp);
    let b = match base {
        Value::Integer(x) => *x as f64,
        Value::Long(x) => *x as f64,
        Value::Decimal(x) => *x,
        _ => return Err(err("Power", &format!("unsupported base type: {base:?}"))),
    };
    let e = match exp {
        Value::Integer(x) => *x as f64,
        Value::Long(x) => *x as f64,
        Value::Decimal(x) => *x,
        _ => return Err(err("Power", &format!("unsupported exp type: {exp:?}"))),
    };
    Ok(Value::Decimal(b.powf(e)))
}

/// CQL `Ln`: natural logarithm.  Returns `null` for non-positive arguments.
pub fn ln(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    let x = numeric_as_f64(a).ok_or_else(|| err("Ln", &format!("unsupported type: {a:?}")))?;
    if x <= 0.0 {
        return Ok(Value::Null);
    }
    Ok(Value::Decimal(x.ln()))
}

/// CQL `Exp`: e raised to the given power.
pub fn exp(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    let x = numeric_as_f64(a).ok_or_else(|| err("Exp", &format!("unsupported type: {a:?}")))?;
    Ok(Value::Decimal(x.exp()))
}

/// CQL `Log(argument, base)`: logarithm of `argument` in `base`.  Returns
/// `null` for non-positive argument or base.
pub fn log(a: &Value, base: &Value) -> Result<Value, EvalError> {
    null2!(a, base);
    let x = numeric_as_f64(a)
        .ok_or_else(|| err("Log", &format!("unsupported argument type: {a:?}")))?;
    let b = numeric_as_f64(base)
        .ok_or_else(|| err("Log", &format!("unsupported base type: {base:?}")))?;
    if x <= 0.0 || b <= 0.0 || b == 1.0 {
        return Ok(Value::Null);
    }
    Ok(Value::Decimal(x.log(b)))
}
