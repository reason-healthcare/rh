  <img src="assets/rust-health-logo-full-color-rgb.svg" alt="Rust Health Logo" height="30px">
</p>

[![CI](https://github.com/reason-healthcare/rh/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/reason-healthcare/rh/actions)
[![Crates.io](https://img.shields.io/crates/v/rh-foundation.svg)](https://crates.io/crates/rh-foundation)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org)

**Rust Health (rh)** is a modern, high-performance toolkit for working with HL7® FHIR®, purpose-built in Rust. It ships as a cross-platform **CLI**, a set of **Rust library crates**, and **WebAssembly-backed npm packages** — all without the overhead of JVM- or .NET-based stacks.

---

## Install the CLI

| Platform | Method | Command |
|----------|--------|---------|
| macOS / Linux | Homebrew | `brew tap reason-healthcare/rh && brew install rh` |
| Linux | Install script | `curl -fsSL https://raw.githubusercontent.com/reason-healthcare/rh/main/scripts/install-rh.sh \| sh` |
| Windows | Chocolatey | `choco install rh` |
| Any | Nix | `nix shell github:reason-healthcare/rh` |
| Any | Docker | `docker pull ghcr.io/reason-healthcare/rh:latest` |
| Any | Cargo | `cargo install rh-cli` |

Pre-built binaries for all platforms are also available on the [GitHub Releases page](https://github.com/reason-healthcare/rh/releases). See the [CLI README](apps/rh-cli/README.md) for full installation details including Docker usage.

### Quick start

```bash
# Download a FHIR package
rh download package hl7.fhir.r4.core 4.0.1

# Evaluate a FHIRPath expression against FHIR data
rh fhirpath eval "Patient.name.given" -d patient.json

# Interactive FHIRPath REPL
rh fhirpath repl

# Compile FSH source to FHIR JSON
rh fsh compile myprofile.fsh --output output/

# Translate a VCL expression to FHIR ValueSet.compose
rh vcl translate "http://loinc.org|718-7*"

# Compile CQL to ELM JSON
rh cql compile measure.cql

# Validate a FHIR resource
rh validate resource -i patient.json

# Generate a StructureDefinition snapshot
rh snapshot generate http://hl7.org/fhir/StructureDefinition/Patient

# Scaffold, build, and pack a FHIR package
rh package init --canonical https://example.org/fhir my-package/
rh package build my-package/
rh package pack my-package/output/
```

Every command supports `--format human|json|ndjson`. Use `--format json` for scripting and automation:

```json
{"ok": true, "result": {}, "errors": [], "meta": {"version": "0.2.1", "command": "rh"}}
```

| Exit code | Meaning |
|-----------|---------|
| `0` | Success |
| `1` | Operational error (I/O, network, missing file) |
| `2` | Usage error from argument parsing |
| `3` | Validation or conformance failure |
| `4` | Parse error (FHIRPath, CQL, FSH, or VCL) |

See `rh --help` or the [CLI README](apps/rh-cli/README.md) for the full command reference.

---

## npm Packages

`rh-fhirpath`, `rh-vcl`, and `rh-cql` compile to WebAssembly and are published as typed TypeScript wrappers on npm. Try them in the **[interactive playground →](https://reason-healthcare.github.io/rh/)**.

### Install

```bash
npm install @reasonhealth/fhirpath
npm install @reasonhealth/vcl
npm install @reasonhealth/cql
```

### Usage

```ts
// Node.js
import { evaluateExpression } from "@reasonhealth/fhirpath/node";

const result = evaluateExpression("name.given", {
  resourceType: "Patient",
  name: [{ given: ["Ada"] }],
});
```

Each package exports three entry points:

| Entry point | Target |
|-------------|--------|
| `@reasonhealth/<pkg>/node` | Node.js |
| `@reasonhealth/<pkg>/web` | Direct browser loading (call `init*()` first) |
| `@reasonhealth/<pkg>/bundler` | Vite, webpack, Rollup, and similar bundlers |

---

## Crates

| Crate | Description | npm |
|-------|-------------|-----|
| [rh-cli](apps/rh-cli/README.md) | Unified CLI for all FHIR tools | — |
| [rh-fhirpath](crates/rh-fhirpath/README.md) | FHIRPath expression parser and evaluator | [`@reasonhealth/fhirpath`](https://www.npmjs.com/package/@reasonhealth/fhirpath) |
| [rh-cql](crates/rh-cql/README.md) | CQL-to-ELM compiler, evaluator, explain mode, and source maps | [`@reasonhealth/cql`](https://www.npmjs.com/package/@reasonhealth/cql) |
| [rh-vcl](crates/rh-vcl/README.md) | ValueSet Compose Language (VCL) parser and translator | [`@reasonhealth/vcl`](https://www.npmjs.com/package/@reasonhealth/vcl) |
| [rh-fsh](crates/rh-fsh/README.md) | FHIR Shorthand (FSH) compiler | — |
| [rh-validator](crates/rh-validator/README.md) | Hybrid FHIR R4 validator with snapshot-based profile validation | — |
| [rh-packager](crates/rh-packager/README.md) | FHIR package assembler with built-in processors | — |
| [rh-codegen](crates/rh-codegen/README.md) | Code generation from FHIR StructureDefinitions | — |
| [rh-foundation](crates/rh-foundation/README.md) | Foundation utilities (errors, HTTP, I/O, package loader, snapshot) | — |
| [rh-hl7_fhir_r4_core](crates/rh-hl7_fhir_r4_core/README.md) | Generated R4 FHIR types for Rust (1,388 public types) | — |
| [rh-hl7_fhir_r5_core](crates/rh-hl7_fhir_r5_core/README.md) | Generated R5 FHIR types for Rust | — |

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing & Development

See [DEVELOPER.md](DEVELOPER.md) for build setup, testing, workspace structure, and release workflow. See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Supported By

This project is proudly supported by [Vermonster](https://vermonster.com) / [ReasonHealth](https://reason.health).

<p>
  <span style="padding: 0 20px; display: inline-block;">
    <a href="https://vermonster.com"><img src="https://www.vermonster.com/images/vermonster-logo.svg" alt="Vermonster Logo" height="20px"></a>
  </span>
  <span>&nbsp;&nbsp;&nbsp;</span>
  <span style="padding: 0 20px; display: inline-block;">
    <a href="https://reason.health"><img src="https://www.vermonster.com/images/reasonhub-logo-full-color-rgb.svg" alt="ReasonHealth Logo" height="20px"></a>
  </span>
</p>
