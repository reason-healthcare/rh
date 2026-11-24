//! FHIR JSON may carry exponents outside the native Decimal arithmetic range.
//! Preserve comparisons and string conversion without expanding powers of ten.
use super::FhirNode;
use crate::evaluator::FhirPathValue as V;
use std::borrow::Cow;
use std::cmp::Ordering;

#[derive(Debug)]
struct Scientific {
    sign: i8,
    /// Significant digits, with leading and trailing zeroes removed.
    digits: String,
    /// Position of the decimal point relative to the first significant digit.
    point: i128,
    /// Exponent of the last supplied digit.
    quantum: i128,
}

impl Scientific {
    fn parse(text: &str) -> Option<Self> {
        let (mantissa, power) = match text.split_once(['e', 'E']) {
            Some((mantissa, exponent)) => (mantissa, i128::from(exponent.parse::<i64>().ok()?)),
            None => (text, 0),
        };
        let sign = if mantissa.starts_with('-') { -1 } else { 1 };
        let mantissa = mantissa.trim_start_matches('-');
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let mut digits = String::from(whole);
        digits.push_str(fraction);
        let trimmed = digits.trim_start_matches('0');
        let point = whole.len() as i128 + power - (digits.len() - trimmed.len()) as i128;
        let quantum = power - fraction.len() as i128;
        Some(Self {
            sign: if trimmed.is_empty() { 0 } else { sign },
            digits: trimmed.trim_end_matches('0').into(),
            point,
            quantum,
        })
    }

    fn compare(&self, other: &Self) -> Ordering {
        let by_sign = self.sign.cmp(&other.sign);
        if by_sign != Ordering::Equal || self.sign == 0 {
            return by_sign;
        }
        let magnitude = self
            .point
            .cmp(&other.point)
            .then_with(|| self.digits.cmp(&other.digits));
        if self.sign < 0 {
            magnitude.reverse()
        } else {
            magnitude
        }
    }

    fn round(&mut self, quantum: i128) {
        let keep = self.point - quantum;
        if self.sign == 0 || keep >= self.digits.len() as i128 {
            return;
        }
        if keep < 0 {
            self.sign = 0;
            return;
        }
        let keep = keep as usize;
        let round_up = self.digits.as_bytes()[keep] >= b'5';
        self.digits.truncate(keep);
        if round_up {
            let nines = self.digits.trim_end_matches('9').len();
            self.digits.truncate(nines);
            if let Some(digit) = self.digits.pop() {
                self.digits.push(char::from(digit as u8 + 1));
            } else {
                self.digits.push('1');
                self.point += 1;
            }
        }
        self.digits
            .truncate(self.digits.trim_end_matches('0').len());
        if self.digits.is_empty() {
            self.sign = 0;
        }
    }
}

impl FhirNode {
    pub(super) fn extreme_numeric(&self) -> Option<(Cow<'_, str>, Option<&str>)> {
        if self.inner != V::Empty {
            return None;
        }
        if matches!(self.fhir_type.as_str(), "decimal" | "System.Decimal") {
            return Some((self.number_text()?, None));
        }
        if self.model.is_subtype(&self.fhir_type, "Quantity")
            && self.raw["system"] == "http://unitsofmeasure.org"
        {
            return Some((
                self.field_number_text("value")?,
                Some(self.raw["code"].as_str()?),
            ));
        }
        None
    }

    pub(crate) fn extreme_numeric_string(&self) -> Option<String> {
        let (literal, unit) = self.extreme_numeric()?;
        let mut text = match literal.split_once(['e', 'E']) {
            Some((mantissa, exponent)) => {
                let digits = exponent
                    .trim_start_matches(['-', '+'])
                    .trim_start_matches('0');
                if digits.is_empty() {
                    format!("{mantissa}E+0")
                } else {
                    let sign = if exponent.starts_with('-') { '-' } else { '+' };
                    format!("{mantissa}E{sign}{digits}")
                }
            }
            None => literal.into_owned(),
        };
        if let Some(unit) = unit {
            text.push_str(&format!(" '{unit}'"));
        }
        Some(text)
    }
}

fn numeric(v: &V) -> Option<(Scientific, Option<&str>)> {
    if let V::Node(node) = v {
        if let Some((text, unit)) = node.extreme_numeric() {
            return Some((Scientific::parse(&text)?, unit));
        }
        return numeric(&node.inner);
    }
    match v {
        V::Integer(v) | V::Long(v) => Some((Scientific::parse(&v.to_string())?, None)),
        V::Number(v) => Some((Scientific::parse(&v.to_string())?, None)),
        V::Quantity { value, unit } => {
            Some((Scientific::parse(&value.to_string())?, unit.as_deref()))
        }
        _ => None,
    }
}

fn pair(a: &V, b: &V) -> Option<((Scientific, Scientific), bool)> {
    if ![a, b]
        .iter()
        .any(|v| matches!(v, V::Node(n) if n.extreme_numeric().is_some()))
    {
        return None;
    }
    let (a, au) = numeric(a)?;
    let (b, bu) = numeric(b)?;
    Some(((a, b), au == bu))
}

pub(super) fn extreme_numeric_order(a: &V, b: &V) -> Option<Option<Ordering>> {
    let ((a, b), same_units) = pair(a, b)?;
    Some(same_units.then(|| a.compare(&b)))
}

pub(crate) fn extreme_numeric_equivalent(a: &V, b: &V) -> Option<bool> {
    let ((mut a, mut b), same_units) = pair(a, b)?;
    if !same_units {
        return Some(false);
    }
    let precision = a.quantum.max(b.quantum);
    a.round(precision);
    b.round(precision);
    Some(a.compare(&b) == Ordering::Equal)
}
