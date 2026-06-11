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
  h3 { color: #fdba74; font-size: 1.05rem; }
  a  { color: #60a5fa; }
  code, pre { background: #1e2533; color: #93c5fd; border-radius: 6px; }
  pre { padding: 0.5em 1em; font-size: 0.8em; }
  table { font-size: 0.82em; border-collapse: collapse; width: 100%; }
  th { background: #1e2533; color: #fb923c; padding: 0.4em 0.8em; text-align: left; }
  td { padding: 0.3em 0.8em; border-bottom: 1px solid #2d3748; }
  ul, ol { margin-top: 0.3em; }
  li { margin-bottom: 0.2em; font-size: 0.9em; }
  .fast  { color: #4ade80; font-weight: bold; }
  .slow  { color: #f87171; }
  .note  { color: #94a3b8; font-size: 0.75em; margin-top: 0.5em; }
  .bar-wrap { background: #1e2533; border-radius: 4px; height: 22px; width: 100%; margin: 3px 0; }
  section.lead h1 { font-size: 2.4rem; }
  section.lead p  { color: #94a3b8; font-size: 1.1rem; }
---

<!-- _class: lead -->

# ⚡ rh Performance
### Real measurements vs. reference implementations

---

## What we measured (macOS Apple Silicon M2)

All numbers are **wall-clock time from process spawn** — no warm-up tricks.

Reference implementations tested:

| Tool | Language | Version |
|---|---|---|
| **rh** | Rust | 0.2.0-beta.2 |
| HAPI FHIR Validator | Java 17 | 6.2.x |
| Firely .NET SDK CLI | .NET 6 | 4.x |
| fhirpath.js (Node.js) | JavaScript | 3.x |
| SUSHI | Node.js / TypeScript | 3.x |

> Java and .NET numbers from published benchmarks and community measurements.
> rh numbers are reproducible: `./docs/demos/run-demo.sh`

---

## Startup time

> *"How long before I get any output?"*

```
$ time rh --version
rh 0.2.0-beta.2

real  0m0.007s      ← 7 milliseconds
```

| Tool | Cold startup | Notes |
|---|---|---|
| **rh** | **7 ms** | Native binary, instant |
| fhirpath.js | ~200 ms | Node.js runtime init |
| Firely .NET CLI | ~180 ms | .NET runtime init |
| HAPI FHIR Validator | **3 000–8 000 ms** | JVM class loading |
| HAPI FHIR Server | **15 000–30 000 ms** | Spring + JVM boot |

<p class="note">* JVM warm-up documented in HAPI FHIR issue tracker and HL7 FHIR Chat.</p>

---

## Binary distribution

> *"What do I need to install?"*

| | **rh** | Java tools | .NET tools | Node tools |
|---|---|---|---|---|
| Binary size | **17 MB** | 400–600 MB JAR+JDK | 80–200 MB | 50–150 MB + node |
| Runtime required | **None** | JDK 11–17 | .NET 6–8 | Node.js 16+ |
| Install command | `brew install rh` | Install JDK, download JAR | Install SDK, install tool | `npm install -g` |
| Works offline | **Yes** | Yes (after JDK install) | Yes | Yes |
| Works in Alpine Linux | **Yes** | Requires JDK layer | Requires .NET layer | Requires Node layer |
| Docker image size | **~25 MB** | ~400 MB with JRE | ~200 MB | ~150 MB |

<p class="note">Single static binary — copy it anywhere, it runs.</p>

---

## FHIRPath evaluation

```
$ time rh fhirpath eval "Patient.name.where(use='official').family" \
       -d patient.json

✅  Result: String("van der Berg")
real  0m0.007s
```

| Tool | Single expression | Notes |
|---|---|---|
| **rh** | **7 ms** | Includes process spawn |
| fhirpath.js (CLI) | ~200–400 ms | Node startup dominates |
| Java FHIRPath (HAPI) | ~3 000–5 000 ms | JVM startup dominates |
| Firely .NET FHIRPath | ~200–400 ms | .NET startup dominates |

> For **library-embedded** (in-process) use, rh is ~50–200 µs per expression.
> The bottleneck for all tools in CLI mode is **runtime startup**, not evaluation.

<p class="note">Measurement: macOS M2, 5 runs averaged.</p>

---

## CQL compilation

```
$ time rh cql compile measure.cql

{ "library": { "identifier": { "id": "DevDaysMeasure" ... } } }
real  0m0.009s      ← 9 milliseconds
```

| Tool | Compile (small library) | Notes |
|---|---|---|
| **rh** | **9 ms** | Parse → Semantic → ELM |
| CQL Translation Service (Java) | ~3 000–6 000 ms | JVM startup + ANTLR warm-up |
| CQL Runner (Node.js) | ~400–800 ms | Transpiled from Java |

> rh-cql is a **native Rust implementation** — not a port or wrapper.
> The Java CQL Translation Service is the HL7 reference implementation.

---

## FSH compilation

```
$ time rh fsh compile profile.fsh

{ "resourceType": "StructureDefinition" … }
real  0m0.009s      ← 9 milliseconds
```

| Tool | Compile (1 profile) | 10 profiles | Notes |
|---|---|---|---|
| **rh** | **9 ms** | **15 ms** | Parallel with rayon |
| SUSHI (Node.js) | ~3 000 ms | ~4 000 ms | Full IG toolchain |
| GoFSH (Node.js) | ~2 000 ms | ~3 000 ms | Node startup |

> SUSHI is the reference FSH compiler — it does much more (full IG build).
> rh-fsh focuses on FSH→FHIR JSON as a composable step in a pipeline.

<p class="note">SUSHI numbers: typical reported warm IG build times from HL7 community.</p>

---

## Validation throughput

```
$ time rh validate batch -i patients-1000.ndjson

  ✅ Valid: 1000 / ❌ Invalid: 0
  real  0m0.611s
```

| Resources | Time | Rate | Memory |
|---|---|---|---|
| 1 | 0.36 s | — | ~65 MB |
| 100 | 0.43 s | ~230/s | ~66 MB |
| **1 000** | **0.61 s** | **~1 600/s** | ~69 MB |
| **10 000** | **2.79 s** | **~3 600/s** | ~80 MB |

> First 0.35 s is profile snapshot loading (one-time per run).
> After that: **~0.25 ms/resource** on average.

---

## Validation: memory comparison

> *Validating 1,000 Patient resources*

| Tool | Peak RSS | Notes |
|---|---|---|
| **rh** | **69 MB** | LRU cache, no GC |
| HAPI FHIR Validator | ~512 MB | JVM heap minimum |
| Firely .NET Validator | ~150–250 MB | .NET GC managed |
| HL7 Java Validator | ~400–600 MB | Reflective loading |

> rh uses **~7× less memory** than the Java validator for the same workload.
> Critical in container/serverless environments with tight memory limits.

<p class="note">rh memory from `/usr/bin/time -l`. HAPI memory from HAPI FHIR documentation and issue reports.</p>

---

## Summary: where rh wins

| Scenario | rh advantage |
|---|---|
| CI/CD pipeline | **~400× faster** first result vs JVM (7 ms vs 3–8 s) |
| Container image | **~20× smaller** than Java validator Docker image |
| Batch processing | **~1 600–3 600 resources/sec** validated |
| Memory budget | **~7× less** RAM than Java at equivalent load |
| Edge / WASM | **Native WASM target** — runs in browsers, Cloudflare Workers |
| Install | **Zero runtime deps** — single binary, curl and done |

### Where Java/JVM tools still win
- Full IG Publisher pipeline (decades of build tooling)
- Terminology server integration depth
- FHIR specification coverage breadth
- Community support and documentation

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

<!-- _class: lead -->

## Try it

```bash
brew tap reason-healthcare/rh && brew install rh
rh --help
```

```
github.com/reason-healthcare/rh
```
