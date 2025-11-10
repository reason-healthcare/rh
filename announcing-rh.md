# Announcing RH: A Modern, Fast Toolkit for FHIR in Rust

Healthcare developers deserve tools that are fast, reliable, and a joy to use. RH (Rust Healthcare) is a modern, high‑performance toolkit for working with HL7® FHIR® that embraces Rust’s safety and speed, a clean modular architecture, and a cohesive, Unix‑style CLI that’s great for local use, CI pipelines, and automation.

RH runs natively on macOS, Linux, and Windows and can be compiled to WebAssembly for browsers and embedded runtimes. It’s designed to be ergonomic for developers, composable for teams, and blazing fast in production.

> HL7 and FHIR are registered trademarks of Health Level Seven International (HL7). RH is an independent open‑source project.

## Why RH (and why Rust)?

- Performance without the baggage: native binaries, tiny startup time, predictable memory usage.
- Safety and correctness: Rust’s type system helps catch bugs early, and RH’s APIs favor explicit, reliable behavior.
- Modular by design: pick a single crate (library) or use the unified CLI that brings it all together.
- Scriptable and CI‑friendly: subcommands follow the Unix philosophy—small, focused, and easy to compose.
- Web‑ready: compile core pieces (like parsers) to WebAssembly for use in browsers and serverless.

## What’s in the box today

RH is a workspace of focused crates plus a unified CLI (`rh`) that exposes their superpowers.

### Unified CLI (`rh`)

One tool to rule your FHIR workflows. Explore `--help` for each subcommand.

```bash
# Get started
cargo run -p rh -- --help

# Evaluate FHIRPath, with a REPL for quick exploration
cargo run -p rh -- fhirpath parse "Patient.name.family"
cargo run -p rh -- fhirpath eval "name.where(use='official').given"
cargo run -p rh -- fhirpath repl

# Parse/translate VCL (ValueSet Compose Language)
cargo run -p rh -- vcl parse "(http://snomed.info/sct)status = \"active\""
cargo run -p rh -- vcl translate "123456" --default-system http://snomed.info/sct
cargo run -p rh -- vcl explain "* - inactive"

# Snapshot generation for StructureDefinitions
cargo run -p rh -- snapshot generate \
  --package hl7.fhir.r4.core@4.0.1 \
  --package hl7.fhir.us.core@6.1.0 \
  http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient

# Validate FHIR resources (single or NDJSON batches)
cargo run -p rh -- validate resource --input patient.json --format json
cargo run -p rh -- validate batch --input patients.ndjson --summary-only

# Download FHIR packages from registries
cargo run -p rh -- download list hl7.fhir.r4.core
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1
```

### FHIRPath Examples

Quick CLI examples you can paste directly:

```bash
# Parse a FHIRPath expression (pretty AST)
cargo run -p rh -- fhirpath parse "name.where(use='official').given"

# Evaluate against a sample Patient
cargo run -p rh -- fhirpath eval "name.family" -d examples/patient.json
# -> ✅ Expression: name.family
# -> Result: Collection([String("Doe")])

# String functions
cargo run -p rh -- fhirpath eval "'hello world'.upper().substring(0, 5)"
# -> ✅ Expression: 'hello world'.upper().substring(0, 5)
# -> Result: String("HELLO")

# Unit conversions (UCUM)
cargo run -p rh -- fhirpath eval "1.0'kg' + 500.0'g'"
# -> ✅ Expression: 1.0'kg' + 500.0'g'
# -> Result: Quantity { value: 1.5, unit: Some("kg") }

# Temporal example (current date)
cargo run -p rh -- fhirpath eval "today()"
# -> ✅ Expression: today()
# -> Result: Date("YYYY-MM-DD")

# Interactive REPL
cargo run -p rh -- fhirpath repl --data examples/patient.json
# fhirpath> telecom.where(system='email').value
# => Collection([String("john.doe@example.com")])
```

### Key libraries (crates)

- rh-codegen — FHIR → Rust code generation
  - Generates idiomatic Rust types (resources, datatypes, bindings) from StructureDefinitions.
  - Emits compile‑time metadata for path resolution and FHIRPath evaluation.
  - Pairs well with the loader to fetch packages and generate types in one step.

- rh-fhirpath — Full‑featured FHIRPath engine
  - Parser and evaluator implemented in pure Rust (nom‑based), with a friendly API and CLI/REPL.
  - Supports path navigation, arithmetic, string ops, collections, and temporal literals; includes unit conversion helpers.

- rh-vcl — ValueSet Compose Language parser + translator
  - Parses the VCL grammar and translates to FHIR `ValueSet.compose`.
  - “Explain” mode produces clear, hierarchical, plain‑English descriptions of expressions.
  - WASM‑friendly for web UIs and documentation tooling.

- rh-snapshot — StructureDefinition snapshot generator
  - Merges base + differential (with multi‑level inheritance) and handles slicing, extensions, and constraints.
  - Caches results for O(1) repeat generation and includes 100+ tests for correctness.

- rh-validator — High‑performance FHIR validator
  - Validates resources against profiles with cardinality, types, slicing, bindings, and FHIRPath invariants.
  - Outputs standard OperationOutcome (R4) and supports NDJSON batch validation with caching.
  - Integrates directly into the `rh validate` CLI for both text and JSON outputs.

- rh-loader — FHIR package loader
  - Downloads packages from npm‑style registries (e.g., packages.fhir.org) with retries and optional auth.
  - Extracts and organizes packages under `~/.fhir/packages` and exposes helpers to list/get versions.

- rh-foundation — Shared utilities
  - Common error handling, JSON and I/O helpers, CLI utilities, optional HTTP client, and WASM helpers.

- rh-hl7_fhir_r4_core — Generated FHIR R4 types for Rust
  - A generated crate demonstrating how RH emits types, metadata, and traits for real‑world development.

## Example: From packages to validation in minutes

```bash
# 1) Discover a package version
cargo run -p rh -- download list hl7.fhir.r4.core --latest

# 2) Download packages to the default cache
cargo run -p rh -- download package hl7.fhir.r4.core 4.0.1

# 3) Generate or use snapshots and validate resources
cargo run -p rh -- snapshot info \
  --package hl7.fhir.r4.core@4.0.1 \
  http://hl7.org/fhir/StructureDefinition/Patient

cargo run -p rh -- validate resource --input patient.json --format json
```

### Short CLI Movie

Record a real session and keep it reproducible:

```bash
# Record (requires asciinema)
bash scripts/record_cli_demo.sh

# Play locally
asciinema play assets/cli-demo.cast
```

If you want to further trim or stylize it, use `asciinema edit` to cut dead time, or convert to GIF with tools like agg/svg-term for embedding.

## What makes it exciting

- Speed you can feel: native Rust performance, efficient parsers, and caching across snapshotting and validation.
- A first‑class CLI: discoverable, consistent, and composable—great for demos and production pipelines.
- Web‑ready tooling: compile parsers and explainers to WebAssembly for docs, UIs, and sandboxed runtimes.
- Clean abstractions: libraries are loosely coupled but integrate smoothly through the CLI.
- Developer ergonomics: strong types, clear errors, and simple, testable modules.

## Ambitious roadmap

These are the big bets we’re excited about exploring. If any of these light you up, we’d love your feedback and help.

- R4B and R5 support
  - Expand codegen, snapshots, and validation across versions; smooth cross‑version migration utilities.

- Terminology at scale
  - Pluggable terminology backends and server integration; performant code validation and expansions.

- SQL‑on‑FHIR views and query runner (`rh-sql`)
  - Define reusable analytical views over FHIR; run them locally or in pipelines.

- CQL tooling (`rh-cql`)
  - Parse, evaluate, and package CQL, with FHIR model wiring and measure workflows.

- FHIR Shorthand (FSH) utilities (`rh-fsh`)
  - Convert and validate FSH; integrate with publisher flows and snapshot generation.

- Publisher and packaging (`rh-publisher`)
  - Opinionated tools to assemble, validate, and publish FHIR packages and IG assets.

- Language bindings and SDKs
  - Thin bindings for Python/Node via WASM/FFI to bring RH’s speed to more ecosystems.

- Editor/IDE integrations
  - Language‑server features for FHIRPath and VCL; inline explainers and type metadata.

- Cloud‑native services
  - Containerized microservices for validation, terminology, and snapshot APIs; horizontal scaling and caching layers.

## Who is RH for?

- Product teams building FHIR‑powered apps and services.
- Data teams operating pipelines over NDJSON bundles and registries.
- Standards and terminology experts who need portable, scriptable tooling.
- Educators and implementers who want fast, self‑contained demos and workshops.

## Get involved

- Try the CLI: `cargo run -p rh -- --help`
- Browse the crates in `crates/` and the CLI in `apps/rh-cli/`
- File issues or ideas, or open a PR—contributions welcome!

If you’re evaluating RH for a team or project, we’d love to hear from you—what problems should we solve next?

— The RH maintainers
