//! Unit conversion and handling for UCUM units
//!
//! This module provides unit conversion between compatible UCUM units using a custom
//! conversion system with predefined conversion factors for medical and scientific units.

use crate::error::*;
use crate::evaluator::values::FhirPathValue;
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
        unit_mappings.insert(
            "mo".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Time,
                base_unit: "s".to_string(),
                to_base_factor: 2629746.0, // Average month in seconds
                from_base_factor: 1.0 / 2629746.0,
            },
        );
        unit_mappings.insert(
            "a".to_string(),
            UnitMapping {
                quantity_type: QuantityType::Time,
                base_unit: "s".to_string(),
                to_base_factor: 31556952.0, // Average year in seconds
                from_base_factor: 1.0 / 31556952.0,
            },
        );

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
            (Some(u1), Some(u2)) => {
                if u1 == u2 {
                    return true; // Same units
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
            _ => false, // One has units, one doesn't
        }
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
        left_value: f64,
        left_unit: &Option<String>,
        right_value: f64,
        right_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        if !self.are_units_compatible(left_unit, right_unit) {
            return Err(FhirPathError::EvaluationError {
                message: format!(
                    "Cannot add quantities with incompatible units: {left_unit:?} and {right_unit:?}"
                ),
            });
        }

        // Convert both to base units
        let (left_base, left_base_unit) = self.to_base_unit(left_value, left_unit)?;
        let (right_base, _right_base_unit) = self.to_base_unit(right_value, right_unit)?;

        // Add the base values
        let result_base = left_base + right_base;

        // Convert back to the left operand's unit (maintain left operand unit for result)
        let result_value = self.convert_from_base_unit(result_base, &left_base_unit, left_unit)?;

        Ok(FhirPathValue::Quantity {
            value: result_value,
            unit: left_unit.clone(),
        })
    }

    /// Subtract two quantities with automatic unit conversion
    pub fn subtract_quantities(
        &self,
        left_value: f64,
        left_unit: &Option<String>,
        right_value: f64,
        right_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        if !self.are_units_compatible(left_unit, right_unit) {
            return Err(FhirPathError::EvaluationError {
                message: format!(
                    "Cannot subtract quantities with incompatible units: {left_unit:?} and {right_unit:?}"
                ),
            });
        }

        // Convert both to base units
        let (left_base, left_base_unit) = self.to_base_unit(left_value, left_unit)?;
        let (right_base, _) = self.to_base_unit(right_value, right_unit)?;

        // Subtract the base values
        let result_base = left_base - right_base;

        // Convert back to the left operand's unit
        let result_value = self.convert_from_base_unit(result_base, &left_base_unit, left_unit)?;

        Ok(FhirPathValue::Quantity {
            value: result_value,
            unit: left_unit.clone(),
        })
    }

    /// Multiply a quantity by a scalar
    pub fn multiply_by_scalar(
        &self,
        value: f64,
        unit: &Option<String>,
        scalar: f64,
    ) -> FhirPathValue {
        FhirPathValue::Quantity {
            value: value * scalar,
            unit: unit.clone(),
        }
    }

    /// Divide a quantity by a scalar
    pub fn divide_by_scalar(
        &self,
        value: f64,
        unit: &Option<String>,
        scalar: f64,
    ) -> FhirPathResult<FhirPathValue> {
        if scalar == 0.0 {
            return Err(FhirPathError::EvaluationError {
                message: "Division by zero".to_string(),
            });
        }

        Ok(FhirPathValue::Quantity {
            value: value / scalar,
            unit: unit.clone(),
        })
    }

    /// Divide two quantities (same units result in dimensionless)
    pub fn divide_quantities(
        &self,
        left_value: f64,
        left_unit: &Option<String>,
        right_value: f64,
        right_unit: &Option<String>,
    ) -> FhirPathResult<FhirPathValue> {
        if right_value == 0.0 {
            return Err(FhirPathError::EvaluationError {
                message: "Division by zero".to_string(),
            });
        }

        if !self.are_units_compatible(left_unit, right_unit) {
            return Err(FhirPathError::EvaluationError {
                message: format!(
                    "Cannot divide quantities with incompatible units: {left_unit:?} and {right_unit:?}"
                ),
            });
        }

        // Convert both to base units
        let (left_base, _) = self.to_base_unit(left_value, left_unit)?;
        let (right_base, _) = self.to_base_unit(right_value, right_unit)?;

        // Divide the base values - result is dimensionless when same units
        let result = left_base / right_base;

        Ok(FhirPathValue::Number(result))
    }
}

impl Default for UnitConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_conversion() {
        let converter = UnitConverter::new();

        // Test kilogram to gram addition
        let result = converter
            .add_quantities(1.0, &Some("kg".to_string()), 500.0, &Some("g".to_string()))
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 1.5); // 1kg + 0.5kg = 1.5kg
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_length_conversion() {
        let converter = UnitConverter::new();

        // Test meter to centimeter addition
        let result = converter
            .add_quantities(1.0, &Some("m".to_string()), 50.0, &Some("cm".to_string()))
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 1.5); // 1m + 0.5m = 1.5m
            assert_eq!(unit, Some("m".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_incompatible_units() {
        let converter = UnitConverter::new();

        // Test adding mass and length (should fail)
        let result =
            converter.add_quantities(1.0, &Some("kg".to_string()), 1.0, &Some("m".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_division() {
        let converter = UnitConverter::new();

        // Test same units division (should result in dimensionless)
        let result = converter
            .divide_quantities(10.0, &Some("kg".to_string()), 5.0, &Some("kg".to_string()))
            .unwrap();
        if let FhirPathValue::Number(ratio) = result {
            assert_eq!(ratio, 2.0);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_scalar_operations() {
        let converter = UnitConverter::new();

        // Test scalar multiplication
        let result = converter.multiply_by_scalar(5.0, &Some("mg".to_string()), 2.0);
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 10.0);
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_temperature_conversion() {
        let converter = UnitConverter::new();

        // Test Celsius to Kelvin conversion: 0°C + 0K = -273.15°C
        let result = converter
            .add_quantities(0.0, &Some("Cel".to_string()), 0.0, &Some("K".to_string()))
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            // 0°C + 0K: 0°C + (-273.15°C) = -273.15°C
            assert!((value - (-273.15)).abs() < 0.01);
            assert_eq!(unit, Some("Cel".to_string()));
        } else {
            panic!("Expected Quantity result");
        }

        // Test same unit temperature addition
        let result = converter
            .add_quantities(
                20.0,
                &Some("Cel".to_string()),
                5.0,
                &Some("Cel".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            // Note: Now with Celsius as base unit, this is simply 20°C + 5°C = 25°C
            assert!((value - 25.0).abs() < 0.01);
            assert_eq!(unit, Some("Cel".to_string()));
        } else {
            panic!("Expected Quantity result");
        }

        // Test temperature subtraction
        let result = converter
            .subtract_quantities(
                100.0,
                &Some("Cel".to_string()),
                0.0,
                &Some("Cel".to_string()),
            )
            .unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            // 100°C - 0°C = 100°C
            assert!((value - 100.0).abs() < 0.01);
            assert_eq!(unit, Some("Cel".to_string()));
        } else {
            panic!("Expected Quantity result");
        }
    }

    #[test]
    fn test_temperature_specific_conversions() {
        let converter = UnitConverter::new();

        // Test freezing point of water: 0°C = 32°F = 273.15K
        // Test 0°C to Celsius (base unit)
        let (celsius_val, _) = converter
            .to_base_unit(0.0, &Some("Cel".to_string()))
            .unwrap();
        assert!((celsius_val - 0.0).abs() < 0.01);

        // Test 32°F to Celsius
        let (celsius_val, _) = converter
            .to_base_unit(32.0, &Some("[degF]".to_string()))
            .unwrap();
        assert!((celsius_val - 0.0).abs() < 0.01);

        // Test 273.15K to Celsius
        let (celsius_val, _) = converter
            .to_base_unit(273.15, &Some("K".to_string()))
            .unwrap();
        assert!((celsius_val - 0.0).abs() < 0.01);

        // Test Celsius to Kelvin
        let kelvin_val = converter
            .convert_from_base_unit(0.0, &Some("Cel".to_string()), &Some("K".to_string()))
            .unwrap();
        assert!((kelvin_val - 273.15).abs() < 0.01);

        // Test Celsius to Fahrenheit
        let fahrenheit_val = converter
            .convert_from_base_unit(0.0, &Some("Cel".to_string()), &Some("[degF]".to_string()))
            .unwrap();
        assert!((fahrenheit_val - 32.0).abs() < 0.01);
    }
}
