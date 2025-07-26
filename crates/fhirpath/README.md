# FHIRPath Parser and Evaluator

A comprehensive Rust implementation of a FHIRPath expression parser and evaluator for FHIR resources.

## Overview

FHIRPath is a path-based navigation and extraction language for FHIR resources, defined by the HL7 FHIR specification. This crate provides:

- **Parser**: Converts FHIRPath expressions into an Abstract Syntax Tree (AST)
- **Evaluator**: Comprehensive evaluation of FHIRPath expressions against FHIR resources
- **AST**: Type-safe representation of FHIRPath expressions with full serialization support

**Implementation Status**: 🚧 **Active Development** - Core parsing complete, arithmetic, comparison, membership operations, collection functions, filtering functions, and string functions implemented, comprehensive evaluation functional

## Architecture

The FHIRPath implementation consists of four main components:

### 1. Parser (`src/parser.rs`)
- **Technology**: nom parser combinators
- **Status**: ✅ Complete for core syntax
- **Features**: Converts FHIRPath string expressions into type-safe AST
- **Dependencies**: Uses the `nom` crate for parsing without build-time code generation

### 2. Abstract Syntax Tree (`src/ast.rs`)
- **Status**: ✅ Complete
- **Features**: Type-safe representation of all FHIRPath expression types
- **Serialization**: Full serde support for JSON serialization

### 3. Evaluator (`src/evaluator.rs`)
- **Status**: ⏳ Expanding implementation
- **Features**: Evaluates expressions against JSON FHIR resources including arithmetic, comparisons, membership, collection functions, filtering functions, and string manipulation functions

### 4. Error Handling (`src/error.rs`)
- **Status**: ✅ Complete
- **Features**: Comprehensive error types with context information

## Features

### ✅ Fully Implemented

#### Expression Types
- **Term expressions**: Simple literals and identifiers
- **Member invocation**: `Patient.name`, `name.given`
- **Array indexing**: `Patient.name[0]`, `telecom[1].value`
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
- **String concatenation**: `'Hello' & ' World'` → `"Hello World"`
- **Proper precedence**: `2 + 3 * 4` → `Integer(14)` (multiplication first)
- **Division semantics**: `/` always returns Number, `div` returns Integer
- **Error handling**: Division by zero, invalid type combinations

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

### ⏳ Partially Implemented

#### Basic Evaluation
- ✅ Literal evaluation
- ✅ Simple member access on JSON objects
- ✅ Basic logical operations
- ✅ Arithmetic operations with proper precedence
- ✅ Comparison operations with type safety
- ✅ Membership operations with collection semantics
- ✅ Collection functions (empty, exists, count, distinct, isDistinct)
- ✅ Filtering functions (where, select)
- ✅ String functions (length, substring, indexOf, replace, startsWith, endsWith, upper, lower, trim, split, join, matches)
- ✅ String concatenation and type conversion
- ❌ Complex path navigation
- ❌ Advanced function execution (repeat, etc.)
- ❌ Type coercion

### ❌ Not Yet Implemented

#### Advanced Expression Types
- **Type operations**: `is`, `as`
- **Polarity operations**: `-value`, `+value`
- **Implies operation**: `condition implies action`

#### Advanced Literals
- **Date literals**: `@2023-01-01`
- **DateTime literals**: `@2023-01-01T12:00:00Z`
- **Time literals**: `@T12:00:00`
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
        "given": ["John"],
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

// Collection operations
let expr = parser.parse("name.count()").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
// result: FhirPathValue::Integer(1)

// Filtering
let expr = parser.parse("name.where(use = 'official').given").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
// result: FhirPathValue::Collection([FhirPathValue::String("John")])
```

### Working Examples

```rust
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

// Function chaining
parser.parse("'  HELLO  '.trim().lower()").unwrap(); // ✅ → "hello"
parser.parse("'a,b,c'.split(',').join('|')").unwrap(); // ✅ → "a|b|c"

// Logical operations
parser.parse("active and name.exists()").unwrap(); // ✅ Works

// Union operations
parser.parse("name.given | name.family").unwrap(); // ✅ Works
```

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

- **Unit tests**: 27 tests covering parser and evaluator
- **Integration tests**: 15+ real-world usage examples including arithmetic, comparisons, membership, collection functions, filtering functions, and string manipulation
- **Parser coverage**: All core syntax elements parse successfully including collection, filtering, and string function calls
- **Evaluator coverage**: Literals, member access, arithmetic, comparison, membership, collection functions, filtering operations, and string manipulation

### Run Tests

```bash
# Run all tests
cargo test --package fhirpath

# Run specific test categories
cargo test --package fhirpath test_collection_functions -- --nocapture
cargo test --package fhirpath test_filtering_functions_integration -- --nocapture
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
- [x] Basic logical operations
- [x] Arithmetic operations with proper precedence
- [x] String concatenation and type conversion
- [ ] Array indexing evaluation
- [ ] Union operation evaluation

### Phase 3: Advanced Parsing (❌ Not Started)
- [ ] Date/time literals
- [ ] Quantity literals
- [ ] Type operators

### Phase 4: Function Implementation (⏳ Collection, filtering, and string functions complete)
- [x] Collection functions (exists, count, empty, distinct, isDistinct)
- [x] Filtering functions (where, select)
- [x] String functions (length, substring, indexOf, replace, startsWith, endsWith, upper, lower, trim, split, join, matches)
- [ ] Math functions (abs, round, etc.)
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
2. **Date/Time Support**: Parsing and evaluation of temporal literals
3. **Performance**: Optimization of parser and evaluator
4. **Error Messages**: Better error reporting with suggestions
5. **FHIR Integration**: Better integration with generated FHIR types

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
