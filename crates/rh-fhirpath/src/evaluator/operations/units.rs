//! Unit conversion and handling for UCUM units
//!
//! This module provides unit conversion between compatible UCUM units using a custom
//! conversion system with predefined conversion factors for medical and scientific units.

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Unit converter for UCUM codes
pub struct UnitConverter {
    /// Mapping of UCUM codes to their conversion factors and unit information
    unit_mappings: HashMap<String, UnitMapping>,
}

#[derive(Debug, Clone)]
struct UnitMapping {
    /// The physical quantity type (mass, length, etc.)
    quantity_type: QuantityType,
    /// Base unit for this quantity type
    base_unit: String,
    /// Conversion factor to base unit
    to_base_factor: f64,
    /// Conversion factor from base unit
    from_base_factor: f64,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Some variants planned for future use
enum QuantityType {
    Mass,
    Length,
    Volume,
    Temperature,
    Pressure,
    Time,
    Dimensionless,
    Unsupported,
}

impl UnitConverter {
    /// Create a new unit converter with UCUM mappings
    pub fn new() -> Self {
        let mut unit_mappings = HashMap::new();

        // Dimensionless UCUM default unit.
        unit_mappings.insert(
            "1".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Dimensionless,
                base_unit: "1".to_string(),
                to_base_factor: 1.0,
                from_base_factor: 1.0,
            },
        );

        // Mass units (base: gram)
        unit_mappings.insert(
            "g".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Mass,
                base_unit: "g".to_string(),
                to_base_factor: 1.0,
                from_base_factor: 1.0,
            },
        );
        unit_mappings.insert(
            "kg".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Mass,
                base_unit: "g".to_string(),
                to_base_factor: 1000.0,
                from_base_factor: 0.001,
            },
        );
        unit_mappings.insert(
            "mg".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Mass,
                base_unit: "g".to_string(),
                to_base_factor: 0.001,
                from_base_factor: 1000.0,
            },
        );
        unit_mappings.insert(
            "ug".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Mass,
                base_unit: "g".to_string(),
                to_base_factor: 0.000001,
                from_base_factor: 1000000.0,
            },
        );
        unit_mappings.insert(
            "lb".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Mass,
                base_unit: "g".to_string(),
                to_base_factor: 453.592,
                from_base_factor: 1.0 / 453.592,
            },
        );
        // UCUM avoirdupois pound and common display aliases
        for alias in ["[lb_av]", "lbs", "lbs."] {
            unit_mappings.insert(
                alias.to_string(),
                UnitMapping {
                    quantity_type: QuantityType::Mass,
                    base_unit: "g".to_string(),
                    to_base_factor: 453.592,
                    from_base_factor: 1.0 / 453.592,
                },
            );
        }

        // Length units (base: meter)
        unit_mappings.insert(
            "m".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Length,
                base_unit: "m".to_string(),
                to_base_factor: 1.0,
                from_base_factor: 1.0,
            },
        );
        unit_mappings.insert(
            "cm".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Length,
                base_unit: "m".to_string(),
                to_base_factor: 0.01,
                from_base_factor: 100.0,
            },
        );
        unit_mappings.insert(
            "mm".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Length,
                base_unit: "m".to_string(),
                to_base_factor: 0.001,
                from_base_factor: 1000.0,
            },
        );
        unit_mappings.insert(
            "km".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Length,
                base_unit: "m".to_string(),
                to_base_factor: 1000.0,
                from_base_factor: 0.001,
            },
        );
        unit_mappings.insert(
            "in".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Length,
                base_unit: "m".to_string(),
                to_base_factor: 0.0254,
                from_base_factor: 1.0 / 0.0254,
            },
        );
        unit_mappings.insert(
            "[in_i]".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Length,
                base_unit: "m".to_string(),
                to_base_factor: 0.0254,
                from_base_factor: 1.0 / 0.0254,
            },
        );
        unit_mappings.insert(
            "ft".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Length,
                base_unit: "m".to_string(),
                to_base_factor: 0.3048,
                from_base_factor: 1.0 / 0.3048,
            },
        );

        // Volume units (base: liter)
        unit_mappings.insert(
            "L".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Volume,
                base_unit: "L".to_string(),
                to_base_factor: 1.0,
                from_base_factor: 1.0,
            },
        );
        unit_mappings.insert(
            "mL".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Volume,
                base_unit: "L".to_string(),
                to_base_factor: 0.001,
                from_base_factor: 1000.0,
            },
        );
        unit_mappings.insert(
            "dL".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Volume,
                base_unit: "L".to_string(),
                to_base_factor: 0.1,
                from_base_factor: 10.0,
            },
        );
        unit_mappings.insert(
            "uL".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Volume,
                base_unit: "L".to_string(),
                to_base_factor: 0.000001,
                from_base_factor: 1000000.0,
            },
        );

        // Time units (base: second)
        unit_mappings.insert(
            "s".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Time,
                base_unit: "s".to_string(),
                to_base_factor: 1.0,
                from_base_factor: 1.0,
            },
        );
        unit_mappings.insert(
            "min".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Time,
                base_unit: "s".to_string(),
                to_base_factor: 60.0,
                from_base_factor: 1.0 / 60.0,
            },
        );
        unit_mappings.insert(
            "h".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Time,
                base_unit: "s".to_string(),
                to_base_factor: 3600.0,
                from_base_factor: 1.0 / 3600.0,
            },
        );
        unit_mappings.insert(
            "d".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Time,
                base_unit: "s".to_string(),
                to_base_factor: 86400.0,
                from_base_factor: 1.0 / 86400.0,
            },
        );
        unit_mappings.insert(
            "wk".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Time,
                base_unit: "s".to_string(),
                to_base_factor: 604800.0, // 7 * 24 * 3600
                from_base_factor: 1.0 / 604800.0,
            },
        );
        // UCUM 'mo' (mean month = 30.44 days) and 'a' (mean year = 365.25 days)
        // are intentionally NOT registered here. FHIRPath treats them as
        // incompatible with the calendar durations 'month'/'year' — comparing
        // `1 'mo' = 1 month` is undefined (empty) per the HL7 conformance suite.

        // FHIRPath calendar-duration names — same factors as their UCUM
        // counterparts so `7 days = 1 week`, `7 days = 1 'wk'`, etc. work.
        let cal_pairs: &[(&str, f64)] = &[
            ("millisecond", 0.001),
            ("milliseconds", 0.001),
            ("second", 1.0),
            ("seconds", 1.0),
            ("minute", 60.0),
            ("minutes", 60.0),
            ("hour", 3600.0),
            ("hours", 3600.0),
            ("day", 86400.0),
            ("days", 86400.0),
            ("week", 604800.0),
            ("weeks", 604800.0),
            // month/year use UCUM mean values for comparison purposes only.
            ("month", 2629746.0),
            ("months", 2629746.0),
            ("year", 31556952.0),
            ("years", 31556952.0),
        ];
        for (name, factor) in cal_pairs {
            unit_mappings.insert(
                name.to_string(),
                UnitMapping {
                    quantity_type: QuantityType::Time,
                    base_unit: "s".to_string(),
                    to_base_factor: *factor,
                    from_base_factor: 1.0 / factor,
                },
            );
        }

        // Pressure units (base: Pascal)
        unit_mappings.insert(
            "Pa".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Pressure,
                base_unit: "Pa".to_string(),
                to_base_factor: 1.0,
                from_base_factor: 1.0,
            },
        );
        unit_mappings.insert(
            "kPa".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Pressure,
                base_unit: "Pa".to_string(),
                to_base_factor: 1000.0,
                from_base_factor: 0.001,
            },
        );
        unit_mappings.insert(
            "mm[Hg]".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Pressure,
                base_unit: "Pa".to_string(),
                to_base_factor: 133.322,
                from_base_factor: 1.0 / 133.322,
            },
        );
        unit_mappings.insert(
            "bar".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Pressure,
                base_unit: "Pa".to_string(),
                to_base_factor: 100000.0,
                from_base_factor: 0.00001,
            },
        );

        // Temperature units (base: Celsius)
        // Note: Temperature conversion requires offset handling, implemented in special methods
        unit_mappings.insert(
            "Cel".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Temperature,
                base_unit: "Cel".to_string(),
                to_base_factor: 1.0,
                from_base_factor: 1.0,
            },
        );
        unit_mappings.insert(
            "K".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Temperature,
                base_unit: "Cel".to_string(),
                to_base_factor: 1.0, // Same scale as Celsius
                from_base_factor: 1.0,
            },
        );
        unit_mappings.insert(
            "[degF]".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Temperature,
                base_unit: "Cel".to_string(),
                to_base_factor: 5.0 / 9.0, // Fahrenheit scale factor
                from_base_factor: 9.0 / 5.0,
            },
        );

        Self { unit_mappings }
    }

    /// Check if two units are compatible for arithmetic operations
    pub fn are_units_compatible(&self, unit1: &Option<String>, unit2: &Option<String>) -> bool {
        match (unit1, unit2) {
            (None, None) => true, // Both dimensionless
            (None, Some(u)) | (Some(u), None) => u == "1",
            (Some(u1), Some(u2)) => {
                if u1 == u2 {
                    return true; // Same units
                }
                if let (Some((base1, power1)), Some((base2, power2))) =
                    (parse_simple_power_unit(u1), parse_simple_power_unit(u2))
                {
                    return power1 == power2
                        && self.are_units_compatible(&Some(base1), &Some(base2));
                }
                // Check if they're the same quantity type
                if let (Some(mapping1), Some(mapping2)) =
                    (self.unit_mappings.get(u1), self.unit_mappings.get(u2))
                {
                    mapping1.quantity_type == mapping2.quantity_type
                } else {
                    false
                }
            }
        }
    }

    /// Convert a quantity value to a compatible target unit.
    pub fn convert_quantity(
        &self,
        value: Decimal,
        unit: &Option<String>,
        target_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        if !self.are_units_compatible(unit, target_unit) {
            return Err(FhirPathError::EvaluationError {
                message: format!("Cannot convert quantity from {unit:?} to {target_unit:?}"),
            });
        }

        let source_unit = unit.clone().or_else(|| Some("1".to_string()));
        let target_unit = target_unit.clone().or_else(|| Some("1".to_string()));
        if source_unit == target_unit {
            return Ok(FhirPathValue::Quantity {
                value,
                unit: target_unit,
            });
        }

        let value_f64 = value.to_f64().unwrap_or(0.0);
        let (base_value, base_unit) = self.to_base_unit(value_f64, &source_unit)?;
        let result = self.convert_from_base_unit(base_value, &base_unit, &target_unit)?;

        Ok(FhirPathValue::Quantity {
            value: Decimal::from_f64(result).unwrap_or(Decimal::ZERO),
            unit: target_unit.clone(),
        })
    }

    /// Convert a quantity to a standard base unit for the quantity type
    pub fn to_base_unit(
        &self,
        value: f64,
        unit: &Option<String>,
    ) -> FhirPathResult<(f64, Option<String>)> {
        match unit {
            None => Ok((value, None)), // Dimensionless
            Some(unit_str) => {
                if let Some((base_unit, power)) = parse_simple_power_unit(unit_str) {
                    let Some(mapping) = self.unit_mappings.get(&base_unit) else {
                        return Ok((value, unit.clone()));
                    };
                    let factor = mapping.to_base_factor.powi(power);
                    let base_value = value * factor;
                    return Ok((base_value, Some(format!("{}{}", mapping.base_unit, power))));
                }
                if let Some(mapping) = self.unit_mappings.get(unit_str) {
                    // Special handling for temperature units (require offset conversion)
                    if mapping.quantity_type == QuantityType::Temperature {
                        let base_value = self.temperature_to_celsius(value, unit_str)?;
                        Ok((base_value, Some(mapping.base_unit.clone())))
                    } else {
                        let base_value = value * mapping.to_base_factor;
                        Ok((base_value, Some(mapping.base_unit.clone())))
                    }
                } else {
                    // Unknown unit, return as-is
                    Ok((value, unit.clone()))
                }
            }
        }
    }

    /// Convert a quantity from base unit to a target unit
    pub fn convert_from_base_unit(
        &self,
        base_value: f64,
        base_unit: &Option<String>,
        target_unit: &Option<String>,
    ) -> FhirPathResult<f64> {
        match (base_unit, target_unit) {
            (None, None) => Ok(base_value), // Both dimensionless
            (Some(_), None) => Err(FhirPathError::EvaluationError {
                message: "Cannot convert from unit to dimensionless".to_string(),
            }),
            (None, Some(_)) => Err(FhirPathError::EvaluationError {
                message: "Cannot convert from dimensionless to unit".to_string(),
            }),
            (Some(base), Some(target)) => {
                if base == target {
                    return Ok(base_value);
                }
                if let (Some((base_base, base_power)), Some((target_base, target_power))) = (
                    parse_simple_power_unit(base),
                    parse_simple_power_unit(target),
                ) {
                    if base_power == target_power {
                        let Some(base_mapping) = self.unit_mappings.get(&base_base) else {
                            return Err(FhirPathError::EvaluationError {
                                message: format!("Unknown base unit: {base_base}"),
                            });
                        };
                        let Some(target_mapping) = self.unit_mappings.get(&target_base) else {
                            return Err(FhirPathError::EvaluationError {
                                message: format!("Unknown target unit: {target_base}"),
                            });
                        };
                        if base_mapping.base_unit == target_mapping.base_unit {
                            return Ok(
                                base_value * target_mapping.from_base_factor.powi(target_power)
                            );
                        }
                    }
                }
                if let Some(target_mapping) = self.unit_mappings.get(target) {
                    if target_mapping.base_unit == *base {
                        // Special handling for temperature units
                        if target_mapping.quantity_type == QuantityType::Temperature {
                            self.celsius_to_temperature(base_value, target)
                        } else {
                            Ok(base_value * target_mapping.from_base_factor)
                        }
                    } else {
                        Err(FhirPathError::EvaluationError {
                            message: format!(
                                "Cannot convert between incompatible units: {base} and {target}"
                            ),
                        })
                    }
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Unknown target unit: {target}"),
                    })
                }
            }
        }
    }

    /// Convert temperature value to Celsius (base unit for temperature)
    fn temperature_to_celsius(&self, value: f64, unit: &str) -> FhirPathResult<f64> {
        match unit {
            "Cel" => Ok(value),
            "K" => Ok(value - 273.15),
            "[degF]" => Ok((value - 32.0) * 5.0 / 9.0),
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Unknown temperature unit: {unit}"),
            }),
        }
    }

    /// Convert Celsius temperature to target temperature unit
    fn celsius_to_temperature(&self, celsius_value: f64, target_unit: &str) -> FhirPathResult<f64> {
        match target_unit {
            "Cel" => Ok(celsius_value),
            "K" => Ok(celsius_value + 273.15),
            "[degF]" => Ok(celsius_value * 9.0 / 5.0 + 32.0),
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Unknown temperature unit: {target_unit}"),
            }),
        }
    }

    /// Add two quantities with automatic unit conversion
    pub fn add_quantities(
        &self,
        left_value: Decimal,
        left_unit: &Option<String>,
        right_value: Decimal,
        right_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        if !self.are_units_compatible(left_unit, right_unit) {
            return Err(FhirPathError::EvaluationError {
                message: format!(
                    "Cannot add quantities with incompatible units: {left_unit:?} and {right_unit:?}"
                ),
            });
        }

        // Convert both to base units (using f64 for unit conversion math)
        let left_f64 = left_value.to_f64().unwrap_or(0.0);
        let right_f64 = right_value.to_f64().unwrap_or(0.0);
        let (left_base, left_base_unit) = self.to_base_unit(left_f64, left_unit)?;
        let (right_base, _right_base_unit) = self.to_base_unit(right_f64, right_unit)?;

        // Add the base values
        let result_base = left_base + right_base;

        // Convert back to the left operand's unit (maintain left operand unit for result)
        let result_unit = self.most_granular_unit(left_unit, right_unit);
        let result_f64 = self.convert_from_base_unit(result_base, &left_base_unit, &result_unit)?;

        Ok(FhirPathValue::Quantity {
            value: Decimal::from_f64_retain(result_f64).unwrap_or(Decimal::ZERO),
            unit: result_unit,
        })
    }

    /// Subtract two quantities with automatic unit conversion
    pub fn subtract_quantities(
        &self,
        left_value: Decimal,
        left_unit: &Option<String>,
        right_value: Decimal,
        right_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        if !self.are_units_compatible(left_unit, right_unit) {
            return Err(FhirPathError::EvaluationError {
                message: format!(
                    "Cannot subtract quantities with incompatible units: {left_unit:?} and {right_unit:?}"
                ),
            });
        }

        let left_f64 = left_value.to_f64().unwrap_or(0.0);
        let right_f64 = right_value.to_f64().unwrap_or(0.0);
        let (left_base, left_base_unit) = self.to_base_unit(left_f64, left_unit)?;
        let (right_base, _) = self.to_base_unit(right_f64, right_unit)?;

        let result_base = left_base - right_base;

        let result_unit = self.most_granular_unit(left_unit, right_unit);
        let result_f64 = self.convert_from_base_unit(result_base, &left_base_unit, &result_unit)?;

        Ok(FhirPathValue::Quantity {
            value: Decimal::from_f64_retain(result_f64).unwrap_or(Decimal::ZERO),
            unit: result_unit,
        })
    }

    fn most_granular_unit(
        &self,
        left_unit: &Option<String>,
        right_unit: &Option<String>,
    ) -> Option<String> {
        match (left_unit, right_unit) {
            (Some(left), Some(right)) => {
                match (self.unit_mappings.get(left), self.unit_mappings.get(right)) {
                    (Some(left_mapping), Some(right_mapping))
                        if left_mapping.quantity_type == right_mapping.quantity_type =>
                    {
                        if left_mapping.to_base_factor <= right_mapping.to_base_factor {
                            Some(left.clone())
                        } else {
                            Some(right.clone())
                        }
                    }
                    _ => Some(left.clone()),
                }
            }
            (Some(unit), None) | (None, Some(unit)) => Some(unit.clone()),
            (None, None) => None,
        }
    }

    /// Multiply a quantity by a scalar
    pub fn multiply_by_scalar(
        &self,
        value: Decimal,
        unit: &Option<String>,
        scalar: Decimal,
    ) -> FhirPathValue {
        FhirPathValue::Quantity {
            value: value * scalar,
            unit: unit.clone(),
        }
    }

    pub fn multiply_quantities(
        &self,
        left_value: Decimal,
        left_unit: &Option<String>,
        right_value: Decimal,
        right_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        let left_f64 = left_value.to_f64().unwrap_or(0.0);
        let right_f64 = right_value.to_f64().unwrap_or(0.0);
        let (left_base, left_base_unit) = self.to_base_unit(left_f64, left_unit)?;
        let (right_base, right_base_unit) = self.to_base_unit(right_f64, right_unit)?;

        let result_f64 = left_base * right_base;
        let unit = compose_product_unit(&left_base_unit, &right_base_unit);
        Ok(FhirPathValue::Quantity {
            value: Decimal::from_f64_retain(result_f64).unwrap_or(Decimal::ZERO),
            unit,
        })
    }

    /// Divide a quantity by a scalar
    pub fn divide_by_scalar(
        &self,
        value: Decimal,
        unit: &Option<String>,
        scalar: Decimal,
    ) -> FhirPathResult<FhirPathValue> {
        if scalar == Decimal::ZERO {
            return Err(FhirPathError::EvaluationError {
                message: "Division by zero".to_string(),
            });
        }

        Ok(FhirPathValue::Quantity {
            value: value / scalar,
            unit: unit.clone(),
        })
    }

    /// Compare two quantities with unit conversion
    /// Returns -1 if left < right, 0 if equal, 1 if left > right
    pub fn compare_quantities(
        &self,
        left_value: Decimal,
        left_unit: &Option<String>,
        right_value: Decimal,
        right_unit: &Option<String>,
    ) -> FhirPathResult<i32> {
        if !self.are_units_compatible(left_unit, right_unit) {
            return Err(FhirPathError::EvaluationError {
                message: format!(
                    "Cannot compare quantities with incompatible units: {left_unit:?} and {right_unit:?}"
                ),
            });
        }

        let left_f64 = left_value.to_f64().unwrap_or(0.0);
        let right_f64 = right_value.to_f64().unwrap_or(0.0);
        let (left_base, _) = self.to_base_unit(left_f64, left_unit)?;
        let (right_base, _) = self.to_base_unit(right_f64, right_unit)?;

        if (left_base - right_base).abs() <= 1e-12 {
            Ok(0)
        } else if left_base < right_base {
            Ok(-1)
        } else if left_base > right_base {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// Divide two quantities (same units result in dimensionless)
    pub fn divide_quantities(
        &self,
        left_value: Decimal,
        left_unit: &Option<String>,
        right_value: Decimal,
        right_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        if right_value == Decimal::ZERO {
            return Err(FhirPathError::EvaluationError {
                message: "Division by zero".to_string(),
            });
        }

        let left_f64 = left_value.to_f64().unwrap_or(0.0);
        let right_f64 = right_value.to_f64().unwrap_or(0.0);
        let (left_base, left_base_unit) = self.to_base_unit(left_f64, left_unit)?;
        let (right_base, right_base_unit) = self.to_base_unit(right_f64, right_unit)?;

        let result = left_base / right_base;

        if self.are_units_compatible(&left_base_unit, &right_base_unit) {
            return Ok(FhirPathValue::Quantity {
                value: Decimal::from_f64_retain(result).unwrap_or(Decimal::ZERO),
                unit: Some("1".to_string()),
            });
        }

        Ok(FhirPathValue::Quantity {
            value: Decimal::from_f64_retain(result).unwrap_or(Decimal::ZERO),
            unit: compose_quotient_unit(&left_base_unit, &right_base_unit),
        })
    }
}

impl Default for UnitConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Unit evaluator that wraps the UnitConverter for consistency with other evaluators
pub struct UnitEvaluator;

impl UnitEvaluator {
    /// Create a new UnitConverter instance
    pub fn new_converter() -> UnitConverter {
        UnitConverter::new()
    }
}

fn compose_product_unit(left: &Option<String>, right: &Option<String>) -> Option<String> {
    match (left.as_deref(), right.as_deref()) {
        (None, None) => None,
        (Some(unit), None) | (None, Some(unit)) => Some(unit.to_string()),
        (Some(left), Some(right)) => {
            if let Some(unit) = multiply_simple_units(left, right) {
                return Some(unit);
            }
            if left == right {
                Some(format!("{left}2"))
            } else {
                Some(format!("{left}.{right}"))
            }
        }
    }
}

fn multiply_simple_units(left: &str, right: &str) -> Option<String> {
    let (left_base, left_power) = parse_simple_power_unit(left).unwrap_or((left.to_string(), 1));
    let (right_base, right_power) =
        parse_simple_power_unit(right).unwrap_or((right.to_string(), 1));
    if left_base == right_base {
        let power = left_power + right_power;
        return if power == 1 {
            Some(left_base)
        } else {
            Some(format!("{left_base}{power}"))
        };
    }
    None
}

fn parse_simple_power_unit(unit: &str) -> Option<(String, i32)> {
    let digits_start = unit
        .char_indices()
        .find_map(|(idx, ch)| ch.is_ascii_digit().then_some(idx))?;
    if digits_start == 0 {
        return None;
    }
    let (base, power) = unit.split_at(digits_start);
    if base.is_empty() || power.is_empty() || !power.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    let power = power.parse::<i32>().ok()?;
    if power <= 1 {
        return None;
    }
    Some((base.to_string(), power))
}

fn compose_quotient_unit(left: &Option<String>, right: &Option<String>) -> Option<String> {
    match (left.as_deref(), right.as_deref()) {
        (None, None) => Some("1".to_string()),
        (Some(unit), None) => Some(unit.to_string()),
        (None, Some(unit)) => Some(format!("1/{unit}")),
        (Some(left), Some(right)) if left == right => Some("1".to_string()),
        (Some(left), Some(right)) => Some(format!("{left}/{right}")),
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
        assert!(result.is_err());
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
    fn test_temperature_conversion() {
        let converter = UnitConverter::new();

        let result = converter
            .add_quantities(
                Decimal::from_str_exact("0.0").unwrap(),
                &Some("Cel".to_string()),
                Decimal::from_str_exact("0.0").unwrap(),
                &Some("K".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert!((value.to_f64().unwrap() - (-273.15)).abs() < 0.01);
            assert_eq!(unit, Some("Cel".to_string()));
        } else {
            panic!("Expected Quantity result");
        }

        let result = converter
            .add_quantities(
                Decimal::from_str_exact("20.0").unwrap(),
                &Some("Cel".to_string()),
                Decimal::from_str_exact("5.0").unwrap(),
                &Some("Cel".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert!((value.to_f64().unwrap() - 25.0).abs() < 0.01);
            assert_eq!(unit, Some("Cel".to_string()));
        } else {
            panic!("Expected Quantity result");
        }

        let result = converter
            .subtract_quantities(
                Decimal::from_str_exact("100.0").unwrap(),
                &Some("Cel".to_string()),
                Decimal::from_str_exact("0.0").unwrap(),
                &Some("Cel".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert!((value.to_f64().unwrap() - 100.0).abs() < 0.01);
            assert_eq!(unit, Some("Cel".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
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
