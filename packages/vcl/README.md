# @reasonhealth/vcl

Typed TypeScript wrapper for the Reason Health VCL WebAssembly build.

```ts
import { translateExpression } from "@reasonhealth/vcl/node";

const result = translateExpression("123456", {
  defaultSystem: "http://snomed.info/sct"
});
```

Exports:

- `@reasonhealth/vcl/node` for Node.js.
- `@reasonhealth/vcl/web` for direct browser loading. Call `initVcl()` before invoking wrapper functions.
- `@reasonhealth/vcl/bundler` for Vite, webpack, Rollup, and similar bundlers.
