---
marp: true
theme: default
paginate: true
style: |
  section {
    font-family: 'Segoe UI', system-ui, sans-serif;
    background: #0f1117;
    color: #e2e8f0;
  }
  h1 { color: #f97316; font-size: 2rem; margin-bottom: 0.3em; }
  h2 { color: #fb923c; font-size: 1.4rem; margin-bottom: 0.3em; }
  h3 { color: #fdba74; font-size: 1.1rem; }
  a  { color: #60a5fa; }
  code, pre { background: #1e2533; color: #93c5fd; border-radius: 6px; }
  pre { padding: 0.5em 1em; font-size: 0.8em; }
  table { font-size: 0.8em; border-collapse: collapse; width: 100%; }
  th { background: #1e2533; color: #fb923c; padding: 0.4em 0.8em; }
  td { padding: 0.3em 0.8em; border-bottom: 1px solid #2d3748; }
  ul, ol { margin-top: 0.3em; }
  li { margin-bottom: 0.25em; font-size: 0.92em; }
  .pill {
    display: inline-block;
    background: #1e2533;
    border: 1px solid #fb923c;
    border-radius: 999px;
    padding: 0.1em 0.7em;
    font-size: 0.75em;
    color: #fb923c;
    margin: 0.1em;
  }
  section.lead h1 { font-size: 2.6rem; }
  section.lead p  { font-size: 1.1rem; color: #94a3b8; }
  section.lead .logo { font-size: 3rem; }
---

<!-- _class: lead -->

# 🦀 Rust Health (`rh`)

### A high-performance FHIR toolkit — built in Rust

**FHIR DevDays 2025**
Brian Kaney · Reason Healthcare

---

## The Question That Started This

> *What if we designed a language optimised for AI agents to write?*

In early 2024 we sketched **AIC** — an experimental compiled language intended for LLM-first authoring:

- Deterministic semantics, no hidden mutations
- Canonical formatter enforced at compile time
- One obvious way to express every construct
- Typed holes (`_`) so partial programs still check
- Short familiar keywords (`fn`, `let`, `match`, `async`)
- ARC memory — no lifetimes, no borrow checker

A pure **token-economy language** where the first compile attempt should succeed.

---

## AIC Looked Very Familiar

```aic
module http.client;
use std::net::http;
use std::json as json;

pub struct User { id: u64; name: string; }
pub enum HttpErr { Status(u16); Decode(string); }

pub async fn fetch_user(id: u64) -> Result<User, HttpErr> effects(net) {
    let url = fmt::format("/users/{0}", id);
    let resp = await http.get(url);
    if resp.status != 200 { return Err(HttpErr::Status(resp.status)); }
    return Ok(json.decode(resp.body).map_err(HttpErr::Decode)?);
}
```

*Structs. Enums. Traits. Pattern matching. Async. Result<T,E>.*

> We had designed… **Rust** — but with ARC and no macros.

---

## So We Chose Rust

AIC and Rust converge on the same core ideas. Rust already has:

| AIC Goal | Rust Reality |
|---|---|
| Deterministic semantics | ✅ Ownership + borrow checker |
| Regular syntax | ✅ Consistent, no magic |
| Compiled to native | ✅ LLVM backend |
| WASM target | ✅ `wasm32-unknown-unknown` |
| No runtime overhead | ✅ Zero-cost abstractions |
| Excellent tooling | ✅ `cargo`, `clippy`, `rustfmt` |

And Rust has a mature ecosystem, a thriving community, and **10 years of production hardening**.

> We chose Rust and applied the AIC design lessons to *how we write* Rust.

---

## Why Rust for FHIR?

The FHIR ecosystem runs on JVM and .NET. That's fine — but there are tradeoffs:

| | JVM / .NET | **rh (Rust)** |
|---|---|---|
| Startup time | Seconds (JVM warm-up) | **Milliseconds** |
| Memory | Hundreds of MB (GC heap) | **Single-digit MB** |
| Native binary | ❌ Requires runtime | **✅ Single static binary** |
| WebAssembly | Limited | **✅ First-class** |
| Cross-platform | Runtime install | **✅ No install needed** |
| Throughput | Good | **3–10× faster** typical |

**Healthcare data is sensitive.** No runtime, no dependency on vendor JDK/SDK updates, no third-party runtime CVEs.

---

## The CLI First Philosophy

> *A great CLI is great for both humans **and** agents.*

```
rh --help
```

```
rh codegen    Generate Rust types from FHIR packages
rh fhirpath   Parse and evaluate FHIRPath expressions
rh fsh        Compile FSH to FHIR JSON
rh validate   Validate FHIR resources against profiles
rh cql        Compile and evaluate CQL
rh package    Assemble, lock, and publish FHIR packages
rh download   Download FHIR packages from registries
```

- Composable Unix-style commands — pipe `rh` output into `rh`
- Machine-readable JSON output
- `--help` everywhere, stable flag names
- Works in CI, Docker, agent loops, and human terminals equally well

---

## Architecture Overview

```
┌─────────────────────────────────────────────┐
│                  rh-cli                      │  single binary
└──────┬──────┬──────┬──────┬──────┬──────────┘
       │      │      │      │      │
   codegen  fhirpath  fsh  validator  packager
       │      │      │         │       │
       └──────┴──────┴─────────┴───────┘
                       │
                rh-foundation         ← shared base
                       │
          rh-hl7-fhir-r4-core   ← generated R4 types
          rh-hl7-fhir-r5-core   ← generated R5 types  🆕
```

**10 crates** — each independently versioned and published to crates.io

---

## `rh-foundation`

> *The bedrock everything else stands on*

- Common error types with context chaining
- Async HTTP client (`reqwest` wrapper)
- FHIR package loader — download from `packages.fhir.org`
- StructureDefinition snapshot generator (differential merging)
- JSON utilities, CLI I/O patterns
- WASM feature flag for browser targets

```toml
rh-foundation = { version = "0.2", features = ["http"] }
```

<span class="pill">crates.io</span> <span class="pill">WASM</span>

---

## `rh-hl7-fhir-r4-core` & `rh-hl7-fhir-r5-core`

> *All FHIR types — generated, typed, serializable*

```bash
rh codegen hl7.fhir.r5.core 5.0.0 \
  --output crates/rh-hl7_fhir_r5_core \
  --crate-name rh-hl7-fhir-r5-core
```

```rust
use rh_hl7_fhir_r5_core::resources::patient::{Patient, PatientMutators};

let patient = Patient::new()
    .set_id("p-001".to_string())
    .set_active(true)
    .add_name(HumanName { family: Some("Smith".to_string()), .. });
```

- **R4**: 1,388 public types · **R5**: 161 resources + 50 profiles 🆕
- Generated structs with builder traits, serde, and compile-time metadata
- Bindings for `required`-strength ValueSet enums

<span class="pill">R4 ✅</span> <span class="pill">R5 ✅ NEW</span>

---

## `rh-codegen`

> *StructureDefinitions → idiomatic Rust*

- Downloads FHIR packages from any npm-style registry
- Parses StructureDefinitions, generates `struct` + builder traits
- Required ValueSet bindings → typed enums
- Compile-time metadata maps for FHIRPath resolution
- Emits `ValidatableResource` impls with invariants + cardinalities
- Handles R4 and R5 differences transparently

```bash
rh codegen hl7.fhir.r4.core 4.0.1 \
  --output ./my-r4-types \
  --crate-name my-fhir-r4
```

<span class="pill">crates.io</span>

---

## `rh-fhirpath`

> *FHIRPath — fast, embeddable, WASM-ready*

- Full FHIRPath parser (nom combinators → AST)
- Evaluator against `serde_json::Value` FHIR resources
- FHIR extensions: `extension()`, `resolve()`, `conformsTo()`
- SQL extensions: `getResourceKey()`, `getReferenceKey()`
- WASM build — use in browsers and Node.js today

```bash
rh fhirpath eval 'Patient.name.where(use = "official").family' \
  --resource patient.json
```

<span class="pill">crates.io</span> <span class="pill">WASM</span>

---

## `rh-vcl`

> *ValueSet Compose Language — a human-readable DSL for ValueSets*

VCL is an ECL-inspired compact syntax for ValueSet definitions, proposed in [FHIR IG Guidance](https://build.fhir.org/ig/FHIR/ig-guidance/vcl.html).

```vcl
http://snomed.info/sct :: 73211009 |Diabetes mellitus|
  OR descendants-of 44054006 |Diabetes mellitus type 2|
```

- Parses VCL → FHIR `ValueSet.compose`
- Hierarchical explain with nesting visualisation
- WASM support — embed in any web tooling

```bash
rh vcl explain "SCT :: descendants-of 44054006"
```

<span class="pill">crates.io</span> <span class="pill">WASM</span>

---

## `rh-fsh`

> *FSH → FHIR JSON — fast enough to use in CI*

- `nom`-based FSH parser
- Exports `StructureDefinition`, `ValueSet`, `CodeSystem`, `Instance`
- `rayon` parallel export — multi-core throughput
- Designed for high-volume IG builds

```bash
rh fsh compile --input ./fsh-source --output ./fhir-json
```

```rust
let result = compile_fsh(fsh_source)?;
// result.structure_definitions, .value_sets, .code_systems, ...
```

<span class="pill">crates.io</span>

---

## `rh-cql`

> *CQL → ELM — a full compiler in pure Rust*

Three-stage pipeline: **Parse → Semantic Analysis (Typed AST) → ELM Emit**

- Complete CQL-to-ELM compiler — compatible with the reference implementation
- Source maps: CQL span ↔ ELM node correlation
- Explain mode: human-readable parse tree and typed AST
- Pure-Rust ELM evaluator with TVL (three-valued logic)
- Context, CodeProperty, and CodeRef in ELM output ✅ (recent fix)

```bash
rh cql compile myLibrary.cql --format json
rh cql eval   myLibrary.cql --expression "InDemographic"
```

<span class="pill">crates.io</span>

---

## `rh-validator`

> *Hybrid FHIR validator — fast enough for real-time use*

- Validates JSON FHIR resources against base R4 rules
- Profile-driven: loads StructureDefinition snapshots at runtime
- FHIRPath invariant evaluation (via `rh-fhirpath`)
- Slicing, extension, cardinality, and binding checks
- Optional terminology service integration (any FHIR TS)
- LRU caching — sub-millisecond repeated validation
- QuestionnaireResponse validation

```bash
rh validate patient.json --profile http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient
```

<span class="pill">crates.io</span>

---

## `rh-packager`

> *Assemble conformant FHIR packages from source*

End-to-end package pipeline in a single `packager.toml`:

```toml
[package]
name = "my.ig"
version = "1.0.0"

[processors]
snapshot  = { enabled = true }
validate  = { enabled = true }
cql       = { enabled = true }
fsh       = { enabled = true }
```

```bash
rh package init my-ig      # scaffold
rh package lock my-ig      # pin canonical references
rh package build my-ig     # assemble + process
```

<span class="pill">crates.io</span>

---

## Install Anywhere

```bash
# macOS / Linux
brew tap reason-healthcare/rh && brew install rh

# Linux (curl-pipe)
curl -fsSL https://raw.githubusercontent.com/reason-healthcare/rh/main/scripts/install-rh.sh | sh

# Windows
choco install rh

# Docker
docker pull ghcr.io/reason-healthcare/rh:latest

# Cargo
cargo install rh-cli
```

Single static binary. No JDK. No .NET. No `npm install`.

---

## WASM: FHIR in the Browser

Three crates ship WebAssembly packages:

| Crate | WASM Use Case |
|---|---|
| `rh-fhirpath` | Evaluate FHIRPath in a browser form |
| `rh-vcl` | VCL editing and explain in a web UI |
| `rh-foundation` | Package loading in WASM contexts |

```bash
just wasm         # build all WASM targets
```

Three targets each: **web**, **Node.js**, **bundler (webpack)**

---

## For Agents

> *The CLI that works as well for code as for humans*

- Every command has `--format json` output
- Stable flag names and exit codes
- Piping: `rh fhirpath eval … | rh validate --stdin`
- `rh` is already used in agent-driven IG authoring workflows at Reason Healthcare

The AIC experiment taught us: **regularity and predictability matter** — for LLMs as much as for compilers.

> An excellent CLI is the best agent API.

---

## What's Next

| | |
|---|---|
| 🆕 `rh-hl7-fhir-r5-core` | R5 generated types — **just landed** |
| 🔜 `rh-hl7-fhir-r6-core` | R6 types as spec stabilises |
| 🔜 R5 validator | Profile validation for R5 resources |
| 🔜 `rh-smart` | SMART App Launch / PKCE helpers |
| 🔜 `rh-subscriptions` | R5 Subscription topic processing |
| 🔜 WASM npm packages | Published to npm registry |
| 🔜 Language Server | LSP for FSH and CQL in editors |

All open source · MIT OR Apache-2.0

---

## Thank You

<br>

### 🦀 `github.com/reason-healthcare/rh`

```bash
cargo install rh-cli
rh --help
```

<br>

**Brian Kaney**
Reason Healthcare
`@bkaney`

<br>

*Questions?*

---

<!-- _class: lead -->

## Appendix: The AIC → Rust Journey

---

## AIC: Key Insights That Shaped `rh`

When we designed AIC, we discovered that the properties making a language good for LLMs are the *same* properties that make code easy to audit and maintain:

1. **Regularity** — one way to do each thing → we write consistent idiomatic Rust
2. **Explicitness** — no magic imports → every `use` in `rh` is deliberate
3. **Local reasoning** — types at the call site → rich return types everywhere
4. **Canonical format** — `rustfmt` + `just fmt` enforced in CI
5. **Actionable diagnostics** — `clippy -D warnings` in `just check`
6. **Typed holes** → `todo!()` and `unimplemented!()` as checkpoints in dev

AIC became our *style guide* for writing Rust. `rh` is what happens when you take those lessons seriously.
