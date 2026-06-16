# FHIRPath WASM Demo

The standalone FHIRPath demo has moved to the shared playground at
`examples/playground`.

Run it from the workspace root:

```bash
pnpm --filter @reason-healthcare/playground dev
```

The playground exercises the FHIRPath, VCL, and CQL WebAssembly packages through
the same TypeScript wrappers that are published from `packages/`.
