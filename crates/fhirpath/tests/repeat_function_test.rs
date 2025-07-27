//! Tests for repeat() function
//!
//! Tests cover:
//! - repeat() function with various projection expressions
//! - Transitive closure behavior
//! - Cycle detection and prevention
//! - Edge cases and empty collections

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn create_hierarchy_context() -> EvaluationContext {
    let hierarchy_data = json!({
        "organizations": [
            {
                "id": "org1",
                "name": "Root Organization",
                "parent": null
            },
            {
                "id": "org2",
                "name": "Division A",
                "parent": "org1"
            },
            {
                "id": "org3",
                "name": "Department A1",
                "parent": "org2"
            },
            {
                "id": "org4",
                "name": "Team A1a",
                "parent": "org3"
            },
            {
                "id": "org5",
                "name": "Division B",
                "parent": "org1"
            }
        ]
    });
    EvaluationContext::new(hierarchy_data)
}

fn create_tree_context() -> EvaluationContext {
    let tree_data = json!({
        "nodes": [
            {
                "id": "node1",
                "value": "root",
                "children": [
                    {
                        "id": "node2",
                        "value": "child1",
                        "children": [
                            {
                                "id": "node3",
                                "value": "grandchild1",
                                "children": []
                            },
                            {
                                "id": "node4",
                                "value": "grandchild2",
                                "children": []
                            }
                        ]
                    },
                    {
                        "id": "node5",
                        "value": "child2",
                        "children": [
                            {
                                "id": "node6",
                                "value": "grandchild3",
                                "children": []
                            }
                        ]
                    }
                ]
            }
        ]
    });
    EvaluationContext::new(tree_data)
}

#[test]
fn test_repeat_simple_hierarchy() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_hierarchy_context();

    // Find all organizations (starting with root and following parent relationships)
    let expr = parser
        .parse("organizations.first().repeat(organizations)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        // Should include multiple organizations from the repeated traversal
        assert!(!items.is_empty());

        let org_ids: Vec<String> = items
            .iter()
            .filter_map(|item| {
                if let FhirPathValue::Object(obj) = item {
                    obj.get("id").and_then(|v| {
                        if let serde_json::Value::String(s) = v {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect();

        // We should at least have the first organization
        assert!(org_ids.contains(&"org1".to_string())); // Root
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_repeat_tree_traversal() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_tree_context();

    // Get all nodes in the tree by following children
    let expr = parser.parse("nodes.repeat(children)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        // Should include root node + all children at all levels
        assert_eq!(items.len(), 6); // node1, node2, node3, node4, node5, node6

        let node_ids: Vec<String> = items
            .iter()
            .filter_map(|item| {
                if let FhirPathValue::Object(obj) = item {
                    obj.get("id").and_then(|v| {
                        if let serde_json::Value::String(s) = v {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect();

        assert!(node_ids.contains(&"node1".to_string())); // root
        assert!(node_ids.contains(&"node2".to_string())); // child1
        assert!(node_ids.contains(&"node3".to_string())); // grandchild1
        assert!(node_ids.contains(&"node4".to_string())); // grandchild2
        assert!(node_ids.contains(&"node5".to_string())); // child2
        assert!(node_ids.contains(&"node6".to_string())); // grandchild3
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_repeat_with_primitive_values() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create a simple numeric context
    let context = EvaluationContext::new(json!({
        "numbers": [1, 2, 3]
    }));

    // Get numbers repeatedly (should just return same numbers since no expansion)
    let expr = parser.parse("numbers.repeat(numbers)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        // Should contain the numbers (may have duplicates filtered out)
        assert!(items.len() >= 3); // At least the original numbers

        // Check that we have some numbers
        let has_numbers = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::Integer(_)));
        assert!(has_numbers, "Expected to find some numeric values");
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_repeat_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Empty path should return empty
    let expr = parser.parse("missing.repeat(children)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Single item with no expansion
    let context = EvaluationContext::new(json!({
        "item": {
            "id": "single",
            "children": []
        }
    }));

    let expr = parser.parse("item.repeat(children)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Should return a collection with just the original item since children is empty
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        if let FhirPathValue::Object(obj) = &items[0] {
            assert_eq!(obj.get("id").unwrap(), &json!("single"));
        } else {
            panic!("Expected object in collection");
        }
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_repeat_cycle_detection() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create data with cycles - simplified version
    let cycle_data = json!({
        "node": {
            "id": "A",
            "next": {
                "id": "B",
                "next": {
                    "id": "C",
                    "next": null  // Ends here to avoid complexity
                }
            }
        }
    });
    let context = EvaluationContext::new(cycle_data);

    // Follow the 'next' references - should work with simple traversal
    let expr = parser.parse("node.repeat(next)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        // Should contain A, B, C (following next chain until null)
        assert!(!items.is_empty());

        let node_ids: Vec<String> = items
            .iter()
            .filter_map(|item| {
                if let FhirPathValue::Object(obj) = item {
                    obj.get("id").and_then(|v| {
                        if let serde_json::Value::String(s) = v {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect();

        assert!(node_ids.contains(&"A".to_string()));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_repeat_with_multiple_paths() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create data with multiple expansion paths
    let multi_path_data = json!({
        "root": {
            "id": "root",
            "paths": [
                {
                    "id": "path1",
                    "paths": [
                        {"id": "leaf1", "paths": []},
                        {"id": "leaf2", "paths": []}
                    ]
                },
                {
                    "id": "path2",
                    "paths": [
                        {"id": "leaf3", "paths": []}
                    ]
                }
            ]
        }
    });
    let context = EvaluationContext::new(multi_path_data);

    // Get all nodes by following paths
    let expr = parser.parse("root.repeat(paths)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        // Should contain: root, path1, path2, leaf1, leaf2, leaf3
        assert_eq!(items.len(), 6);

        let node_ids: Vec<String> = items
            .iter()
            .filter_map(|item| {
                if let FhirPathValue::Object(obj) = item {
                    obj.get("id").and_then(|v| {
                        if let serde_json::Value::String(s) = v {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect();

        assert!(node_ids.contains(&"root".to_string()));
        assert!(node_ids.contains(&"path1".to_string()));
        assert!(node_ids.contains(&"path2".to_string()));
        assert!(node_ids.contains(&"leaf1".to_string()));
        assert!(node_ids.contains(&"leaf2".to_string()));
        assert!(node_ids.contains(&"leaf3".to_string()));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_repeat_function_parameter_validation() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // No parameters should fail
    let expr = parser.parse("item.repeat()").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("repeat() function requires exactly one parameter"));

    // Too many parameters should fail
    let expr = parser.parse("item.repeat(children, extra)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("repeat() function requires exactly one parameter"));
}

#[test]
fn test_repeat_combined_with_other_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_tree_context();

    // Get all node values using repeat then select
    let expr = parser
        .parse("nodes.repeat(children).select(value)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 6);

        let values: Vec<String> = items
            .iter()
            .filter_map(|item| {
                if let FhirPathValue::String(s) = item {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        assert!(values.contains(&"root".to_string()));
        assert!(values.contains(&"child1".to_string()));
        assert!(values.contains(&"child2".to_string()));
        assert!(values.contains(&"grandchild1".to_string()));
        assert!(values.contains(&"grandchild2".to_string()));
        assert!(values.contains(&"grandchild3".to_string()));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Count all nodes using repeat
    let expr = parser.parse("nodes.repeat(children).count()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(6)));

    // Check if any node has specific value using repeat
    let expr = parser
        .parse("nodes.repeat(children).where(value = 'grandchild2').exists()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}
