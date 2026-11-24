# @reasonhealth/fhirpath

Typed TypeScript wrapper for the Reason Health FHIRPath WebAssembly build.

```ts
import { evaluateExpression } from "@reasonhealth/fhirpath/node";

const result = evaluateExpression("name.given", {
  resourceType: "Patient",
  name: [{ given: ["Ada"] }]
});
```

Pass resource JSON as a string to preserve decimal precision and scientific notation.
In JSON mode, `result.data` contains the exact JSON output and `result.value` uses
JavaScript `Number` for numeric fields.

Exports:

- `@reasonhealth/fhirpath/node` for Node.js.
- `@reasonhealth/fhirpath/web` for direct browser loading. Call `initFhirPath()` before invoking wrapper functions.
- `@reasonhealth/fhirpath/bundler` for Vite, webpack, Rollup, and similar bundlers.
