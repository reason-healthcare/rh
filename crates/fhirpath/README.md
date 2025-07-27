# FHIRPath Parser and Evaluator

A comprehensive Rust implementation of a FHIRPath expression parser and evaluator for FHIR resources.

## Background

FHIRPath is a path-based navigation and extraction language for FHIR resources, defined by the HL7 FHIR specification. This crate provides a complete implementation with:

- **Parser**: Converts FHIRPath expressions into an Abstract Syntax Tree (AST) using nom parser combinators
- **Evaluator**: Comprehensive evaluation of FHIRPath expressions against FHIR resources
- **Type Safety**: Rust-native type checking and error handling with comprehensive error types
- **Performance**: Optimized for production use with efficient parsing and evaluation

## Implementation Status

### Core FHIRPath

| Feature | Status | Notes |
|---------|--------|--------|
| **Navigation and paths** | | |
| Simple path navigation (`.`) | ✅ | `Patient.name` - basic dot notation |
| Nested property access | ✅ | `Patient.name.given` - accessing nested structures |
| Array indexing (`[]`) | ✅ | `name[0]`, `telecom[1].value` with nested support |
| **Tree navigation** | |
| `children()` | ❌ | Not implemented |
| `descendants()` | ❌ | Not implemented |
| **Literals** | | |
| Boolean | ✅ | `true`, `false` |
| Integer | ✅ | `42`, `-10` |
| Decimal | ✅ | `3.14`, `-0.5` |
| String | ✅ | `'hello world'` |
| Date | ✅ | `@2023-01-01` |
| DateTime | ✅ | `@2023-01-01T12:30:45Z` |
| Time | ✅ | `@T12:30:45` |
| Quantity | ✅ | `5'mg'`, `37.2'Cel'` with UCUM units |
| **Equality** | |
| `=` (equals) | ✅ | Equality comparison |
| `!=` (not equals) | ✅ | Inequality comparison |
| `~` (equivalent) | ✅ | Equivalence |
| `!~` (not equivalent) | ✅ | Non-equivalence |
| **Comparison** | |
| `<` (less than) | ✅ | Numeric, string, boolean |
| `<=` (less or equal) | ✅ | Numeric, string, boolean |
| `>` (greater than) | ✅ | Numeric, string, boolean |
| `>=` (greater or equal) | ✅ | Numeric, string, boolean |
| **Types** | | 
| `is` (type test) | ✅ | Type checking: `value is String`, `42 is System.Integer` |
| `as` (type cast) | ✅ | Type casting: `value as String` (returns empty if cast fails) |
| **Collections** | |
| `in` (membership) | ✅ | Value in collection |
| `contains` | ✅ | Collection contains value |
| `subsetOf(other)` | ✅ | Test if this collection is a subset of other |
| `supersetOf(other)` | ✅ | Test if this collection is a superset of other |
| **Boolean logic** | |
| `and` | ✅ | Logical AND |
| `or` | ✅ | Logical OR |
| `xor` | ✅ | Logical XOR |
| `implies` | ✅ | Logical implication: if left is true, return right; if left is false, return true |
| **Unary** | |
| `-` (negation) | ✅ | `-5`, `-weight` |
| `+` (positive) | ✅ | `+value` |
| **Math** | | 
| `+` (addition) | ✅ | Numbers, quantities, strings |
| `-` (subtraction) | ✅ | Numbers, quantities |
| `*` (multiplication) | ✅ | Numbers, quantities |
| `/` (division) | ✅ | Numbers, quantities |
| `div` (integer division) | ✅ | Integer division |
| `mod` (modulo) | ✅ | Modulo operation |
| `&` (concatenation) | ✅ | String concatenation |
| **Date/Time Arithmetic** | |
| Date + duration | ✅ | `@2025-01-01 + 6 months` |
| DateTime + duration | ✅ | `now() - 10 days` |
| Function arithmetic | ✅ | `today() + 2 years` |
| Compound durations | ✅ | `6 months`, `24 hours` |
| Precision handling | ✅ | year(s), month(s), day(s), hour(s), minute(s), second(s) |
| **Utility functions** | |
| `now()` | ✅ | Current date/time UTC |
| `today()` | ✅ | Current date local |
| `timeOfDay()` | ✅ | Current time local |
| **Control flow** | |
| `iif(criterion, true-result [, otherwise-result])` | ✅ | Immediate if (conditional operator) like C's `? :` operator |
| **Operator precedence** | | |
| `()` (Parentheses/Grouping) | ✅ | Highest precedence: `(age + 5) * 2`, `(name.given \| name.family).count()` |
| Function calls | ✅ | `name.where(use='official').count()` - after member access |
| Unary operators (`+`, `-`) | ✅ | `-age`, `+value` - right-associative |
| `*`, `/`, `div`, `mod` | ✅ | Left-to-right: `10 / 2 * 3` → `(10 / 2) * 3` = 15 |
| `+`, `-` (binary) | ✅ | Left-to-right: `10 - 3 + 2` → `(10 - 3) + 2` = 9 |
| `&` (concatenation) | ✅ | Left-to-right: `'a' & 'b' & 'c'` → `'abc'` |
| `\|` (union) | ✅ | Left-to-right: `1 \| 2 \| 3` creates collection |
| `in`, `contains` | ✅ | Membership: `'apple' in fruits` |
| `<`, `<=`, `>`, `>=` | ✅ | Comparison: `age >= 18 and age < 65` |
| `=`, `!=`, `~`, `!~` | ✅ | Equality: `name.family = 'Smith'` |
| `and` | ✅ | Logical AND: `active and name.exists()` |
| `or`, `xor` | ✅ | Logical OR/XOR (lowest precedence) |
| **Existence** | |
| `empty()` | ✅ | Test if collection is empty |
| `exists()` | ✅ | Test if any items exist |
| `count()` | ✅ | Count items in collection |
| **Filtering and projection** | |
| `where(criteria)` | ✅ | Filter collection by criteria |
| `select(projection)` | ✅ | Transform each item |
| `repeat(projection)` | ✅ | Recursively apply projection expression |
| `ofType(type)` | ✅ | Filter collection by type specifier |
| **Subsetting** | |
| `single()` | ✅ | Return single item (error if != 1) |
| `first()` | ✅ | Return first item |
| `last()` | ✅ | Return last item |
| `tail()` | ✅ | All items except first |
| `skip(num)` | ✅ | Skip first num items |
| `take(num)` | ✅ | Take first num items |
| **Combining** | |
| `union(other)` | ✅ | Union operator |
| `combine(other)` | ✅ | Merge collections preserving duplicates |
| `intersect(other)` | ✅ | Items in both collections |
| `exclude(other)` | ✅ | Items not in other collection |
| **Conversion** | |
| `toBoolean()` | ✅ | Convert to boolean |
| `convertsToBoolean()` | ✅ | Test if value can be converted to boolean |
| `toInteger()` | ✅ | Convert to integer |
| `convertsToInteger()` | ✅ | Test if value can be converted to integer |
| `toLong()` | ✅ | Convert to long integer (64-bit) |
| `convertsToLong()` | ✅ | Test if value can be converted to long |
| `toDate()` | ✅ | Convert to date |
| `convertsToDate()` | ✅ | Test if value can be converted to date |
| `toDateTime()` | ✅ | Convert to datetime |
| `convertsToDateTime()` | ✅ | Test if value can be converted to datetime |
| `toString()` | ✅ | Convert to string |
| `convertsToString()` | ✅ | Test if value can be converted to string |
| `toTime()` | ✅ | Convert to time |
| `convertsToTime()` | ✅ | Test if value can be converted to time |
| `toQuantity([unit])` | ✅ | Convert to quantity with optional unit |
| `convertsToQuantity([unit])` | ✅ | Test if value can be converted to quantity |
| `distinct()` | ✅ | Remove duplicates |
| `isDistinct()` | ✅ | Test if all items unique |
| **Boolean collection operations** | |
| `all()` | ✅ | Test if all items are truthy |
| `allTrue()` | ✅ | Test if all items are boolean true |
| `anyTrue()` | ✅ | Test if any item is boolean true |
| `allFalse()` | ✅ | Test if all items are boolean false |
| `anyFalse()` | ✅ | Test if any item is boolean false |
| **String Manipulation** | |
| `length()` | ✅ | String length |
| `substring(start, length?)` | ✅ | Extract substring |
| `upper()` | ✅ | Convert to uppercase |
| `lower()` | ✅ | Convert to lowercase |
| `trim()` | ✅ | Remove whitespace |
| **Additional String Functions** | |
| `startsWith(prefix)` | ✅ | Test string prefix |
| `endsWith(suffix)` | ✅ | Test string suffix |
| `indexOf(substring)` | ✅ | Find substring index |
| `replace(pattern, replacement)` | ✅ | Replace occurrences |
| `split(delimiter)` | ✅ | Split into collection |
| `join(delimiter)` | ✅ | Join collection |
| `matches(pattern)` | ✅ | Pattern matching |
| **Null and empty** | | |
| Empty collection literal: `{}` | ✅ | `{}` - represents empty collection |
| Empty collection evaluation | ✅ | Empty results from operations (e.g., out-of-bounds indexing) |
| Null propagation in operations | ✅ | Operations on empty collections return empty |
| Null-safe equality (`~`) | ✅ | `value ~ null` - handles null comparisons |
| Null-safe inequality (`!~`) | ✅ | `value !~ null` - handles null comparisons |
| Empty collection in boolean context | ✅ | Empty collections evaluate to `false` in logical operations |
| Empty collection indexing | ✅ | `{}[0]` returns empty (no error) |
| Empty collection functions | ✅ | Functions like `count()`, `exists()` work on empty collections |
| Missing field access | ✅ | `nonexistent.field` returns empty (no error) |
| **Singleton Evaluation of Collections** | | |
| Single-item collections | ✅ | Collections with one item behave as singletons in operations |
| Automatic unwrapping | ✅ | `[42]` automatically becomes `42` in scalar contexts |
| Arithmetic with singletons | ✅ | `[5] + [3]` → `8` (automatic unwrapping) |
| Comparison with singletons | ✅ | `[5] > [3]` → `true` (automatic unwrapping) |
| String operations on singletons | ✅ | `['hello'].upper()` → `'HELLO'` |
| Function calls on singletons | ✅ | `[42].abs()` → `42` (functions work on singleton values) |
| Singleton preservation | ✅ | Operations that should return collections maintain collection type |
| Multi-item collection handling | ✅ | Operations on multi-item collections work element-wise where appropriate |
| **Aggregates** | | |
| `aggregate(aggregator, init?)` | ❌ | Not implemented |

### Extension: FHIR

| Feature | Status | Notes |
|---------|--------|--------|
| **FHIR Variables** | | |
| `%resource` | ❌ | Not implemented |
| `%context` | ❌ | Not implemented |
| `%rootResource` | ❌ | Not implemented |
| `%sct` | ❌ | Not implemented |
| `%loinc` | ❌ | Not implemented |
| `%"vs-[name]"` | ❌ | Not implemented |
| `%"ext-[name]"` | ❌ | Not implemented |
| **Additional Functions** | | |
| `extension(url)` | ❌ | Not implemented |
| `hasValue()` | ❌ | Not implemented |
| `getValue()` | ❌ | Not implemented |
| `trace()` | ❌ | Not implemented |
| `resolve()` | ❌ | Not implemented |
| `ofType(Identifier)` | ✅ | Filter collection by type specifier |
| `elementDefinition()` | ❌ | Not implemented |
| `slice(structure, name)` | ❌ | Not implemented |
| `checkModifiers()` | ❌ | Not implemented |
| `conformsTo(structure)` | ❌ | Not implemented |
| `memberOf(valueset)` | ❌ | Not implemented |
| `subsumes(code)` | ❌ | Not implemented |
| `subsumedBy(code)` | ❌ | Not implemented |
| `htmlChecks` | ❌ | Not implemented |
| **Paths and polymorphic items** | | |
| Polymorphic field access (value[x]) | ❌ | `Observation.value` for `valueString`, `valueQuantity` etc. |
| Type-specific polymorphic access | ❌ | `Observation.valueString`, `Observation.valueQuantity` |
| Resource type navigation | ❌ | Navigation based on resource type context |
| Profile-aware navigation | ❌ | Navigation based on profile constraints |

### Extension: SQL-on-FHIR

| Feature | Status | Notes |
|---------|--------|--------|
| `getResourceKey()` | ❌ | Not implemented |
| `getReferenceKey(resource?)` | ❌ | Not implemented |


## Usage

### CLI

You can use the `rh` cli. For help:
```
cargo run -p rh -- fhirpath --help
```

There is also an interactive REPL:
```
cargo run -p rh -- fhirpath repl
```

### Basic Example

```rust
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext};
use serde_json::json;

let patient = json!({
    "resourceType": "Patient",
    "name": [{"given": ["John"], "family": "Doe"}],
    "birthDate": "1974-12-25"
});

let parser = FhirPathParser::new();
let evaluator = FhirPathEvaluator::new();
let context = EvaluationContext::new(patient);

// Extract patient name
let expr = parser.parse("name.given").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();

// Date arithmetic
let expr = parser.parse("now() - 30 days").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();

// String operations
let expr = parser.parse("name.family.upper()").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
```

### Quantity Operations

```rust
// Unit conversion
let expr = parser.parse("1.0'kg' + 500.0'g'").unwrap(); // → 1.5kg

// Temperature conversion
let expr = parser.parse("20.0'Cel' + 273.15'K'").unwrap(); // → 293.15Cel
```

### Date/Time Arithmetic

```rust
// Current time minus duration
let expr = parser.parse("now() - 10 days").unwrap();

// Date literals with arithmetic
let expr = parser.parse("@2025-01-01 + 6 months").unwrap(); // → 2025-07-01

// Complex expressions
let expr = parser.parse("today() - 1 year + 30 days").unwrap();
```

### Long Type Conversions

The FHIRPath evaluator supports explicit Long types (64-bit integers) with the `L` suffix and conversion functions:

```rust
// Long literals with L suffix
let expr = parser.parse("42L").unwrap(); // → Long(42)

// Convert values to Long
let expr = parser.parse("true.toLong()").unwrap(); // → Long(1)
let expr = parser.parse("false.toLong()").unwrap(); // → Long(0)
let expr = parser.parse("'123'.toLong()").unwrap(); // → Long(123)
let expr = parser.parse("42.0.toLong()").unwrap(); // → Long(42) - whole numbers only

// Test conversion compatibility
let expr = parser.parse("'123'.convertsToLong()").unwrap(); // → Boolean(true)
let expr = parser.parse("'abc'.convertsToLong()").unwrap(); // → Boolean(false)
let expr = parser.parse("42.5.convertsToLong()").unwrap(); // → Boolean(false)
```

### Date/DateTime Conversions

The FHIRPath evaluator supports conversion between Date and DateTime types with conversion functions:

```rust
// Convert values to Date
let expr = parser.parse("@2023-01-15.toDate()").unwrap(); // → Date("2023-01-15") - identity
let expr = parser.parse("@2023-01-15T10:30:45.toDate()").unwrap(); // → Date("2023-01-15") - extract date part
let expr = parser.parse("'2023-12-25'.toDate()").unwrap(); // → Date("2023-12-25")
let expr = parser.parse("'2023-12-25T23:59:59Z'.toDate()").unwrap(); // → Date("2023-12-25")

// Convert values to DateTime
let expr = parser.parse("@2023-01-15T10:30:45.toDateTime()").unwrap(); // → DateTime("2023-01-15T10:30:45") - identity
let expr = parser.parse("@2023-01-15.toDateTime()").unwrap(); // → DateTime("2023-01-15T00:00:00") - add midnight
let expr = parser.parse("'2023-12-25'.toDateTime()").unwrap(); // → DateTime("2023-12-25T00:00:00")
let expr = parser.parse("'2023-12-25T23:59:59'.toDateTime()").unwrap(); // → DateTime("2023-12-25T23:59:59")

// Test conversion compatibility
let expr = parser.parse("'2023-01-15'.convertsToDate()").unwrap(); // → Boolean(true)
let expr = parser.parse("'2023-01-15T10:30:45'.convertsToDate()").unwrap(); // → Boolean(true)
let expr = parser.parse("'not-a-date'.convertsToDate()").unwrap(); // → Boolean(false)
let expr = parser.parse("'2023-01-15'.convertsToDateTime()").unwrap(); // → Boolean(true)
let expr = parser.parse("'2023-01-15T10:30:45'.convertsToDateTime()").unwrap(); // → Boolean(true)
let expr = parser.parse("'not-a-datetime'.convertsToDateTime()").unwrap(); // → Boolean(false)
```

### String Conversions

The FHIRPath evaluator supports conversion of most value types to String with conversion functions:

```rust
// Convert values to String
let expr = parser.parse("'hello world'.toString()").unwrap(); // → String("hello world") - identity
let expr = parser.parse("true.toString()").unwrap(); // → String("true")
let expr = parser.parse("false.toString()").unwrap(); // → String("false")
let expr = parser.parse("42.toString()").unwrap(); // → String("42")
let expr = parser.parse("42L.toString()").unwrap(); // → String("42")
let expr = parser.parse("3.14.toString()").unwrap(); // → String("3.14")
let expr = parser.parse("42.0.toString()").unwrap(); // → String("42") - removes unnecessary decimals
let expr = parser.parse("@2023-01-15.toString()").unwrap(); // → String("2023-01-15")
let expr = parser.parse("@2023-01-15T10:30:45.toString()").unwrap(); // → String("2023-01-15T10:30:45")
let expr = parser.parse("@T10:30:45.toString()").unwrap(); // → String("T10:30:45")
let expr = parser.parse("5'mg'.toString()").unwrap(); // → String("5 'mg'")

// Test conversion compatibility
let expr = parser.parse("42.convertsToString()").unwrap(); // → Boolean(true)
let expr = parser.parse("true.convertsToString()").unwrap(); // → Boolean(true)
let expr = parser.parse("@2023-01-15.convertsToString()").unwrap(); // → Boolean(true)
let expr = parser.parse("5'mg'.convertsToString()").unwrap(); // → Boolean(true)
```

### Time Conversions

The FHIRPath evaluator supports conversion of various value types to Time with conversion functions:

```rust
// Convert values to Time
let expr = parser.parse("@T10:30:45.toTime()").unwrap(); // → Time("T10:30:45") - identity
let expr = parser.parse("'10:30:45'.toTime()").unwrap(); // → Time("T10:30:45")
let expr = parser.parse("'23:59:59.123'.toTime()").unwrap(); // → Time("T23:59:59.123")
let expr = parser.parse("'T14:25:36'.toTime()").unwrap(); // → Time("T14:25:36")
let expr = parser.parse("@2023-01-15T10:30:45.toTime()").unwrap(); // → Time("T10:30:45") - extract time part
let expr = parser.parse("@2023-01-15T23:59:59Z.toTime()").unwrap(); // → Time("T23:59:59") - with timezone
let expr = parser.parse("@2023-01-15T14:25:36+05:30.toTime()").unwrap(); // → Time("T14:25:36") - with offset

// Test conversion compatibility
let expr = parser.parse("'10:30:45'.convertsToTime()").unwrap(); // → Boolean(true)
let expr = parser.parse("@T14:25:36.convertsToTime()").unwrap(); // → Boolean(true)
let expr = parser.parse("@2023-01-15T10:30:45.convertsToTime()").unwrap(); // → Boolean(true)
let expr = parser.parse("'not-a-time'.convertsToTime()").unwrap(); // → Boolean(false)
let expr = parser.parse("'25:00:00'.convertsToTime()").unwrap(); // → Boolean(false) - invalid hour
let expr = parser.parse("42.convertsToTime()").unwrap(); // → Boolean(false)
```

### Quantity Conversions

The FHIRPath evaluator supports conversion of various value types to Quantity with optional unit specification:

```rust
// Convert numbers to quantities without units
let expr = parser.parse("42.toQuantity()").unwrap(); // → Quantity { value: 42.0, unit: None }
let expr = parser.parse("3.14.toQuantity()").unwrap(); // → Quantity { value: 3.14, unit: None }
let expr = parser.parse("100L.toQuantity()").unwrap(); // → Quantity { value: 100.0, unit: None }

// Convert with unit parameter
let expr = parser.parse("5.toQuantity('mg')").unwrap(); // → Quantity { value: 5.0, unit: Some("mg") }
let expr = parser.parse("37.2.toQuantity('Cel')").unwrap(); // → Quantity { value: 37.2, unit: Some("Cel") }
let expr = parser.parse("120.toQuantity('mm[Hg]')").unwrap(); // → Quantity { value: 120.0, unit: Some("mm[Hg]") }

// Convert strings to quantities
let expr = parser.parse("'42.7'.toQuantity()").unwrap(); // → Quantity { value: 42.7, unit: None }
let expr = parser.parse("'98.6'.toQuantity('[degF]')").unwrap(); // → Quantity { value: 98.6, unit: Some("[degF]") }

// Unit override for existing quantities
let expr = parser.parse("5'mg'.toQuantity()").unwrap(); // → Quantity { value: 5.0, unit: Some("mg") } - identity
let expr = parser.parse("5'mg'.toQuantity('g')").unwrap(); // → Quantity { value: 5.0, unit: Some("g") } - unit override

// Test conversion compatibility
let expr = parser.parse("42.convertsToQuantity()").unwrap(); // → Boolean(true)
let expr = parser.parse("'123.45'.convertsToQuantity()").unwrap(); // → Boolean(true)
let expr = parser.parse("5'mg'.convertsToQuantity()").unwrap(); // → Boolean(true)
let expr = parser.parse("'not-a-number'.convertsToQuantity()").unwrap(); // → Boolean(false)
let expr = parser.parse("true.convertsToQuantity()").unwrap(); // → Boolean(false)
```

## Architecture

### Parser (`src/parser.rs`)
- **Technology**: nom parser combinators for zero-copy parsing
- **Features**: Complete FHIRPath syntax support including temporal and quantity literals
- **Performance**: Efficient recursive descent parser with proper error recovery

### AST (`src/ast.rs`)
- **Design**: Type-safe representation of all FHIRPath expressions
- **Serialization**: Full serde support for JSON serialization/deserialization
- **Extensibility**: Easy to extend for new expression types

### Evaluator (`src/evaluator/`)
- **Architecture**: Modular evaluation with separate modules for different operation types
- **Type System**: Runtime type checking with automatic conversions where appropriate
- **Performance**: Optimized evaluation with lazy collection processing

### Error Handling (`src/error.rs`)
- **Parse Errors**: Detailed location information with helpful messages
- **Evaluation Errors**: Context-aware runtime errors with type information
- **Recovery**: Graceful error handling with partial results where possible


### Working Examples


NOTE: See the [examples directory](./examples/) for complete, runnable examples with detailed output and explanations.

```bash
# Run individual examples, e.g.:
cargo run --example to_boolean --package fhirpath
cargo run --example to_integer --package fhirpath
cargo run --example long_conversion --package fhirpath
cargo run --example date_conversion --package fhirpath
cargo run --example string_conversion --package fhirpath
cargo run --example time_conversion --package fhirpath
cargo run --example quantity_conversion --package fhirpath
cargo run --example unit_conversion_example --package fhirpath
cargo run --example temperature_conversion_example --package fhirpath  
cargo run --example datetime_functions_example --package fhirpath
cargo run --example iif_function --package fhirpath
```


```rust
// Array indexing operations
parser.parse("name[0]").unwrap();             // ✅ → First name object
parser.parse("name[1].family").unwrap();      // ✅ → Second name's family
parser.parse("name[0].given[0]").unwrap();    // ✅ → First given name
parser.parse("name[0].given[1]").unwrap();    // ✅ → Second given name
parser.parse("name[10]").unwrap();            // ✅ → Empty (out of bounds)
parser.parse("(10 | 20 | 30)[1]").unwrap();  // ✅ → Integer(20)

// Arithmetic operations
parser.parse("1 + 2").unwrap();              // ✅ → Integer(3)
parser.parse("10 - 3").unwrap();             // ✅ → Integer(7)
parser.parse("4 * 5").unwrap();              // ✅ → Integer(20)
parser.parse("15 / 3").unwrap();             // ✅ → Number(5.0)
parser.parse("17 div 5").unwrap();           // ✅ → Integer(3)
parser.parse("17 mod 5").unwrap();           // ✅ → Integer(2)
parser.parse("'Hello' & ' World'").unwrap(); // ✅ → "Hello World"

// Comparison operations
parser.parse("5 > 3").unwrap();              // ✅ → Boolean(true)
parser.parse("'apple' < 'banana'").unwrap(); // ✅ → Boolean(true)
parser.parse("2 + 3 > 4").unwrap();          // ✅ → Boolean(true)

// Membership operations
parser.parse("'apple' in fruits").unwrap();     // ✅ → Boolean evaluation
parser.parse("fruits contains 'apple'").unwrap(); // ✅ → Boolean evaluation

// Collection functions
parser.parse("name.empty()").unwrap();       // ✅ → Boolean evaluation
parser.parse("telecom.count()").unwrap();    // ✅ → Integer evaluation
parser.parse("items.distinct()").unwrap();   // ✅ → Collection evaluation

// Boolean collection functions
parser.parse("(true | false | true).all()").unwrap();     // ✅ → Boolean(false) - not all truthy  
parser.parse("(true | true | true).allTrue()").unwrap();  // ✅ → Boolean(true) - all boolean true
parser.parse("(false | true | false).anyTrue()").unwrap(); // ✅ → Boolean(true) - has boolean true
parser.parse("(false | false).allFalse()").unwrap();      // ✅ → Boolean(true) - all boolean false
parser.parse("(true | false | true).anyFalse()").unwrap(); // ✅ → Boolean(true) - has boolean false

// Subsetting functions
parser.parse("(1 | 2 | 3).first()").unwrap(); // ✅ → Integer(1)
parser.parse("(1 | 2 | 3).last()").unwrap();  // ✅ → Integer(3)
parser.parse("(42).single()").unwrap();       // ✅ → Integer(42)
parser.parse("(1 | 2 | 3 | 4).tail()").unwrap(); // ✅ → Collection([2, 3, 4])
parser.parse("(1 | 2 | 3 | 4).skip(2)").unwrap(); // ✅ → Collection([3, 4])
parser.parse("(1 | 2 | 3 | 4).take(2)").unwrap(); // ✅ → Collection([1, 2])

// Filtering functions
parser.parse("name.where(use = 'official')").unwrap(); // ✅ → Filtered collection
parser.parse("name.select(family)").unwrap(); // ✅ → Projected collection
parser.parse("(1 | 'hello' | true | 3.14).ofType(String)").unwrap(); // ✅ → Collection(["hello"])
parser.parse("(1 | 'hello' | true | 3.14).ofType(Integer)").unwrap(); // ✅ → Collection([1])
parser.parse("Bundle.entry.resource.ofType(Patient)").unwrap(); // ✅ → Patient resources only

// String functions
parser.parse("'hello'.length()").unwrap();   // ✅ → Integer(5)
parser.parse("'HELLO'.lower()").unwrap();    // ✅ → "hello"
parser.parse("'  text  '.trim()").unwrap();  // ✅ → "text"
parser.parse("'hello world'.substring(6)").unwrap(); // ✅ → "world"
parser.parse("'hello'.startsWith('he')").unwrap(); // ✅ → Boolean(true)
parser.parse("'a,b,c'.split(',')").unwrap(); // ✅ → Collection(["a", "b", "c"])

// Math functions
parser.parse("(-5).abs()").unwrap();         // ✅ → Integer(5) - absolute value
parser.parse("3.7.ceiling()").unwrap();      // ✅ → Integer(4) - round up
parser.parse("3.2.floor()").unwrap();        // ✅ → Integer(3) - round down
parser.parse("16.sqrt()").unwrap();          // ✅ → Number(4.0) - square root
parser.parse("2.power(3)").unwrap();         // ✅ → Number(8.0) - 2³
parser.parse("3.14159.round(2)").unwrap();   // ✅ → Number(3.14) - round to 2 decimals
parser.parse("8.log(2)").unwrap();           // ✅ → Number(3.0) - log₂(8)
parser.parse("1.exp()").unwrap();            // ✅ → Number(2.718...) - e¹
parser.parse("(-3.7).abs().ceiling()").unwrap(); // ✅ → Integer(4) - function chaining

// Control flow functions (conditional logic)
parser.parse("''.iif(5 > 3, 'Greater', 'Not greater')").unwrap(); // ✅ → "Greater"
parser.parse("''.iif(2 > 5, 'True', 'False')").unwrap(); // ✅ → "False"
parser.parse("''.iif(age >= 18, 'Adult')").unwrap(); // ✅ → "Adult" or Empty
parser.parse("''.iif(name.exists(), name.family, 'Unknown')").unwrap(); // ✅ → Conditional value
parser.parse("''.iif(status = 'active', 'ACTIVE', status.upper())").unwrap(); // ✅ → Status processing

// Date/time functions
parser.parse("now()").unwrap();              // ✅ → DateTime("2025-07-26T14:26:23.320Z") - current UTC datetime
parser.parse("today()").unwrap();            // ✅ → Date("2025-07-26") - current local date
parser.parse("timeOfDay()").unwrap();        // ✅ → Time("10:26:23.320") - current local time
parser.parse("now().exists()").unwrap();     // ✅ → Boolean(true) - datetime functions in expressions
parser.parse("(now() | today()).count()").unwrap(); // ✅ → Integer(2) - datetime functions in collections

// Function chaining
parser.parse("'  HELLO  '.trim().lower()").unwrap(); // ✅ → "hello"
parser.parse("'a,b,c'.split(',').join('|')").unwrap(); // ✅ → "a|b|c"

// Logical operations
parser.parse("active and name.exists()").unwrap(); // ✅ Works
parser.parse("true implies false").unwrap();       // ✅ → Boolean(false) - logical implication
parser.parse("false implies true").unwrap();       // ✅ → Boolean(true) - false implies anything
parser.parse("false implies false").unwrap();      // ✅ → Boolean(true) - false implies anything

// Type operations
parser.parse("true is Boolean").unwrap();       // ✅ → Boolean(true) - type checking
parser.parse("'hello' is String").unwrap();     // ✅ → Boolean(true) - type checking
parser.parse("42 is Integer").unwrap();         // ✅ → Boolean(true) - type checking
parser.parse("3.14 is Decimal").unwrap();       // ✅ → Boolean(true) - type checking
parser.parse("true is String").unwrap();        // ✅ → Boolean(false) - type mismatch
parser.parse("'hello' as String").unwrap();     // ✅ → String("hello") - successful cast
parser.parse("42 as Integer").unwrap();         // ✅ → Integer(42) - successful cast
parser.parse("true as String").unwrap();        // ✅ → Empty - failed cast
parser.parse("true is System.Boolean").unwrap(); // ✅ → Boolean(true) - system types
parser.parse("42 is System.Integer").unwrap();   // ✅ → Boolean(true) - system types

// Union operations
parser.parse("name.given | name.family").unwrap(); // ✅ Works

// Complex union operations
parser.parse("(1 | 2 | 3)").unwrap();          // ✅ → Collection([1, 2, 3])
parser.parse("(42 | 'hello' | true)").unwrap(); // ✅ → Mixed type collection
parser.parse("((1 | 2) | (3 | 4))").unwrap();   // ✅ → Nested union flattening
parser.parse("(10 | 20 | 30)[1]").unwrap();     // ✅ → Integer(20) (union + indexing)

// Collection combining operations
parser.parse("(1 | 2 | 3).combine((3 | 4 | 5))").unwrap(); // ✅ → Collection([1, 2, 3, 3, 4, 5]) - preserves duplicates
parser.parse("strings1.combine(strings2)").unwrap();       // ✅ → Merge collections keeping all items
parser.parse("patients.combine(observations)").unwrap();   // ✅ → Combine FHIR resources
parser.parse("collection1.combine(collection2).distinct()").unwrap(); // ✅ → Merge and remove duplicates

// Complex indexing with filtering
parser.parse("name.where(use = 'official')[0].given[0]").unwrap(); // ✅ Works

// Temporal literals
parser.parse("@2023-01-01").unwrap();                // ✅ → Date literal
parser.parse("@2023-01-01T12:30:45").unwrap();       // ✅ → DateTime literal
parser.parse("@2023-01-01T00:00:00Z").unwrap();      // ✅ → DateTime with UTC timezone
parser.parse("@2023-01-01T12:30:45+05:30").unwrap(); // ✅ → DateTime with timezone offset
parser.parse("@T12:30:45").unwrap();                 // ✅ → Time literal

// Quantity literals and arithmetic
parser.parse("5'mg'").unwrap();                       // ✅ → Quantity with UCUM unit
parser.parse("5 'mg'").unwrap();                      // ✅ → Same quantity (space supported)
parser.parse("37.2'Cel'").unwrap();                   // ✅ → Temperature quantity
parser.parse("98.6'[degF]'").unwrap();                // ✅ → Fahrenheit temperature
parser.parse("15 'mm[Hg]'").unwrap();                 // ✅ → Blood pressure quantity (space supported)
parser.parse("273.15'K'").unwrap();                   // ✅ → Kelvin temperature
parser.parse("2'wk'").unwrap();                       // ✅ → Calendar duration
parser.parse("5'mg' + 3'mg'").unwrap();               // ✅ → 8mg (same units)
parser.parse("10'kg' * 2").unwrap();                  // ✅ → 20kg (scalar multiplication)
parser.parse("120'mm[Hg]' / 60'mm[Hg]'").unwrap();   // ✅ → 2.0 (dimensionless ratio)
parser.parse("15 'mm[Hg]' + 5 'mm[Hg]'").unwrap();   // ✅ → 20mm[Hg] (with spaces)

// Unit conversion examples (linear units)
parser.parse("1.0'kg' + 500.0'g'").unwrap();         // ✅ → 1.5kg (500g→0.5kg, then add)
parser.parse("2.0'L' + 250.0'mL'").unwrap();         // ✅ → 2.25L (250mL→0.25L, then add)
parser.parse("1.0'h' + 30.0'min'").unwrap();         // ✅ → 1.5h (30min→0.5h, then add)
parser.parse("100.0'Pa' + 1.0'kPa'").unwrap();       // ✅ → 1100Pa (1kPa→1000Pa, then add)

// Temperature conversion examples (offset-based)
parser.parse("0.0'Cel' + 273.15'K'").unwrap();       // ✅ → 273.15Cel (273.15K→273.15°C, then add)
parser.parse("32.0'[degF]' + 0.0'Cel'").unwrap();    // ✅ → 32.0[degF] (0°C→32°F, then add)
parser.parse("20.0'Cel' + 5.0'Cel'").unwrap();       // ✅ → 25.0Cel (same unit, direct add)
parser.parse("100.0'Cel' - 373.15'K'").unwrap();     // ✅ → 0.0Cel (373.15K→100°C, then subtract)

// Cross-unit operations
parser.parse("1.0'ft' + 12.0'in'").unwrap();         // ✅ → 2.0ft (12in→1ft, then add)
parser.parse("500.0'mg' / 0.5'g'").unwrap();         // ✅ → 1.0 (dimensionless: 0.5g→500mg, ratio)

// Empty collection indexing
parser.parse("{}[0]").unwrap(); // ✅ → Empty (graceful handling)
```

## Unit Conversion System

The FHIRPath library includes a comprehensive unit conversion system that automatically handles conversions between compatible UCUM units during arithmetic operations. This enables seamless calculations between different units of the same physical quantity, with special support for temperature units that require offset-based conversions.

### Supported Unit Categories

#### Mass Units
- `g` (gram) - base unit
- `kg` (kilogram) = 1000 g
- `mg` (milligram) = 0.001 g  
- `ug` (microgram) = 0.000001 g
- `lb` (pound) = 453.592 g

#### Length Units
- `m` (meter) - base unit
- `cm` (centimeter) = 0.01 m
- `mm` (millimeter) = 0.001 m
- `km` (kilometer) = 1000 m
- `in` or `[in_i]` (inch) = 0.0254 m
- `ft` (foot) = 0.3048 m

#### Volume Units
- `L` (liter) - base unit
- `mL` (milliliter) = 0.001 L
- `dL` (deciliter) = 0.1 L
- `uL` (microliter) = 0.000001 L

#### Time Units
- `s` (second) - base unit
- `min` (minute) = 60 s
- `h` (hour) = 3600 s
- `d` (day) = 86400 s
- `wk` (week) = 604800 s
- `mo` (month) = 2629746 s (average)
- `a` (year) = 31556952 s (average)

#### Pressure Units
- `Pa` (pascal) - base unit
- `kPa` (kilopascal) = 1000 Pa
- `mm[Hg]` (millimeter of mercury) = 133.322 Pa
- `bar` (bar) = 100000 Pa

#### Temperature Units (with Offset Conversion)
- `Cel` (celsius) - base unit
- `K` (kelvin) - offset conversion: K = Cel + 273.15
- `[degF]` (fahrenheit) - offset conversion: °F = (Cel × 9/5) + 32

Temperature units are handled with special offset-based conversion logic to ensure accurate conversions between Celsius, Kelvin, and Fahrenheit scales. Unlike linear unit conversions, temperature conversions account for the different zero points of each scale.

### Unit Conversion Examples

```rust
// Addition and Subtraction with automatic conversion
parser.parse("1.0'kg' + 500.0'g'").unwrap();      // → 1.5 kg (500g → 0.5kg)
parser.parse("2.0'L' + 250.0'mL'").unwrap();      // → 2.25 L (250mL → 0.25L)
parser.parse("1.0'h' + 30.0'min'").unwrap();      // → 1.5 h (30min → 0.5h)
parser.parse("1.5'm' + 50.0'cm'").unwrap();       // → 2.0 m (50cm → 0.5m)

// Temperature conversions with offset handling (Celsius as base unit)
parser.parse("0.0'Cel' + 273.15'K'").unwrap();    // → 273.15 Cel (273.15K → 273.15°C, then add)
parser.parse("32.0'[degF]' + 0.0'Cel'").unwrap(); // → 32.0 [degF] (0°C → 32°F, then add)
parser.parse("20.0'Cel' + 5.0'Cel'").unwrap();    // → 25.0 Cel (same unit addition)
parser.parse("100.0'Cel' - 273.15'K'").unwrap();  // → 100.0 Cel (273.15K → 0°C, then subtract)

// Cross-temperature scale conversions  
parser.parse("0.0'K' + 0.0'Cel'").unwrap();       // → -273.15 K (0°C → -273.15K in Kelvin result)
parser.parse("32.0'[degF]' - 32.0'[degF]'").unwrap(); // → 0.0 [degF] (same unit subtraction)

// Pressure unit conversions
parser.parse("120.0'mm[Hg]' + 10.0'kPa'").unwrap(); // → Automatic conversion and addition
parser.parse("1.0'bar' / 2.0").unwrap();           // → 0.5 bar (scalar division)

// Multiplication and Division by scalars
parser.parse("5.0'mg' * 3.0").unwrap();           // → 15.0 mg
parser.parse("100.0'mL' / 4.0").unwrap();         // → 25.0 mL
parser.parse("37.0'Cel' * 2.0").unwrap();         // → 74.0 Cel

// Division of compatible quantities (dimensionless result)
parser.parse("10.0'kg' / 2.0'kg'").unwrap();      // → 5.0 (dimensionless)
parser.parse("1.0'kg' / 500.0'g'").unwrap();      // → 2.0 (1kg = 1000g, 1000/500 = 2)
parser.parse("212.0'[degF]' / 100.0'Cel'").unwrap(); // → 2.12 (212°F = 100°C, ratio calculation)

// Error cases - incompatible units
parser.parse("1.0'kg' + 1.0'm'").unwrap();        // → Error: incompatible units
parser.parse("20.0'Cel' + 5.0'kg'").unwrap();     // → Error: cannot add temperature and mass
```

### Conversion Process

The unit conversion system follows different processes depending on the unit type:

#### Linear Unit Conversions (Mass, Length, Volume, Time, Pressure)
1. **Compatibility Check**: Verify both quantities are of the same physical type (mass, length, volume, etc.)
2. **Base Unit Conversion**: Convert both values to their respective base units using multiplicative factors
3. **Arithmetic Operation**: Perform the operation on base unit values  
4. **Result Conversion**: Convert back to the left operand's unit system

#### Temperature Unit Conversions (Offset-Based)
1. **Compatibility Check**: Verify both quantities are temperature units
2. **Base Unit Conversion**: Convert both values to Celsius using offset formulas:
   - Kelvin → Celsius: `Cel = K - 273.15`
   - Fahrenheit → Celsius: `Cel = (°F - 32) × 5/9`
   - Celsius → Celsius: `Cel = Cel` (identity)
3. **Arithmetic Operation**: Perform the operation on Celsius values
4. **Result Conversion**: Convert back to the left operand's unit using inverse offset formulas:
   - Celsius → Kelvin: `K = Cel + 273.15` 
   - Celsius → Fahrenheit: `°F = (Cel × 9/5) + 32`
   - Celsius → Celsius: `Cel = Cel` (identity)

This dual approach ensures accurate conversions for both proportional quantities (where zero means "nothing") and interval scales like temperature (where zero is arbitrary).

### Base Units by Category
- **Mass**: gram (g)
- **Length**: meter (m) 
- **Volume**: liter (L)
- **Time**: second (s)
- **Pressure**: pascal (Pa)
- **Temperature**: celsius (Cel) - with special offset-based conversion handling

### Error Handling

The system provides clear error messages for:
- **Incompatible units**: `Cannot add quantities with incompatible units: Some("kg") and Some("m")`
- **Division by zero**: `Division by zero`
- **Unknown units**: `Unknown unit: xyz`
- **Temperature conversion errors**: `Unknown temperature unit: xyz`
- **Mixed unit types**: `Cannot add quantities with incompatible units: Some("Cel") and Some("kg")`

See `examples/unit_conversion_example.rs` and `examples/temperature_conversion_example.rs` for comprehensive examples of all supported conversions and operations.

## Grammar Support

This implementation is based on the official FHIRPath grammar from:
https://build.fhir.org/ig/HL7/FHIRPath/fhirpath.g4

The parser is implemented using the `nom` parser combinator library, providing a pure Rust implementation without requiring build-time code generation. The parser supports the complete core FHIRPath syntax including:

- Path expressions with proper precedence
- All literal value types
- Function invocations with parameters
- Boolean and arithmetic logic
- Comparison operations
- Union operations
- Collection and string manipulation

### Parser Architecture

The parser is built with operator precedence in mind:

1. OR/XOR expressions (lowest precedence)
2. AND expressions
3. Equality expressions
4. Comparison expressions
5. Membership expressions
6. Arithmetic expressions (with proper precedence)
7. Union expressions
8. Invocation expressions (member access, indexing, function calls)
9. Terms (literals, functions, parentheses) (highest precedence)

## Testing

The implementation includes comprehensive tests:

- **Unit tests**: tests covering parser and evaluator including temporal literals
- **Integration tests**: real-world usage examples including arithmetic, comparisons, membership, collection functions, filtering functions, string manipulation, math functions, array indexing, union operations, and temporal literals
- **Parser coverage**: All core syntax elements parse successfully including collection, filtering, string function calls, array indexing, union operations, and temporal literals
- **Evaluator coverage**: Literals (including temporal), member access, array indexing (including nested indexing), union operations (including mixed types and nested unions), arithmetic, comparison, membership, collection functions, filtering operations, and string manipulation
- **Edge case coverage**: Out-of-bounds indexing, empty collection handling, single-element array preservation, union with empty values, temporal literal parsing validation

### Run Tests

```bash
# Run all tests
cargo test --package fhirpath

# Run specific test categories
cargo test --package fhirpath test_collection_functions -- --nocapture
cargo test --package fhirpath test_filtering_functions_integration -- --nocapture
cargo test --package fhirpath test_array_indexing -- --nocapture
cargo test --package fhirpath test_union_operations -- --nocapture
cargo test --package fhirpath strings -- --nocapture
cargo test --package fhirpath --test string_integration_test
cargo test --package fhirpath --test comprehensive_string_test

# Run with output
cargo test --package fhirpath test_parser_examples -- --nocapture
```

## Contributing

Contributions are welcome!

### Development Guidelines

Follow the workspace conventions:
- Use `cargo fmt` before committing
- Run `cargo clippy` to check for issues
- Write comprehensive tests for new features
- Update documentation for API changes
- Follow conventional commit message format

## References

- [FHIRPath Specification](http://hl7.org/fhirpath/)
- [FHIRPath Grammar](https://build.fhir.org/ig/HL7/FHIRPath/fhirpath.g4)
- [FHIR R4 Documentation](http://hl7.org/fhir/R4/)

## License

This project is licensed under the MIT OR Apache-2.0 license.
