//! Built-in function registration and evaluation for FHIRPath

use crate::error::*;
use crate::evaluator::collection::CollectionEvaluator;
use crate::evaluator::datetime::DateTimeEvaluator;
use crate::evaluator::math::MathEvaluator;
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

        // Boolean collection functions
        self.register_boolean_collection_functions();

        // Subsetting functions
        self.register_subsetting_functions();

        // String functions
        self.register_string_functions();

        // Math functions
        self.register_math_functions();

        // Date/time functions
        self.register_datetime_functions();

        // Control flow functions
        self.register_control_flow_functions();
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

    /// Register boolean collection functions
    fn register_boolean_collection_functions(&mut self) {
        // all() function
        self.functions.insert(
            "all".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::all(target)
            }),
        );

        // allTrue() function
        self.functions.insert(
            "allTrue".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::all_true(target)
            }),
        );

        // anyTrue() function
        self.functions.insert(
            "anyTrue".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::any_true(target)
            }),
        );

        // allFalse() function
        self.functions.insert(
            "allFalse".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::all_false(target)
            }),
        );

        // anyFalse() function
        self.functions.insert(
            "anyFalse".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::any_false(target)
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

        // combine() function
        self.functions.insert(
            "combine".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "combine() requires exactly one parameter (other collection)"
                            .to_string(),
                    });
                }
                CollectionEvaluator::combine(target, &params[0])
            }),
        );

        // subsetOf() function
        self.functions.insert(
            "subsetOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "subsetOf() requires exactly one parameter (other collection)"
                            .to_string(),
                    });
                }
                CollectionEvaluator::subset_of(target, &params[0])
            }),
        );

        // supersetOf() function
        self.functions.insert(
            "supersetOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "supersetOf() requires exactly one parameter (other collection)"
                            .to_string(),
                    });
                }
                CollectionEvaluator::superset_of(target, &params[0])
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

    /// Register all math functions
    fn register_math_functions(&mut self) {
        // abs() function
        self.functions.insert(
            "abs".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                MathEvaluator::abs(target)
            }),
        );

        // ceiling() function
        self.functions.insert(
            "ceiling".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                MathEvaluator::ceiling(target)
            }),
        );

        // exp() function
        self.functions.insert(
            "exp".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                MathEvaluator::exp(target)
            }),
        );

        // floor() function
        self.functions.insert(
            "floor".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                MathEvaluator::floor(target)
            }),
        );

        // ln() function
        self.functions.insert(
            "ln".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| MathEvaluator::ln(target)),
        );

        // log() function - requires base parameter
        self.functions.insert(
            "log".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "log() requires exactly one parameter (base)".to_string(),
                    });
                }
                MathEvaluator::log(target, &params[0])
            }),
        );

        // power() function - requires exponent parameter
        self.functions.insert(
            "power".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() != 1 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "power() requires exactly one parameter (exponent)".to_string(),
                    });
                }
                MathEvaluator::power(target, &params[0])
            }),
        );

        // round() function - optional precision parameter
        self.functions.insert(
            "round".to_string(),
            Box::new(
                |target: &FhirPathValue, params: &[FhirPathValue]| match params.len() {
                    0 => MathEvaluator::round(target, None),
                    1 => MathEvaluator::round(target, Some(&params[0])),
                    _ => Err(FhirPathError::InvalidOperation {
                        message: "round() accepts at most one parameter (precision)".to_string(),
                    }),
                },
            ),
        );

        // sqrt() function
        self.functions.insert(
            "sqrt".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                MathEvaluator::sqrt(target)
            }),
        );

        // truncate() function
        self.functions.insert(
            "truncate".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                MathEvaluator::truncate(target)
            }),
        );
    }

    /// Register all date/time functions  
    fn register_datetime_functions(&mut self) {
        // now() function - returns current date and time
        self.functions.insert(
            "now".to_string(),
            Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "now() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::now()
            }),
        );

        // today() function - returns current date
        self.functions.insert(
            "today".to_string(),
            Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "today() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::today()
            }),
        );

        // timeOfDay() function - returns current time
        self.functions.insert(
            "timeOfDay".to_string(),
            Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "timeOfDay() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::time_of_day()
            }),
        );

        // yearOf() function - extract year from Date or DateTime
        self.functions.insert(
            "yearOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "yearOf() function takes no parameters".to_string(),
                    });
                }
                match target {
                    FhirPathValue::Collection(items) => {
                        let mut results = Vec::new();
                        for item in items {
                            results.push(DateTimeEvaluator::year_of(item)?);
                        }
                        Ok(FhirPathValue::Collection(results))
                    }
                    single => DateTimeEvaluator::year_of(single),
                }
            }),
        );

        // monthOf() function - extract month from Date or DateTime
        self.functions.insert(
            "monthOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "monthOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::month_of(target)
            }),
        );

        // dayOf() function - extract day from Date or DateTime
        self.functions.insert(
            "dayOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "dayOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::day_of(target)
            }),
        );

        // hourOf() function - extract hour from Time or DateTime
        self.functions.insert(
            "hourOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "hourOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::hour_of(target)
            }),
        );

        // minuteOf() function - extract minute from Time or DateTime
        self.functions.insert(
            "minuteOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "minuteOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::minute_of(target)
            }),
        );

        // secondOf() function - extract second from Time or DateTime
        self.functions.insert(
            "secondOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "secondOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::second_of(target)
            }),
        );

        // millisecondOf() function - extract millisecond from Time or DateTime
        self.functions.insert(
            "millisecondOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "millisecondOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::millisecond_of(target)
            }),
        );

        // timezoneOffsetOf() function - extract timezone offset from DateTime
        self.functions.insert(
            "timezoneOffsetOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "timezoneOffsetOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::timezone_offset_of(target)
            }),
        );

        // dateOf() function - extract date from DateTime
        self.functions.insert(
            "dateOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "dateOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::date_of(target)
            }),
        );

        // timeOf() function - extract time from DateTime
        self.functions.insert(
            "timeOf".to_string(),
            Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: "timeOf() function takes no parameters".to_string(),
                    });
                }
                DateTimeEvaluator::time_of(target)
            }),
        );
    }

    /// Register control flow functions
    fn register_control_flow_functions(&mut self) {
        // iif() function - immediate if (conditional operator)
        self.functions.insert(
            "iif".to_string(),
            Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
                if params.len() < 2 || params.len() > 3 {
                    return Err(FhirPathError::InvalidOperation {
                        message: "iif() requires 2 or 3 parameters: criterion, true-result [, otherwise-result]".to_string(),
                    });
                }

                let criterion = &params[0];
                let true_result = &params[1];
                let otherwise_result = params.get(2);

                CollectionEvaluator::iif(criterion, true_result, otherwise_result)
            }),
        );
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
