# FHIRPath trace() Function

The `trace()` function is a diagnostic utility that logs information about the values flowing through a FHIRPath expression, while passing those values through unchanged. This is invaluable for debugging complex FHIRPath queries.

## Specification

According to the [FHIRPath specification](https://www.hl7.org/fhirpath/#tracename-string-projection-expression-collection):

```
trace(name : String [, projection: Expression]) : collection
```

The `trace()` function adds a String representation of the input collection to a diagnostic log using the provided `name` argument. The function returns the input collection unchanged, making it a pass-through operation perfect for debugging.

## Parameters

- **name** (required): A String that labels this trace point in the logs
- **projection** (optional): An Expression to evaluate on each item in the collection. If provided, the projection result is logged instead of the original values, but the original collection is still returned

## Behavior

1. **Logs diagnostic information** to help debug expression evaluation
2. **Returns input unchanged** - never modifies the data flowing through the expression
3. **Outputs to two destinations**:
   - `stderr` via `eprintln!` for immediate visibility
   - `EvaluationContext.trace_logs` for programmatic access

## Usage Examples

### Basic Trace

```rust
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

let parser = FhirPathParser::new();
let evaluator = FhirPathEvaluator::new();
let data = json!({"name": [{"family": "Smith", "given": ["John"]}]});
let context = EvaluationContext::new(data);

// Trace the names being processed
let expr = parser.parse("Patient.name.trace('patient-names').family").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();

// Result is "Smith" (unchanged by trace)
// Trace log contains the full name object
```

### Trace with Projection

```rust
// Log only the family names, but continue processing the full name objects
let expr = parser.parse("Patient.name.trace('families', family).given").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();

// Result is ["John"] (given names)
// Trace log shows "Smith" (the projection)
```

### Multiple Traces in a Chain

```rust
// Track data at multiple points in the evaluation
let expr = parser.parse(
    "Patient.name.trace('step1').where(use='official').trace('step2').family"
).unwrap();

// Trace logs will show:
// - step1: All names before filtering
// - step2: Only official names after filtering
```

## CLI Usage

### In `rh fhirpath eval`

```bash
cargo run -p rh -- fhirpath eval -d patient.json "Patient.name.trace('names').family"
```

Output includes a trace log section:
```
📋 Trace logs:
  [TRACE:names] Collection([Object(Object {"family": String("Smith"), "given": Array [String("John")]})])

✅ Expression: Patient.name.trace('names').family
Result: String("Smith")
```

### In the REPL

```bash
cargo run -p rh -- fhirpath repl -d patient.json
```

```
fhirpath> Patient.name.trace('debug').family

📋 Trace logs:
  [TRACE:debug] Collection([Object(Object {"family": String("Smith"), "given": Array [String("John")]})])

=> String("Smith")
```

## WASM Support

When using FHIRPath in WebAssembly, trace logs are included in the evaluation result:

```javascript
const result = evaluate_fhirpath(
    "Patient.name.trace('names').family",
    patientJson
);

const parsed = JSON.parse(result.data);
console.log(parsed.result);  // The actual result
console.log(parsed.trace);   // Array of trace log entries
```

Each trace log entry contains:
```javascript
{
    "name": "names",
    "value": "Collection([Object(...)])"
}
```

## Programmatic Access

You can access trace logs programmatically from the `EvaluationContext`:

```rust
let context = EvaluationContext::new(data);
let result = evaluator.evaluate(&expr, &context).unwrap();

// Get all trace logs
let logs = context.get_trace_logs();
for log in logs {
    println!("[{}] {}", log.name, log.value);
}

// Clear logs for the next evaluation
context.clear_trace_logs();
```

## Use Cases

### 1. Debugging Complex Queries

```fhirpath
Bundle.entry.resource
    .trace('all-resources')
    .where(resourceType='Patient')
    .trace('patients-only')
    .where(active=true)
    .trace('active-patients')
    .name
    .trace('patient-names')
    .family
```

This shows the data at each filtering step, making it easy to identify where unexpected filtering occurs.

### 2. Understanding Data Transformations

```fhirpath
Patient.name
    .trace('raw-names', $this)
    .select(family + ', ' + given.first())
    .trace('formatted-names')
```

See both the raw data and the transformed output.

### 3. Performance Debugging

```fhirpath
// Check what's being processed at expensive operations
Bundle.entry.resource.trace('before-descendants').descendants().trace('after-descendants')
```

### 4. Projection Verification

```fhirpath
// Verify projection extracts expected values
Patient.name.trace('ids-only', id).family
```

Logs show only the IDs but the expression continues with full name objects.

## Implementation Details

### Thread-Safe Log Collection

Trace logs are stored in `Rc<RefCell<Vec<TraceLog>>>` within the `EvaluationContext`, allowing:
- Multiple references to the same log collection
- Mutation through interior mutability
- Shared logs across context clones

### Backward Compatibility

The function maintains backward compatibility by:
- Still outputting to `stderr` via `eprintln!` (visible immediately in terminals)
- Adding structured logs to the context (accessible programmatically)

### Performance

- Minimal overhead when trace is not used
- Logs are only formatted when needed
- Shared log collection avoids unnecessary cloning

## Testing

Comprehensive test coverage includes:
- Basic name-only tracing
- Projection parameter usage
- Empty collections
- Single values
- Chaining multiple traces
- Use within `where()` clauses
- Error cases (missing/invalid parameters)
- Type preservation
- Programmatic log access
- Log clearing

Run tests with:
```bash
cargo test -p rh-fhirpath --test trace_function_test
cargo test -p rh-fhirpath --test trace_logs_test
```

## See Also

- [FHIRPath Specification - trace()](https://www.hl7.org/fhirpath/#tracename-string-projection-expression-collection)
- [Example: trace_function_demo.rs](examples/trace_function_demo.rs)
- [Tests: trace_function_test.rs](tests/trace_function_test.rs)
- [Tests: trace_logs_test.rs](tests/trace_logs_test.rs)
