## ADDED Requirements

### Requirement: Recognized constructs commit to required continuations

After recognizing an infix operator or declaration starter, the parser SHALL propagate a malformed
required continuation instead of backtracking to a shorter successful parse.

#### Scenario: Malformed right operand reports after its operator

- **WHEN** a recognized infix operator is followed by a malformed nested right operand
- **THEN** parsing fails at the malformed operand rather than reporting trailing content beginning
  at the preceding operator

#### Scenario: Malformed temporal phrase reports its own location

- **WHEN** a temporal relationship is recognized but its required direction or operand is malformed
- **THEN** the error span identifies the temporal phrase or malformed operand

#### Scenario: Invalid declaration propagates from its starter

- **WHEN** a recognized library declaration starter is followed by an invalid declaration
- **THEN** the statement loop propagates that declaration error rather than ending successfully

### Requirement: Complete library parsing requires structural end of input

The complete-library parser SHALL include end of input in its grammar and SHALL reject unconsumed
source through the normal parse-error path.

#### Scenario: Valid complete library reaches EOF

- **WHEN** a valid library and its permitted trailing trivia are parsed
- **THEN** parsing succeeds only after consuming the complete input

#### Scenario: Unexpected trailing construct is rejected structurally

- **WHEN** source remains after the final valid declaration
- **THEN** the complete-library parser returns a located parse error instead of a successful result
  plus a post-parse remainder

### Requirement: Parser commitment preserves valid alternatives

Commitment SHALL NOT change valid parsing of `between ... and ...`, query clauses, collection
separators, or alternate statement productions.

#### Scenario: Delimiter-sensitive controls remain valid

- **WHEN** focused valid examples exercise each delimiter-sensitive or alternative construct
- **THEN** each example produces its pre-change AST shape without a committed parse error

### Requirement: Parse errors preserve the deepest useful span

Conversion from parser errors to `CqlError` SHALL preserve the source span of the deepest useful
committed failure.

#### Scenario: Nested failure location survives conversion

- **WHEN** a nested parser failure is converted to the public CQL error type
- **THEN** its line and column identify the nested failing construct rather than an earlier Boolean
  operator or the start of trailing content
