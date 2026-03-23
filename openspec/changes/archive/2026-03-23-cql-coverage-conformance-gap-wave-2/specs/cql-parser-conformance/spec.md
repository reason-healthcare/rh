## ADDED Requirements

### Requirement: Parser covers wave-2 parse-stage operator gaps
The parser SHALL recognize any wave-2 operator forms still marked parse-stage gaps in `SPEC_COVERAGE.md`, including interval `size of` syntax if not already supported.

#### Scenario: Interval size phrase parses
- **WHEN** `size of Interval[1, 5]` is parsed
- **THEN** the AST contains a size-related unary expression node (or equivalent parse representation) instead of a parse error

#### Scenario: Existing neighboring syntax does not regress
- **WHEN** interval unary phrases (`start of`, `end of`, `width of`, `point from`) are parsed after wave-2 parser updates
- **THEN** they continue to produce their existing AST forms without regression
