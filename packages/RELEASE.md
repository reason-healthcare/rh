# Releasing NPM Packages

This document describes the local release process for the public
`@reason-healthcare/*` WebAssembly packages published to npmjs.com.

Published packages:

- `@reason-healthcare/fhirpath`
- `@reason-healthcare/vcl`
- `@reason-healthcare/cql`

The private playground package is not published.

## Prerequisites

- Node.js 24 or later
- pnpm 10.11.0
- Rust stable with the `wasm32-unknown-unknown` target
- `wasm-pack`
- npm account with publish access to the `@reason-healthcare` scope

Install local prerequisites:

```bash
pnpm install
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Authenticate with npm:

```bash
npm login
npm whoami
```

For automation from a local shell, `NODE_AUTH_TOKEN` may also be set to an npm
automation token with publish access.

## Versioning

Keep npm package versions aligned with the Rust workspace version unless there
is a deliberate package-only patch release.

Before publishing, verify package versions:

```bash
node -e "for (const p of ['fhirpath','vcl','cql']) console.log(p, require('./packages/' + p + '/package.json').version)"
```

When bumping a full workspace release, update package versions in:

- `packages/fhirpath/package.json`
- `packages/vcl/package.json`
- `packages/cql/package.json`

Then run:

```bash
pnpm install
```

Commit `package.json` and `pnpm-lock.yaml` changes with the release version
bump.

## Validate Before Publish

Run the package gates from the repository root:

```bash
pnpm -r build
pnpm -r test
pnpm -r pack:dry-run
```

`pnpm -r build` compiles the Rust crates to WebAssembly through `wasm-pack`,
copies the generated artifacts into each package, and runs TypeScript builds.

`pnpm -r pack:dry-run` confirms that each publishable package includes:

- `dist/`
- `wasm/`
- `wasm-node/`
- `wasm-bundler/`
- `README.md`

The playground is private and has no `pack:dry-run` script.

## Publish

Publish packages from the repository root in this order:

```bash
npm publish --workspace @reason-healthcare/fhirpath --access public
npm publish --workspace @reason-healthcare/vcl --access public
npm publish --workspace @reason-healthcare/cql --access public
```

Use `--tag beta` for beta releases if the version is prerelease and should not
become npm's `latest` dist-tag:

```bash
npm publish --workspace @reason-healthcare/fhirpath --access public --tag beta
npm publish --workspace @reason-healthcare/vcl --access public --tag beta
npm publish --workspace @reason-healthcare/cql --access public --tag beta
```

Use `--provenance` only when publishing from a supported CI environment with
OIDC enabled. For the local release process, omit `--provenance`.

## Verify Published Packages

Confirm package metadata and dist-tags:

```bash
npm view @reason-healthcare/fhirpath version dist-tags
npm view @reason-healthcare/vcl version dist-tags
npm view @reason-healthcare/cql version dist-tags
```

Optionally install into a temporary project:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
pnpm init
pnpm add @reason-healthcare/fhirpath @reason-healthcare/vcl @reason-healthcare/cql
```

## Recovery

If a publish fails because the version already exists, do not republish over it;
npm versions are immutable. Bump to a new patch or prerelease version and repeat
the validation steps.

If a package was published with the wrong dist-tag, move tags explicitly:

```bash
npm dist-tag add @reason-healthcare/fhirpath@<version> beta
npm dist-tag rm @reason-healthcare/fhirpath latest
```

Repeat for `vcl` and `cql` as needed.
