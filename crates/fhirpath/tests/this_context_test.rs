use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn create_test_data() -> EvaluationContext {
    let data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "use": "official",
                "given": ["John", "William"],
                "family": "Doe"
            },
            {
                "use": "nickname",
                "given": ["Johnny"],
                "family": "Doe"
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234",
                "use": "home"
            },
            {
                "system": "email",
                "value": "john@example.com",
                "use": "work"
            }
        ],
        "scores": [85, 92, 78, 96, 88],
        "categories": ["A", "B", "AA", "AB", "BA", "BB"],
        "numbers": [1, 2, 3, 4, 5]
    });
    EvaluationContext::new(data)
}

#[test]
fn test_this_in_where_function_basic() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test with numeric comparison using $this
    let expr = parser.parse("scores.where($this > 90)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2); // 92 and 96
        assert!(matches!(items[0], FhirPathValue::Integer(92)));
        assert!(matches!(items[1], FhirPathValue::Integer(96)));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_this_in_where_function_string_comparison() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test the exact example from the spec: name.given.where($this > 'ba' and $this < 'bc')
    // This should find given names between 'ba' and 'bc' alphabetically
    let expr = parser
        .parse("name.given.where($this > 'ba' and $this < 'bc')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Should find nothing because "John", "William", "Johnny" are all outside 'ba'-'bc' range
    assert!(matches!(result, FhirPathValue::Empty));

    // Test with a range that should find "John"
    let expr = parser
        .parse("name.given.where($this >= 'J' and $this < 'K')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2); // "John" and "Johnny"
        assert!(matches!(items[0], FhirPathValue::String(ref s) if s == "John"));
        assert!(matches!(items[1], FhirPathValue::String(ref s) if s == "Johnny"));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_this_in_where_function_string_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test categories that start with 'A'
    let expr = parser
        .parse("categories.where($this.startsWith('A'))")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3); // "A", "AA", "AB"
        assert!(matches!(items[0], FhirPathValue::String(ref s) if s == "A"));
        assert!(matches!(items[1], FhirPathValue::String(ref s) if s == "AA"));
        assert!(matches!(items[2], FhirPathValue::String(ref s) if s == "AB"));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Test categories that contain 'B'
    let expr = parser
        .parse("categories.where($this.contains('B'))")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4); // "B", "AB", "BA", "BB"
        assert!(matches!(items[0], FhirPathValue::String(ref s) if s == "B"));
        assert!(matches!(items[1], FhirPathValue::String(ref s) if s == "AB"));
        assert!(matches!(items[2], FhirPathValue::String(ref s) if s == "BA"));
        assert!(matches!(items[3], FhirPathValue::String(ref s) if s == "BB"));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_this_in_where_function_complex_expressions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test with complex boolean logic
    let expr = parser
        .parse("numbers.where($this mod 2 = 0 or $this > 4)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3); // 2 (even), 4 (even), 5 (> 4)
        assert!(matches!(items[0], FhirPathValue::Integer(2)));
        assert!(matches!(items[1], FhirPathValue::Integer(4)));
        assert!(matches!(items[2], FhirPathValue::Integer(5)));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Test with arithmetic on $this
    let expr = parser.parse("numbers.where($this * 2 > 6)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2); // 4 (4*2=8>6), 5 (5*2=10>6)
        assert!(matches!(items[0], FhirPathValue::Integer(4)));
        assert!(matches!(items[1], FhirPathValue::Integer(5)));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_this_in_where_function_with_objects() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test filtering objects using $this to access properties
    let expr = parser
        .parse("telecom.where($this.system = 'phone')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // When only one item matches, FHIRPath returns a single object, not a collection
    if let FhirPathValue::Object(obj) = result {
        assert_eq!(obj["system"], "phone");
        assert_eq!(obj["value"], "555-1234");
    } else {
        panic!("Expected single object result, got: {result:?}");
    }

    // Test filtering objects with multiple conditions
    let expr = parser.parse("name.where($this.use = 'official')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Object(obj) = result {
        assert_eq!(obj["use"], "official");
        assert_eq!(obj["family"], "Doe");
    } else {
        panic!("Expected single object result, got: {result:?}");
    }
}

#[test]
fn test_this_in_select_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test select with $this transformation
    let expr = parser.parse("numbers.select($this * 2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 5); // All numbers doubled
        assert!(matches!(items[0], FhirPathValue::Integer(2)));
        assert!(matches!(items[1], FhirPathValue::Integer(4)));
        assert!(matches!(items[2], FhirPathValue::Integer(6)));
        assert!(matches!(items[3], FhirPathValue::Integer(8)));
        assert!(matches!(items[4], FhirPathValue::Integer(10)));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Test select with $this property access
    let expr = parser.parse("telecom.select($this.value)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0], FhirPathValue::String(ref s) if s == "555-1234"));
        assert!(matches!(items[1], FhirPathValue::String(ref s) if s == "john@example.com"));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_this_with_chained_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test chaining where with select using $this
    let expr = parser
        .parse("numbers.where($this > 3).select($this + 10)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2); // 4 and 5 become 14 and 15
        assert!(matches!(items[0], FhirPathValue::Integer(14)));
        assert!(matches!(items[1], FhirPathValue::Integer(15)));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Test nested where operations with $this
    let expr = parser
        .parse("name.given.where($this.length() > 4).where($this.startsWith('W'))")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "William");
    } else {
        panic!("Expected single string result, got: {result:?}");
    }
}

#[test]
fn test_this_context_isolation() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test that $this refers to the current item in the collection being filtered
    // This expression should filter telecom items where the system is 'phone',
    // then select the value of those items
    let expr = parser
        .parse("telecom.where($this.system = 'phone').select($this.value)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "555-1234");
    } else {
        panic!("Expected single string result, got: {result:?}");
    }

    // Test that $this in nested operations refers to the appropriate context
    let expr = parser
        .parse("name.where($this.given.where($this = 'John').exists())")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Object(obj) = result {
        assert_eq!(obj["use"], "official");
        if let Some(given) = obj.get("given") {
            assert!(given.as_array().unwrap().contains(&json!("John")));
        }
    } else {
        panic!("Expected single object result, got: {result:?}");
    }
}

#[test]
fn test_this_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test with empty collection
    let context = EvaluationContext::new(json!({
        "empty": []
    }));

    let expr = parser.parse("empty.where($this > 0)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Test with single item
    let context = EvaluationContext::new(json!({
        "single": [42]
    }));

    let expr = parser.parse("single.where($this = 42)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(42)));

    // Test $this on non-collection
    let context = EvaluationContext::new(json!({
        "value": 42
    }));

    let expr = parser.parse("value.where($this > 40)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(42)));
}

#[test]
fn test_this_with_different_data_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test with mixed types
    let context = EvaluationContext::new(json!({
        "mixed": [1, "hello", true, 3.15, null]
    }));

    // Filter for strings using type check
    let expr = parser.parse("mixed.where($this is String)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "hello");
    } else {
        panic!("Expected single string result, got: {result:?}");
    }

    // Filter for numbers (both integers and decimals)
    let expr = parser
        .parse("mixed.where($this is Integer or $this is Decimal)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2); // 1 and 3.15
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[1], FhirPathValue::Number(n) if (n - 3.15).abs() < f64::EPSILON));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}
