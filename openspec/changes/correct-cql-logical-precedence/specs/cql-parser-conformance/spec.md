## ADDED Requirements

### Requirement: Logical operators follow CQL precedence

The parser SHALL group logical operators from lowest to highest precedence as `implies`,
`or`/`xor`, `and`, then comparison and higher-precedence expressions.

#### Scenario: And binds more tightly than implies

- **WHEN** `false implies true and false` is parsed
- **THEN** the AST groups as `false implies (true and false)`

#### Scenario: And binds more tightly than or and xor

- **WHEN** unparenthesized expressions mix `and` with `or` or `xor`
- **THEN** each `and` subexpression is grouped before the surrounding `or` or `xor`

#### Scenario: Or and xor bind more tightly than implies

- **WHEN** unparenthesized expressions mix `implies` with `or` or `xor`
- **THEN** each `or` or `xor` subexpression is grouped before the surrounding `implies`

### Requirement: Logical associativity is explicit

The parser SHALL keep `and`, `or`, and `xor` left-associative and SHALL implement repeated
`implies` with the associativity defined by CQL 1.5.3.

#### Scenario: Left-associative logical chains

- **WHEN** three operands are joined by only `and`, only `or`, or only `xor`
- **THEN** the AST groups the first two operands before the third

#### Scenario: Repeated implies matches the reference grammar

- **WHEN** an unparenthesized chain containing repeated `implies` is parsed
- **THEN** its AST and evaluated value match the CQL 1.5.3 grammar and reference translator

### Requirement: Explicit grouping and host expressions remain stable

Parentheses SHALL override logical precedence, and the corrected hierarchy SHALL apply consistently
inside function bodies and query `where` clauses.

#### Scenario: Parentheses override default grouping

- **WHEN** parentheses specify a grouping different from default logical precedence
- **THEN** the AST preserves the parenthesized grouping

#### Scenario: Functions and queries use the corrected hierarchy

- **WHEN** equivalent mixed logical expressions appear in a function body and a query `where`
  clause
- **THEN** both contexts produce the same corrected logical grouping
