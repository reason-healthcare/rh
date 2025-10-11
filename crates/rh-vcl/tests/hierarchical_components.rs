#[cfg(test)]
mod tests {
    use rh_vcl::{explainer::VclExplainer, parser::parse_vcl};

    #[test]
    fn test_hierarchical_component_nesting() {
        let explainer = VclExplainer::new();

        // Test a nested expression with multiple levels
        let vcl = "code1, (parent = value1; (child = value2))";
        let ast = parse_vcl(vcl).expect("Failed to parse VCL");
        let result = explainer
            .explain_with_text(&ast, vcl)
            .expect("Failed to explain VCL");

        // Verify that components have correct nesting levels
        let components = result.components;

        // Find specific components and check their nesting levels
        let code1_component = components
            .iter()
            .find(|c| c.component == "code1")
            .expect("code1 component not found");
        assert_eq!(
            code1_component.nesting_level, 0,
            "code1 should be at root level"
        );

        let conjunction_component = components
            .iter()
            .find(|c| c.component == ",")
            .expect("conjunction component not found");
        assert_eq!(
            conjunction_component.nesting_level, 0,
            "conjunction should be at root level"
        );

        let parent_component = components
            .iter()
            .find(|c| c.component == "parent")
            .expect("parent component not found");
        assert_eq!(
            parent_component.nesting_level, 2,
            "parent should be at nesting level 2 (inside first parentheses, which is treated as a nested sub-expression)"
        );

        let disjunction_component = components
            .iter()
            .find(|c| c.component == ";")
            .expect("disjunction component not found");
        assert_eq!(
            disjunction_component.nesting_level, 2,
            "disjunction should be at nesting level 2 (inside first parentheses)"
        );

        let child_component = components
            .iter()
            .find(|c| c.component == "child")
            .expect("child component not found");
        assert_eq!(
            child_component.nesting_level, 4,
            "child should be at nesting level 4 (inside double-nested parentheses)"
        );

        // Verify that deeper nested components have higher nesting levels
        let max_nesting_level = components.iter().map(|c| c.nesting_level).max().unwrap();
        assert_eq!(max_nesting_level, 4, "Maximum nesting level should be 4");
    }

    #[test]
    fn test_simple_expression_has_zero_nesting() {
        let explainer = VclExplainer::new();

        let vcl = "simple_code";
        let ast = parse_vcl(vcl).expect("Failed to parse VCL");
        let result = explainer
            .explain_with_text(&ast, vcl)
            .expect("Failed to explain VCL");

        // All components should be at nesting level 0 for simple expressions
        for component in &result.components {
            assert_eq!(
                component.nesting_level, 0,
                "Component '{}' should be at nesting level 0 in simple expression",
                component.component
            );
        }
    }

    #[test]
    fn test_system_uri_with_nested_content() {
        let explainer = VclExplainer::new();

        let vcl = "(http://snomed.info/sct)(parent = 123456), *";
        let ast = parse_vcl(vcl).expect("Failed to parse VCL");
        let result = explainer
            .explain_with_text(&ast, vcl)
            .expect("Failed to explain VCL");

        // System URI should be at level 0
        let system_component = result
            .components
            .iter()
            .find(|c| c.component_type == "System URI")
            .expect("system URI component not found");
        assert_eq!(
            system_component.nesting_level, 0,
            "System URI should be at root level"
        );

        // Filter components inside system should be at level 1 (nested within system context)
        let property_component = result
            .components
            .iter()
            .find(|c| c.component == "parent")
            .expect("parent component not found");
        assert_eq!(
            property_component.nesting_level, 1,
            "Property within system should be at nesting level 1"
        );

        // Conjunction and subsequent components
        let conjunction_component = result
            .components
            .iter()
            .find(|c| c.component == ",")
            .expect("conjunction component not found");
        assert_eq!(
            conjunction_component.nesting_level, 0,
            "conjunction should be at root level"
        );
    }
}
