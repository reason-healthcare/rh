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

        // Conversion functions
        self.register_conversion_functions();
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

    /// Register conversion functions
    fn register_conversion_functions(&mut self) {
        // toBoolean() function
        self.functions.insert(
            "toBoolean".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| Self::to_boolean(target)),
        );

        // convertsToBoolean() function
        self.functions.insert(
            "convertsToBoolean".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                Self::converts_to_boolean(target)
            }),
        );

        // toInteger() function
        self.functions.insert(
            "toInteger".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| Self::to_integer(target)),
        );

        // convertsToInteger() function
        self.functions.insert(
            "convertsToInteger".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                Self::converts_to_integer(target)
            }),
        );

        // toLong() function
        self.functions.insert(
            "toLong".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| Self::to_long(target)),
        );

        // convertsToLong() function
        self.functions.insert(
            "convertsToLong".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                Self::converts_to_long(target)
            }),
        );

        // toDate() function
        self.functions.insert(
            "toDate".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| Self::to_date(target)),
        );

        // convertsToDate() function
        self.functions.insert(
            "convertsToDate".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                Self::converts_to_date(target)
            }),
        );

        // toDateTime() function
        self.functions.insert(
            "toDateTime".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| Self::to_datetime(target)),
        );

        // convertsToDateTime() function
        self.functions.insert(
            "convertsToDateTime".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                Self::converts_to_datetime(target)
            }),
        );

        // toString() function
        self.functions.insert(
            "toString".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| Self::to_string(target)),
        );

        // convertsToString() function
        self.functions.insert(
            "convertsToString".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                Self::converts_to_string(target)
            }),
        );
    }

    /// Convert a value to Boolean according to FHIRPath specification
    fn to_boolean(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return empty
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),

            // If the collection contains more than one item, return empty
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::to_boolean(&items[0])
                } else {
                    // Multiple items - return empty per spec
                    Ok(FhirPathValue::Empty)
                }
            }

            // Single item conversions
            FhirPathValue::Boolean(b) => Ok(FhirPathValue::Boolean(*b)),

            FhirPathValue::Integer(i) => match *i {
                1 => Ok(FhirPathValue::Boolean(true)),
                0 => Ok(FhirPathValue::Boolean(false)),
                _ => Ok(FhirPathValue::Empty), // Not convertible
            },

            FhirPathValue::Number(n) => {
                if (*n - 1.0).abs() < f64::EPSILON {
                    Ok(FhirPathValue::Boolean(true))
                } else if (*n - 0.0).abs() < f64::EPSILON {
                    Ok(FhirPathValue::Boolean(false))
                } else {
                    Ok(FhirPathValue::Empty) // Not convertible
                }
            }

            FhirPathValue::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "t" | "yes" | "y" | "1" | "1.0" => Ok(FhirPathValue::Boolean(true)),
                    "false" | "f" | "no" | "n" | "0" | "0.0" => Ok(FhirPathValue::Boolean(false)),
                    _ => Ok(FhirPathValue::Empty), // Not convertible
                }
            }

            // All other types cannot be converted to Boolean
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// Check if a value can be converted to Boolean according to FHIRPath specification
    fn converts_to_boolean(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return false
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

            // If the collection contains more than one item, return false
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::converts_to_boolean(&items[0])
                } else {
                    // Multiple items - cannot convert
                    Ok(FhirPathValue::Boolean(false))
                }
            }

            // Single item checks
            FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::Integer(1 | 0) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(false)), // Not convertible

            FhirPathValue::Number(n) => {
                if (*n - 1.0).abs() < f64::EPSILON || (*n - 0.0).abs() < f64::EPSILON {
                    Ok(FhirPathValue::Boolean(true))
                } else {
                    Ok(FhirPathValue::Boolean(false)) // Not convertible
                }
            }

            FhirPathValue::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "t" | "yes" | "y" | "1" | "1.0" | "false" | "f" | "no" | "n" | "0"
                    | "0.0" => Ok(FhirPathValue::Boolean(true)),
                    _ => Ok(FhirPathValue::Boolean(false)), // Not convertible
                }
            }

            // All other types cannot be converted to Boolean
            _ => Ok(FhirPathValue::Boolean(false)),
        }
    }

    /// Convert a value to Integer according to FHIRPath specification
    fn to_integer(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return empty
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),

            // If the collection contains more than one item, return empty
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::to_integer(&items[0])
                } else {
                    // Multiple items - return empty per spec
                    Ok(FhirPathValue::Empty)
                }
            }

            // Single item conversions
            FhirPathValue::Integer(i) => Ok(FhirPathValue::Integer(*i)),

            FhirPathValue::Boolean(b) => {
                if *b {
                    Ok(FhirPathValue::Integer(1))
                } else {
                    Ok(FhirPathValue::Integer(0))
                }
            }

            FhirPathValue::Number(n) => {
                // Check if it's a whole number
                if n.fract() == 0.0 && n.is_finite() {
                    // Check if within integer range
                    if *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                        Ok(FhirPathValue::Integer(*n as i64))
                    } else {
                        Ok(FhirPathValue::Empty) // Out of range
                    }
                } else {
                    Ok(FhirPathValue::Empty) // Not a whole number
                }
            }

            FhirPathValue::String(s) => {
                match s.trim().parse::<i64>() {
                    Ok(i) => Ok(FhirPathValue::Integer(i)),
                    Err(_) => Ok(FhirPathValue::Empty), // Not convertible
                }
            }

            // All other types cannot be converted to Integer
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// Check if a value can be converted to Integer according to FHIRPath specification
    fn converts_to_integer(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return false
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

            // If the collection contains more than one item, return false
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::converts_to_integer(&items[0])
                } else {
                    // Multiple items - cannot convert
                    Ok(FhirPathValue::Boolean(false))
                }
            }

            // Single item checks
            FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::Number(n) => {
                // Check if it's a whole number within integer range
                if n.fract() == 0.0
                    && n.is_finite()
                    && *n >= i64::MIN as f64
                    && *n <= i64::MAX as f64
                {
                    Ok(FhirPathValue::Boolean(true))
                } else {
                    Ok(FhirPathValue::Boolean(false)) // Not convertible
                }
            }

            FhirPathValue::String(s) => {
                match s.trim().parse::<i64>() {
                    Ok(_) => Ok(FhirPathValue::Boolean(true)),
                    Err(_) => Ok(FhirPathValue::Boolean(false)), // Not convertible
                }
            }

            // All other types cannot be converted to Integer
            _ => Ok(FhirPathValue::Boolean(false)),
        }
    }

    /// Convert a value to Long according to FHIRPath specification
    fn to_long(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return empty
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),

            // If the collection contains more than one item, return empty
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::to_long(&items[0])
                } else {
                    // Multiple items - return empty per spec
                    Ok(FhirPathValue::Empty)
                }
            }

            // Single item conversions
            FhirPathValue::Long(l) => Ok(FhirPathValue::Long(*l)),

            FhirPathValue::Integer(i) => Ok(FhirPathValue::Long(*i)),

            FhirPathValue::Boolean(b) => {
                if *b {
                    Ok(FhirPathValue::Long(1))
                } else {
                    Ok(FhirPathValue::Long(0))
                }
            }

            FhirPathValue::Number(n) => {
                // Check if it's a whole number
                if n.fract() == 0.0 && n.is_finite() {
                    // Check if within i64 range
                    if *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                        Ok(FhirPathValue::Long(*n as i64))
                    } else {
                        Ok(FhirPathValue::Empty) // Out of range
                    }
                } else {
                    Ok(FhirPathValue::Empty) // Not a whole number
                }
            }

            FhirPathValue::String(s) => {
                match s.trim().parse::<i64>() {
                    Ok(l) => Ok(FhirPathValue::Long(l)),
                    Err(_) => Ok(FhirPathValue::Empty), // Not convertible
                }
            }

            // All other types cannot be converted to Long
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// Check if a value can be converted to Long according to FHIRPath specification
    fn converts_to_long(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return false
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

            // If the collection contains more than one item, return false
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::converts_to_long(&items[0])
                } else {
                    // Multiple items - cannot convert
                    Ok(FhirPathValue::Boolean(false))
                }
            }

            // Single item checks
            FhirPathValue::Long(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::Number(n) => {
                // Check if it's a whole number within i64 range
                if n.fract() == 0.0
                    && n.is_finite()
                    && *n >= i64::MIN as f64
                    && *n <= i64::MAX as f64
                {
                    Ok(FhirPathValue::Boolean(true))
                } else {
                    Ok(FhirPathValue::Boolean(false)) // Not convertible
                }
            }

            FhirPathValue::String(s) => {
                match s.trim().parse::<i64>() {
                    Ok(_) => Ok(FhirPathValue::Boolean(true)),
                    Err(_) => Ok(FhirPathValue::Boolean(false)), // Not convertible
                }
            }

            // All other types cannot be converted to Long
            _ => Ok(FhirPathValue::Boolean(false)),
        }
    }

    /// Convert a value to Date according to FHIRPath specification
    fn to_date(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return empty
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),

            // If the collection contains more than one item, return empty
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::to_date(&items[0])
                } else {
                    // Multiple items - return empty per spec
                    Ok(FhirPathValue::Empty)
                }
            }

            // Single item conversions
            FhirPathValue::Date(d) => Ok(FhirPathValue::Date(d.clone())),

            FhirPathValue::DateTime(dt) => {
                // Extract date part from datetime
                if let Some(date_part) = dt.split('T').next() {
                    Ok(FhirPathValue::Date(date_part.to_string()))
                } else {
                    Ok(FhirPathValue::Empty) // Invalid datetime format
                }
            }

            FhirPathValue::String(s) => {
                // Try to parse as date (YYYY-MM-DD format)
                if Self::is_valid_date_string(s) {
                    Ok(FhirPathValue::Date(s.clone()))
                } else {
                    // Try to extract date from datetime string
                    if let Some(date_part) = s.split('T').next() {
                        if Self::is_valid_date_string(date_part) {
                            Ok(FhirPathValue::Date(date_part.to_string()))
                        } else {
                            Ok(FhirPathValue::Empty) // Not convertible
                        }
                    } else {
                        Ok(FhirPathValue::Empty) // Not convertible
                    }
                }
            }

            // All other types cannot be converted to Date
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// Check if a value can be converted to Date according to FHIRPath specification
    fn converts_to_date(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return false
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

            // If the collection contains more than one item, return false
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::converts_to_date(&items[0])
                } else {
                    // Multiple items - cannot convert
                    Ok(FhirPathValue::Boolean(false))
                }
            }

            // Single item checks
            FhirPathValue::Date(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::DateTime(dt) => {
                // Check if we can extract a valid date part
                if let Some(date_part) = dt.split('T').next() {
                    Ok(FhirPathValue::Boolean(Self::is_valid_date_string(
                        date_part,
                    )))
                } else {
                    Ok(FhirPathValue::Boolean(false)) // Invalid datetime format
                }
            }

            FhirPathValue::String(s) => {
                // Check if it's a valid date or can extract date from datetime
                if Self::is_valid_date_string(s) {
                    Ok(FhirPathValue::Boolean(true))
                } else if let Some(date_part) = s.split('T').next() {
                    Ok(FhirPathValue::Boolean(Self::is_valid_date_string(
                        date_part,
                    )))
                } else {
                    Ok(FhirPathValue::Boolean(false)) // Not convertible
                }
            }

            // All other types cannot be converted to Date
            _ => Ok(FhirPathValue::Boolean(false)),
        }
    }

    /// Convert a value to DateTime according to FHIRPath specification
    fn to_datetime(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return empty
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),

            // If the collection contains more than one item, return empty
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::to_datetime(&items[0])
                } else {
                    // Multiple items - return empty per spec
                    Ok(FhirPathValue::Empty)
                }
            }

            // Single item conversions
            FhirPathValue::DateTime(dt) => Ok(FhirPathValue::DateTime(dt.clone())),

            FhirPathValue::Date(d) => {
                // Convert date to datetime by adding midnight time
                Ok(FhirPathValue::DateTime(format!("{d}T00:00:00")))
            }

            FhirPathValue::String(s) => {
                // Try to parse as datetime
                if Self::is_valid_datetime_string(s) {
                    Ok(FhirPathValue::DateTime(s.clone()))
                } else if Self::is_valid_date_string(s) {
                    // Convert date string to datetime
                    Ok(FhirPathValue::DateTime(format!("{s}T00:00:00")))
                } else {
                    Ok(FhirPathValue::Empty) // Not convertible
                }
            }

            // All other types cannot be converted to DateTime
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// Check if a value can be converted to DateTime according to FHIRPath specification
    fn converts_to_datetime(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return false
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

            // If the collection contains more than one item, return false
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::converts_to_datetime(&items[0])
                } else {
                    // Multiple items - cannot convert
                    Ok(FhirPathValue::Boolean(false))
                }
            }

            // Single item checks
            FhirPathValue::DateTime(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::Date(_) => Ok(FhirPathValue::Boolean(true)),

            FhirPathValue::String(s) => {
                // Check if it's a valid datetime or date that can be converted
                Ok(FhirPathValue::Boolean(
                    Self::is_valid_datetime_string(s) || Self::is_valid_date_string(s),
                ))
            }

            // All other types cannot be converted to DateTime
            _ => Ok(FhirPathValue::Boolean(false)),
        }
    }

    /// Helper function to validate date string format (YYYY-MM-DD)
    fn is_valid_date_string(s: &str) -> bool {
        // Basic validation for YYYY-MM-DD format
        if s.len() < 8 || s.len() > 10 {
            return false;
        }

        // Check for basic date pattern
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        // Validate year (4 digits)
        if parts[0].len() != 4 || !parts[0].chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        // Validate month (1-12)
        if parts[1].is_empty()
            || parts[1].len() > 2
            || !parts[1].chars().all(|c| c.is_ascii_digit())
        {
            return false;
        }
        if let Ok(month) = parts[1].parse::<u32>() {
            if !(1..=12).contains(&month) {
                return false;
            }
        } else {
            return false;
        }

        // Validate day (1-31, basic check)
        if parts[2].is_empty()
            || parts[2].len() > 2
            || !parts[2].chars().all(|c| c.is_ascii_digit())
        {
            return false;
        }
        if let Ok(day) = parts[2].parse::<u32>() {
            if !(1..=31).contains(&day) {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    /// Helper function to validate datetime string format
    fn is_valid_datetime_string(s: &str) -> bool {
        // Basic validation for ISO 8601 datetime format
        if !s.contains('T') {
            return false;
        }

        let parts: Vec<&str> = s.split('T').collect();
        if parts.len() != 2 {
            return false;
        }

        // Validate date part
        if !Self::is_valid_date_string(parts[0]) {
            return false;
        }

        // Basic time validation (HH:MM:SS or HH:MM:SS.sss with optional timezone)
        let time_part = parts[1];
        if time_part.is_empty() {
            return false;
        }

        // Remove timezone info for basic validation
        let time_only = if let Some(tz_pos) = time_part.find(&['+', '-', 'Z'][..]) {
            &time_part[..tz_pos]
        } else {
            time_part
        };

        // Split by colon to get time components
        let time_components: Vec<&str> = time_only.split(':').collect();
        if time_components.len() < 2 || time_components.len() > 3 {
            return false;
        }

        // Validate hour (00-23)
        if time_components[0].len() != 2 || !time_components[0].chars().all(|c| c.is_ascii_digit())
        {
            return false;
        }
        if let Ok(hour) = time_components[0].parse::<u32>() {
            if hour > 23 {
                return false;
            }
        } else {
            return false;
        }

        // Validate minute (00-59)
        if time_components[1].len() != 2 || !time_components[1].chars().all(|c| c.is_ascii_digit())
        {
            return false;
        }
        if let Ok(minute) = time_components[1].parse::<u32>() {
            if minute > 59 {
                return false;
            }
        } else {
            return false;
        }

        // Validate seconds if present (00-59, may include decimal)
        if time_components.len() == 3 {
            let seconds_part = time_components[2];
            if seconds_part.is_empty() {
                return false;
            }

            // Handle decimal seconds
            let seconds_str = if let Some(dot_pos) = seconds_part.find('.') {
                &seconds_part[..dot_pos]
            } else {
                seconds_part
            };

            if seconds_str.len() != 2 || !seconds_str.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
            if let Ok(seconds) = seconds_str.parse::<u32>() {
                if seconds > 59 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Convert a value to String according to FHIRPath specification
    fn to_string(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return empty
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),

            // If the collection contains more than one item, return empty
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::to_string(&items[0])
                } else {
                    // Multiple items - return empty per spec
                    Ok(FhirPathValue::Empty)
                }
            }

            // Single item conversions
            FhirPathValue::String(s) => Ok(FhirPathValue::String(s.clone())),

            FhirPathValue::Boolean(b) => Ok(FhirPathValue::String(if *b {
                "true".to_string()
            } else {
                "false".to_string()
            })),

            FhirPathValue::Integer(i) => Ok(FhirPathValue::String(i.to_string())),

            FhirPathValue::Long(l) => Ok(FhirPathValue::String(l.to_string())),

            FhirPathValue::Number(n) => {
                // Format number without unnecessary trailing zeros
                let formatted = if n.fract() == 0.0 {
                    format!("{n:.0}")
                } else {
                    // Remove trailing zeros but keep at least one decimal place if needed
                    let mut formatted = format!("{n}");
                    if formatted.contains('.') {
                        formatted = formatted.trim_end_matches('0').to_string();
                        if formatted.ends_with('.') {
                            formatted.push('0');
                        }
                    }
                    formatted
                };
                Ok(FhirPathValue::String(formatted))
            }

            FhirPathValue::Date(d) => Ok(FhirPathValue::String(d.clone())),

            FhirPathValue::DateTime(dt) => Ok(FhirPathValue::String(dt.clone())),

            FhirPathValue::Time(t) => {
                // Remove the leading 'T' from time format for string conversion
                let time_str = if let Some(stripped) = t.strip_prefix('T') {
                    stripped.to_string()
                } else {
                    t.clone()
                };
                Ok(FhirPathValue::String(time_str))
            }

            FhirPathValue::Quantity { value, unit } => match unit {
                Some(u) => Ok(FhirPathValue::String(format!("{value} '{u}'"))),
                None => Ok(FhirPathValue::String(value.to_string())),
            },

            // All other types cannot be converted to String (return empty)
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// Check if a value can be converted to String according to FHIRPath specification
    fn converts_to_string(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            // If the collection is empty, return false
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

            // If the collection contains more than one item, return false
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::converts_to_string(&items[0])
                } else {
                    // Multiple items - cannot convert
                    Ok(FhirPathValue::Boolean(false))
                }
            }

            // Single item checks - most types can be converted to string
            FhirPathValue::String(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Long(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Number(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Date(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::DateTime(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Time(_) => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Quantity { .. } => Ok(FhirPathValue::Boolean(true)),

            // Complex types (Object, DateTimePrecision) cannot be converted to string
            _ => Ok(FhirPathValue::Boolean(false)),
        }
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
