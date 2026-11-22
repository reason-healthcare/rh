//! Checked decimal conversion for the supported subset of UCUM and calendar units.

use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::types::FhirPathValue;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::{Decimal, MathematicalOps};
use std::collections::BTreeMap;

use FhirPathValue::{Empty, Quantity};

/// Convert between supported UCUM codes and FHIRPath calendar duration names.
pub struct UnitConverter {
    // Keep construction through new() and Default.
    _private: (),
}

type Dimensions = BTreeMap<String, i32>;

struct UnitExpression {
    dimensions: Dimensions,
    symbols: Dimensions,
    factor: Decimal,
}

impl UnitExpression {
    fn parse(code: &str) -> Option<Self> {
        let mut expression = Self {
            dimensions: Dimensions::new(),
            symbols: Dimensions::new(),
            factor: Decimal::ONE,
        };
        if atom(code).is_some() {
            expression.insert(code, 1)?;
            return Some(expression);
        }
        let (numerator, denominator) = code.split_once('/').unwrap_or((code, ""));
        if !numerator.is_empty() {
            for term in numerator.split('.') {
                let (symbol, power) = parse_term(term)?;
                expression.insert(symbol, power)?;
            }
        }
        if code.contains('/') {
            for term in denominator.split('.') {
                let (symbol, power) = parse_term(term)?;
                expression.insert(symbol, -power)?;
            }
        } else if numerator.is_empty() {
            return None;
        }
        Some(expression)
    }

    fn insert(&mut self, symbol: &str, power: i32) -> Option<()> {
        let (base, base_power, factor) = atom(symbol)?;
        self.factor = self
            .factor
            .checked_mul(factor.checked_powi(i64::from(power))?)?;
        if self.factor.is_zero() {
            return None;
        }
        if symbol != "1" {
            merge_power(&mut self.symbols, symbol, power)?;
            merge_power(&mut self.dimensions, base, power.checked_mul(base_power)?)?;
        }
        Some(())
    }
}

fn merge_power(map: &mut Dimensions, symbol: &str, power: i32) -> Option<()> {
    let power = map.get(symbol).copied().unwrap_or(0).checked_add(power)?;
    if power == 0 {
        map.remove(symbol);
    } else {
        map.insert(symbol.to_owned(), power);
    }
    Some(())
}

fn parse_term(term: &str) -> Option<(&str, i32)> {
    if is_calendar(term) {
        return None;
    }
    if atom(term).is_some() {
        return Some((term, 1));
    }
    let last_symbol = term.rfind(|c: char| !c.is_ascii_digit() && c != '-')? + 1;
    let (symbol, power) = term.split_at(last_symbol);
    let power: i32 = power.parse().ok()?;
    // This also bounds the cost of exponentiation and excludes i32::MIN.
    (power.abs_diff(0) <= 28 && !is_calendar(symbol)).then_some((symbol, power))
}

fn atom(code: &str) -> Option<(&'static str, i32, Decimal)> {
    if let Some((time, calendar)) = time_unit(code) {
        return Some(("s", 1, time.factor(calendar)));
    }
    let (base, power, factor) = match code {
        "1" => ("1", 1, "1"),
        "g" => ("g", 1, "1"),
        "kg" => ("g", 1, "1000"),
        "mg" => ("g", 1, "0.001"),
        "ug" => ("g", 1, "0.000001"),
        "ng" => ("g", 1, "0.000000001"),
        "lb" | "[lb_av]" | "lbs" | "lbs." => ("g", 1, "453.59237"),
        "m" => ("m", 1, "1"),
        "cm" => ("m", 1, "0.01"),
        "mm" => ("m", 1, "0.001"),
        "km" => ("m", 1, "1000"),
        "in" | "in_i" | "[in_i]" => ("m", 1, "0.0254"),
        "ft" | "[ft_i]" => ("m", 1, "0.3048"),
        "L" | "l" => ("m", 3, "0.001"),
        "mL" | "ml" => ("m", 3, "0.000001"),
        "dL" | "dl" => ("m", 3, "0.0001"),
        "uL" | "ul" => ("m", 3, "0.000000001"),
        "Pa" => ("Pa", 1, "1"),
        "kPa" => ("Pa", 1, "1000"),
        "mm[Hg]" => ("Pa", 1, "133.322"),
        "bar" => ("Pa", 1, "100000"),
        "K" => ("K", 1, "1"),
        _ => return None,
    };
    Some((
        base,
        power,
        Decimal::from_str_exact(factor).expect("constant decimal factor"),
    ))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TimeUnit {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl TimeUnit {
    fn factor(self, calendar: bool) -> Decimal {
        use TimeUnit::*;
        match (self, calendar) {
            (Millisecond, _) => Decimal::new(1, 3),
            (Second, _) => Decimal::ONE,
            (Minute, _) => Decimal::from(60),
            (Hour, _) => Decimal::from(3600),
            (Day, _) => Decimal::from(86_400),
            (Week, _) => Decimal::from(604_800),
            (Month, true) => Decimal::from(2_592_000),
            (Year, true) => Decimal::from(31_536_000),
            (Month, false) => Decimal::from(2_629_800),
            (Year, false) => Decimal::from(31_557_600),
        }
    }

    fn calendar_code(self) -> &'static str {
        match self {
            Self::Millisecond => "millisecond",
            Self::Second => "second",
            Self::Minute => "minute",
            Self::Hour => "hour",
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }

    fn convert(self, value: Decimal, target: Self, calendar: bool) -> Option<Decimal> {
        match (self, target, calendar) {
            (Self::Year, Self::Month, true) => value.checked_mul(Decimal::from(12)),
            (Self::Month, Self::Year, true) => value.checked_div(Decimal::from(12)),
            _ => scale(value, self.factor(calendar), target.factor(calendar)),
        }
    }
}

fn time_unit(code: &str) -> Option<(TimeUnit, bool)> {
    use TimeUnit::*;
    Some(match code {
        "ms" => (Millisecond, false),
        "s" => (Second, false),
        "min" => (Minute, false),
        "h" => (Hour, false),
        "d" => (Day, false),
        "wk" => (Week, false),
        "mo" => (Month, false),
        "a" => (Year, false),
        "millisecond" | "milliseconds" => (Millisecond, true),
        "second" | "seconds" => (Second, true),
        "minute" | "minutes" => (Minute, true),
        "hour" | "hours" => (Hour, true),
        "day" | "days" => (Day, true),
        "week" | "weeks" => (Week, true),
        "month" | "months" => (Month, true),
        "year" | "years" => (Year, true),
        _ => return None,
    })
}

fn is_affine(code: &str) -> bool {
    matches!(code, "Cel" | "[degF]")
}

fn is_temperature(code: &str) -> bool {
    code == "K" || is_affine(code)
}

fn to_celsius(value: Decimal, code: &str) -> Option<Decimal> {
    match code {
        "Cel" => Some(value),
        "K" => value.checked_sub(Decimal::new(27315, 2)),
        "[degF]" => value
            .checked_sub(Decimal::from(32))?
            .checked_mul(Decimal::from(5))?
            .checked_div(Decimal::from(9)),
        _ => None,
    }
}

fn from_celsius(value: Decimal, code: &str) -> Option<Decimal> {
    match code {
        "Cel" => Some(value),
        "K" => value.checked_add(Decimal::new(27315, 2)),
        "[degF]" => value
            .checked_mul(Decimal::new(18, 1))?
            .checked_add(Decimal::from(32)),
        _ => None,
    }
}

fn is_calendar(code: &str) -> bool {
    time_unit(code).is_some_and(|(_, calendar)| calendar)
}

fn scale(value: Decimal, source: Decimal, target: Decimal) -> Option<Decimal> {
    if source == target {
        return Some(value);
    }
    // Cancel the larger factor before applying it to the value whenever
    // the resulting factor is exact. For recurring ratios divide last.
    let ratio = source.checked_div(target).filter(|ratio| !ratio.is_zero());
    let result =
        if let Some(ratio) = ratio.filter(|ratio| ratio.checked_mul(target) == Some(source)) {
            value.checked_mul(ratio)
        } else {
            value
                .checked_mul(source)
                .and_then(|scaled| scaled.checked_div(target))
                .or_else(|| ratio.and_then(|ratio| value.checked_mul(ratio)))
        };
    result.filter(|result| !result.is_zero() || value.is_zero())
}

fn convert(value: Decimal, source: &str, target: &str) -> Option<Decimal> {
    if let (Some((from, calendar)), Some((to, _))) = (time_unit(source), time_unit(target)) {
        return from.convert(value, to, calendar);
    }
    if is_temperature(source) && is_temperature(target) {
        return if source == target {
            Some(value)
        } else {
            from_celsius(to_celsius(value, source)?, target)
        };
    }
    if is_calendar(source) || is_calendar(target) {
        return None;
    }
    let source = UnitExpression::parse(source)?;
    let target = UnitExpression::parse(target)?;
    if source.dimensions != target.dimensions {
        return None;
    }
    scale(value, source.factor, target.factor)
}

fn incompatible(operation: &str, left: &str, right: &str) -> FhirPathError {
    FhirPathError::EvaluationError {
        message: format!("Cannot {operation} quantities with units '{left}' and '{right}'"),
    }
}

fn quantity(value: Option<Decimal>, unit: Option<String>) -> FhirPathValue {
    value.map_or(Empty, |value| Quantity { value, unit })
}

impl UnitConverter {
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Whether the converter can explicitly convert between the two units.
    pub fn are_units_compatible(&self, left: &Option<String>, right: &Option<String>) -> bool {
        convert(Decimal::ZERO, code(left), code(right)).is_some()
    }

    /// Whether implicit equality or ordering can compare the two units.
    pub fn are_quantities_comparable(&self, left: &Option<String>, right: &Option<String>) -> bool {
        if let (Some((l, lc)), Some((r, rc))) = (time_unit(code(left)), time_unit(code(right))) {
            if lc != rc && (l >= TimeUnit::Month || r >= TimeUnit::Month) {
                return false;
            }
        }
        self.are_units_compatible(left, right)
    }

    pub fn convert_quantity(
        &self,
        value: Decimal,
        unit: &Option<String>,
        target_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        let source = code(unit);
        let target = code(target_unit);
        Ok(quantity(
            convert(value, source, target),
            Some(target.to_owned()),
        ))
    }

    /// Compatibility interface for callers that work with floating point quantities.
    pub fn to_base_unit(
        &self,
        value: f64,
        unit: &Option<String>,
    ) -> FhirPathResult<(f64, Option<String>)> {
        let Some(original) = unit else {
            return Ok((value, None));
        };
        let base = if is_temperature(original) {
            "Cel".to_owned()
        } else {
            render_units(
                &UnitExpression::parse(original)
                    .ok_or_else(|| incompatible("convert", original, "base"))?
                    .dimensions,
            )
        };
        let value = self.convert_from_base_unit(value, unit, &Some(base.clone()))?;
        Ok((value, Some(base)))
    }

    pub fn convert_from_base_unit(
        &self,
        value: f64,
        unit: &Option<String>,
        target_unit: &Option<String>,
    ) -> FhirPathResult<f64> {
        let source = code(unit);
        let target = code(target_unit);
        Decimal::from_f64(value)
            .and_then(|value| convert(value, source, target))
            .and_then(|value| value.to_f64())
            .ok_or_else(|| incompatible("convert", source, target))
    }

    pub fn add_quantities(
        &self,
        a: Decimal,
        au: &Option<String>,
        b: Decimal,
        bu: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        Ok(self.add_or_subtract(a, au, b, bu, Decimal::checked_add))
    }

    pub fn subtract_quantities(
        &self,
        a: Decimal,
        au: &Option<String>,
        b: Decimal,
        bu: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        Ok(self.add_or_subtract(a, au, b, bu, Decimal::checked_sub))
    }

    fn add_or_subtract(
        &self,
        a: Decimal,
        au: &Option<String>,
        b: Decimal,
        bu: &Option<String>,
        op: fn(Decimal, Decimal) -> Option<Decimal>,
    ) -> FhirPathValue {
        let left = code(au);
        let right = code(bu);
        if is_affine(left) || is_affine(right) {
            return Empty;
        }
        let result_unit = if let (Some((l, lc)), Some((r, rc))) =
            (time_unit(left), time_unit(right))
        {
            if (l >= TimeUnit::Month || r >= TimeUnit::Month) && (lc || rc) && (l != r || lc != rc)
            {
                return Empty;
            }
            if lc || rc {
                l.min(r).calendar_code()
            } else if l <= r {
                left
            } else {
                right
            }
        } else {
            let Some(chosen) = granularity(left, right, false) else {
                return Empty;
            };
            chosen
        };
        let values = convert(a, left, result_unit).zip(convert(b, right, result_unit));
        quantity(
            values.and_then(|(a, b)| op(a, b)),
            (au.is_some() || bu.is_some()).then(|| result_unit.to_owned()),
        )
    }

    pub fn multiply_by_scalar(
        &self,
        value: Decimal,
        unit: &Option<String>,
        scalar: Decimal,
    ) -> FhirPathValue {
        self.multiply_quantities(value, unit, scalar, &None)
            .unwrap_or(Empty)
    }

    pub fn divide_by_scalar(
        &self,
        value: Decimal,
        unit: &Option<String>,
        scalar: Decimal,
    ) -> FhirPathResult<FhirPathValue> {
        self.divide_quantities(value, unit, scalar, &None)
    }

    pub fn multiply_quantities(
        &self,
        a: Decimal,
        au: &Option<String>,
        b: Decimal,
        bu: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        Ok(product_or_quotient(a, au, b, bu, false).unwrap_or(Empty))
    }

    pub fn divide_quantities(
        &self,
        a: Decimal,
        au: &Option<String>,
        b: Decimal,
        bu: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        Ok(product_or_quotient(a, au, b, bu, true).unwrap_or(Empty))
    }

    pub fn compare_quantities(
        &self,
        a: Decimal,
        au: &Option<String>,
        b: Decimal,
        bu: &Option<String>,
    ) -> FhirPathResult<i32> {
        let (left, right) = (code(au), code(bu));
        if !self.are_quantities_comparable(au, bu) {
            return Err(incompatible("compare", left, right));
        }
        let (a, b) = comparison_values(a, left, b, right)
            .or_else(|| {
                let target = granularity(left, right, false)?;
                Some((convert(a, left, target)?, convert(b, right, target)?))
            })
            .ok_or_else(|| incompatible("compare", left, right))?;
        Ok(a.cmp(&b) as i32)
    }

    pub fn equivalent_quantities(
        &self,
        a: Decimal,
        au: &Option<String>,
        b: Decimal,
        bu: &Option<String>,
    ) -> bool {
        let Some((a, b)) = comparison_values(a, code(au), b, code(bu)) else {
            return false;
        };
        let precision = a.normalize().scale().min(b.normalize().scale());
        a.round_dp(precision) == b.round_dp(precision)
    }
}

fn code(unit: &Option<String>) -> &str {
    unit.as_deref().unwrap_or("1")
}

fn comparison_values(
    a: Decimal,
    left: &str,
    b: Decimal,
    right: &str,
) -> Option<(Decimal, Decimal)> {
    if is_temperature(left) && is_temperature(right) {
        return if left == right {
            Some((a, b))
        } else {
            Some((to_celsius(a, left)?, to_celsius(b, right)?))
        };
    }
    let target = granularity(left, right, true)?;
    Some((convert(a, left, target)?, convert(b, right, target)?))
}

fn granularity<'a>(left: &'a str, right: &'a str, least: bool) -> Option<&'a str> {
    if let (Some((l, _)), Some((r, _))) = (time_unit(left), time_unit(right)) {
        return Some(if l == r || (l > r) == least {
            left
        } else {
            right
        });
    }
    let a = UnitExpression::parse(left)?;
    let b = UnitExpression::parse(right)?;
    if a.dimensions != b.dimensions {
        return None;
    }
    Some(if a.factor == b.factor || (a.factor > b.factor) == least {
        left
    } else {
        right
    })
}

fn product_or_quotient(
    a: Decimal,
    au: &Option<String>,
    b: Decimal,
    bu: &Option<String>,
    divide: bool,
) -> Option<FhirPathValue> {
    let (left, right) = (code(au), code(bu));
    let calendar_operand = is_calendar(left) || is_calendar(right);
    let scalar_operand = left == "1" || right == "1";
    if is_affine(left) || is_affine(right) || (calendar_operand && !scalar_operand) {
        return None;
    }
    // Preserve measurement codes when multiplying or dividing by a scalar.
    if right == "1" || (left == "1" && !divide) {
        let source = if right == "1" { au } else { bu };
        let operand = if right == "1" { left } else { right };
        UnitExpression::parse(operand)?;
        return Some(quantity(ratio_arithmetic(a, b, divide), source.clone()));
    }
    if calendar_operand {
        return None; // A reciprocal calendar duration is undefined.
    }
    let a_units = UnitExpression::parse(left)?;
    let b_units = UnitExpression::parse(right)?;
    let mut a_dimensions = a_units.dimensions.clone();
    let mut a_symbols = a_units.symbols.clone();
    let sign = if divide { -1 } else { 1 };
    for (symbol, power) in &b_units.dimensions {
        merge_power(&mut a_dimensions, symbol, sign * power)?;
    }
    for (symbol, power) in &b_units.symbols {
        merge_power(&mut a_symbols, symbol, sign * power)?;
    }
    let mixed_units = a_units.symbols.keys().any(|left| {
        b_units.symbols.keys().any(|right| {
            left != right
                && atom(left)
                    .zip(atom(right))
                    .is_some_and(|((a, _, _), (b, _, _))| a == b)
        })
    });
    let (a, b, result_symbols) = if mixed_units {
        // Normalize aliases so shared dimensions cancel even when original
        // symbols differ (e.g. cm / m).
        (
            scale(a, a_units.factor, Decimal::ONE)?,
            scale(b, b_units.factor, Decimal::ONE)?,
            a_dimensions,
        )
    } else {
        (a, b, a_symbols)
    };
    Some(quantity(
        ratio_arithmetic(a, b, divide),
        Some(render_units(&result_symbols)),
    ))
}

fn ratio_arithmetic(left: Decimal, right: Decimal, divide: bool) -> Option<Decimal> {
    if divide {
        left.checked_div(right)
            .filter(|result| !result.is_zero() || left.is_zero())
    } else {
        left.checked_mul(right)
            .filter(|result| !result.is_zero() || left.is_zero() || right.is_zero())
    }
}

fn render_units(dimensions: &Dimensions) -> String {
    let mut numerator = Vec::new();
    let mut denominator = Vec::new();
    for (symbol, power) in dimensions {
        let magnitude = power.unsigned_abs();
        let term = if magnitude == 1 {
            symbol.clone()
        } else {
            format!("{symbol}{magnitude}")
        };
        if *power > 0 {
            numerator.push(term)
        } else {
            denominator.push(term)
        }
    }
    match (numerator.is_empty(), denominator.is_empty()) {
        (true, true) => "1".to_string(),
        (false, true) => numerator.join("."),
        (_, false) => format!("{}/{}", numerator.join("."), denominator.join(".")),
    }
}

impl Default for UnitConverter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UnitEvaluator;

impl UnitEvaluator {
    pub fn new_converter() -> UnitConverter {
        UnitConverter::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::ToPrimitive;
    use rust_decimal::Decimal;

    #[test]
    fn test_mass_conversion() {
        let converter = UnitConverter::new();

        let result = converter
            .add_quantities(
                Decimal::from_str_exact("1.0").unwrap(),
                &Some("kg".to_string()),
                Decimal::from_str_exact("500.0").unwrap(),
                &Some("g".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert!((value.to_f64().unwrap() - 1500.0).abs() < 0.001);
            assert_eq!(unit, Some("g".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_length_conversion() {
        let converter = UnitConverter::new();

        let result = converter
            .add_quantities(
                Decimal::from_str_exact("1.0").unwrap(),
                &Some("m".to_string()),
                Decimal::from_str_exact("50.0").unwrap(),
                &Some("cm".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert!((value.to_f64().unwrap() - 150.0).abs() < 0.001);
            assert_eq!(unit, Some("cm".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_incompatible_units() {
        let converter = UnitConverter::new();

        let result = converter.add_quantities(
            Decimal::from_str_exact("1.0").unwrap(),
            &Some("kg".to_string()),
            Decimal::from_str_exact("1.0").unwrap(),
            &Some("m".to_string()),
        );
        assert_eq!(result.unwrap(), Empty);
    }

    #[test]
    fn test_division() {
        let converter = UnitConverter::new();

        let result = converter
            .divide_quantities(
                Decimal::from_str_exact("10.0").unwrap(),
                &Some("kg".to_string()),
                Decimal::from_str_exact("5.0").unwrap(),
                &Some("kg".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("2.0").unwrap());
            assert_eq!(unit.as_deref(), Some("1"));
        } else {
            panic!("Expected Quantity result with unit '1', got {result:?}");
        }
    }

    #[test]
    fn test_multiply_quantities_compound_unit() {
        let converter = UnitConverter::new();

        let result = converter
            .multiply_quantities(
                Decimal::from_str_exact("2.0").unwrap(),
                &Some("cm".to_string()),
                Decimal::from_str_exact("2.0").unwrap(),
                &Some("m".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert!((value.to_f64().unwrap() - 0.04).abs() < 0.001);
            assert_eq!(unit.as_deref(), Some("m2"));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_divide_quantities_compound_unit() {
        let converter = UnitConverter::new();

        let result = converter
            .divide_quantities(
                Decimal::from_str_exact("4.0").unwrap(),
                &Some("g".to_string()),
                Decimal::from_str_exact("2.0").unwrap(),
                &Some("m".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("2.0").unwrap());
            assert_eq!(unit.as_deref(), Some("g/m"));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_scalar_operations() {
        let converter = UnitConverter::new();

        let result = converter.multiply_by_scalar(
            Decimal::from_str_exact("5.0").unwrap(),
            &Some("mg".to_string()),
            Decimal::from_str_exact("2.0").unwrap(),
        );
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("10.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_affine_arithmetic_is_undefined() {
        let converter = UnitConverter::new();
        let celsius = Some("Cel".to_string());
        let kelvin = Some("K".to_string());
        let fahrenheit = Some("[degF]".to_string());
        assert_eq!(
            converter
                .add_quantities(Decimal::ZERO, &celsius, Decimal::ZERO, &kelvin)
                .unwrap(),
            Empty
        );
        assert_eq!(
            converter
                .subtract_quantities(Decimal::from(100), &celsius, Decimal::ZERO, &celsius)
                .unwrap(),
            Empty
        );
        assert_eq!(
            converter
                .multiply_quantities(Decimal::ONE, &fahrenheit, Decimal::from(2), &None)
                .unwrap(),
            Empty
        );
    }

    #[test]
    fn test_temperature_specific_conversions() {
        let converter = UnitConverter::new();

        let (celsius_val, _) = converter
            .to_base_unit(0.0, &Some("Cel".to_string()))
            .unwrap();
        assert!((celsius_val - 0.0).abs() < 0.01);

        let (celsius_val, _) = converter
            .to_base_unit(32.0, &Some("[degF]".to_string()))
            .unwrap();
        assert!((celsius_val - 0.0).abs() < 0.01);

        let (celsius_val, _) = converter
            .to_base_unit(273.15, &Some("K".to_string()))
            .unwrap();
        assert!((celsius_val - 0.0).abs() < 0.01);

        let kelvin_val = converter
            .convert_from_base_unit(0.0, &Some("Cel".to_string()), &Some("K".to_string()))
            .unwrap();
        assert!((kelvin_val - 273.15).abs() < 0.01);

        let fahrenheit_val = converter
            .convert_from_base_unit(0.0, &Some("Cel".to_string()), &Some("[degF]".to_string()))
            .unwrap();
        assert!((fahrenheit_val - 32.0).abs() < 0.01);
    }
}
