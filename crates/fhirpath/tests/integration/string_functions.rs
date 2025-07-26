//! String function tests
//! 
//! Tests for string manipulation functions in FHIRPath expressions

use super::*;

#[test]
fn test_string_length_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test length on string literal
    let expr = parser.parse("'hello'.length()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(5));

    // Test length on empty string
    let expr = parser.parse("''.length()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(0));

    // Test length on longer string
    let expr = parser.parse("'hello world'.length()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(11));
}

#[test]
fn test_string_case_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test upper() function
    let expr = parser.parse("'hello world'.upper()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("HELLO WORLD".to_string()));

    // Test lower() function
    let expr = parser.parse("'HELLO WORLD'.lower()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello world".to_string()));

    // Test mixed case
    let expr = parser.parse("'HeLLo WoRLd'.upper()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("HELLO WORLD".to_string()));

    let expr = parser.parse("'HeLLo WoRLd'.lower()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello world".to_string()));
}

#[test]
fn test_string_trim_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test trim with leading and trailing spaces
    let expr = parser.parse("'  hello world  '.trim()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello world".to_string()));

    // Test trim with no spaces
    let expr = parser.parse("'hello'.trim()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello".to_string()));

    // Test trim with only spaces
    let expr = parser.parse("'   '.trim()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("".to_string()));

    // Test trim with tabs and newlines
    let expr = parser.parse("'\t\nhello\t\n'.trim()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello".to_string()));
}

#[test]
fn test_string_substring_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test substring with start index only
    let expr = parser.parse("'hello world'.substring(6)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("world".to_string()));

    // Test substring with start and length
    let expr = parser.parse("'hello world'.substring(0, 5)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello".to_string()));

    // Test substring with middle portion
    let expr = parser.parse("'hello world'.substring(2, 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("llo".to_string()));

    // Test substring beyond string length
    let expr = parser.parse("'hello'.substring(10)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("".to_string()));

    // Test substring with length beyond string
    let expr = parser.parse("'hello'.substring(2, 10)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("llo".to_string()));
}

#[test]
fn test_string_starts_ends_with_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test startsWith - positive case
    let expr = parser.parse("'hello world'.startsWith('hello')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test startsWith - negative case
    let expr = parser.parse("'hello world'.startsWith('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test endsWith - positive case
    let expr = parser.parse("'hello world'.endsWith('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test endsWith - negative case
    let expr = parser.parse("'hello world'.endsWith('hello')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test with empty string
    let expr = parser.parse("'hello'.startsWith('')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'hello'.endsWith('')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_string_index_of_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test indexOf - found case
    let expr = parser.parse("'hello world'.indexOf('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(6));

    // Test indexOf - not found case
    let expr = parser.parse("'hello world'.indexOf('xyz')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(-1));

    // Test indexOf - first occurrence
    let expr = parser.parse("'hello hello'.indexOf('hello')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(0));

    // Test indexOf - single character
    let expr = parser.parse("'hello world'.indexOf('o')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(4));

    // Test indexOf - empty string
    let expr = parser.parse("'hello'.indexOf('')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(0));
}

#[test]
fn test_string_replace_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test replace - single character
    let expr = parser.parse("'hello world'.replace('l', 'x')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hexxo worxd".to_string()));

    // Test replace - substring
    let expr = parser.parse("'hello world'.replace('world', 'universe')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello universe".to_string()));

    // Test replace - no matches
    let expr = parser.parse("'hello world'.replace('xyz', 'abc')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello world".to_string()));

    // Test replace - empty replacement
    let expr = parser.parse("'hello world'.replace(' ', '')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("helloworld".to_string()));

    // Test replace - multiple occurrences
    let expr = parser.parse("'aa bb aa'.replace('aa', 'cc')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("cc bb cc".to_string()));
}

#[test]
fn test_string_split_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test split with comma
    let expr = parser.parse("'a,b,c'.split(',')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], FhirPathValue::String("a".to_string()));
        assert_eq!(items[1], FhirPathValue::String("b".to_string()));
        assert_eq!(items[2], FhirPathValue::String("c".to_string()));
    } else {
        panic!("Expected collection result from split");
    }

    // Test split with space
    let expr = parser.parse("'hello world test'.split(' ')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], FhirPathValue::String("hello".to_string()));
        assert_eq!(items[1], FhirPathValue::String("world".to_string()));
        assert_eq!(items[2], FhirPathValue::String("test".to_string()));
    } else {
        panic!("Expected collection result from split");
    }

    // Test split with no delimiter found
    let expr = parser.parse("'hello'.split(',')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], FhirPathValue::String("hello".to_string()));
    } else {
        panic!("Expected collection result from split");
    }

    // Test split with empty string
    let expr = parser.parse("''.split(',')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], FhirPathValue::String("".to_string()));
    } else {
        panic!("Expected collection result from split");
    }
}

#[test]
fn test_string_join_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test join with collection of strings
    // Note: This test assumes we can create collections directly in FHIRPath
    // In practice, collections would come from split() or other operations
    
    // We'll test this by first splitting then joining
    let expr = parser.parse("'a,b,c'.split(',').join('|')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("a|b|c".to_string()));

    // Test join with different delimiter
    let expr = parser.parse("'hello world test'.split(' ').join('-')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello-world-test".to_string()));

    // Test join with empty delimiter
    let expr = parser.parse("'a,b,c'.split(',').join('')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("abc".to_string()));

    // Test join with space delimiter
    let expr = parser.parse("'a,b,c'.split(',').join(' ')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("a b c".to_string()));
}

#[test]
fn test_string_matches_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test matches with digit pattern
    let expr = parser.parse("'123'.matches('\\d+')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'abc'.matches('\\d+')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test matches with word pattern
    let expr = parser.parse("'hello123'.matches('\\w+')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'hello-world'.matches('\\w+')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false)); // hyphen not included in \w+

    // Test matches with literal pattern
    let expr = parser.parse("'hello'.matches('hello')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'hello'.matches('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_string_functions_with_property_access() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    
    // Create a patient with string properties
    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "DOE",
            "given": ["  John  ", "James"]
        }],
        "gender": "male"
    });
    let context = EvaluationContext::new(patient);

    // Test string functions on FHIR data properties
    let expr = parser.parse("gender.upper()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "MALE");
    } else {
        panic!("Expected string result from gender.upper()");
    }

    let expr = parser.parse("gender.length()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(4));

    let expr = parser.parse("id.startsWith('ex')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_chained_string_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test chaining multiple string functions
    let expr = parser.parse("'  HELLO WORLD  '.trim().lower()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello world".to_string()));

    let expr = parser.parse("'hello,world'.split(',').join(' ').upper()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("HELLO WORLD".to_string()));

    let expr = parser.parse("'Hello World'.replace(' ', '_').lower()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello_world".to_string()));

    let expr = parser.parse("'test123'.substring(0, 4).upper()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("TEST".to_string()));
}

#[test]
fn test_string_functions_error_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test functions on non-string values should return appropriate errors
    // Note: These tests verify that the parser accepts the expressions
    // Error handling during evaluation depends on the evaluator implementation
    
    // Test that these expressions parse correctly
    let error_cases = vec![
        "42.length()",          // Should error: number has no length
        "true.upper()",         // Should error: boolean has no upper
        "42.substring(0, 2)",   // Should error: number has no substring
    ];

    for expr_str in error_cases {
        match parser.parse(expr_str) {
            Ok(_expr) => {
                println!("✓ Parsed (will error at evaluation): {expr_str}");
                // The expression parses but should error during evaluation
            }
            Err(e) => {
                panic!("Failed to parse {expr_str}: {e:?}");
            }
        }
    }
}
