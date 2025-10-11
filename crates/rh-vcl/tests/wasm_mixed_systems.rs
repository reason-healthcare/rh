//! WASM integration tests for mixed systems functionality
//!
//! These tests verify that the WASM bindings correctly handle mixed explicit
//! and default systems, which was a reported issue in the demo.

#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use rh_vcl::wasm::*;
    use wasm_bindgen_test::*;

    // Enable testing in both browser and Node.js environments
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_mixed_systems_basic() {
        // Test the basic mixed systems case: explicit LOINC + default SNOMED
        let options = TranslateOptions {
            default_system: Some("http://snomed.info/sct".to_string()),
            separate_conjunction_includes: false,
        };

        let result =
            translate_vcl_expression("(http://loinc.org)12345-6 ; concept <<123455", &options);

        assert!(result.success());
        assert!(result.error().is_none());

        let data = result.data().unwrap();
        let compose: serde_json::Value = serde_json::from_str(&data).unwrap();

        // Should have two includes
        let includes = compose["include"].as_array().unwrap();
        assert_eq!(includes.len(), 2);

        // First include should be LOINC with code
        assert_eq!(includes[0]["system"], "http://loinc.org");
        assert!(includes[0]["concept"].is_array());
        assert_eq!(includes[0]["concept"][0]["code"], "12345-6");

        // Second include should be SNOMED with filter
        assert_eq!(includes[1]["system"], "http://snomed.info/sct");
        assert!(includes[1]["filter"].is_array());
        assert_eq!(includes[1]["filter"][0]["property"], "concept");
        assert_eq!(includes[1]["filter"][0]["op"], "is-a");
        assert_eq!(includes[1]["filter"][0]["value"], "123455");
    }

    #[wasm_bindgen_test]
    fn test_mixed_systems_with_filters() {
        // Test the original reported case with filters
        let options = TranslateOptions {
            default_system: Some("http://snomed.info/sct".to_string()),
            separate_conjunction_includes: false,
        };

        let result = translate_vcl_expression(
            "(http://loinc.org)(parent^{LP46821-2,LP259418-4}) ; concept <<123455",
            &options,
        );

        assert!(result.success());
        assert!(result.error().is_none());

        let data = result.data().unwrap();
        let compose: serde_json::Value = serde_json::from_str(&data).unwrap();

        // Should have exactly two includes
        let includes = compose["include"].as_array().unwrap();
        assert_eq!(
            includes.len(),
            2,
            "Expected 2 includes but got {}: {}",
            includes.len(),
            data
        );

        // First include: LOINC with parent filter
        assert_eq!(includes[0]["system"], "http://loinc.org");
        assert!(includes[0]["filter"].is_array());
        assert_eq!(includes[0]["filter"][0]["property"], "parent");
        assert_eq!(includes[0]["filter"][0]["op"], "in");
        assert_eq!(includes[0]["filter"][0]["value"], "LP46821-2,LP259418-4");

        // Second include: SNOMED with concept filter
        assert_eq!(includes[1]["system"], "http://snomed.info/sct");
        assert!(includes[1]["filter"].is_array());
        assert_eq!(includes[1]["filter"][0]["property"], "concept");
        assert_eq!(includes[1]["filter"][0]["op"], "is-a");
        assert_eq!(includes[1]["filter"][0]["value"], "123455");
    }

    #[wasm_bindgen_test]
    fn test_mixed_systems_three_parts() {
        // Test the specific case mentioned: LOINC + SNOMED + default code
        // This matches the exact CLI test case that produces 3 includes
        let options = TranslateOptions {
            default_system: Some("http://snomed.info/sct".to_string()),
            separate_conjunction_includes: false,
        };

        let result = translate_vcl_expression(
            "(http://loinc.org)(parent^{LP46821-2,LP259418-4}) ; concept <<123455 ; 123",
            &options,
        );

        assert!(result.success());
        assert!(result.error().is_none());

        let data = result.data().unwrap();
        let compose: serde_json::Value = serde_json::from_str(&data).unwrap();

        // Should have exactly THREE includes (this is what was failing in the demo)
        let includes = compose["include"].as_array().unwrap();
        assert_eq!(
            includes.len(),
            3,
            "Expected 3 includes but got {}: {}",
            includes.len(),
            data
        );

        // First include: LOINC with parent filter
        assert_eq!(includes[0]["system"], "http://loinc.org");
        assert_eq!(includes[0]["filter"][0]["property"], "parent");

        // Second include: SNOMED with concept filter
        assert_eq!(includes[1]["system"], "http://snomed.info/sct");
        assert_eq!(includes[1]["filter"][0]["property"], "concept");

        // Third include: SNOMED with simple code (using default system)
        assert_eq!(includes[2]["system"], "http://snomed.info/sct");
        assert_eq!(includes[2]["concept"][0]["code"], "123");
    }

    #[wasm_bindgen_test]
    fn test_all_explicit_systems() {
        // Test that all explicit systems work correctly (no default should be used)
        let options = TranslateOptions {
            default_system: Some("http://acme.org".to_string()),
            separate_conjunction_includes: false,
        };

        let result = translate_vcl_expression(
            "(http://loinc.org)12345-6 ; (http://snomed.info/sct)concept <<654321",
            &options,
        );

        assert!(result.success());
        let data = result.data().unwrap();
        let compose: serde_json::Value = serde_json::from_str(&data).unwrap();

        let includes = compose["include"].as_array().unwrap();
        assert_eq!(includes.len(), 2);

        // Both should use explicit systems, not the ACME default
        assert_eq!(includes[0]["system"], "http://loinc.org");
        assert_eq!(includes[1]["system"], "http://snomed.info/sct");

        // Verify no ACME system is present
        for include in includes {
            assert_ne!(include["system"], "http://acme.org");
        }
    }

    #[wasm_bindgen_test]
    fn test_all_default_systems() {
        // Test that all parts use default when no explicit systems
        let options = TranslateOptions {
            default_system: Some("http://acme.org".to_string()),
            separate_conjunction_includes: false,
        };

        let result = translate_vcl_expression("12345-6 ; concept <<654321", &options);

        assert!(result.success());
        let data = result.data().unwrap();
        let compose: serde_json::Value = serde_json::from_str(&data).unwrap();

        let includes = compose["include"].as_array().unwrap();
        assert_eq!(includes.len(), 2);

        // Both should use default system
        assert_eq!(includes[0]["system"], "http://acme.org");
        assert_eq!(includes[1]["system"], "http://acme.org");
    }

    #[wasm_bindgen_test]
    fn test_no_default_system() {
        // Test behavior when no default system is provided
        let options = TranslateOptions {
            default_system: None,
            separate_conjunction_includes: false,
        };

        let result =
            translate_vcl_expression("(http://loinc.org)12345-6 ; concept <<654321", &options);

        assert!(result.success());
        let data = result.data().unwrap();
        let compose: serde_json::Value = serde_json::from_str(&data).unwrap();

        let includes = compose["include"].as_array().unwrap();
        assert_eq!(includes.len(), 2);

        // First should have LOINC system
        assert_eq!(includes[0]["system"], "http://loinc.org");

        // Second should have empty/null system since no default provided
        assert!(includes[1]["system"].is_null() || includes[1]["system"] == "");
    }
}
