# FHIRPath Parser and Evaluator

A comprehensive Rust implementation of a FHIRPath expression parser and evaluator for FHIR resources.

- **Complete FHIRPath Support**: Parse and evaluate FHIRPath expressions with quantity and temporal literal support
- **Mathematical Operations**: Arithmetic, comparison, and logical operators with quantity support
- **String Functions**: Comprehensive string manipulation capabilities
- **Collection Operations**: Work with FHIR collections and lists including subsetting functions
- **Temporal Literals**: Support for date (@2023-01-01), datetime (@2023-01-01T12:30:45), and time (@T12:30:45) literals
- **Quantity Literals**: Support for UCUM units (5'mg', 37'Cel') and calendar durations (2'wk', 6'mo') with automatic unit conversion
- **Unit Conversion**: Automatic conversion between compatible units (kg↔g, L↔mL, h↔min) during arithmetic operations
- **Type Safety**: Rust-native type checking and error handling

## Overview

FHIRPath is a path-based navigation and extraction language for FHIR resources, defined by the HL7 FHIR specification. This crate provides:

- **Parser**: Converts FHIRPath expressions into an Abstract Syntax Tree (AST)
- **Evaluator**: Comprehensive evaluation of FHIRPath expressions against FHIR resources
- **AST**: Type-safe representation of FHIRPath expressions with full serialization support

**Implementation Status**: 🚧 **Active Development** - Core parsing complete, arithmetic, comparison, membership operations, collection functions, filtering functions, string functions, **math functions**, **array indexing with full nested support**, and **union operations** implemented, comprehensive evaluation functional

## Architecture

The FHIRPath implementation consists of four main components:

### 1. Parser (`src/parser.rs`)
- **Technology**: nom parser combinators
- **Status**: ✅ Complete for core syntax
- **Features**: Converts FHIRPath string expressions into type-safe AST including temporal literals (date, datetime, time)
- **Dependencies**: Uses the `nom` crate for parsing without build-time code generation

### 2. Abstract Syntax Tree (`src/ast.rs`)
- **Status**: ✅ Complete
- **Features**: Type-safe representation of all FHIRPath expression types
- **Serialization**: Full serde support for JSON serialization

### 3. Evaluator (`src/evaluator.rs`)
- **Status**: ⏳ Expanding implementation
- **Features**: Evaluates expressions against JSON FHIR resources including arithmetic, comparisons, membership, collection functions, filtering functions, string manipulation functions, and mathematical functions

### 4. Error Handling (`src/error.rs`)
- **Status**: ✅ Complete
- **Features**: Comprehensive error types with context information

## Features

### ✅ Fully Implemented

#### Expression Types
- **Term expressions**: Simple literals and identifiers
- **Member invocation**: `Patient.name`, `name.given`
- **Array indexing**: `Patient.name[0]`, `telecom[1].value`, `name[0].given[0]` (with full nested indexing support)
- **Union operations**: `name.given | name.family`
- **Logical operations**: `active and birthDate.exists()`
- **Equality operations**: `use = 'official'`, `active != false`
- **Comparison operations**: `age > 18`, `weight <= 100`, `'apple' < 'banana'`
- **Membership operations**: `value in collection`, `list contains item`
- **Arithmetic operations**: `1 + 2`, `age * 2`, `'Hello' & ' World'`
- **Parenthesized expressions**: `(name.given | name.family).exists()`

#### Literal Values
- **Boolean**: `true`, `false`
- **Integers**: `42`, `-10` (distinct from decimals)
- **Numbers**: `3.14`, `-0.5` (floating-point)
- **Strings**: `'hello world'`, `'patient name'`
- **Date**: `@2023-01-01`, `@1990-12-25` (ISO 8601 date format)
- **DateTime**: `@2023-01-01T12:30:45`, `@2023-01-01T00:00:00Z`, `@2023-01-01T12:30:45+05:30` (ISO 8601 datetime format with optional timezone)
- **Time**: `@T12:30:45`, `@T00:00:00`, `@T23:59:59` (ISO 8601 time format)
- **Quantity**: `5'mg'`, `37.2'Cel'`, `120'mm[Hg]'`, `2'wk'` (value with UCUM unit or calendar duration)
- **Null**: `{}`

#### Special Variables
- **Context reference**: `$this`
- **Index reference**: `$index`
- **Total reference**: `$total`
- **External constants**: `%context`, `%resource`

#### Function Support
- **Function calls**: `name.count()`, `telecom.exists()`
- **Parameterized functions**: `name.where(use = 'official')`

#### Arithmetic Operations
- **Integer operations**: `1 + 2` → `Integer(3)`, `5 * 6` → `Integer(30)`
- **Mixed type operations**: `2.5 + 3` → `Number(5.5)`, `10 / 4` → `Number(2.5)`
- **Quantity operations**: `5'mg' + 3'mg'` → `8mg`, `10'kg' * 2` → `20kg`, `120'mm[Hg]' / 60'mm[Hg]'` → `2.0`
- **Unit conversion**: `1.0'kg' + 500.0'g'` → `1.5kg`, `2.0'L' + 250.0'mL'` → `2.25L` (automatic conversion)
- **String concatenation**: `'Hello' & ' World'` → `"Hello World"`
- **Proper precedence**: `2 + 3 * 4` → `Integer(14)` (multiplication first)
- **Division semantics**: `/` always returns Number, `div` returns Integer
- **Unit compatibility**: Compatible units automatically converted, scalar operations preserve units
- **Error handling**: Division by zero, invalid type combinations, incompatible units

#### Comparison Operations
- **Numeric comparisons**: `5 > 3` → `Boolean(true)`, `age >= 18` → evaluation
- **String comparisons**: `'apple' < 'banana'` → `Boolean(true)` (lexicographic)
- **Boolean comparisons**: `false < true` → `Boolean(true)` (false < true)
- **Mixed numeric types**: `5 > 4.9` → `Boolean(true)`, `4.99 <= 5` → `Boolean(true)`
- **Proper precedence**: `2 + 3 > 4` → `Boolean(true)` (arithmetic first)
- **Type safety**: Invalid comparisons properly rejected with clear errors

#### Membership Operations
- **Value membership**: `'apple' in fruits` → `Boolean(true)` if fruits contains 'apple'
- **Collection contains**: `fruits contains 'apple'` → `Boolean(true)` if fruits contains 'apple'
- **Single value membership**: `value in 'value'` → `Boolean(true)` (single values treated as collections)
- **Type safety**: Membership tests use equality comparison semantics
- **Proper precedence**: Same precedence as equality operators, left-associative

#### Collection Functions
- **empty()**: Test if collection is empty
- **exists()**: Test if any items exist
- **count()**: Count items in collection
- **distinct()**: Remove duplicates
- **isDistinct()**: Test if all items unique

#### Subsetting Functions
- **single()**: Return the single item in a collection (error if not exactly one item)
- **first()**: Return the first item in a collection
- **last()**: Return the last item in a collection
- **tail()**: Return all items except the first
- **skip(num)**: Skip the first `num` items and return the rest
- **take(num)**: Return the first `num` items
- **intersect(other)**: Return items that appear in both collections
- **exclude(other)**: Return items that don't appear in the other collection

#### Filtering Functions
- **where(criteria)**: Filter collection by criteria
- **select(projection)**: Transform each item in collection

#### String Functions
- **length()**: Get string length
- **substring(start, length?)**: Extract substring
- **startsWith(prefix)**: Test if string starts with prefix
- **endsWith(suffix)**: Test if string ends with suffix
- **indexOf(substring)**: Find index of substring (returns -1 if not found)
- **replace(pattern, replacement)**: Replace all occurrences
- **upper()**: Convert to uppercase
- **lower()**: Convert to lowercase
- **trim()**: Remove leading/trailing whitespace
- **split(delimiter)**: Split string into collection
- **join(delimiter)**: Join collection with delimiter
- **matches(pattern)**: Test if string matches pattern

#### Math Functions
- **abs()**: Absolute value of number
- **ceiling()**: Round up to nearest integer
- **exp()**: e raised to the power of number (e^x)
- **floor()**: Round down to nearest integer
- **ln()**: Natural logarithm (base e)
- **log(base)**: Logarithm with specified base
- **power(exponent)**: Raise number to specified power
- **round(precision?)**: Round to specified decimal places (default 0)
- **sqrt()**: Square root of number
- **truncate()**: Remove fractional part (toward zero)

### ⏳ Partially Implemented

#### Basic Evaluation
- ✅ Literal evaluation (including temporal literals)
- ✅ Simple member access on JSON objects
- ✅ Array indexing with bounds checking and nested indexing support
- ✅ Basic logical operations
- ✅ Arithmetic operations with proper precedence
- ✅ Comparison operations with type safety
- ✅ Membership operations with collection semantics
- ✅ Collection functions (empty, exists, count, distinct, isDistinct)
- ✅ Subsetting functions (single, first, last, tail, skip, take, intersect, exclude)
- ✅ Filtering functions (where, select)
- ✅ String functions (length, substring, indexOf, replace, startsWith, endsWith, upper, lower, trim, split, join, matches)
- ✅ Math functions (abs, ceiling, exp, floor, ln, log, power, round, sqrt, truncate)
- ✅ Polarity operations (-, +)
- ✅ String concatenation and type conversion
- ❌ Complex path navigation
- ❌ Advanced function execution (repeat, etc.)
- ❌ Type coercion

### ❌ Not Yet Implemented

#### Advanced Expression Types
- **Type operations**: `is`, `as`
- **Implies operation**: `condition implies action`

#### Advanced Literals
- **Quantity literals**: `5 'mg'`, `10 'cm'`
- **Long numbers**: `1000L`

## FHIRPath Operator Support

### Operators

| Operator | Description | Parser | Evaluator | Status |
|----------|-------------|---------|-----------|---------|
| `.` | Member access | ✅ | ✅ | Complete |
| `[]` | Indexer | ✅ | ✅ | Complete |
| `()` | Function call | ✅ | ✅ | Complete |
| `()` | Grouping | ✅ | ✅ | Complete |

### Arithmetic Operators

| Operator | Description | Parser | Evaluator | Status |
|----------|-------------|---------|-----------|---------|
| `+` | Addition | ✅ | ✅ | Complete |
| `-` | Subtraction | ✅ | ✅ | Complete |
| `*` | Multiplication | ✅ | ✅ | Complete |
| `/` | Division | ✅ | ✅ | Complete |
| `div` | Integer division | ✅ | ✅ | Complete |
| `mod` | Modulo | ✅ | ✅ | Complete |
| `&` | String concatenation | ✅ | ✅ | Complete |

### Comparison Operators

| Operator | Description | Parser | Evaluator | Status |
|----------|-------------|---------|-----------|---------|
| `=` | Equality | ✅ | ✅ | Complete |
| `!=` | Inequality | ✅ | ✅ | Complete |
| `~` | Equivalence | ✅ | ✅ | Complete |
| `!~` | Non-equivalence | ✅ | ✅ | Complete |
| `<` | Less than | ✅ | ✅ | Complete |
| `<=` | Less than or equal | ✅ | ✅ | Complete |
| `>` | Greater than | ✅ | ✅ | Complete |
| `>=` | Greater than or equal | ✅ | ✅ | Complete |

### Logical Operators

| Operator | Description | Parser | Evaluator | Status |
|----------|-------------|---------|-----------|---------|
| `and` | Logical AND | ✅ | ✅ | Complete |
| `or` | Logical OR | ✅ | ✅ | Complete |
| `xor` | Logical XOR | ✅ | ✅ | Complete |
| `implies` | Logical implication | ✅ | ❌ | Parse only |

### Collection Operators

| Operator | Description | Parser | Evaluator | Status |
|----------|-------------|---------|-----------|---------|
| `\|` | Union | ✅ | ✅ | Complete |
| `in` | Membership test | ✅ | ✅ | Complete |
| `contains` | Contains test | ✅ | ✅ | Complete |

## Usage

### Basic Parsing

```rust
use fhirpath::FhirPathParser;

let parser = FhirPathParser::new();

// Simple expressions
let expr = parser.parse("Patient.name.given").unwrap();
let expr = parser.parse("name.where(use = 'official')").unwrap();
let expr = parser.parse("age > 18 and active = true").unwrap();
```

### Expression Evaluation

```rust
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext, FhirPathValue};
use serde_json::json;

let patient = json!({
    "resourceType": "Patient",
    "active": true,
    "name": [{
        "use": "official",
        "given": ["John", "James"],
        "family": "Doe"
    }, {
        "use": "usual",
        "given": ["Johnny"],
        "family": "Doe"
    }],
    "birthDate": "1974-12-25"
});

let parser = FhirPathParser::new();
let evaluator = FhirPathEvaluator::new();
let context = EvaluationContext::new(patient);

// Simple member access
let expr = parser.parse("resourceType").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
// result: FhirPathValue::String("Patient")

// Array indexing
let expr = parser.parse("name[0].family").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
// result: FhirPathValue::String("Doe")

// Nested array indexing
let expr = parser.parse("name[0].given[1]").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
// result: FhirPathValue::String("James")

// Collection operations
let expr = parser.parse("name.count()").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
// result: FhirPathValue::Integer(2)

// Filtering with indexing
let expr = parser.parse("name.where(use = 'official')[0].given[0]").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
// result: FhirPathValue::String("John")
```

### Working Examples

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

// Function chaining
parser.parse("'  HELLO  '.trim().lower()").unwrap(); // ✅ → "hello"
parser.parse("'a,b,c'.split(',').join('|')").unwrap(); // ✅ → "a|b|c"

// Logical operations
parser.parse("active and name.exists()").unwrap(); // ✅ Works

// Union operations
parser.parse("name.given | name.family").unwrap(); // ✅ Works

// Complex union operations
parser.parse("(1 | 2 | 3)").unwrap();          // ✅ → Collection([1, 2, 3])
parser.parse("(42 | 'hello' | true)").unwrap(); // ✅ → Mixed type collection
parser.parse("((1 | 2) | (3 | 4))").unwrap();   // ✅ → Nested union flattening
parser.parse("(10 | 20 | 30)[1]").unwrap();     // ✅ → Integer(20) (union + indexing)

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
parser.parse("37.2'Cel'").unwrap();                   // ✅ → Temperature quantity
parser.parse("2'wk'").unwrap();                       // ✅ → Calendar duration
parser.parse("5'mg' + 3'mg'").unwrap();               // ✅ → 8mg (same units)
parser.parse("10'kg' * 2").unwrap();                  // ✅ → 20kg (scalar multiplication)
parser.parse("120'mm[Hg]' / 60'mm[Hg]'").unwrap();   // ✅ → 2.0 (dimensionless ratio)

// Unit conversion examples
parser.parse("1.0'kg' + 500.0'g'").unwrap();         // ✅ → 1.5kg (automatic conversion)
parser.parse("2.0'L' + 250.0'mL'").unwrap();         // ✅ → 2.25L (mL→L conversion)
parser.parse("1.0'h' + 30.0'min'").unwrap();         // ✅ → 1.5h (min→h conversion)

// Empty collection indexing
parser.parse("{}[0]").unwrap(); // ✅ → Empty (graceful handling)
```

## Unit Conversion System

The FHIRPath library includes a comprehensive unit conversion system that automatically handles conversions between compatible UCUM units during arithmetic operations. This enables seamless calculations between different units of the same physical quantity.

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

### Unit Conversion Examples

```rust
// Addition and Subtraction with automatic conversion
parser.parse("1.0'kg' + 500.0'g'").unwrap();      // → 1.5 kg (500g → 0.5kg)
parser.parse("2.0'L' + 250.0'mL'").unwrap();      // → 2.25 L (250mL → 0.25L)
parser.parse("1.0'h' + 30.0'min'").unwrap();      // → 1.5 h (30min → 0.5h)
parser.parse("1.5'm' + 50.0'cm'").unwrap();       // → 2.0 m (50cm → 0.5m)

// Multiplication and Division by scalars
parser.parse("5.0'mg' * 3.0").unwrap();           // → 15.0 mg
parser.parse("100.0'mL' / 4.0").unwrap();         // → 25.0 mL

// Division of compatible quantities (dimensionless result)
parser.parse("10.0'kg' / 2.0'kg'").unwrap();      // → 5.0 (dimensionless)
parser.parse("1.0'kg' / 500.0'g'").unwrap();      // → 2.0 (1kg = 1000g, 1000/500 = 2)

// Error cases - incompatible units
parser.parse("1.0'kg' + 1.0'm'").unwrap();        // → Error: incompatible units
```

### Conversion Process

The unit conversion system follows this process:

1. **Compatibility Check**: Verify both quantities are of the same physical type (mass, length, volume, etc.)
2. **Base Unit Conversion**: Convert both values to their respective base units
3. **Arithmetic Operation**: Perform the operation on base unit values  
4. **Result Conversion**: Convert back to the left operand's unit system

### Base Units by Category
- **Mass**: gram (g)
- **Length**: meter (m) 
- **Volume**: liter (L)
- **Time**: second (s)
- **Pressure**: pascal (Pa)

### Error Handling

The system provides clear error messages for:
- **Incompatible units**: `Cannot add quantities with incompatible units: Some("kg") and Some("m")`
- **Division by zero**: `Division by zero`
- **Unknown units**: `Unknown target unit: xyz`

See `examples/unit_conversion_example.rs` for comprehensive examples of all supported conversions and operations.

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

## Development Roadmap

### Phase 1: Core Parser (✅ Complete)
- [x] Basic expression parsing
- [x] Operator precedence
- [x] Function call syntax
- [x] Literal values
- [x] Error handling

### Phase 2: Basic Evaluation (✅ Complete)
- [x] Literal evaluation
- [x] Simple member access
- [x] Array indexing evaluation with bounds checking and nested support
- [x] Basic logical operations
- [x] Arithmetic operations with proper precedence
- [x] String concatenation and type conversion
- [x] Union operation evaluation

### Phase 3: Advanced Parsing (⏳ Quantity literals complete)
- [x] Date/time literals
- [x] Quantity literals
- [ ] Type operators

### Phase 4: Function Implementation (⏳ Collection, subsetting, filtering, and string functions complete)
- [x] Collection functions (exists, count, empty, distinct, isDistinct)
- [x] Subsetting functions (single, first, last, tail, skip, take, intersect, exclude)
- [x] Filtering functions (where, select)
- [x] String functions (length, substring, indexOf, replace, startsWith, endsWith, upper, lower, trim, split, join, matches)
- [x] Math functions (abs, round, etc.)
- [ ] Date/time functions (now, today, etc.)

### Phase 5: Advanced Features (❌ Not Started)
- [ ] Type system and coercion
- [ ] Performance optimization
- [ ] FHIR-specific functions
- [ ] Error recovery and suggestions

## Integration with FHIR Ecosystem

The FHIRPath crate is designed to integrate with the broader FHIR code generation ecosystem:

### Use Cases

1. **FHIR Validation**: Use FHIRPath expressions to validate FHIR resources
2. **Resource Querying**: Query FHIR data using FHIRPath expressions
3. **Template Processing**: Use FHIRPath in FHIR resource templates
4. **CLI Tools**: Integrate FHIRPath evaluation into command-line tools

### Workspace Integration

- **Common dependencies**: Managed through workspace Cargo.toml
- **Error handling patterns**: Uses `anyhow` and `thiserror`
- **Serde integration**: Compatible with FHIR JSON processing
- **Testing conventions**: Follows workspace testing standards

## Contributing

This implementation covers the core FHIRPath functionality but there are areas where contributions are welcome:

### Priority Areas

1. **Function Implementation**: Math and date/time functions
2. **Performance**: Optimization of parser and evaluator
3. **Error Messages**: Better error reporting with suggestions
4. **FHIR Integration**: Better integration with generated FHIR types
5. **Quantity Literals**: Support for unit-based quantities like `5 'mg'`

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
