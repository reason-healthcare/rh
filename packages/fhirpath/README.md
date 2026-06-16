# @reasonhealth/fhirpath

Typed TypeScript wrapper for the Reason Health FHIRPath WebAssembly build.

```ts
import { evaluateExpression } from "@reasonhealth/fhirpath/node";

const result = evaluateExpression("name.given", {
  resourceType: "Patient",
  name: [{ given: ["Ada"] }]
});
```

Exports:

- `@reasonhealth/fhirpath/node` for Node.js.
- `@reasonhealth/fhirpath/web` for direct browser loading. Call `initFhirPath()` before invoking wrapper functions.
- `@reasonhealth/fhirpath/bundler` for Vite, webpack, Rollup, and similar bundlers.
