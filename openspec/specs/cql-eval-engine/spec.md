## ADDED Requirements

### Requirement: Value enum represents CQL runtime values

The eval engine SHALL define a `Value` enum representing all CQL runtime values:
- `Null` — CQL null
- `Boolean(bool)`
- `Integer(i64)`
- `Long(i128)`
- `Decimal(Decimal)` — arbitrary precision
- `String(String)`
- `Date(CqlDate)` — partial date (year, year-month, year-month-day)
- `DateTime(CqlDateTime)` — partial date-time with timezone offset
- `Time(CqlTime)` — partial time
- `Quantity { value: Decimal, unit: String }`
- `Ratio { numerator: Quantity, denominator: Quantity }`
- `Code { code: String, system: String, display: Option<String>, version: Option<String> }`
- `Concept { codes: Vec<Code>, display: Option<String> }`
- `List(Vec<Value>)`
- `Tuple(BTreeMap<String, Value>)`
- `Interval { low: Option<Box<Value>>, high: Option<Box<Value>>, low_closed: bool, high_closed: bool }`

#### Scenario: Null value
- **WHEN** a CQL expression evaluates to `null`
- **THEN** the runtime value is `Value::Null`

#### Scenario: List value
- **WHEN** a CQL expression `{ 1, 2, 3 }` is evaluated
- **THEN** the runtime value is `Value::List` containing three `Value::Integer` elements

### Requirement: Three-valued logic for Boolean operations

The eval engine SHALL implement CQL three-valued logic where `null` propagates through Boolean operations per the CQL specification:
- `true and null` → `null`
- `false and null` → `false`
- `true or null` → `true`
- `false or null` → `null`
- `not null` → `null`

#### Scenario: Null propagation in AND
- **WHEN** `true and null` is evaluated
- **THEN** the result is `Value::Null`

#### Scenario: Short-circuit in AND
- **WHEN** `false and null` is evaluated
- **THEN** the result is `Value::Boolean(false)`

#### Scenario: Short-circuit in OR
- **WHEN** `true or null` is evaluated
- **THEN** the result is `Value::Boolean(true)`

### Requirement: EvalContext provides evaluation environment

The eval engine SHALL define an `EvalContext` that provides:
- A deterministic clock (`now()`, `today()`, `timeOfDay()`) injected at construction
- A timezone offset for date/time evaluation
- A `DataProvider` trait reference for retrieve operations
- A `TerminologyProvider` trait reference for terminology operations
- Parameter values
- A current context value (e.g., the Patient resource)

#### Scenario: Deterministic clock
- **WHEN** `Now()` is evaluated with a clock set to `2024-01-15T10:30:00-05:00`
- **THEN** the result is `Value::DateTime` representing `2024-01-15T10:30:00-05:00`

#### Scenario: Context value
- **WHEN** an expression evaluates in `Patient` context with a specific patient resource
- **THEN** `Patient` references resolve to the context value provided in `EvalContext`

### Requirement: DataProvider trait for retrieve operations

The eval engine SHALL define a `DataProvider` trait:

```
fn retrieve(context, data_type, code_path, codes, date_path, date_range) -> Result<Vec<Value>>
```

The trait SHALL be implementable for in-memory test data and for external data sources (FHIR servers).

#### Scenario: In-memory retrieve
- **WHEN** `[Observation]` is evaluated with an in-memory `DataProvider` containing two Observation resources
- **THEN** the result is a `Value::List` with two `Value::Tuple` elements

#### Scenario: Filtered retrieve
- **WHEN** `[Observation: "Blood Pressure"]` is evaluated with code filtering
- **THEN** the `DataProvider` receives the code filter and returns only matching resources

### Requirement: TerminologyProvider trait for terminology operations

The eval engine SHALL define a `TerminologyProvider` trait with:
- `in_valueset(code, valueset_url) -> Result<bool>`
- `expand_valueset(valueset_url) -> Result<Vec<Code>>`
- `lookup(code, property) -> Result<Option<Value>>`

#### Scenario: ValueSet membership check
- **WHEN** `code in "http://example.org/ValueSet/my-codes"` is evaluated
- **THEN** the `TerminologyProvider.in_valueset()` is called with the code and valueset URL

### Requirement: Core operator evaluation

The eval engine SHALL evaluate the following operator families:
- **Arithmetic**: Add, Subtract, Multiply, Divide, Modulo, Negate, Abs, Ceiling, Floor, Truncate, Round, Power, Ln, Exp, Log, MinValue, MaxValue, Predecessor, Successor
- **Comparison**: Equal, Equivalent, Less, Greater, LessOrEqual, GreaterOrEqual
- **Logical**: And, Or, Not, Implies, Xor
- **String**: Concatenate, Combine, Split, SplitOnMatches, Length, Upper, Lower, StartsWith, EndsWith, Matches, ReplaceMatches, Indexer, PositionOf, LastPositionOf, Substring
- **Date/Time**: date/time construction, component extraction (year, month, day, etc.), date arithmetic (add/subtract quantities), SameAs, SameOrBefore, SameOrAfter, duration calculations
- **Interval**: Interval construction, Contains, In, Includes, IncludedIn, Overlaps, Meets, Starts, Ends, ProperContains, ProperIn, ProperIncludes, ProperIncludedIn, Union, Intersect, Except, Width, Start, End, PointFrom, Expand, Collapse
- **List**: construction, Exists, Count, Sum, Min, Max, Avg, Median, Mode, StdDev, PopulationStdDev, Variance, PopulationVariance, AllTrue, AnyTrue, First, Last, IndexOf, Contains, In, Includes, IncludedIn, Union, Intersect, Except, Flatten, Distinct, Sort, Filter, ForEach, Repeat, SingletonFrom
- **Type**: Is, As, Convert, ToBoolean, ToInteger, ToDecimal, ToLong, ToString, ToDate, ToDateTime, ToTime, ToQuantity, ToConcept, ToList, ToInterval
- **Null**: IsNull, IsTrue, IsFalse, Coalesce, IfNull

#### Scenario: Integer addition
- **WHEN** `2 + 3` is evaluated
- **THEN** the result is `Value::Integer(5)`

#### Scenario: Null arithmetic
- **WHEN** `2 + null` is evaluated
- **THEN** the result is `Value::Null`

#### Scenario: Date arithmetic
- **WHEN** `@2024-01-15 + 3 months` is evaluated
- **THEN** the result is `Value::Date` representing `2024-04-15`

### Requirement: Query evaluation

The eval engine SHALL evaluate CQL query expressions with:
- Single-source queries (from clause with one source)
- Where clause filtering
- Return clause projection
- Sort clause ordering
- Let clause local bindings

Multi-source queries (joins) are deferred to a future phase.

#### Scenario: Simple query with where
- **WHEN** `from [Observation] O where O.status = 'final'` is evaluated
- **THEN** only observations with status 'final' are returned

#### Scenario: Query with return projection
- **WHEN** `from [Observation] O return O.value` is evaluated
- **THEN** the result is a list of observation values

### Requirement: evaluate_elm API function

The eval engine SHALL expose a public `evaluate_elm(library, expression_name, context) -> Result<Value>` function that evaluates a named expression from an ELM library in the given `EvalContext`.

#### Scenario: Evaluate named expression
- **WHEN** `evaluate_elm(library, "MyExpression", context)` is called
- **THEN** the named expression is evaluated and the result `Value` is returned

#### Scenario: Evaluate missing expression
- **WHEN** `evaluate_elm(library, "NonExistent", context)` is called
- **THEN** an error is returned indicating the expression was not found

### Requirement: evaluate_elm_with_trace produces trace events

The `evaluate_elm_with_trace()` function SHALL return both the result `Value` and a list of `TraceEvent` records. Each `TraceEvent` SHALL contain:
- `event_id`: Sequential step index
- `elm_node_id`: The ELM node being evaluated (for source-map correlation)
- `op`: The operation name
- `inputs`: The input values (redaction-capable)
- `output`: The result value
- `children`: Nested trace events for sub-expressions

#### Scenario: Trace of arithmetic
- **WHEN** `evaluate_elm_with_trace(library, "X", context)` is called where `X` is `2 + 3`
- **THEN** the trace contains an `Add` event with inputs `[Integer(2), Integer(3)]` and output `Integer(5)`, plus child events for each literal

### Requirement: CLI eval subcommand

The CLI SHALL provide an `eval` subcommand that accepts a CQL file path, an expression name, and optional data/context parameters. It SHALL print the result value to stdout.

#### Scenario: CLI eval
- **WHEN** `rh cql eval library.cql --expression MyExpression` is run
- **THEN** the expression is evaluated and the result is printed to stdout

#### Scenario: CLI eval with trace
- **WHEN** `rh cql eval library.cql --expression MyExpression --trace` is run
- **THEN** the expression is evaluated and the trace is printed to stdout
