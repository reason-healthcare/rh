## ADDED Requirements

### Requirement: Parser supports predecessor and successor keywords

The parser SHALL recognize `predecessor of <expression>` and `successor of <expression>` as unary expressions producing `UnaryOperator::Predecessor` and `UnaryOperator::Successor` AST nodes.

#### Scenario: Predecessor of integer
- **WHEN** `predecessor of 5` is parsed
- **THEN** the AST contains a `UnaryExpression` with operator `Predecessor` and operand `Literal(Integer(5))`

#### Scenario: Successor of date
- **WHEN** `successor of @2024-01-15` is parsed
- **THEN** the AST contains a `UnaryExpression` with operator `Successor` and operand `Literal(Date(...))`

### Requirement: Parser supports power operator syntax

The parser SHALL recognize the `^` operator as a binary power operator with correct precedence (higher than multiplicative, lower than unary).

#### Scenario: Power operator
- **WHEN** `2 ^ 3` is parsed
- **THEN** the AST contains a `BinaryExpression` with operator `Power` and operands `Literal(Integer(2))` and `Literal(Integer(3))`

#### Scenario: Power precedence
- **WHEN** `2 * 3 ^ 4` is parsed
- **THEN** the AST groups as `2 * (3 ^ 4)` — power binds tighter than multiplication

### Requirement: Parser supports minimum and maximum type expressions

The parser SHALL recognize `minimum <TypeSpecifier>` and `maximum <TypeSpecifier>` as expressions that produce the minimum/maximum value of the given type.

#### Scenario: Minimum Integer
- **WHEN** `minimum Integer` is parsed
- **THEN** the AST contains a node representing the minimum value of the Integer type

#### Scenario: Maximum DateTime
- **WHEN** `maximum DateTime` is parsed
- **THEN** the AST contains a node representing the maximum value of the DateTime type

### Requirement: Parser supports aggregate clause in queries

The parser SHALL recognize the `aggregate` clause in query expressions with the syntax: `aggregate [all] <identifier> [starting <expression>] : <expression>`.

#### Scenario: Simple aggregate
- **WHEN** `from [Observation] O return aggregate Sum starting 0 : Sum + O.value` is parsed
- **THEN** the AST contains a query with an `aggregate` clause including the accumulator identifier, starting expression, and body expression

#### Scenario: Aggregate all
- **WHEN** a query with `aggregate all` is parsed
- **THEN** the AST correctly captures the `all` modifier on the aggregate clause

### Requirement: Parser correctly handles let clauses in queries

The parser SHALL correctly parse `let` clauses in query expressions with the syntax: `let <identifier> : <expression>`. Multiple let clauses SHALL be supported in a single query.

#### Scenario: Single let clause
- **WHEN** `from [Observation] O let V: O.value where V is not null` is parsed
- **THEN** the AST contains a query with one `LetClause` binding `V` to `O.value`

#### Scenario: Multiple let clauses
- **WHEN** a query with `let A: expr1 let B: expr2` is parsed
- **THEN** the AST contains two `LetClause` entries

### Requirement: Parser supports extended timing phrase keywords

The parser SHALL recognize the following timing phrase keywords that are currently missing or partially implemented:
- `starts` / `ends` as timing phrase qualifiers
- `occurs` as a timing phrase qualifier
- `properly` as a timing phrase modifier (e.g., `properly includes`, `properly during`)

#### Scenario: Starts before
- **WHEN** `A starts before B` is parsed
- **THEN** the AST contains a `TimingExpression` with `starts` qualifier and `before` relationship

#### Scenario: Properly includes
- **WHEN** `A properly includes B` is parsed
- **THEN** the AST contains a timing or interval expression with `properly` modifier

#### Scenario: Occurs during
- **WHEN** `A occurs during B` is parsed
- **THEN** the AST contains a `TimingExpression` with `occurs` qualifier and `during` relationship

### Requirement: Parser conformance reaches 90% on jvmTest suite

The parser SHALL pass at least 90% of the 119 jvmTest conformance test files (≥107 files), up from the current 75.6% (90 files).

#### Scenario: Conformance threshold met
- **WHEN** the jvmTest conformance suite is run
- **THEN** at least 107 of 119 test files parse successfully

#### Scenario: No regressions
- **WHEN** the jvmTest conformance suite is run after parser changes
- **THEN** all 90 previously passing test files continue to pass

### Requirement: Parser covers wave-2 parse-stage operator gaps

The parser SHALL recognize any wave-2 operator forms still marked parse-stage gaps in `SPEC_COVERAGE.md`, including interval `size of` syntax if not already supported.

#### Scenario: Interval size phrase parses
- **WHEN** `size of Interval[1, 5]` is parsed
- **THEN** the AST contains a size-related unary expression node (or equivalent parse representation) instead of a parse error

#### Scenario: Existing neighboring syntax does not regress
- **WHEN** interval unary phrases (`start of`, `end of`, `width of`, `point from`) are parsed after wave-2 parser updates
- **THEN** they continue to produce their existing AST forms without regression
