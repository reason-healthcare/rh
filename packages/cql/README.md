# @reasonhealth/cql

Typed TypeScript wrapper for the Reason Health CQL WebAssembly build.

```ts
import { compile, evaluate } from "@reasonhealth/cql/node";

const compiled = compile("library Test version '1.0' define X: 1 + 2");
const result = evaluate(compiled.data!, { expression: "X" });
```

SQL-on-FHIR helpers are also exposed from each entry point:

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
- `lowerCheck(source, options?)` reports whether the library can lower to the target, defaulting to `sql-on-fhir`.
- `emitViewDefinitions(source, options?)` emits SQL-on-FHIR ViewDefinition JSON.
- `emitSql(source, options?)` emits SQL text plus the generated ViewDefinition dependencies.
- `emitSqlQueryLibrary(source, options?)` emits a SQLQuery FHIR Library artifact plus SQL text and ViewDefinition dependencies.
