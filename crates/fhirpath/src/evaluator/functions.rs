//! Built-in function registration and evaluation for FHIRPath

use crate::error::*;
use crate::evaluator::collection::CollectionEvaluator;
use crate::evaluator::strings::StringEvaluator;
use crate::evaluator::values::FhirPathValue;
use std::collections::HashMap;

/// Type alias for FHIRPath function implementation
type FhirPathFunction =
    Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue>>;

/// Built-in function registry
pub struct FunctionRegistry {
    functions: HashMap<String, FhirPathFunction>,
}

impl FunctionRegistry {
    /// Create a new function registry with built-in functions
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        registry.register_builtin_functions();
        registry
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&FhirPathFunction> {
        self.functions.get(name)
    }

    /// Register all built-in functions
    fn register_builtin_functions(&mut self) {
        // Collection functions
        self.register_empty_function();
        self.register_exists_function();
        self.register_count_function();
        self.register_distinct_function();
        self.register_is_distinct_function();

        // Subsetting functions
        self.register_subsetting_functions();

        // String functions
        self.register_string_functions();
    }

    /// Register the empty() function
    fn register_empty_function(&mut self) {
        self.functions.insert(
            "empty".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::is_empty(target)
            }),
        );
    }

    /// Register the exists() function
    fn register_exists_function(&mut self) {
        self.functions.insert(
            "exists".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::exists(target)
            }),
        );
    }

    /// Register the count() function
    fn register_count_function(&mut self) {
        self.functions.insert(
            "count".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::count(target)
            }),
        );
    }

    /// Register the distinct() function
    fn register_distinct_function(&mut self) {
        self.functions.insert(
            "distinct".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::distinct(target)
            }),
        );
    }

    /// Register the isDistinct() function
    fn register_is_distinct_function(&mut self) {
        self.functions.insert(
            "isDistinct".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::is_distinct(target)
            }),
        );
    }

    /// Register all subsetting functions
    fn register_subsetting_functions(&mut self) {
        // single() function
        self.functions.insert(
            "single".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::single(target)
            }),
        );

        // first() function
        self.functions.insert(
            "first".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::first(target)
            }),
        );

        // last() function
        self.functions.insert(
            "last".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::last(target)
            }),
        );

        // tail() function
        self.functions.insert(
            "tail".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::tail(target)
            }),
        );

        // skip() function
        self.functions.insert(
            "skip".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "skip() requires exactly one parameter (count)".to_string(),
                    });
                }
                match &params[0] {
                    FhirPathValue::Integer(count) => CollectionEvaluator::skip(target, *count),
                    _ => Err(FhirPathError::InvalidOperation {
                        message: "skip() count parameter must be an integer".to_string(),
                    }),
                }
            }),
        );

        // take() function
        self.functions.insert(
            "take".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "take() requires exactly one parameter (count)".to_string(),
                    });
                }
                match &params[0] {
                    FhirPathValue::Integer(count) => CollectionEvaluator::take(target, *count),
                    _ => Err(FhirPathError::InvalidOperation {
                        message: "take() count parameter must be an integer".to_string(),
                    }),
                }
            }),
        );

        // intersect() function
        self.functions.insert(
            "intersect".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "intersect() requires exactly one parameter (other collection)"
                            .to_string(),
                    });
                }
                CollectionEvaluator::intersect(target, &params[0])
            }),
        );

        // exclude() function
        self.functions.insert(
            "exclude".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "exclude() requires exactly one parameter (other collection)"
                            .to_string(),
                    });
                }
                CollectionEvaluator::exclude(target, &params[0])
            }),
        );
    }

    /// Register all string manipulation functions
    fn register_string_functions(&mut self) {
        // length() function
        self.functions.insert(
            "length".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                StringEvaluator::length(target)
            }),
        );

        // upper() function
        self.functions.insert(
            "upper".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                StringEvaluator::upper(target)
            }),
        );

        // lower() function
        self.functions.insert(
            "lower".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                StringEvaluator::lower(target)
            }),
        );

        // trim() function
        self.functions.insert(
            "trim".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                StringEvaluator::trim(target)
            }),
        );

        // substring() function
        self.functions.insert(
            "substring".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "substring() requires at least one parameter (start index)"
                            .to_string(),
                    });
                }
                let start = &params[0];
                let length = params.get(1);
                StringEvaluator::substring(target, start, length)
            }),
        );

        // startsWith() function
        self.functions.insert(
            "startsWith".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "startsWith() requires exactly one parameter".to_string(),
                    });
                }
                StringEvaluator::starts_with(target, &params[0])
            }),
        );

        // endsWith() function
        self.functions.insert(
            "endsWith".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "endsWith() requires exactly one parameter".to_string(),
                    });
                }
                StringEvaluator::ends_with(target, &params[0])
            }),
        );

        // indexOf() function
        self.functions.insert(
            "indexOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "indexOf() requires exactly one parameter".to_string(),
                    });
                }
                StringEvaluator::index_of(target, &params[0])
            }),
        );

        // replace() function
        self.functions.insert(
            "replace".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 2 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "replace() requires exactly two parameters (pattern, replacement)"
                            .to_string(),
                    });
                }
                StringEvaluator::replace(target, &params[0], &params[1])
            }),
        );

        // split() function
        self.functions.insert(
            "split".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "split() requires exactly one parameter (delimiter)".to_string(),
                    });
                }
                StringEvaluator::split(target, &params[0])
            }),
        );

        // join() function
        self.functions.insert(
            "join".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "join() requires exactly one parameter (delimiter)".to_string(),
                    });
                }
                StringEvaluator::join(target, &params[0])
            }),
        );

        // matches() function
        self.functions.insert(
            "matches".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "matches() requires exactly one parameter (pattern)".to_string(),
                    });
                }
                StringEvaluator::matches(target, &params[0])
            }),
        );

        // contains() function
        self.functions.insert(
            "contains".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "contains() requires exactly one parameter (substring)"
                            .to_string(),
                    });
                }
                StringEvaluator::contains(target, &params[0])
            }),
        );

        // Note: The contains() function above provides string containment checking.
        // This is distinct from FHIRPath collection contains operations.
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
