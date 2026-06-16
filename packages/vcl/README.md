# @reason-healthcare/vcl

Typed TypeScript wrapper for the Reason Health VCL WebAssembly build.

```ts
import { translateExpression } from "@reason-healthcare/vcl/node";

const result = translateExpression("123456", {
  defaultSystem: "http://snomed.info/sct"
});
```

Exports:

- `@reason-healthcare/vcl/node` for Node.js.
- `@reason-healthcare/vcl/web` for direct browser loading. Call `initVcl()` before invoking wrapper functions.
- `@reason-healthcare/vcl/bundler` for Vite, webpack, Rollup, and similar bundlers.
