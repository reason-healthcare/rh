# Reason Health Playground

Interactive Vite playground for the Reason Health WebAssembly packages.

The playground runs the same TypeScript wrappers that are published from
`packages/`:

- `@reason-healthcare/fhirpath` for FHIRPath parsing, validation, and evaluation.
- `@reason-healthcare/vcl` for VCL validation, explanation, and ValueSet compose translation.
- `@reason-healthcare/cql` for CQL compile-to-ELM JSON.

## Prerequisites

From a fresh checkout, install the workspace tooling:

```bash
pnpm install
rustup target add wasm32-unknown-unknown
```

The package build also needs `wasm-pack`. CI installs it through
`taiki-e/install-action`; locally, install it with:

```bash
cargo install wasm-pack
```

## Run Locally

From the repository root:

```bash
pnpm --filter @reason-healthcare/playground dev
```

The `predev` script first builds the three local WASM wrapper packages:

```bash
pnpm --filter @reason-healthcare/playground build:deps
```

After dependency builds finish, Vite prints a local URL like:

```text
http://127.0.0.1:5173/rh/
```

Open that URL in a browser. The `/rh/` base path matches the GitHub Pages
deployment path.

## Build

Build the full playground bundle:

```bash
pnpm --filter @reason-healthcare/playground build
```

This runs TypeScript checking and `vite build`. The output goes to
`examples/playground/dist/`, which is intentionally gitignored.

To rebuild all npm packages and the playground from the workspace root:

```bash
pnpm -r build
```

## Test

Run the playground smoke tests:

```bash
pnpm --filter @reason-healthcare/playground test
```

Run all package and playground tests:

```bash
pnpm -r test
```

## GitHub Pages

`.github/workflows/pages.yml` deploys the playground on pushes to `main` and on
manual workflow dispatch.

The workflow:

1. Installs Rust with the `wasm32-unknown-unknown` target.
2. Installs `wasm-pack`.
3. Installs pnpm and Node.js.
4. Builds `@reason-healthcare/fhirpath`, `@reason-healthcare/vcl`, and
   `@reason-healthcare/cql`.
5. Builds `@reason-healthcare/playground`.
6. Uploads `examples/playground/dist/` to GitHub Pages.

## Troubleshooting

If Vite fails to resolve a WASM package, rebuild the wrapper artifacts:

```bash
pnpm --filter @reason-healthcare/playground build:deps
```

If generated JavaScript appears under `examples/playground/src/` or
`examples/playground/test/`, remove it. The playground TypeScript config uses
`noEmit`; source directories should contain `.ts` and `.css` files only.

If `wasm-pack` warns that no prebuilt `wasm-bindgen` binary is available for
your platform, it falls back to installing/building the tool locally. That
warning is non-fatal when the command exits successfully.

If pnpm reports ignored build scripts for `esbuild`, Vite can still build in
the current workspace setup as long as `pnpm --filter @reason-healthcare/playground build`
passes.
