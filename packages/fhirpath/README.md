# @reason-healthcare/fhirpath

Typed TypeScript wrapper for the Reason Health FHIRPath WebAssembly build.

```ts
import { evaluateExpression } from "@reason-healthcare/fhirpath/node";

const result = evaluateExpression("name.given", {
  resourceType: "Patient",
  name: [{ given: ["Ada"] }]
});
```

Exports:

- `@reason-healthcare/fhirpath/node` for Node.js.
- `@reason-healthcare/fhirpath/web` for direct browser loading. Call `initFhirPath()` before invoking wrapper functions.
- `@reason-healthcare/fhirpath/bundler` for Vite, webpack, Rollup, and similar bundlers.
