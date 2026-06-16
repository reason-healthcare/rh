# @reason-healthcare/cql

Typed TypeScript wrapper for the Reason Health CQL WebAssembly build.

```ts
import { compile, evaluate } from "@reason-healthcare/cql/node";

const compiled = compile("library Test version '1.0' define X: 1 + 2");
const result = evaluate(compiled.data!, { expression: "X" });
```

Exports:

- `@reason-healthcare/cql/node` for Node.js.
- `@reason-healthcare/cql/web` for direct browser loading. Call `initCql()` before invoking wrapper functions.
- `@reason-healthcare/cql/bundler` for Vite, webpack, Rollup, and similar bundlers.
