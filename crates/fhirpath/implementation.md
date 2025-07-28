# FHIRPath Implementation Status

This document provides a comprehensive overview of all supported operations and functions in the FHIRPath Rust implementation.

## Operators

### Arithmetic Operators
| Status | Operator | Symbol | Description |
|--------|----------|--------|-------------|
| ✅ | Addition | `+` | Add numbers, concatenate strings, add durations to dates |
| ✅ | Subtraction | `-` | Subtract numbers, subtract durations from dates |
| ✅ | Multiplication | `*` | Multiply numbers and quantities |
| ✅ | Division | `/` | Divide numbers and quantities |
| ✅ | Integer Division | `div` | Integer division (truncated result) |
| ✅ | Modulo | `mod` | Remainder after division |
| ✅ | Unary Plus | `+value` | Positive number (identity operation) |
| ✅ | Unary Minus | `-value` | Negative number (negation) |
| ✅ | String Concatenation | `&` | Concatenate strings |

### Comparison Operators
| Status | Operator | Symbol | Description |
|--------|----------|--------|-------------|
| ✅ | Equal | `=` | Equality comparison |
| ✅ | Not Equal | `!=` | Inequality comparison |
| ✅ | Equivalent | `~` | Equivalence (null-safe equality) |
| ✅ | Not Equivalent | `!~` | Non-equivalence (null-safe inequality) |
| ✅ | Less Than | `<` | Less than comparison |
| ✅ | Less Than or Equal | `<=` | Less than or equal comparison |
| ✅ | Greater Than | `>` | Greater than comparison |
| ✅ | Greater Than or Equal | `>=` | Greater than or equal comparison |

### Logical Operators
| Status | Operator | Symbol | Description |
|--------|----------|--------|-------------|
| ✅ | AND | `and` | Logical AND operation |
| ✅ | OR | `or` | Logical OR operation |
| ✅ | XOR | `xor` | Logical exclusive OR |
| ✅ | Implies | `implies` | Logical implication (if-then logic) |

### Type Operators
| Status | Operator | Symbol | Description |
|--------|----------|--------|-------------|
| ✅ | Type Test | `is` | Test if value is of specified type |
| ✅ | Type Cast | `as` | Cast value to specified type |

### Membership Operators
| Status | Operator | Symbol | Description |
|--------|----------|--------|-------------|
| ✅ | In | `in` | Test if value is in collection |
| ✅ | Contains | `contains` | Test if collection contains value |

### Collection Operators
| Status | Operator | Symbol | Description |
|--------|----------|--------|-------------|
| ✅ | Union | `\|` | Combine collections (union) |
| ✅ | Indexing | `[n]` | Access collection element by index |

### Navigation Operators
| Status | Operator | Symbol | Description |
|--------|----------|--------|-------------|
| ✅ | Member Access | `.` | Access object property |
| ✅ | Function Call | `.function()` | Call function on target |

## Functions

### Collection Functions

#### Basic Collection Operations
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `empty()` | `empty() : Boolean` | Test if collection is empty |
| ✅ | `exists()` | `exists() : Boolean` | Test if collection has any items |
| ✅ | `count()` | `count() : Integer` | Count items in collection |
| ✅ | `distinct()` | `distinct() : collection` | Remove duplicate items |
| ✅ | `isDistinct()` | `isDistinct() : Boolean` | Test if all items are unique |

#### Tree Navigation
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `children()` | `children() : collection` | Get immediate child nodes |
| ✅ | `descendants()` | `descendants() : collection` | Get all descendant nodes recursively |

#### Boolean Collection Operations
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `not()` | `not() : Boolean` | Returns true if the input collection evaluates to false, and false if it evaluates to true |
| ✅ | `all()` | `all() : Boolean` | Test if all items are truthy |
| ✅ | `allTrue()` | `allTrue() : Boolean` | Test if all items are boolean true |
| ✅ | `anyTrue()` | `anyTrue() : Boolean` | Test if any item is boolean true |
| ✅ | `allFalse()` | `allFalse() : Boolean` | Test if all items are boolean false |
| ✅ | `anyFalse()` | `anyFalse() : Boolean` | Test if any item is boolean false |

#### Subsetting Functions
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `single()` | `single() : T` | Return single item (error if count != 1) |
| ✅ | `first()` | `first() : T` | Return first item |
| ✅ | `last()` | `last() : T` | Return last item |
| ✅ | `tail()` | `tail() : collection` | Return all items except first |
| ✅ | `skip(n)` | `skip(n: Integer) : collection` | Skip first n items |
| ✅ | `take(n)` | `take(n: Integer) : collection` | Take first n items |

#### Set Operations
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `intersect(other)` | `intersect(other: collection) : collection` | Items in both collections |
| ✅ | `exclude(other)` | `exclude(other: collection) : collection` | Items not in other collection |
| ✅ | `combine(other)` | `combine(other: collection) : collection` | Merge collections preserving duplicates |
| ❌ | `union(other)` | `union(other: collection) : collection` | Merge collections remove duplicates |
| ✅ | `subsetOf(other)` | `subsetOf(other: collection) : Boolean` | Test if subset of other |
| ✅ | `supersetOf(other)` | `supersetOf(other: collection) : Boolean` | Test if superset of other |

#### Filtering and Projection
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `where(criteria)` | `where(criteria: expression) : collection` | Filter collection by criteria |
| ✅ | `select(projection)` | `select(projection: expression) : collection` | Transform each item |
| ✅ | `repeat(projection)` | `repeat(projection: expression) : collection` | Recursively apply projection |
| ✅ | `ofType(type)` | `ofType(type: TypeSpecifier) : collection` | Filter by type |

#### Control Flow
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `iif(condition, true_result, otherwise_result?)` | `iif(condition: Boolean, true_result: T, otherwise_result?: T) : T` | Conditional expression (ternary operator) |

### String Functions

#### Basic String Operations
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `length()` | `length() : Integer` | Get string length |
| ✅ | `upper()` | `upper() : String` | Convert to uppercase |
| ✅ | `lower()` | `lower() : String` | Convert to lowercase |
| ✅ | `trim()` | `trim() : String` | Remove leading/trailing whitespace |

#### String Analysis
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `startsWith(prefix)` | `startsWith(prefix: String) : Boolean` | Test if string starts with prefix |
| ✅ | `endsWith(suffix)` | `endsWith(suffix: String) : Boolean` | Test if string ends with suffix |
| ✅ | `contains(substring)` | `contains(substring: String) : Boolean` | Test if string contains substring |
| ✅ | `indexOf(substring)` | `indexOf(substring: String) : Integer` | Find index of substring |
| ❌ | `lastIndexOf(substring)` | `lastIndexOf(substring: String) : Integer` | Find 0-based index of substring |

#### String Manipulation
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `substring(start, length?)` | `substring(start: Integer, length?: Integer) : String` | Extract substring |
| ✅ | `replace(pattern, replacement)` | `replace(pattern: String, replacement: String) : String` | Replace occurrences |
| ✅ | `split(delimiter)` | `split(delimiter: String) : collection<String>` | Split into collection |
| ✅ | `join(delimiter)` | `join(delimiter: String) : String` | Join collection with delimiter |
| ✅ | `toChars()` | `toChars() : collection<String>` | Convert to character collection |

#### Regular Expressions
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `matches(regex)` | `matches(regex: String) : Boolean` | Test if string matches regex |
| ✅ | `matchesFull(regex)` | `matchesFull(regex: String) : Boolean` | Test if entire string matches regex |
| ✅ | `replaceMatches(regex, substitution)` | `replaceMatches(regex: String, substitution: String) : String` | Replace regex matches with substitution |

#### Additional String Functions 
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ❌ | `encode(format)` | `encode(format : String) : String` | Encode using format: hex, nase64, urlbase64 |
| ❌ | `decode(format)` | `decode(format : String) : String` | Decode using format: hex, nase64, urlbase64 |
| ❌ | `escape(target)` | `escape(target : String) : String` | Escape using target: html or json |
| ❌ | `unescape(target)` | `unescape(target : String) : String` | Un-escape using target: html or json |

### Math Functions

#### Basic Math Operations
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `abs()` | `abs() : Number` | Absolute value |
| ✅ | `ceiling()` | `ceiling() : Integer` | Round up to nearest integer |
| ✅ | `floor()` | `floor() : Integer` | Round down to nearest integer |
| ✅ | `round(precision?)` | `round(precision?: Integer) : Number` | Round to specified precision |
| ✅ | `truncate()` | `truncate() : Integer` | Truncate to integer |
| ✅ | `sqrt()` | `sqrt() : Number` | Square root |

#### Advanced Math Operations
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `power(exponent)` | `power(exponent: Number) : Number` | Raise to power |
| ✅ | `exp()` | `exp() : Number` | Natural exponential (e^x) |
| ✅ | `ln()` | `ln() : Number` | Natural logarithm |
| ✅ | `log(base)` | `log(base: Number) : Number` | Logarithm with specified base |

### DateTime Functions

#### Current Date/Time
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `now()` | `now() : DateTime` | Current date and time (UTC) |
| ✅ | `today()` | `today() : Date` | Current date (local) |
| ✅ | `timeOfDay()` | `timeOfDay() : Time` | Current time (local) |

#### Date/Time Component Extraction
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `yearOf()` | `yearOf() : Integer` | Extract year from date/datetime |
| ✅ | `monthOf()` | `monthOf() : Integer` | Extract month from date/datetime |
| ✅ | `dayOf()` | `dayOf() : Integer` | Extract day from date/datetime |
| ✅ | `hourOf()` | `hourOf() : Integer` | Extract hour from time/datetime |
| ✅ | `minuteOf()` | `minuteOf() : Integer` | Extract minute from time/datetime |
| ✅ | `secondOf()` | `secondOf() : Integer` | Extract second from time/datetime |
| ✅ | `millisecondOf()` | `millisecondOf() : Integer` | Extract millisecond from time/datetime |
| ✅ | `timezoneOffsetOf()` | `timezoneOffsetOf() : Decimal` | Extract timezone offset from datetime |
| ✅ | `dateOf()` | `dateOf() : Date` | Extract date part from datetime |
| ✅ | `timeOf()` | `timeOf() : Time` | Extract time part from datetime |

### Conversion Functions

#### Boolean Conversions
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `toBoolean()` | `toBoolean() : Boolean` | Convert to boolean |
| ✅ | `convertsToBoolean()` | `convertsToBoolean() : Boolean` | Test if convertible to boolean |

#### Numeric Conversions
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `toInteger()` | `toInteger() : Integer` | Convert to integer |
| ✅ | `convertsToInteger()` | `convertsToInteger() : Boolean` | Test if convertible to integer |
| ✅ | `toLong()` | `toLong() : Long` | Convert to long integer (64-bit) |
| ✅ | `convertsToLong()` | `convertsToLong() : Boolean` | Test if convertible to long |

#### String Conversions
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `toString()` | `toString() : String` | Convert to string |
| ✅ | `convertsToString()` | `convertsToString() : Boolean` | Test if convertible to string |

#### Date/Time Conversions
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `toDate()` | `toDate() : Date` | Convert to date |
| ✅ | `convertsToDate()` | `convertsToDate() : Boolean` | Test if convertible to date |
| ✅ | `toDateTime()` | `toDateTime() : DateTime` | Convert to datetime |
| ✅ | `convertsToDateTime()` | `convertsToDateTime() : Boolean` | Test if convertible to datetime |
| ✅ | `toTime()` | `toTime() : Time` | Convert to time |
| ✅ | `convertsToTime()` | `convertsToTime() : Boolean` | Test if convertible to time |

#### Quantity Conversions
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ✅ | `toQuantity(unit?)` | `toQuantity(unit?: String) : Quantity` | Convert to quantity with optional unit |
| ✅ | `convertsToQuantity(unit?)` | `convertsToQuantity(unit?: String) : Boolean` | Test if convertible to quantity |

### Aggregates
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ❌ | `aggregate(aggregator, init?)` | aggregate(aggregator : expression [, init : value]) : value | General-purpose aggregation |

### Reflection `type()`
| Status | Kind     | Result    | Description |
|--------|----------|-----------|-------------|
| ❌ | Primitive Type | `SimpleTypeInfo { namespace: string, name: string, baseType: TypeSpecifier }` | For primitive types such as String and Integer (e.g. `('John' \| 'Mary').type()`) |
| ❌ | Class Type | `ClassInfo { namespace: string, name: string, baseType: TypeSpecifier, element: List<ClassInfoElement> }` | For class types (e.g. `Patient.maritalStatus.type()`) |
| ❌ | Collection Type | `ListTypeInfo { elementType: TypeSpecifier }` | For collection types (e.g. `Patient.address.type()`) |
| ❌ | Anonymous Type | `TupleTypeInfo { element: List<TupleTypeInfoElement> }` | For types that have no associated name (e.g. `Patient.contact.single().type()`) |


### Utility
| Status | Function | Signature | Description |
|--------|----------|-----------|-------------|
| ❌ | `trace(name, projection?)` | trace(name : String [, projection: Expression]) : collection | Adds a String representation of the input collection to the diagnostic log, using the name argument as the name in the log. |
| ❌ | `defineVariable(name, expression)` | defineVariable(name: String [, expr: expression]) | Defines a variable named name that is accessible in subsequent expressions. |
| ❌ | `lowBoundary(precision)` | lowBoundary([precision: Integer]): Decimal \| Date \| DateTime \| Time | The least possible value of the input to the specified precision. |
| ❌ | `highBoundary(precision)` | highBoundary([precision: Integer]): Decimal \| Date \| DateTime \| Time | The greatest possible value of the input to the specified precision. |
| ❌ | `precision()` | precision() : Integer | If the input collection contains a single item, this function will return the number of digits of precision. |

## Literals

### Primitive Literals
| Status | Type | Syntax | Examples |
|--------|------|--------|----------|
| ✅ | Boolean | `true` or `false` | `true`, `false` |
| ✅ | Integer | Whole numbers | `42`, `-10`, `0` |
| ✅ | Long | Numbers with 'L' suffix | `42L`, `-10L` |
| ✅ | Decimal | Numbers with decimal point | `3.14`, `-0.5`, `1.0` |
| ✅ | String | Single-quoted text | `'hello'`, `'world'` |
| ✅ | Null | Empty collection | `{}` |

### Temporal Literals
| Status | Type | Syntax | Examples |
|--------|------|--------|----------|
| ✅ | Date | `@YYYY-MM-DD` | `@2023-01-01`, `@2023-12-31` |
| ✅ | DateTime | `@YYYY-MM-DDTHH:mm:ss[.fff][Z\|±HH:mm]` | `@2023-01-01T12:30:45Z`, `@2023-01-01T09:30:00-05:00` |
| ✅ | Time | `@THH:mm:ss[.fff]` | `@T12:30:45`, `@T09:30:00.123` |

### Quantity Literals
| Status | Type | Syntax | Examples |
|--------|------|--------|----------|
| ✅ | Quantity | `number 'unit'` | `5 'mg'`, `37.2 'Cel'`, `180 'cm'` |
| ✅ | Dimensionless | `number` | `42`, `3.14` (treated as quantity without unit) |

### DateTime Precision Literals
| Status | Precision | Syntax | Examples |
|--------|-----------|--------|----------|
| ✅ | Year | `year` or `years` | `year`, `years` |
| ✅ | Month | `month` or `months` | `month`, `months` |
| ✅ | Week | `week` or `weeks` | `week`, `weeks` |
| ✅ | Day | `day` or `days` | `day`, `days` |
| ✅ | Hour | `hour` or `hours` | `hour`, `hours` |
| ✅ | Minute | `minute` or `minutes` | `minute`, `minutes` |
| ✅ | Second | `second` or `seconds` | `second`, `seconds` |
| ✅ | Millisecond | `millisecond` or `milliseconds` | `millisecond`, `milliseconds` |


## Special Variables

### Context Variables
| Status | Variable | Description |
|--------|----------|-------------|
| ✅ | `$this` | Current context item in iteration |
| ✅ | `$index` | Current index in iteration |
| ✅ | `$total` | Total count in iteration |

### External Constants
| Status | Variable | Description |
|--------|----------|-------------|
| ✅ | `%[name]` | External constant reference |

## Operator Precedence

The following table shows operator precedence from highest to lowest:

| Precedence | Operators | Associativity | Description |
|------------|-----------|---------------|-------------|
| 1 | `()` | - | Parentheses/Grouping |
| 2 | `.`, `[]`, function calls | Left-to-right | Member access, indexing, function calls |
| 3 | `+`, `-` (unary) | Right-to-left | Unary plus and minus |
| 4 | `*`, `/`, `div`, `mod` | Left-to-right | Multiplicative operations |
| 5 | `+`, `-` (binary) | Left-to-right | Additive operations |
| 6 | `&` | Left-to-right | String concatenation |
| 7 | `\|` | Left-to-right | Union |
| 8 | `is`, `as` | Left-to-right | Type operations |
| 9 | `in`, `contains` | Left-to-right | Membership operations |
| 10 | `<`, `<=`, `>`, `>=` | Left-to-right | Inequality comparisons |
| 11 | `=`, `!=`, `~`, `!~` | Left-to-right | Equality comparisons |
| 12 | `and` | Left-to-right | Logical AND |
| 13 | `or`, `xor` | Left-to-right | Logical OR and XOR |
| 14 | `implies` | Right-to-left | Logical implication |


### Extension: FHIR

| Status | Feature | Notes |
|--------|---------|--------|
| **FHIR Variables** | | |
| ❌ | `%resource` | Not implemented |
| ❌ | `%context` | Not implemented |
| ❌ | `%rootResource` | Not implemented |
| ❌ | `%sct` | Not implemented |
| ❌ | `%loinc` | Not implemented |
| ❌ | `%"vs-[name]"` | Not implemented |
| ❌ | `%"ext-[name]"` | Not implemented |
| **Additional Functions** | | |
| ❌ | `extension(url)` | Not implemented |
| ❌ | `hasValue()` | Not implemented |
| ❌ | `getValue()` | Not implemented |
| ❌ | `trace()` | Not implemented |
| ❌ | `resolve()` | Not implemented |
| ✅ | `ofType(Identifier)` | Filter collection by type specifier |
| ❌ | `elementDefinition()` | Not implemented |
| ❌ | `slice(structure, name)` | Not implemented |
| ❌ | `checkModifiers()` | Not implemented |
| ❌ | `conformsTo(structure)` | Not implemented |
| ❌ | `memberOf(valueset)` | Not implemented |
| ❌ | `subsumes(code)` | Not implemented |
| ❌ | `subsumedBy(code)` | Not implemented |
| ❌ | `htmlChecks` | Not implemented |
| **Paths and polymorphic items** | | |
| ❌ | Polymorphic field access (value[x]) | `Observation.value` for `valueString`, `valueQuantity` etc. |
| ❌ | Type-specific polymorphic access | `Observation.valueString`, `Observation.valueQuantity` |
| ❌ | Resource type navigation | Navigation based on resource type context |
| ❌ | Profile-aware navigation | Navigation based on profile constraints |

### Extension: SQL-on-FHIR

| Status | Feature | Notes |
|--------|---------|--------|
| ❌ | `getResourceKey()` | Not implemented |
| ❌ | `getReferenceKey(resource?)` | Not implemented |

## Implementation Architecture

### Parser
- **Technology**: nom parser combinators for zero-copy parsing
- **Features**: Complete FHIRPath syntax support including temporal and quantity literals
- **Performance**: Efficient recursive descent parser with proper error recovery

### Evaluator
- **Architecture**: Modular evaluation with separate modules for different operation types
- **Type System**: Runtime type checking with automatic conversions where appropriate
- **Performance**: Optimized evaluation with lazy collection processing

### Error Handling
- **Parse Errors**: Detailed location information with helpful messages
- **Evaluation Errors**: Context-aware runtime errors with type information
- **Recovery**: Graceful error handling with partial results where possible

### Testing
- **Unit Tests**: Comprehensive coverage of all functions and operations
- **Integration Tests**: Real-world usage scenarios with FHIR data
- **Performance Tests**: Benchmarks for critical operations
- **Edge Case Tests**: Thorough testing of boundary conditions and error cases

## Notes

- ✅ = Fully implemented and tested
- ❌ = Not implemented
- All functions handle empty collections gracefully
- Singleton collections are automatically unwrapped in scalar contexts
- Type conversions follow FHIRPath specification rules
- Regular expression functions use Rust's `regex` crate with full Unicode support
- Date/time arithmetic supports all standard precision units
- Quantity operations support UCUM unit conversions where applicable
