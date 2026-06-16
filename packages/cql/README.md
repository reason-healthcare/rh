# @reasonhealth/cql

Typed TypeScript wrapper for the Reason Health CQL WebAssembly build.

```ts
import { compile, evaluate } from "@reasonhealth/cql/node";

const compiled = compile("library Test version '1.0' define X: 1 + 2");
const result = evaluate(compiled.data!, { expression: "X" });
```

Exports:

- `@reasonhealth/cql/node` for Node.js.
- `@reasonhealth/cql/web` for direct browser loading. Call `initCql()` before invoking wrapper functions.
- `@reasonhealth/cql/bundler` for Vite, webpack, Rollup, and similar bundlers.
