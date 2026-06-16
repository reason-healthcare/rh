---
marp: true
header: '<div class="rh-devdays-header"><img class="fhir-logo" src="fhir-logo.svg" alt="" /><span class="brand"><strong>HL7</strong><sup>®</sup> <strong>FHIR</strong><sup>®</sup> &nbsp;DevDays</span><span class="stamp">JUNE | 2026</span></div>'
theme: default
paginate: true
style: |
  section {
    font-family: 'Aptos', 'Segoe UI', system-ui, sans-serif;
    background:
      linear-gradient(
        to bottom,
        #ffffff 0 82px,
        #ececec 82px calc(100% - 36px),
        #f5d71f calc(100% - 36px) 100%
      );
    color: #445674;
    font-size: 25px;
    line-height: 1.28;
    padding: 92px 86px 54px;
  }
  section:not(.lead) {
    place-content: start start;
  }
  section > * {
    position: relative;
    z-index: 1;
  }
  section::after {
    font-size: 16px;
    color: #445674;
    right: 34px;
    bottom: 6px;
  }
  section > header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 82px;
    padding: 0;
    box-sizing: border-box;
    color: #3f3f3f;
    font-size: 22px;
    background: #ffffff;
  }
  section > header .rh-devdays-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    padding: 0 42px;
    box-sizing: border-box;
  }
  section > header .rh-devdays-header > :nth-child(1),
  section > header .rh-devdays-header > :nth-child(2) {
    display: inline-flex;
    align-items: center;
  }
  .fhir-logo {
    height: 1.4em;
    margin-right: 6px;
  }
  section > header .brand {
    font-size: 1.18em;
    letter-spacing: 0.01em;
    font-weight: 400;
    white-space: nowrap;
  }
  section > header .brand sup {
    font-size: 0.55em;
  }
  section > header .stamp {
    font-size: 0.92em;
    font-weight: 700;
    letter-spacing: 0.06em;
    margin-left: auto;
  }
  h1 {
    color: #1092bb;
    font-size: 1.52em;
    font-weight: 500;
    margin: 0 0 0.55em;
  }
  h2 {
    color: #1092bb;
    font-size: 1.2em;
    font-weight: 500;
    margin: 0 0 0.55em;
    padding-top: 8px;
  }
  h3 {
    color: #516176;
    font-size: 0.98em;
    font-weight: 500;
  }
  p,
  ul,
  ol,
  blockquote {
    margin-top: 0.3em;
    margin-bottom: 0.5em;
  }
  ul,
  ol {
    padding-left: 1.15em;
  }
  li {
    margin-bottom: 0.28em;
    font-size: 0.9em;
  }
  strong {
    color: #31445f;
  }
  a {
    color: #1092bb;
  }
  blockquote {
    color: #3d5068;
    font-style: italic;
    font-size: 0.88em;
    background: linear-gradient(135deg, #eef3f8, #e4ecf4);
    border-left: 4px solid #1092bb;
    border-radius: 0 6px 6px 0;
    padding: 0.6em 1em;
    margin: 0.6em 0;
  }
  code {
    background: transparent;
    color: #2f5575;
    padding: 0.08em 0.24em;
    border-radius: 4px;
  }
  pre {
    background: #e6edf3;
    color: #2f5575;
    border-radius: 8px;
    padding: 0.65em 1em;
    font-size: 0.68em;
    line-height: 1.35;
    border: 1px solid #d2dde7;
  }
  table {
    font-size: 0.72em;
    border-collapse: collapse;
    background: rgba(255, 255, 255, 0.72);
  }
  th {
    background: #dfe7ee;
    color: #31445f;
    text-align: left;
    padding: 0.45em 0.8em;
    border-bottom: 2px solid #c7d2dc;
  }
  td {
    padding: 0.34em 0.8em;
    border-bottom: 1px solid #d7e0e8;
  }
  .pill {
    display: inline-block;
    background: #ffffff;
    border: 1px solid #c5d3df;
    border-radius: 999px;
    padding: 0.1em 0.7em;
    font-size: 0.75em;
    color: #445674;
    margin: 0.1em;
  }
  .note {
    color: #6d7d94;
    font-size: 0.68em;
    margin-top: 0.5em;
  }
  section.lead {
    background-image: url('devdays-background.png');
    background-size: cover;
    background-position: center;
    padding: 116px 74px 54px;
  }
  section.lead::before {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
    background:
      linear-gradient(to bottom, #ffffff 0 82px, rgba(0, 0, 0, 0.45) 82px);
    z-index: 0;
  }
  section.lead h1 {
    display: block;
    margin: 8px 34px 0;
    padding: 20px 28px 18px;
    text-align: center;
    font-size: 1.42em;
    color: #ffffff;
    background: rgba(255, 226, 50, 0.92);
    box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.35);
    border-radius: 6px 6px 0 0;
  }
  section.lead h3 {
    display: block;
    margin: 0 34px 28px;
    padding: 12px 24px 14px;
    text-align: center;
    font-size: 0.9em;
    color: #ffffff;
    background: rgba(122, 115, 119, 0.85);
    letter-spacing: 0.01em;
    box-shadow: -2px 2px 0 rgba(255, 255, 255, 0.35), 2px 2px 0 rgba(255, 255, 255, 0.35), 0 2px 0 rgba(255, 255, 255, 0.35);
  }
  section.lead p {
    color: #ffffff;
    font-size: 0.98em;
    max-width: 9.5em;
    margin-left: 30px;
    text-shadow: 0 1px 0 rgba(0, 0, 0, 0.14);
  }
  section.lead p strong {
    display: block;
    color: #ffd334;
    font-size: 1.26em;
    line-height: 1.08;
    margin-bottom: 0.28em;
  }
  section.lead .attribution {
    box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.35);
    background-color: rgba(122, 115, 119, 0.85);
    color: #ffffff;
    padding: 14px 24px 16px;
    margin: 0 34px;
    border-radius: 0 0 6px 6px;
    font-size: 0.88em;
    text-shadow: none;
    max-width: none;
    white-space: nowrap;
    position: relative;
    overflow: hidden;
  }
  section.lead .attribution a {
    color: #ffffff;
  }
  section.lead .attribution::after {
    content: '';
    position: absolute;
    right: -30px;
    top: 50%;
    transform: translateY(-50%);
    width: 220px;
    height: 220px;
    background-image: url('reason-health-logo.png');
    background-repeat: no-repeat;
    background-position: center;
    background-size: contain;
    opacity: 0.35;
    filter: grayscale(100%);
    pointer-events: none;
  }
  section.lead .attribution strong {
    color: #ffffff;
    font-size: 1.1em;
  }
---

<!-- _class: lead -->

# ![Rust Health](rust-health-logo.svg)

### A high-performance FHIR toolkit — built in Rust

<div class="attribution">

**FHIR DevDays 2026**
Brian Kaney · Vermonster / Reason Healthcare
https://reason.health

</div>

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

https://github.com/bkaney/aic

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
┌──────────────────────────────────────────────────────┐
│                        rh-cli                        │  single binary
└──────┬──────┬────────┬───────┬─────────┬──────┬──────┘
       │      │        │       │         │      │
   codegen  fhirpath  fsh  validator  packager cql
       │      │        │       │         │      │
       └──────┴────────┴───────┴─────────┴──────┘
                       │
            rh-hl7-fhir-r4-core   ← generated R4 types
            rh-hl7-fhir-r5-core   ← generated R5 types  🆕
```

**11 Rust workspace packages** — CLI plus independently reusable crates, with the WASM-facing crates also wrapped for npm

---

## FHIR Types — `rh-hl7-fhir-r4-core` & `rh-hl7-fhir-r5-core`

```bash
rh codegen hl7.fhir.r5.core 5.0.0 --output crates/rh-hl7_fhir_r5_core
```

```rust
use rh_hl7_fhir_r5_core::resources::patient::{Patient, PatientMutators};

let patient = Patient::new()
    .set_id("p-001".to_string())
    .set_active(true);
```

- FHIR **R4** and **R5** 🆕
- Generated structs with serde, builders, and compile-time metadata
- Bindings for `required`-strength ValueSet enums
- `rh-codegen` downloads packages, parses StructureDefinitions, and emits idiomatic Rust

<span class="pill">R4 ✅</span> <span class="pill">R5 ✅ NEW</span>

---

## FHIRPath — `rh-fhirpath`

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

---

## ValueSet DSL — `rh-vcl` *(experimental)*

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

---

## FSH Compiler — `rh-fsh`

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

---

## CQL Engine — `rh-cql`

> *CQL → ELM — a full compiler in pure Rust*

Three-stage pipeline: **Parse → Semantic Analysis (Typed AST) → ELM Emit**

- CQL-to-ELM compiler — compatible ELM JSON for the supported subset
- Source maps: CQL span ↔ ELM node correlation
- Explain mode: human-readable parse tree and typed AST
- **ELM/CQL execution engine** — evaluate supported CQL expressions against FHIR data
- Pure-Rust ELM evaluator with TVL (three-valued logic)
- Context, CodeProperty, and CodeRef in ELM output ✅ (recent fix)

```bash
rh cql compile myLibrary.cql --format json
rh cql eval   myLibrary.cql --expression "InDemographic"
```

---

## FHIR Validator — `rh-validator`

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

---

## FHIR Packager — `rh-packager`

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

Typed npm packages backed by the same Rust crates:

- **`@reasonhealth/fhirpath`** — FHIRPath parse/eval
- **`@reasonhealth/vcl`** — VCL explain/translate
- **`@reasonhealth/cql`** — CQL compile/evaluate

Targets: **web**, **Node.js**, and **bundlers**.

### Playground

https://reason-healthcare.github.io/rh/

```
pnpm --filter @reasonhealth/playground dev
```

FHIRPath + VCL + CQL run in the browser with no server round-trip.

---

## For Agents

> *The CLI that works as well for code as for humans*

- Every command has `--format json` output
- Stable flag names and exit codes
- Piping: `rh fhirpath eval … | rh validate --stdin`
- CI gates now check docs drift, MSRV, WASM compilation, and npm package/playground builds
- `rh` is already used in agent-driven IG authoring workflows at Reason Healthcare

The AIC experiment taught us: **regularity and predictability matter** — for LLMs as much as for compilers.

> An excellent CLI is the best agent API.

---

## Performance, In Context

| Scenario | `rh` demo number | Reference implementation context | Fair comparison note |
|---|---:|---:|---|
| CLI startup | **~7 ms** | JVM tools often **3–8 s**; Node/.NET CLIs often **~180–400 ms** | Cold process wall time; runtime startup dominates references |
| FHIRPath CLI eval | **~7 ms** | Node/.NET CLIs **~200–400 ms**; Java CLI **seconds** | Single expression over small Patient JSON; not an in-process engine benchmark |
| CQL compile | **~9 ms** | Java CQL Translation Service commonly **seconds** when cold | `rh` supports a growing subset; Java reference has broader maturity |
| Batch validation | **1,000 Patients in ~0.61 s** | Java validators are strong but heavier to start and host | Base/profile rules with cached snapshots; terminology depth differs by setup |
| Distribution | **single native binary + WASM packages** | JVM/.NET/Node require installed runtimes | Smaller deploy surface, but reference stacks have deeper ecosystem coverage |

**Transparent claim:** Rust gives `rh` a major advantage for cold-start CLIs, CI loops, edge/browser deployment, and agent workflows. The reference implementations still lead in specification breadth, IG Publisher integration, terminology-server depth, and community validation history.

---

## rh in a CI pipeline

```yaml
# .github/workflows/fhir-ci.yml
- name: Validate FHIR resources
  run: |
    curl -fsSL https://install.reason.health/rh | sh
    rh validate batch -i output/*.ndjson --strict

- name: Compile FSH
  run: rh fsh compile input/ --output fhir-output/

- name: Check CQL
  run: rh cql validate measures/*.cql
```

**Total added CI time: < 2 seconds** (including download + all validations)

vs. Java-based CI: ~30–60 s JVM startup alone

---

## Thank You

### ![Rust Health](rust-health-logo.svg) `github.com/reason-healthcare/rh`

```bash
cargo install rh-cli
rh --help
```

**Brian Kaney**
Reason Healthcare
`@bkaney`

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
