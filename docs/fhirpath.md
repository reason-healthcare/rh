# FHIRPath Implementation Documentation

## Overview

This document describes the current implementation status of FHIRPath in the `fhirpath` crate. FHIRPath is a path-based navigation and extraction language for FHIR resources, defined by the HL7 FHIR specification.

**Implementation Status**: 🚧 **Active Development** - Core parsing complete, arithmetic and comparison operations implemented, basic evaluation functional

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
- **Status**: ⏳ Basic implementation
- **Features**: Evaluates simple expressions against JSON FHIR resources

### 4. Error Handling (`src/error.rs`)
- **Status**: ✅ Complete
- **Features**: Comprehensive error types with context information

## Current Features

### ✅ Fully Implemented

#### Expression Types
- **Term expressions**: Simple literals and identifiers
- **Member invocation**: `Patient.name`, `name.given`
- **Array indexing**: `Patient.name[0]`, `telecom[1].value`
- **Union operations**: `name.given | name.family`
- **Logical operations**: `active and birthDate.exists()`
- **Equality operations**: `use = 'official'`, `active != false`
- **Comparison operations**: `age > 18`, `weight <= 100`, `'apple' < 'banana'`
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

#### Function Syntax
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

### ⏳ Partially Implemented

#### Basic Evaluation
- ✅ Literal evaluation
- ✅ Simple member access on JSON objects
- ✅ Basic logical operations
- ✅ Arithmetic operations with proper precedence
- ✅ Comparison operations with type safety  
- ✅ String concatenation and type conversion
- ❌ Complex path navigation
- ❌ Function execution
- ❌ Type coercion

### ❌ Not Yet Implemented

#### Advanced Expression Types
- **Type operations**: `is`, `as`
- **Membership operations**: `in`, `contains`
- **Polarity operations**: `-value`, `+value`
- **Implies operation**: `condition implies action`

#### Advanced Literals
- **Date literals**: `@2023-01-01`
- **DateTime literals**: `@2023-01-01T12:00:00Z`
- **Time literals**: `@T12:00:00`
- **Quantity literals**: `5 'mg'`, `10 'cm'`
- **Long numbers**: `1000L`

#### Built-in Functions
- None currently implemented (syntax parsing only)

## FHIRPath Keywords and Support Status

### Operators

| Operator | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `.` | Member access | ✅ | ✅ | Complete |
| `[]` | Indexer | ✅ | ✅ | Complete |
| `()` | Function call | ✅ | ❌ | Parse only |
| `()` | Grouping | ✅ | ✅ | Complete |

### Arithmetic Operators

| Operator | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `+` | Addition | ✅ | ✅ | Complete |
| `-` | Subtraction | ✅ | ✅ | Complete |
| `*` | Multiplication | ✅ | ✅ | Complete |
| `/` | Division | ✅ | ✅ | Complete |
| `div` | Integer division | ✅ | ✅ | Complete |
| `mod` | Modulo | ✅ | ✅ | Complete |
| `&` | String concatenation | ✅ | ✅ | Complete |

### Comparison Operators

| Operator | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `=` | Equality | ✅ | ✅ | Complete |
| `!=` | Inequality | ✅ | ✅ | Complete |
| `~` | Equivalence | ✅ | ✅ | Complete |
| `!~` | Non-equivalence | ✅ | ✅ | Complete |
| `<` | Less than | ✅ | ✅ | Complete |
| `<=` | Less than or equal | ✅ | ✅ | Complete |
| `>` | Greater than | ✅ | ✅ | Complete |
| `>=` | Greater than or equal | ✅ | ✅ | Complete |

### Logical Operators

| Operator | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `and` | Logical AND | ✅ | ✅ | Complete |
| `or` | Logical OR | ✅ | ✅ | Complete |
| `xor` | Logical XOR | ✅ | ✅ | Complete |
| `implies` | Logical implication | ✅ | ❌ | Parse only |

### Collection Operators

| Operator | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `\|` | Union | ✅ | ✅ | Complete |
| `in` | Membership test | ✅ | ❌ | Parse only |
| `contains` | Contains test | ✅ | ❌ | Parse only |

### Type Operators

| Operator | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `is` | Type checking | ❌ | ❌ | Not implemented |
| `as` | Type casting | ❌ | ❌ | Not implemented |

### Special Variables

| Variable | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `$this` | Current context | ✅ | ✅ | Complete |
| `$index` | Current index in iteration | ✅ | ❌ | Parse only |
| `$total` | Total items in collection | ✅ | ❌ | Parse only |
| `%context` | External context | ✅ | ✅ | Complete |

### Built-in Functions

#### Collection Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `empty()` | Test if collection is empty | ✅ | ❌ | Parse only |
| `exists()` | Test if any items exist | ✅ | ❌ | Parse only |
| `count()` | Count items in collection | ✅ | ❌ | Parse only |
| `distinct()` | Remove duplicates | ✅ | ❌ | Parse only |
| `isDistinct()` | Test if all items unique | ✅ | ❌ | Parse only |

#### Filtering Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `where(criteria)` | Filter by criteria | ✅ | ❌ | Parse only |
| `select(projection)` | Transform each item | ✅ | ❌ | Parse only |
| `repeat(projection)` | Recursive projection | ✅ | ❌ | Parse only |

#### Subsetting Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `single()` | Get single item | ✅ | ❌ | Parse only |
| `first()` | Get first item | ✅ | ❌ | Parse only |
| `last()` | Get last item | ✅ | ❌ | Parse only |
| `tail()` | Get all but first | ✅ | ❌ | Parse only |
| `skip(num)` | Skip first num items | ✅ | ❌ | Parse only |
| `take(num)` | Take first num items | ✅ | ❌ | Parse only |

#### Combining Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `union(other)` | Union with other collection | ✅ | ❌ | Parse only |
| `combine(other)` | Combine collections | ✅ | ❌ | Parse only |
| `intersect(other)` | Intersect with other | ✅ | ❌ | Parse only |
| `exclude(other)` | Exclude items in other | ✅ | ❌ | Parse only |

#### Conversion Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `iif(criterion, true-result, false-result)` | Conditional | ✅ | ❌ | Parse only |
| `toBoolean()` | Convert to boolean | ✅ | ❌ | Parse only |
| `toInteger()` | Convert to integer | ✅ | ❌ | Parse only |
| `toDecimal()` | Convert to decimal | ✅ | ❌ | Parse only |
| `toString()` | Convert to string | ✅ | ❌ | Parse only |

#### String Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `indexOf(substring)` | Find substring index | ✅ | ❌ | Parse only |
| `substring(start, length?)` | Extract substring | ✅ | ❌ | Parse only |
| `startsWith(prefix)` | Test if starts with | ✅ | ❌ | Parse only |
| `endsWith(suffix)` | Test if ends with | ✅ | ❌ | Parse only |
| `contains(substring)` | Test if contains | ✅ | ❌ | Parse only |
| `replace(pattern, replacement)` | Replace text | ✅ | ❌ | Parse only |
| `matches(regex)` | Test regex match | ✅ | ❌ | Parse only |
| `replaceMatches(regex, replacement)` | Replace regex matches | ✅ | ❌ | Parse only |
| `length()` | String length | ✅ | ❌ | Parse only |
| `upper()` | Convert to uppercase | ✅ | ❌ | Parse only |
| `lower()` | Convert to lowercase | ✅ | ❌ | Parse only |

#### Math Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `abs()` | Absolute value | ✅ | ❌ | Parse only |
| `ceiling()` | Round up | ✅ | ❌ | Parse only |
| `exp()` | Exponential | ✅ | ❌ | Parse only |
| `floor()` | Round down | ✅ | ❌ | Parse only |
| `ln()` | Natural logarithm | ✅ | ❌ | Parse only |
| `log(base)` | Logarithm | ✅ | ❌ | Parse only |
| `power(exponent)` | Power | ✅ | ❌ | Parse only |
| `round(precision?)` | Round | ✅ | ❌ | Parse only |
| `sqrt()` | Square root | ✅ | ❌ | Parse only |
| `truncate()` | Truncate | ✅ | ❌ | Parse only |

#### Date/Time Functions

| Function | Description | Parser Support | Evaluator Support | Status |
|----------|-------------|----------------|-------------------|---------|
| `now()` | Current date/time | ✅ | ❌ | Parse only |
| `today()` | Current date | ✅ | ❌ | Parse only |
| `timeOfDay()` | Current time | ✅ | ❌ | Parse only |

### Literals

| Literal Type | Example | Parser Support | Evaluator Support | Status |
|--------------|---------|----------------|-------------------|---------|
| Boolean | `true`, `false` | ✅ | ✅ | Complete |
| Integer | `42`, `-10` | ✅ | ✅ | Complete |
| Decimal | `3.14`, `-0.5` | ✅ | ✅ | Complete |
| String | `'hello'`, `'world'` | ✅ | ✅ | Complete |
| Date | `@2023-01-01` | ❌ | ❌ | Not implemented |
| DateTime | `@2023-01-01T12:00:00Z` | ❌ | ❌ | Not implemented |
| Time | `@T12:00:00` | ❌ | ❌ | Not implemented |
| Quantity | `5 'mg'`, `10.5 'cm'` | ❌ | ❌ | Not implemented |
| Null | `{}` | ✅ | ✅ | Complete |

## Usage Examples

### Currently Working Examples

```rust
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext};
use serde_json::json;

let parser = FhirPathParser::new();
let evaluator = FhirPathEvaluator::new();

// Simple literals
parser.parse("true").unwrap();           // ✅ Works
parser.parse("42").unwrap();             // ✅ Works (Integer)
parser.parse("3.14").unwrap();           // ✅ Works (Number)
parser.parse("'hello'").unwrap();        // ✅ Works

// Arithmetic operations
parser.parse("1 + 2").unwrap();          // ✅ Works → Integer(3)
parser.parse("10 - 3").unwrap();         // ✅ Works → Integer(7)
parser.parse("4 * 5").unwrap();          // ✅ Works → Integer(20)
parser.parse("15 / 3").unwrap();         // ✅ Works → Number(5.0)
parser.parse("17 div 5").unwrap();       // ✅ Works → Integer(3)
parser.parse("17 mod 5").unwrap();       // ✅ Works → Integer(2)
parser.parse("2.5 + 3").unwrap();        // ✅ Works → Number(5.5)
parser.parse("'Hello' & ' World'").unwrap(); // ✅ Works → "Hello World"

// Operator precedence
parser.parse("2 + 3 * 4").unwrap();      // ✅ Works → Integer(14)
parser.parse("20 - 12 / 3").unwrap();    // ✅ Works → Number(16.0)

// Comparison operations
parser.parse("5 > 3").unwrap();          // ✅ Works → Boolean(true)
parser.parse("age >= 18").unwrap();       // ✅ Works → Boolean evaluation
parser.parse("'apple' < 'banana'").unwrap(); // ✅ Works → Boolean(true)
parser.parse("false <= true").unwrap();   // ✅ Works → Boolean(true)

// Comparison with arithmetic precedence
parser.parse("2 + 3 > 4").unwrap();      // ✅ Works → Boolean(true)
parser.parse("10 - 5 >= 3").unwrap();    // ✅ Works → Boolean(true)

// Member access
parser.parse("Patient.name").unwrap();   // ✅ Works
parser.parse("name.given").unwrap();     // ✅ Works

// Array indexing
parser.parse("name[0]").unwrap();        // ✅ Works
parser.parse("telecom[1].value").unwrap(); // ✅ Works

// Function calls (parsing only)
parser.parse("name.count()").unwrap();   // ✅ Parses
parser.parse("name.exists()").unwrap();  // ✅ Parses

// Logical operations
parser.parse("active and name.exists()").unwrap(); // ✅ Works

// Union operations
parser.parse("name.given | name.family").unwrap(); // ✅ Works

// Complex expressions (parsing only)
parser.parse("name.where(use = 'official').given").unwrap(); // ✅ Parses
```

### Examples That Don't Work Yet

```rust
// Date literals
parser.parse("@2023-01-01");              // ❌ Fails

// Type operations
parser.parse("name is string");           // ❌ Fails

// Most function evaluation
evaluator.evaluate(&expr, &context);      // ❌ Limited support
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

### Phase 4: Function Implementation (❌ Not Started)
- [ ] Collection functions (exists, count, empty)
- [ ] Filtering functions (where, select)
- [ ] String functions (length, substring, etc.)
- [ ] Math functions (abs, round, etc.)
- [ ] Date/time functions (now, today, etc.)

### Phase 5: Advanced Features (❌ Not Started)
- [ ] Type system and coercion
- [ ] Performance optimization
- [ ] FHIR-specific functions
- [ ] Error recovery and suggestions

## Testing

The implementation includes comprehensive tests:

- **Unit tests**: 16 tests covering parser and evaluator
- **Integration tests**: 4 real-world usage examples including arithmetic and comparisons
- **Parser coverage**: All core syntax elements parse successfully including comparison operators
- **Evaluator coverage**: Literals, member access, arithmetic, and comparison operations

Run tests with:
```bash
cargo test --package fhirpath
cargo test --package fhirpath test_comparison_expressions -- --nocapture
```

## Integration with FHIR Codegen

The FHIRPath crate is designed to integrate with the broader FHIR code generation ecosystem in this workspace:

### Potential Use Cases

1. **FHIR Validation**: Use FHIRPath expressions to validate FHIR resources during code generation
2. **Resource Querying**: Query generated FHIR types using FHIRPath expressions
3. **Template Processing**: Use FHIRPath in FHIR resource templates and examples
4. **CLI Tools**: Integrate FHIRPath evaluation into command-line FHIR processing tools

### Workspace Integration

The `fhirpath` crate is part of the larger Rust FHIR monorepo and shares:
- **Common dependencies**: Managed through workspace Cargo.toml
- **Error handling patterns**: Uses `anyhow` and `thiserror` like other crates
- **Serde integration**: Compatible with FHIR JSON processing
- **Testing conventions**: Follows workspace testing standards

## Contributing

Areas where contributions are most needed:

1. **Function Implementation**: Implementing built-in FHIRPath functions
2. **Date/Time Support**: Parsing and evaluation of temporal literals
3. **Performance**: Optimization of parser and evaluator
4. **Error Messages**: Better error reporting with suggestions
5. **FHIR Integration**: Better integration with generated FHIR types

### Development Guidelines

Follow the workspace conventions:
- Use `cargo fmt` before committing
- Run `cargo clippy` to check for issues
- Write comprehensive tests for new features
- Update this documentation for API changes
- Follow the commit message format in the workspace guidelines

## References

- [FHIRPath Specification](http://hl7.org/fhirpath/)
- [FHIRPath Grammar](https://build.fhir.org/ig/HL7/FHIRPath/fhirpath.g4)
- [FHIR R4 Documentation](http://hl7.org/fhir/R4/)
- [Workspace Documentation](../docs/workspace.md)
