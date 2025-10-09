# FFQ Examples

This directory contains examples demonstrating various FFQ (FHIR Filter Query) capabilities.

## Examples

### Basic Usage
- [`01_basic_hierarchy.ffq`](01_basic_hierarchy.ffq) - Basic hierarchy operations
- [`02_property_filters.ffq`](02_property_filters.ffq) - Property equality and membership
- [`03_aliases.ffq`](03_aliases.ffq) - Using aliases for cleaner queries

### Advanced Operations  
- [`04_minus_operations.ffq`](04_minus_operations.ffq) - Exclusion operations with minus (-)
- [`05_not_operations.ffq`](05_not_operations.ffq) - Negation operations with NOT (!)
- [`06_complex_expressions.ffq`](06_complex_expressions.ffq) - Complex nested expressions

### Real-World Scenarios
- [`07_snomed_examples.ffq`](07_snomed_examples.ffq) - SNOMED CT use cases
- [`08_loinc_examples.ffq`](08_loinc_examples.ffq) - LOINC laboratory codes
- [`09_medication_examples.ffq`](09_medication_examples.ffq) - Medication terminology

## Running Examples

You can run any example using the FFQ CLI:

```bash
# Parse and show AST
cargo run --bin rh -- ffq parse --file examples/01_basic_hierarchy.ffq

# Translate to FHIR ValueSet.compose
cargo run --bin rh -- ffq translate --file examples/01_basic_hierarchy.ffq

# Interactive REPL (copy/paste examples)
cargo run --bin rh -- ffq repl
```

## FHIR ValueSet.compose Output

Each example shows how FFQ expressions translate to FHIR ValueSet.compose JSON, which can be used directly in FHIR ValueSet resources for terminology filtering and expansion.