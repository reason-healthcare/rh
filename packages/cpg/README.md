# @reasonhealth/cpg

Typed TypeScript wrapper for the Reason Health Clinical Practice Guidelines WebAssembly build.

```ts
import { applyPlanDefinition } from "@reasonhealth/cpg/node";

const result = applyPlanDefinition(
  planDefinition,
  "Patient/123",
  contentBundle
);
```

Exports:

- `@reasonhealth/cpg/node` for Node.js.
- `@reasonhealth/cpg/web` for direct browser loading. Call `initCpg()` before invoking wrapper functions.
- `@reasonhealth/cpg/bundler` for Vite, webpack, Rollup, and similar bundlers.
