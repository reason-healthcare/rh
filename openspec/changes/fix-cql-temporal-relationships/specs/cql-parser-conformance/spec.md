## ADDED Requirements

### Requirement: Parser supports zero-offset temporal relationships

The parser SHALL recognize all CQL 1.5.3 zero-offset temporal relationship spellings and SHALL
represent them with the existing relative-timing AST using `offset: None`.

#### Scenario: Prefix relationship forms parse

- **WHEN** expressions using `on before`, `on or before`, `on after`, or `on or after` are parsed
- **THEN** each expression produces the corresponding exclusive or inclusive timing relationship
  with no quantity offset

#### Scenario: Suffix relationship forms parse

- **WHEN** expressions using `before`, `before or on`, `after`, or `after or on` are parsed
- **THEN** each expression produces the corresponding exclusive or inclusive timing relationship
  with no quantity offset

#### Scenario: Precision and operand shapes are preserved

- **WHEN** zero-offset relationships are parsed with no precision, with `month of` or `hour of`
  precision, and with point or interval operands
- **THEN** the AST preserves the supplied precision and consumes both operands completely

### Requirement: Medication-request timing function parses without rewriting

The parser SHALL parse the exact `Is Current Medication Request` function from the regression
fixture in both multiline and single-line form without source rewrites.

#### Scenario: Multiline function has the expected outer AST

- **WHEN** the multiline function is parsed
- **THEN** its outer AST groups as `And(And(status-in, intent-in), Or(null-test,
  on-or-after))`
- **THEN** `end of FHIRHelpers.ToInterval(...)` consumes the complete qualified invocation

#### Scenario: Physical line layout does not change AST structure

- **WHEN** equivalent multiline and single-line forms of the function are parsed
- **THEN** their expression AST structures are equivalent apart from source locations

### Requirement: Neighboring expression syntax remains parseable

The parser SHALL preserve existing support for repeated conjunctions, parenthesized logical
operands, nested member access, qualified calls, and `end of` while adding temporal forms.

#### Scenario: Existing components retain their AST forms

- **WHEN** focused regressions parse each neighboring expression form
- **THEN** parsing succeeds and produces its pre-change AST shape
