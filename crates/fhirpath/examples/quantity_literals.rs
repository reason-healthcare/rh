//! Quantity Literals Example
//!
//! This example demonstrates the quantity literal support in FHIRPath expressions.
//! Quantity literals represent values with units (UCUM or calendar durations) and are
//! essential for healthcare calculations involving measurements, dosages, and time periods.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() {
    println!("⚖️  FHIRPath Quantity Literals Examples\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // === Basic Quantity Literals ===
    println!("📏 Basic Quantity Literals:");

    // Weight measurements
    demo_quantity(
        &parser,
        &evaluator,
        "70.5'kg'",
        "Patient weight in kilograms",
    );
    demo_quantity(&parser, &evaluator, "155'lb'", "Patient weight in pounds");

    // Temperature measurements
    demo_quantity(
        &parser,
        &evaluator,
        "37.2'Cel'",
        "Body temperature in Celsius",
    );
    demo_quantity(
        &parser,
        &evaluator,
        "98.9'[degF]'",
        "Body temperature in Fahrenheit",
    );

    // Blood pressure
    demo_quantity(
        &parser,
        &evaluator,
        "120'mm[Hg]'",
        "Systolic blood pressure",
    );
    demo_quantity(
        &parser,
        &evaluator,
        "80'mm[Hg]'",
        "Diastolic blood pressure",
    );

    // Medication dosages
    demo_quantity(
        &parser,
        &evaluator,
        "5'mg'",
        "Medication dose in milligrams",
    );
    demo_quantity(&parser, &evaluator, "2.5'mg/kg'", "Weight-based dosage");
    demo_quantity(&parser, &evaluator, "500'mL'", "Fluid volume");

    // === Calendar Duration Units ===
    println!("\n📅 Calendar Duration Examples:");

    demo_quantity(&parser, &evaluator, "2'wk'", "Treatment duration in weeks");
    demo_quantity(
        &parser,
        &evaluator,
        "30'd'",
        "Prescription duration in days",
    );
    demo_quantity(&parser, &evaluator, "6'mo'", "Follow-up period in months");
    demo_quantity(&parser, &evaluator, "1'a'", "Annual screening interval");

    // === Dimensionless Quantities ===
    println!("\n🔢 Dimensionless Quantities:");

    demo_quantity(&parser, &evaluator, "1.5''", "Ratio or scale factor");
    demo_quantity(&parser, &evaluator, "0.95''", "Percentage as decimal");

    // === Quantity Arithmetic ===
    println!("\n🧮 Quantity Arithmetic Operations:");

    // Addition and subtraction (same units)
    demo_calculation(
        &parser,
        &evaluator,
        "70'kg' + 2.5'kg'",
        "Weight gain calculation",
    );
    demo_calculation(
        &parser,
        &evaluator,
        "120'mm[Hg]' - 15'mm[Hg]'",
        "Blood pressure change",
    );

    // Multiplication and division
    demo_calculation(
        &parser,
        &evaluator,
        "5'mg' * 2",
        "Double the medication dose",
    );
    demo_calculation(
        &parser,
        &evaluator,
        "100'mL' / 4",
        "Divide fluid into portions",
    );
    demo_calculation(
        &parser,
        &evaluator,
        "10'mg' / 5'mg'",
        "Dose ratio (dimensionless)",
    );

    // Complex calculations
    demo_calculation(
        &parser,
        &evaluator,
        "(75'kg' + 5'kg') * 0.2",
        "20% of adjusted weight",
    );

    // === Healthcare Calculation Examples ===
    println!("\n🏥 Healthcare Calculation Examples:");

    let patient_data = json!({
        "patient": {
            "weight": "75'kg'",
            "height": "175'cm'",
            "age": "45'a'"
        },
        "medication": {
            "dose_per_kg": "0.1'mg/kg'",
            "max_dose": "10'mg'"
        },
        "vitals": {
            "temperature": "38.5'Cel'",
            "bp_systolic": "140'mm[Hg]'",
            "bp_diastolic": "90'mm[Hg]'"
        }
    });

    let _context = EvaluationContext::new(patient_data);

    // Note: These are simplified examples - real FHIRPath evaluation would work with
    // proper FHIR resources and support direct quantity evaluation from strings
    println!("  📋 Patient Data Available:");
    println!("    Weight: 75kg, Height: 175cm, Age: 45 years");
    println!("    Temperature: 38.5°C, BP: 140/90 mmHg");
    println!("    Medication: 0.1mg/kg (max 10mg)");

    // === Error Handling Examples ===
    println!("\n⚠️  Error Handling Examples:");

    // Different unit errors
    demo_error(
        &parser,
        &evaluator,
        "5'mg' + 3'kg'",
        "Adding incompatible units",
    );
    demo_error(
        &parser,
        &evaluator,
        "10'L' - 2'mL'",
        "Subtracting different volume units",
    );
    demo_error(
        &parser,
        &evaluator,
        "120'mm[Hg]' + 5",
        "Adding number to quantity with units",
    );

    // Division by zero
    demo_error(&parser, &evaluator, "10'mg' / 0", "Division by zero");

    // === Advanced UCUM Examples ===
    println!("\n🔬 Advanced UCUM Unit Examples:");

    // Complex UCUM units
    demo_quantity(&parser, &evaluator, "1.2'g/dL'", "Hemoglobin concentration");
    demo_quantity(&parser, &evaluator, "150'umol/L'", "Creatinine level");
    demo_quantity(&parser, &evaluator, "7.4'pH'", "Blood pH level");
    demo_quantity(
        &parser,
        &evaluator,
        "4.5'10*6/uL'",
        "White blood cell count",
    );
    demo_quantity(&parser, &evaluator, "2.5'ng/mL'", "Biomarker concentration");

    println!("\n✅ Quantity literals demonstration complete!");
    println!("Key Features Demonstrated:");
    println!("  • UCUM unit support for precise healthcare measurements");
    println!("  • Calendar duration units for time-based calculations");
    println!("  • Arithmetic operations with unit compatibility checking");
    println!("  • Dimensionless quantities for ratios and percentages");
    println!("  • Proper error handling for incompatible operations");
    println!("  • Healthcare-specific examples (vitals, dosages, lab values)");
}

fn demo_quantity(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    expression: &str,
    description: &str,
) {
    let empty_context = EvaluationContext::new(json!({}));
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, &empty_context) {
            Ok(result) => match result {
                FhirPathValue::Quantity { value, unit } => {
                    let unit_str = unit.unwrap_or_else(|| "".to_string());
                    println!("  {description}: {expression} = {value}{unit_str}");
                }
                _ => {
                    println!("  {description}: {expression} = {result:?} (unexpected type)");
                }
            },
            Err(e) => {
                println!("  {description}: {expression} → ERROR: {e}");
            }
        },
        Err(e) => {
            println!("  {description}: {expression} → PARSE ERROR: {e}");
        }
    }
}

fn demo_calculation(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    expression: &str,
    description: &str,
) {
    let empty_context = EvaluationContext::new(json!({}));
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, &empty_context) {
            Ok(result) => {
                let result_str = match result {
                    FhirPathValue::Quantity { value, unit } => {
                        let unit_str = unit.unwrap_or_else(|| "".to_string());
                        format!("{value}{unit_str}")
                    }
                    FhirPathValue::Number(n) => n.to_string(),
                    FhirPathValue::Integer(i) => i.to_string(),
                    _ => format!("{result:?}"),
                };
                println!("  {description}: {expression} = {result_str}");
            }
            Err(e) => {
                println!("  {description}: {expression} → ERROR: {e}");
            }
        },
        Err(e) => {
            println!("  {description}: {expression} → PARSE ERROR: {e}");
        }
    }
}

fn demo_error(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    expression: &str,
    description: &str,
) {
    let empty_context = EvaluationContext::new(json!({}));
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, &empty_context) {
            Ok(result) => {
                println!("  {description}: {expression} = {result:?} (unexpected success)");
            }
            Err(e) => {
                println!("  {description}: {expression} → ✓ Correctly handled error: {e}");
            }
        },
        Err(e) => {
            println!("  {description}: {expression} → Parse error: {e}");
        }
    }
}
