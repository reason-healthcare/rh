//! Example demonstrating CompilerOptions usage.
//!
//! Shows how to configure the CQL-to-ELM translator with various options.

use rh_cql::options::{CompilerOption, CompilerOptions, ErrorSeverity, SignatureLevel};
use rh_cql::ExpressionTranslator;

fn main() {
    println!("=== CompilerOptions Example ===\n");

    // Default options (recommended settings)
    println!("1. Default Options");
    let default_options = CompilerOptions::default();
    println!("   Annotations: {}", default_options.annotations_enabled());
    println!("   Locators: {}", default_options.locators_enabled());
    println!(
        "   Result Types: {}",
        default_options.result_types_enabled()
    );
    println!(
        "   List Demotion: {}",
        default_options.list_demotion_enabled()
    );
    println!(
        "   List Promotion: {}",
        default_options.list_promotion_enabled()
    );
    println!("   Error Level: {:?}", default_options.error_level);
    println!("   Signature Level: {:?}", default_options.signature_level);
    println!("   Options String: {}", default_options.options_to_string());
    println!();

    // Empty options (no options enabled)
    println!("2. Empty Options (no options enabled)");
    let empty_options = CompilerOptions::new();
    println!("   Annotations: {}", empty_options.annotations_enabled());
    println!("   Locators: {}", empty_options.locators_enabled());
    println!();

    // Debug options (all annotation-related options enabled)
    println!("3. Debug Options");
    let debug_options = CompilerOptions::debug();
    println!("   Annotations: {}", debug_options.annotations_enabled());
    println!("   Locators: {}", debug_options.locators_enabled());
    println!("   Result Types: {}", debug_options.result_types_enabled());
    println!("   Options String: {}", debug_options.options_to_string());
    println!();

    // Strict options (disable implicit conversions)
    println!("4. Strict Options");
    let strict_options = CompilerOptions::strict();
    println!(
        "   List Traversal: {}",
        strict_options.list_traversal_enabled()
    );
    println!(
        "   List Demotion: {}",
        strict_options.list_demotion_enabled()
    );
    println!(
        "   List Promotion: {}",
        strict_options.list_promotion_enabled()
    );
    println!(
        "   Method Invocation: {}",
        strict_options.method_invocation_enabled()
    );
    println!("   Options String: {}", strict_options.options_to_string());
    println!();

    // Custom options using builder pattern
    println!("5. Custom Options (Builder Pattern)");
    let custom_options = CompilerOptions::new()
        .with_option(CompilerOption::EnableAnnotations)
        .with_option(CompilerOption::EnableLocators)
        .with_option(CompilerOption::EnableResultTypes)
        .with_option(CompilerOption::DisableListDemotion)
        .with_option(CompilerOption::DisableListPromotion)
        .with_signature_level(SignatureLevel::All)
        .with_error_level(ErrorSeverity::Warning)
        .with_compatibility_level("1.5");

    println!("   Annotations: {}", custom_options.annotations_enabled());
    println!("   Result Types: {}", custom_options.result_types_enabled());
    println!("   Signature Level: {:?}", custom_options.signature_level);
    println!("   Error Level: {:?}", custom_options.error_level);
    println!("   Options String: {}", custom_options.options_to_string());
    println!();

    // Demonstrate translator with options
    println!("6. ExpressionTranslator with Options");

    // Without annotations - no local IDs
    let mut translator_no_annotations = ExpressionTranslator::with_options(CompilerOptions::new());
    let literal = rh_cql::parser::ast::Literal::Integer(42);
    let elm_expr = translator_no_annotations.translate_literal(&literal);
    if let rh_cql::elm::Expression::Literal(lit) = &elm_expr {
        println!(
            "   Without annotations - local_id: {:?}",
            lit.element.local_id
        );
    }

    // With annotations - has local IDs
    let mut translator_with_annotations = ExpressionTranslator::new(); // Default has annotations
    let elm_expr = translator_with_annotations.translate_literal(&literal);
    if let rh_cql::elm::Expression::Literal(lit) = &elm_expr {
        println!("   With annotations - local_id: {:?}", lit.element.local_id);
    }
    println!();

    // Demonstrate parsing options from string
    println!("7. Parse Options from String");
    let options_str = "EnableAnnotations,EnableLocators,DisableListDemotion";
    let parsed = CompilerOptions::parse_options(options_str);
    println!("   Parsed '{options_str}': {parsed:?}");
    println!();

    // Demonstrate serialization
    println!("8. JSON Serialization");
    let options = CompilerOptions::default();
    let json = serde_json::to_string_pretty(&options).unwrap();
    println!("{json}");
    println!();

    // Demonstrate SignatureLevel variants
    println!("9. SignatureLevel Variants");
    for level in [
        SignatureLevel::None,
        SignatureLevel::Differing,
        SignatureLevel::Overloads,
        SignatureLevel::All,
    ] {
        println!("   {level:?}");
    }
    println!();

    // Demonstrate ErrorSeverity variants
    println!("10. ErrorSeverity Variants");
    for severity in [
        ErrorSeverity::Info,
        ErrorSeverity::Warning,
        ErrorSeverity::Error,
    ] {
        println!("   {severity:?}");
    }

    println!("\n=== Example Complete ===");
}
