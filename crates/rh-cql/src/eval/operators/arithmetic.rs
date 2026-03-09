//! CQL arithmetic operators (CQL §12).
//!
//! Exports: [`add`], [`subtract`], [`multiply`], [`divide`], [`modulo`],
//! [`negate`], [`abs`], [`ceiling`], [`floor`], [`truncate`], [`round`],
//! [`power`], [`ln`], [`exp`], [`log`].

use super::super::context::EvalError;
use super::super::value::{CqlQuantity, Value};
use super::utils::{err, null1, null2};

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::value::Value;

    #[test]
    fn add_integers() {
        assert_eq!(
            add(&Value::Integer(3), &Value::Integer(4)).unwrap(),
            Value::Integer(7)
        );
    }

    #[test]
    fn add_null_propagates() {
        assert_eq!(add(&Value::Null, &Value::Integer(1)).unwrap(), Value::Null);
        assert_eq!(add(&Value::Integer(1), &Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn subtract_decimals() {
        assert_eq!(
            subtract(&Value::Decimal(5.5), &Value::Decimal(2.5)).unwrap(),
            Value::Decimal(3.0)
        );
    }

    #[test]
    fn multiply_long() {
        assert_eq!(
            multiply(&Value::Long(100), &Value::Long(200)).unwrap(),
            Value::Long(20000)
        );
    }

    #[test]
    fn divide_integers_yields_decimal() {
        assert_eq!(
            divide(&Value::Integer(7), &Value::Integer(2)).unwrap(),
            Value::Decimal(3.5)
        );
    }

    #[test]
    fn divide_by_zero_yields_null() {
        assert_eq!(
            divide(&Value::Integer(5), &Value::Integer(0)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn modulo_integer() {
        assert_eq!(
            modulo(&Value::Integer(10), &Value::Integer(3)).unwrap(),
            Value::Integer(1)
        );
    }

    #[test]
    fn modulo_by_zero_yields_null() {
        assert_eq!(
            modulo(&Value::Integer(5), &Value::Integer(0)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn negate_decimal() {
        assert_eq!(
            negate(&Value::Decimal(1.25)).unwrap(),
            Value::Decimal(-1.25)
        );
    }

    #[test]
    fn abs_negative_integer() {
        assert_eq!(abs(&Value::Integer(-7)).unwrap(), Value::Integer(7));
    }

    #[test]
    fn ceiling_decimal() {
        assert_eq!(ceiling(&Value::Decimal(1.1)).unwrap(), Value::Integer(2));
        assert_eq!(ceiling(&Value::Decimal(-1.1)).unwrap(), Value::Integer(-1));
    }

    #[test]
    fn floor_decimal() {
        assert_eq!(floor(&Value::Decimal(1.9)).unwrap(), Value::Integer(1));
        assert_eq!(floor(&Value::Decimal(-1.1)).unwrap(), Value::Integer(-2));
    }

    #[test]
    fn truncate_decimal() {
        assert_eq!(truncate(&Value::Decimal(3.9)).unwrap(), Value::Integer(3));
        assert_eq!(truncate(&Value::Decimal(-3.9)).unwrap(), Value::Integer(-3));
    }

    #[test]
    fn round_default_precision() {
        assert_eq!(
            round(&Value::Decimal(3.567), None).unwrap(),
            Value::Decimal(4.0)
        );
    }

    #[test]
    fn round_with_precision() {
        assert_eq!(
            round(&Value::Decimal(3.567), Some(&Value::Integer(2))).unwrap(),
            Value::Decimal(3.57)
        );
    }

    #[test]
    fn power_integer_base() {
        assert_eq!(
            power(&Value::Integer(2), &Value::Integer(10)).unwrap(),
            Value::Decimal(1024.0)
        );
    }

    #[test]
    fn ln_positive() {
        let result = ln(&Value::Decimal(std::f64::consts::E)).unwrap();
        if let Value::Decimal(x) = result {
            assert!((x - 1.0).abs() < 1e-10);
        } else {
            panic!("expected Decimal");
        }
    }

    #[test]
    fn ln_non_positive_yields_null() {
        assert_eq!(ln(&Value::Decimal(0.0)).unwrap(), Value::Null);
        assert_eq!(ln(&Value::Decimal(-1.0)).unwrap(), Value::Null);
    }

    #[test]
    fn exp_zero_is_one() {
        assert_eq!(exp(&Value::Decimal(0.0)).unwrap(), Value::Decimal(1.0));
    }

    #[test]
    fn log_base10() {
        let result = log(&Value::Decimal(1000.0), &Value::Decimal(10.0)).unwrap();
        if let Value::Decimal(x) = result {
            assert!((x - 3.0).abs() < 1e-10);
        } else {
            panic!("expected Decimal");
        }
    }
}
