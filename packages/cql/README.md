# @reasonhealth/cql

Typed TypeScript wrapper for the Reason Health CQL WebAssembly build.

```ts
import { compile, evaluate } from "@reasonhealth/cql/node";

const compiled = compile("library Test version '1.0' define X: 1 + 2");
const result = evaluate(compiled.data!, { expression: "X" });
```

SQL-on-FHIR helpers are also exposed from each entry point:

These experimental helpers provide inspection and artifact scaffolding, not
validated measure execution. The current relational plan is a shallow
diagnostic tree; SQL emission bypasses it, builds retrieve CTEs, and selects the
first one. Its `code IS NOT NULL` check does not implement value-set membership.
Complete clinical predicates and included-library dependency closure are not
preserved in emitted artifacts. A `supported: true` classification is not an
executable SQL support guarantee; a `fallbackNodes` label does not provide a
CQL fallback executor in the analytics runtime. See
[current relational-algebra status](../../crates/rh-cql/ARCHITECTURE.md#experimental-relational-algebra).

```ts
import {
  dataRequirements,
  emitSql,
  emitSqlQueryLibrary,
  emitViewDefinitions,
  lowerCheck,
  relationalPlan
} from "@reasonhealth/cql/node";

const source = `
library DiabetesMeasure version '1.0.0'
using FHIR version '4.0.1'
valueset "Diabetes": 'http://example.org/fhir/ValueSet/diabetes'
context Patient
define "Diabetes Conditions":
  [Condition: "Diabetes"]
`;

const requirements = dataRequirements(source);
const report = lowerCheck(source);
const plan = relationalPlan(source);
const views = emitViewDefinitions(source);
const sql = emitSql(source);
const sqlLibrary = emitSqlQueryLibrary(source);
```

Exports:

- `@reasonhealth/cql/node` for Node.js.
- `@reasonhealth/cql/web` for direct browser loading. Call `initCql()` before invoking wrapper functions.
- `@reasonhealth/cql/bundler` for Vite, webpack, Rollup, and similar bundlers.

Main functions:

- `compile(source, options?)` compiles CQL to ELM JSON.
- `evaluate(elmJson, options)` evaluates a named ELM expression.
- `explainParse(source)` and `explainCompile(source, options?)` return human-readable diagnostics.
- `inspect(source, options?)` summarizes compiled ELM.
- `dataRequirements(source, options?)` extracts resource, retrieve, terminology, and parameter requirements.
- `relationalPlan(source, options?)` builds the first-pass relational plan.
- `lowerCheck(source, options?)` inventories node-kind classifications; the target is a report label defaulting to `sql-on-fhir`, not a backend execution check.
- `emitViewDefinitions(source, options?)` emits SQL-on-FHIR ViewDefinition JSON.
- `emitSql(source, options?)` emits retrieval-skeleton SQL text plus the generated ViewDefinition dependencies.
- `emitSqlQueryLibrary(source, options?)` packages retrieval-skeleton SQL in a FHIR Library artifact with SQL text and ViewDefinition dependencies.
