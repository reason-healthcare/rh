use rh_vcl::{parse_vcl, VclExplainer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let explainer = VclExplainer::new();

    // Test simple code
    let expr = "(123456, 789012)";
    println!("Expression: {expr}");

    match parse_vcl(expr) {
        Ok(parsed) => match explainer.explain(&parsed) {
            Ok(result) => {
                println!("Explanation: {}", result.explanation);
                println!("Type: {:?}", result.expression_type);
                println!("Translatable: {}", result.translatable_to_fhir);
                if !result.components.is_empty() {
                    println!("Components:");
                    for component in &result.components {
                        println!(
                            "  - {} ({}): {}",
                            component.component, component.component_type, component.meaning
                        );
                    }
                }
            }
            Err(e) => println!("Error explaining: {e}"),
        },
        Err(e) => println!("Parse error: {e}"),
    }

    println!();

    // Test filter expression
    let expr = "display = \"Diabetes\"";
    println!("Expression: {expr}");

    match parse_vcl(expr) {
        Ok(parsed) => match explainer.explain(&parsed) {
            Ok(result) => {
                println!("Explanation: {}", result.explanation);
                println!("Type: {:?}", result.expression_type);
                println!("Translatable: {}", result.translatable_to_fhir);
                if !result.components.is_empty() {
                    println!("Components:");
                    for component in &result.components {
                        println!(
                            "  - {} ({}): {}",
                            component.component, component.component_type, component.meaning
                        );
                    }
                }
            }
            Err(e) => println!("Error explaining: {e}"),
        },
        Err(e) => println!("Parse error: {e}"),
    }

    println!();

    // Test non-translatable expression
    let expr = "B.codeprop";
    println!("Expression: {expr}");

    match parse_vcl(expr) {
        Ok(parsed) => match explainer.explain(&parsed) {
            Ok(result) => {
                println!("Explanation: {}", result.explanation);
                println!("Type: {:?}", result.expression_type);
                println!("Translatable: {}", result.translatable_to_fhir);
            }
            Err(e) => println!("Error explaining: {e}"),
        },
        Err(e) => println!("Parse error: {e}"),
    }

    Ok(())
}
