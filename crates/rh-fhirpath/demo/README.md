# FHIRPath WASM Demo

The standalone FHIRPath demo has moved to the shared playground at
`examples/playground`. See `examples/playground/README.md` for the full setup,
run, build, test, deployment, and troubleshooting notes.

Run it from the workspace root:

```bash
pnpm --filter @reason-healthcare/playground dev
```

The playground exercises the FHIRPath, VCL, and CQL WebAssembly packages through
the same TypeScript wrappers that are published from `packages/`.
