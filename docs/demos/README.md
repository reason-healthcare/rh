# FHIR DevDays 2025 — Live Demos

Scripts and data files for the `rh` live demo session.

## Prerequisites

```bash
# Install rh (any method)
brew tap reason-healthcare/rh && brew install rh

# Or from source (this repo)
cargo build --release -p rh-cli
export RH=./target/release/rh
```

## Running the demos

```bash
cd docs/demos

# Interactive — press Enter between each demo
./run-demo.sh

# Auto-advance (3-second pauses)
./run-demo.sh auto

# Single demo
./run-demo.sh 2        # run only demo 2
```

## Demo catalogue

| # | Title | Commands | What it shows |
|---|---|---|---|
| 1 | **Instant startup** | `time rh --version` | 7 ms cold start vs JVM 3–8 s |
| 2 | **FHIRPath** | `rh fhirpath eval` | Navigate and filter FHIR resources |
| 3 | **Validate one resource** | `rh validate resource` | Valid ✅ and invalid ❌ with messages |
| 4 | **Batch validation** | `rh validate batch` | 1,000 resources in ~0.6 s |
| 5 | **FSH → FHIR JSON** | `rh fsh compile` | Profile authoring in milliseconds |
| 6 | **CQL → ELM** | `rh cql compile` + `explain` | Full compiler in 9 ms |
| 7 | **VCL** | `rh vcl explain` + `translate` | ValueSet DSL → FHIR compose |
| 8 | **Pipes** | `rh fsh … \| rh validate` | Unix composability |

## Data files

| File | Description |
|---|---|
| `data/patient.json` | A realistic Patient resource used in demos 2, 3, 8 |
| `data/patient-invalid.json` | An invalid Patient for demo 3 |
| `data/demo-profiles.fsh` | A FSH profile for demo 5 |
| `data/demo-measure.cql` | A CQL measure library for demo 6 |
| `data/patients-1000.ndjson` | 1,000 Patient resources for demo 4 |

## Performance slides

See `../devdays-2025-perf.md` — 13 slides with real benchmark numbers
comparing rh to HAPI FHIR (Java), Firely (.NET), fhirpath.js (Node), and SUSHI.

Render to HTML:
```bash
npx @marp-team/marp-cli@latest ../devdays-2025-perf.md --html
```
