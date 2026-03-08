# rh-cql Architecture

This document describes the architectural design of the rh-cql CQL-to-ELM compiler, highlighting key design choices and their benefits.

## Overview

The rh-cql compiler transforms Clinical Quality Language (CQL) source code into Expression Logical Model (ELM) representation. The architecture follows a clean separation between parsing, semantic analysis, and code generation.

```
┌─────────────────────────────────────────────────────────────┐
│                       CQL Source                            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Parser (pest)                           │
│                   parser/cql.pest                           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      CQL AST                                │
│                   parser/ast.rs                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  Semantic Analysis                          │
│    LibraryBuilder + ExpressionTranslator (builder.rs)       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     ELM Output                              │
│                      elm/*.rs                               │
└─────────────────────────────────────────────────────────────┘
```

## Design Principles

### 1. Separation of AST and ELM

The compiler maintains distinct representations for the parsed CQL syntax (AST) and the output format (ELM):

| Layer | Location | Purpose |
|-------|----------|---------|
| **CQL AST** | `parser/ast.rs` | Faithful representation of CQL syntax |
| **ELM** | `elm/*.rs` | HL7 Expression Logical Model output |

**Benefits:**
- **Language Independence**: CQL syntax changes don't affect ELM output structure
- **Multiple Backends**: Architecture supports targeting formats other than ELM (e.g., SQL, bytecode)
- **Clear Responsibilities**: Parser focuses on syntax, translator focuses on semantics
- **Testability**: Each layer can be tested independently

### 2. Immutable Data Structures

All AST and ELM types are immutable. New structures are created rather than mutating existing ones.

```rust
// AST nodes are immutable enums
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp { left: Box<Expression>, op: BinaryOperator, right: Box<Expression> },
    // ...
}

// ELM nodes are built fresh, not mutated
let elm_add = elm::Add {
    operand: vec![left_elm, right_elm],
    ..Default::default()
};
```

**Benefits:**
- **Thread Safety**: No mutation means no data races
- **Predictability**: No spooky action at a distance
- **Debugging**: State doesn't change unexpectedly
- **Rust Idioms**: Leverages Rust's ownership model naturally

### 3. Enum-Based Type Dispatch

Rather than using the visitor pattern common in OOP languages, we use Rust's exhaustive pattern matching on enums.

```rust
// Instead of visitor classes:
fn translate_expression(&mut self, expr: &ast::Expression) -> Result<elm::Expression> {
    match expr {
        ast::Expression::Literal(lit) => self.translate_literal(lit),
        ast::Expression::BinaryOp { left, op, right } => {
            self.translate_binary_op(left, op, right)
        }
        ast::Expression::Query(query) => self.translate_query(query),
        // Compiler ensures all variants are handled
    }
}
```

**Benefits:**
- **Exhaustiveness**: Compiler catches unhandled cases
- **No Boilerplate**: No visitor interface or accept methods needed
- **Locality**: Related code stays together
- **Performance**: No virtual dispatch overhead

### 4. Single-Pass Translation

The compiler performs semantic analysis and ELM generation in a single pass over the AST, rather than building an intermediate typed AST.

```rust
// Type resolution and ELM generation happen together
fn translate_binary_op(&mut self, left: &ast::Expression, op: &BinaryOperator, right: &ast::Expression) 
    -> Result<elm::Expression> 
{
    let left_elm = self.translate_expression(left)?;
    let right_elm = self.translate_expression(right)?;
    
    // Type checking happens here
    let result_type = self.resolve_binary_operator_type(op, &left_elm, &right_elm)?;
    
    // ELM is generated immediately
    Ok(self.build_binary_expression(op, left_elm, right_elm, result_type))
}
```

**Benefits:**
- **Simplicity**: One pass is easier to understand than multiple passes
- **Performance**: No intermediate allocations for typed AST
- **Sufficient**: For ELM generation, we don't need a separate type inference phase

### 5. Builder Pattern for Complex Construction

The `LibraryBuilder` manages compiler state and provides methods for constructing ELM elements:

```rust
impl LibraryBuilder {
    pub fn resolve_call(&mut self, name: &str, args: Vec<elm::Expression>) -> Result<elm::Expression>;
    pub fn add_expression_def(&mut self, def: elm::ExpressionDef);
    pub fn resolve_type(&mut self, type_name: &str) -> Result<DataType>;
    // ...
}
```

**Benefits:**
- **Centralized State**: Symbol tables, type resolution, error collection in one place
- **Encapsulation**: Callers don't need to know internal details
- **Testability**: Builder can be configured for different test scenarios

## Module Structure

```
src/
├── lib.rs              # Public API
├── builder.rs          # LibraryBuilder - orchestrates compilation
├── parser/
│   ├── mod.rs          # Parser entry point
│   ├── cql.pest        # PEG grammar for CQL
│   └── ast.rs          # CQL AST types
├── elm/
│   ├── mod.rs          # ELM module exports
│   ├── expression.rs   # ELM expression types
│   ├── library.rs      # ELM library structure
│   ├── types.rs        # ELM type specifiers
│   └── annotation.rs   # ELM annotations
├── types/
│   └── mod.rs          # Type system (DataType, etc.)
└── system/
    └── mod.rs          # System library operators
```

## Comparison to Reference Implementation

The Java/Kotlin reference CQL translator (cqframework/clinical_quality_language) is undergoing a similar architectural refactor in [PR #1640](https://github.com/cqframework/clinical_quality_language/pull/1640) to introduce an intermediate AST. Key observations:

| Aspect | Reference (Current) | Reference (PR #1640) | rh-cql |
|--------|---------------------|----------------------|--------|
| AST/ELM Separation | Mixed in `Cql2ElmVisitor` | Separate CQL AST | ✅ Already separate |
| Type Inference | During ELM generation | Separate phase | Single pass |
| Visitor Pattern | Class-based visitors | `AstWalker` class | Enum matching |
| Mutability | Mutable ELM nodes | Mutable AST nodes | Immutable |

The rh-cql design anticipated many of the architectural improvements being introduced in the reference implementation.

## Extension Points

### Adding New Expression Types

1. Add variant to `ast::Expression` enum
2. Add case to `translate_expression` match
3. Add corresponding ELM type if needed
4. Compiler ensures exhaustiveness

### Supporting New Backends

The AST layer is independent of ELM. To target a different output format:

1. Create new output types (e.g., `sql/*.rs`)
2. Implement new translator (e.g., `SqlTranslator`)
3. Reuse existing parser and AST

### Adding Compiler Options

Options are configured via `CompilerOptions`:

```rust
let options = CompilerOptions::default()
    .with_annotations(true)
    .with_detailed_errors(true);

let builder = LibraryBuilder::new(options);
```

## Error Handling

Errors are collected with source location information:

```rust
pub struct CompilerError {
    pub message: String,
    pub severity: Severity,
    pub location: Option<SourceLocation>,
}
```

The compiler continues after errors when possible, collecting multiple errors per compilation for better user experience.

## Future Considerations

While the current architecture is well-suited for CQL-to-ELM translation, these extensions could be added if needed:

1. **Typed AST**: Add explicit type annotations to AST nodes for advanced analysis
2. **Generic Traversal**: Add walker/visitor utilities for the AST
3. **Incremental Compilation**: Cache and reuse partial results
4. **Language Server Protocol**: IDE integration for real-time feedback

These would be additive changes that don't require restructuring the core architecture.
