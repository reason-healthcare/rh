# FHIR DevDays 2026 — Live Demos

Scripts and fixture data for the `rh` live demo session.

## Prerequisites

```bash
# macOS / Linux — install rh
brew tap reason-healthcare/rh && brew install rh

# Or run from source (this repo)
cargo build --release -p rh-cli
export RH=./target/release/rh
```

## Running the demos

```bash
cd docs/demos

./run-demo.sh          # interactive — press Enter between each demo
./run-demo.sh auto     # auto-advance with 3-second pauses
./run-demo.sh 3        # run a single demo by number
```

## Quick reference

| # | Title | Key command | Headline |
|---|---|---|---|
| 1 | [Instant startup](#demo-1--instant-startup) | `time rh --version` | 7 ms vs JVM 3–8 s |
| 2 | [FHIRPath](#demo-2--fhirpath-evaluation) | `rh fhirpath eval` | Navigate FHIR in milliseconds |
| 3 | [Validate one resource](#demo-3--validate-a-single-resource) | `rh validate resource` | ✅ and ❌ with actionable messages |
| 4 | [Batch validation](#demo-4--batch-validation) | `rh validate batch` | 1,000 resources in ~0.6 s |
| 5 | [FSH → FHIR JSON](#demo-5--fsh--fhir-json) | `rh fsh compile` | Profile authoring in 9 ms |
| 6 | [CQL → ELM](#demo-6--cql--elm-compilation) | `rh cql compile` + `explain` | Full compiler in 9 ms |
| 7 | [VCL](#demo-7--valueset-compose-language-vcl) | `rh vcl explain` + `translate` | ValueSet DSL → FHIR compose |
| 8 | [Pipes](#demo-8--composability-and-pipes) | `rh … \| rh validate` | Unix-style composability |
| 9 | [FHIRPath REPL](#demo-9--fhirpath-repl-run-manually) | `rh fhirpath repl` | Live FHIRPath exploration |
| 10 | [VCL REPL](#demo-10--vcl-repl-run-manually) | `rh vcl repl --explain --translate` | Iterative ValueSet authoring |
| 11 | [CQL REPL](#demo-11--cql-repl-run-manually) | `rh cql repl` | Instant ELM feedback loop |

---

## Demo 1 — Instant startup

```bash
time rh --version
```

```
rh 0.2.0-beta.2
real  0m0.007s
```

**What's happening:** `rh` is a native binary compiled directly to machine code by the Rust compiler (via LLVM). There is no virtual machine, no JIT warm-up, and no class-loading phase. The OS maps the binary into memory and jumps straight to `main`.

**What the output means:** The `real` time — `0.007s` (7 milliseconds) — is the full wall-clock time from the moment the shell forks the process to the moment it exits. This includes file-system lookup, dynamic linker resolution, and the actual logic. It is not a cached or warm result; every invocation starts cold.

**Why it matters for the audience:** The HL7 Java validator and HAPI FHIR tooling start a JVM first. The JVM must load thousands of classes before a single line of application code runs — typically 3–8 seconds on a laptop, 15–30 seconds for a full Spring server. For CI pipelines, agent loops, or repeated scripted calls, that constant overhead compounds. rh's 7 ms is effectively zero overhead.

---

## Demo 2 — FHIRPath evaluation

```bash
rh fhirpath eval "Patient.name.where(use='official').family" \
               -d data/patient.json
```

```
✅ Expression: Patient.name.where(use='official').family
Result: String("van der Berg")
```

```bash
rh fhirpath eval "Patient.name.given" -d data/patient.json
```

```
✅ Expression: Patient.name.given
Result: Collection([String("Pieter"), String("Jan"), String("Pete")])
```

```bash
rh fhirpath parse "Patient.name.where(use='official').family"
```

```
✅ Successfully parsed: Patient.name.where(use='official').family
AST: Patient.name.where(use = 'official').family
```

**What's happening:** The `eval` subcommand parses the FHIRPath expression with `rh-fhirpath` (a nom-based parser), builds an AST, then walks the FHIR JSON document evaluating each node in the path. `where(use='official')` is a filter function that keeps only `HumanName` entries whose `use` field equals `"official"`. `.family` then plucks the `family` string from each matching entry.

The second expression returns a `Collection` because `name.given` is a repeating element — the patient has two official given names (`Pieter`, `Jan`) and one nickname (`Pete`), so all three are collected across both name entries.

`parse` shows the normalized AST without evaluating — useful for debugging expressions or verifying that an expression parses as intended before running it against data.

**What the output means:**
- `String("van der Berg")` — a single-value FHIRPath result; the official family name
- `Collection([…])` — FHIRPath always returns a collection; when there are multiple values they are all listed
- `AST:` — the canonical representation after parsing; whitespace is normalized, implicit steps are made explicit

**Why it matters:** FHIRPath is used throughout FHIR: in invariants, SearchParameter expressions, CQL model mappings, and questionnaire enableWhen conditions. Being able to evaluate or debug an expression from the command line in milliseconds — without standing up a server — is a significant developer-experience win.

---

## Demo 3 — Validate a single resource

**Valid resource:**

```bash
rh validate resource -i data/patient.json
```

```
✅ FHIR resource is valid (Patient)
⚠️  2 warning(s)

Issues:
  1. ⚠️  [Profile: …/us-core-patient] Profile reference has not been checked
             because it could not be found
  2. ⚠️  Failed to parse invariant dom-6: …
```

**Invalid resource (`data/patient-invalid.json`):**

```bash
rh validate resource -i data/patient-invalid.json
```

```
❌ FHIR validation failed (Patient)
  Errors: 1

Issues:
  1. ❌ ele-1: All FHIR elements must have a @value or children [at name[0]]
```

**What's happening:** The validator loads the R4 core StructureDefinitions from the local FHIR package cache (`~/.fhir/packages/hl7.fhir.r4.core`), generates a snapshot for `Patient` (merging differential into the full element list), and then evaluates each compiled rule against the resource JSON.

Rules applied include:
- **Cardinality** — min/max occurrence checks per element
- **Type** — field values match their declared FHIR type
- **Invariants** — FHIRPath expressions attached to elements (e.g., `ele-1` requires every element to have either a value or at least one child)
- **Binding** — codes in bound elements are checked against the ValueSet (when a terminology server is configured)

**What the output means:**
- `✅ FHIR resource is valid` — all error-severity rules passed
- `⚠️` — a warning-severity issue; the resource is still considered valid
- The US Core profile warning tells us the validator found a `meta.profile` reference it couldn't resolve locally — it would need the US Core package installed to validate against it
- `dom-6` is a known FHIRPath invariant in R4 that uses XHTML syntax (`.text.div`) not yet fully supported; this is a known rh limitation
- `ele-1` on the invalid resource catches `name[0]` being an empty JSON object `{}` — a `HumanName` must have at least one child element (family, given, text, etc.)

**Why it matters:** Immediate, local validation without a server. Outputs are human-readable by default and machine-readable with `-f json` — making it equally useful for a developer's inner loop and an automated CI gate.

---

## Demo 4 — Batch validation

```bash
time rh validate batch -i data/patients-1000.ndjson
```

```
📋 Batch Validation Summary:
  Total resources: 1000
  ✅ Valid: 1000
  ❌ Invalid: 0
  Total errors: 0
  Total warnings: 1000

real  0m0.611s
```

**What's happening:** The input is a newline-delimited JSON (NDJSON) file containing one Patient JSON object per line. The validator reads each line, parses it, and runs the full rule set. The StructureDefinition snapshot is compiled **once** on the first resource and then cached in memory using an LRU cache for all subsequent resources of the same type. That is why the per-resource marginal cost drops sharply after the first one.

The 1,000 warnings are the same `dom-6` invariant issue seen in Demo 3 — one per resource. They don't affect the valid/invalid count.

**What the output means:**
- The first ~0.35 s is the snapshot generation for `Patient` — a one-time cost per process invocation
- The remaining ~0.25 s processes all 1,000 resources — roughly **0.25 ms per resource**
- Peak memory for this run is **~69 MB** — the LRU cache holds the compiled snapshot and rules

**Why it matters:** At ~1,600 resources/second on a laptop, rh can validate an entire patient population export, a full IG test suite, or a CI artifact in seconds — without a server. The Java HL7 validator processes roughly 10–50 resources/second in batch mode (after JVM warm-up), uses 512 MB+ of heap, and requires a running JVM.

---

## Demo 5 — FSH → FHIR JSON

```bash
rh fsh compile data/demo-profiles.fsh
```

```json
{
  "abstract": false,
  "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
  "derivation": "constraint",
  "description": "A minimal patient profile for FHIR DevDays",
  "differential": {
    "element": [
      { "id": "Patient.name",       "min": 1, "max": "*" },
      { "id": "Patient.birthDate",  "min": 1, "max": "1" },
      { "id": "Patient.identifier", "min": 1, "max": "*" },
      { "id": "Patient.gender",     "min": 1, "max": "1" }
    ]
  },
  "resourceType": "StructureDefinition",
  ...
}
```

**What's happening:** FHIR Shorthand (FSH) is a domain-specific language for authoring FHIR conformance resources. `rh-fsh` uses a `nom`-based parser to tokenise and parse the FSH grammar, then maps each FSH construct to its FHIR JSON equivalent. The constraint rules (`* name 1..*`) become `ElementDefinition` entries in the `differential` with the appropriate `min`/`max` cardinalities. The binding rule (`* gender from … (required)`) adds an `ElementDefinition` with a `binding` object.

**What the output means:** The output is a valid FHIR R4 `StructureDefinition` JSON, ready to be loaded into a FHIR server or used as input to `rh validate`. The `differential` contains only the elements that the profile constrains beyond its base; a snapshot (the fully-merged element list) can be generated with `rh snapshot generate`.

**Why it matters:** The reference FSH compiler is SUSHI (Node.js). SUSHI is a full IG build orchestrator — it is excellent for building complete Implementation Guides but carries Node.js startup overhead (~3 s) and is designed to operate on a whole IG directory. `rh fsh compile` is a composable step: it takes FSH and produces FHIR JSON in ~9 ms, suitable for use as a pipeline stage or a pre-commit hook.

---

## Demo 6 — CQL → ELM compilation

```bash
rh cql compile data/demo-measure.cql
```

```json
{
  "library": {
    "identifier": { "id": "DevDaysMeasure", "version": "1.0.0" },
    "schemaIdentifier": { "id": "urn:hl7-org:elm", "version": "r1" },
    "usings": { "def": [{ "localIdentifier": "FHIR", "version": "4.0.1" }] },
    "statements": {
      "def": [
        { "name": "In Age Range", "expression": { "type": "In", ... } },
        { "name": "Initial Population", ... },
        ...
      ]
    }
  }
}
```

```bash
rh cql explain data/demo-measure.cql
```

**What's happening:** CQL (Clinical Quality Language) is the authoring language for FHIR quality measures and decision support. `rh-cql` runs a **three-stage pipeline**:

1. **Parse** — tokenises the CQL source and builds a concrete syntax tree using a hand-written recursive descent parser
2. **Semantic analysis** — resolves identifiers, checks types, infers expression types, and builds a typed AST
3. **ELM emission** — walks the typed AST and serialises it to ELM (Expression Logical Model) JSON, the canonical interchange format for CQL

The `explain` command externalises the intermediate representations — useful for debugging a measure or understanding how rh interprets an expression.

**What the output means:** The output is ELM JSON — the same format produced by the HL7 Java CQL Translation Service. It can be handed directly to a CQL engine (such as cql-execution, the JavaScript reference evaluator, or rh's own `rh cql eval`) for execution against patient data.

**Why it matters:** The Java CQL Translation Service is the HL7 reference implementation. It produces correct ELM but requires a JVM (~3–6 s startup) and is typically run as a persistent microservice rather than a CLI tool. `rh cql compile` produces compatible ELM JSON in ~9 ms as a one-shot command, enabling it to be embedded in build pipelines, pre-commit hooks, or agent-driven authoring workflows.

---

## Demo 7 — ValueSet Compose Language (VCL)

```bash
rh vcl explain "concept << 44054006"
```

```
✅ VCL Expression Explanation:

Original VCL:    concept << 44054006
Explanation:     codes where concept is a type of '44054006'
Translatable:    true

Components:
  • concept   (Property): The 'concept' property of codes
  • IsA       (Operator): is a type of
  • '44054006' (Value):   The target value '44054006'
```

```bash
rh vcl translate "concept << 44054006" -s "http://snomed.info/sct" -f json
```

```json
{
  "include": [{
    "system": "http://snomed.info/sct",
    "filter": [{ "property": "concept", "op": "is-a", "value": "44054006" }]
  }]
}
```

**What's happening:** VCL (ValueSet Compose Language) is a compact DSL for expressing ValueSet definitions, proposed in the [FHIR IG Guidance](https://build.fhir.org/ig/FHIR/ig-guidance/vcl.html). It is inspired by SNOMED CT's Expression Constraint Language (ECL) and allows a ValueSet to be described in a short URL-safe string rather than verbose FHIR JSON.

`rh vcl explain` parses the VCL expression into an AST and describes each component in plain English. `rh vcl translate` converts the AST to a `ValueSet.compose` JSON fragment that can be embedded directly in a FHIR `ValueSet` resource.

**What the output means:**
- `concept << 44054006` — in VCL, `<<` means "is-a" (descendants-of), so this selects all SNOMED codes that are sub-types of `44054006` (Diabetes mellitus type 2)
- The translated `filter` with `op: "is-a"` is exactly what a FHIR terminology server expects in a `ValueSet.compose.include` to perform hierarchical expansion
- The `-s` flag supplies the default code system URI for codes that don't include it inline

**Why it matters:** Writing `ValueSet.compose` JSON by hand is verbose and error-prone. VCL lets you express the same intent in a single readable string — short enough to embed in a URL. `rh-vcl` also ships as a **WebAssembly package**, so this same parsing and translation logic can run directly in a browser or Cloudflare Worker without a server round-trip.

---

## Demo 8 — Composability and pipes

```bash
echo '{"resourceType":"Patient","id":"inline"}' | rh validate resource
```

```
✅ FHIR resource is valid (Patient)
```

```bash
rh validate resource -i data/patient.json -f json \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print('issues:', len(d['issues']))"
```

```
issues: 2
```

```bash
rh fhirpath eval "Patient.id" -d data/patient.json -f json | python3 -m json.tool
```

```json
"devdays-demo"
```

```bash
head -5 data/patients-1000.ndjson | rh validate batch
```

```
📋 Batch Validation Summary:
  Total resources: 5  ✅ Valid: 5
```

**What's happening:** Every `rh` command reads from `stdin` when no `-i` file is given, and `-f json` switches any command's output to machine-readable JSON. This makes `rh` a natural participant in Unix pipelines alongside `jq`, `awk`, `curl`, `xargs`, and CI tooling.

The demos show four composability patterns:
1. **JSON-in from stdin** — pipe any FHIR JSON directly to `rh validate resource`; no temp file needed
2. **JSON-out to downstream tools** — `-f json` produces stable, documented JSON; pipe it to Python, jq, or any other tool
3. **FHIRPath JSON mode** — `-f json` on `fhirpath eval` produces the result as a JSON value (not a human label), suitable for programmatic consumption
4. **Streaming batch** — `validate batch` accepts NDJSON from stdin, enabling streaming workflows: `curl … | rh validate batch --strict`

**Why it matters:** This is the Unix philosophy applied to FHIR tooling. Each `rh` subcommand does one thing well and communicates through text streams. An agent, a CI script, or a developer at a terminal can compose these building blocks — validate, evaluate, compile, translate — without needing a custom integration for each combination.

---

## Data files

| File | Used in | Description |
|---|---|---|
| `data/patient.json` | Demos 2, 3, 8 | A realistic Patient with multilingual name, two name uses, address |
| `data/patient-invalid.json` | Demo 3 | A Patient with an empty `name[{}]` object — triggers `ele-1` invariant |
| `data/demo-profiles.fsh` | Demo 5 | A single FSH Profile constraining five Patient fields |
| `data/demo-measure.cql` | Demo 6 | A CQL measure library with Initial Population, Numerator, Denominator |
| `data/patients-1000.ndjson` | Demo 4, 8 | 1,000 Patient NDJSON rows with varied names, genders, and birth dates |

---

## Performance slides

`../devdays-2025-perf.md` — 13 slides with reproducible benchmark numbers comparing rh to HAPI FHIR (Java), Firely (.NET), fhirpath.js (Node.js), and SUSHI.

```bash
npx @marp-team/marp-cli@latest ../devdays-2025-perf.md --html
```

---

## Demo 9 — FHIRPath REPL *(run manually)*

```bash
rh fhirpath repl -d data/patient.json
```

The REPL starts with the patient loaded and ready. Type any FHIRPath expression and press Enter to evaluate it instantly against the loaded resource.

**Session to walk through:**

```
🔍 FHIRPath Interactive REPL
Commands: .help, .data, .quit

> Patient.name.family
=> String("van der Berg")

> Patient.birthDate
=> Date("1988-11-01")

> Patient.name.count()
=> Integer(2)

> Patient.name.where(use='official').given
=> Collection([String("Pieter"), String("Jan")])

> Patient.name.where(use='official').family | Patient.id
=> Collection([String("van der Berg"), String("devdays-demo")])

> .data
{ ... full patient JSON ... }

> .quit
Goodbye!
```

**What's happening:** Each expression is parsed and evaluated immediately against the in-memory patient JSON loaded at startup. The REPL supports the full FHIRPath spec including functions (`count()`, `where()`, `exists()`, `matches()`), arithmetic, boolean logic, and the union operator (`|`).

`.data` prints the currently loaded resource — useful for checking a field name or value mid-session. `.load <file>` switches the active resource without restarting the REPL. History is persistent across sessions (arrow keys).

**Good expressions to show live:**

| Expression | What it demonstrates |
|---|---|
| `Patient.name.family` | Simple path navigation |
| `Patient.name.count()` | Aggregate function |
| `Patient.name.where(use='official').given` | Filter function |
| `Patient.name.where(use='official').family \| Patient.id` | Union — two results combined |
| `Patient.birthDate` | Typed result — `Date(…)` not a string |
| `Patient.identifier.where(system='http://example.org/mrn').value` | Filtering by child field |
| `Patient.name.exists()` | Boolean existence check |

**Why it matters:** A REPL turns FHIRPath from something you write in a spec or mapping file into something you can *explore*. Useful for building SearchParameter expressions, debugging CQL model paths, or just understanding the shape of a real resource. No server, no boilerplate — just type and see.

---

## Demo 10 — VCL REPL *(run manually)*

```bash
rh vcl repl --explain --translate -s "http://snomed.info/sct"
```

`--explain` prints a plain-English description of each expression. `--translate` shows the FHIR `ValueSet.compose` fragment alongside. `-s` sets the default code system so you don't have to prefix every code.

**Session to walk through:**

```
🚀 VCL Interactive REPL
Default code system: http://snomed.info/sct
Commands: .help, .exit

> concept << 44054006

💡 Explanation:
  codes where concept is a type of '44054006'
  Components: concept (Property) · IsA (Operator) · '44054006' (Value)

🔄 FHIR Translation:
  { "include": [{ "system": "http://snomed.info/sct",
      "filter": [{ "property": "concept", "op": "is-a", "value": "44054006" }] }] }

> status = "active"

💡 Explanation:
  codes where status equals "active"

🔄 FHIR Translation:
  { "include": [{ "system": "http://snomed.info/sct",
      "filter": [{ "property": "status", "op": "=", "value": "active" }] }] }

> status = "active"; concept << 44054006

💡 Explanation:
  codes where status equals "active" OR codes where concept is a type of '44054006'

🔄 FHIR Translation:
  { "include": [
      { "filter": [{ "property": "status", "op": "=", "value": "active" }] },
      { "filter": [{ "property": "concept", "op": "is-a", "value": "44054006" }] }
  ]}

> .exit
Goodbye! 👋
```

**What's happening:** Each VCL expression is parsed into an AST, explained in plain English, and translated to a `ValueSet.compose` JSON fragment in one step. The `;` operator is VCL's union (`OR`) — it produces a second `include` entry in the compose. The REPL maintains your default system setting across expressions.

**What the output means:**

- `💡 Explanation` — what the expression *means* in words; useful for verifying intent and for audience members unfamiliar with VCL
- `🔄 FHIR Translation` — the `ValueSet.compose.include` array that a FHIR terminology server would receive; this is the exact JSON you would embed in a `ValueSet` resource
- `op: "is-a"` — the FHIR filter operator for hierarchical subsumption; a terminology server like tx.fhir.org knows to traverse the SNOMED hierarchy when it sees this

**Good expressions to show live:**

| VCL | What it demonstrates |
|---|---|
| `44054006` | Single concept by code |
| `concept << 44054006` | Descendants-of (subsumption) |
| `status = "active"` | Property filter (equality) |
| `status = "active"; concept << 44054006` | Union of two filters → two `include` entries |

**Why it matters:** Writing `ValueSet.compose` JSON by hand is verbose. VCL lets you express the same definition in a URL-safe string short enough to embed in a hyperlink. The REPL makes it easy to iterate on an expression and see immediately whether it translates to what you intended — without writing any JSON.

---

## Demo 11 — CQL REPL *(run manually)*

```bash
rh cql repl
```

The CQL REPL compiles CQL source to ELM on demand. It supports multi-line input — press **Enter twice** (blank line) to trigger compilation.

**Session to walk through:**

```
CQL Compiler REPL
Enter CQL source (multi-line, blank line to compile)
Commands: :quit, :help, :debug, :compact

> define "Greeting": 'Hello from rh-cql!'
>                                          ← blank line triggers compile

{
  "library": {
    "identifier": { "id": "Anonymous" },
    "statements": { "def": [{
      "name": "Greeting",
      "expression": {
        "type": "Literal",
        "valueType": "…String",
        "value": "Hello from rh-cql!"
      }
    }]}
  }
}

> :compact                                 ← switch to one-line JSON output
Compact output: ON

> define "Adult": AgeInYears() >= 18
>
{"library":{"statements":{"def":[{"name":"Adult","expression":{"type":"GreaterOrEqual","operand":[{"type":"FunctionRef","name":"AgeInYears"},{"type":"Literal","value":"18"}]}}]}}}

> define "InPopulation":
    AgeInYearsAt(@2024-01-01) in Interval[45, 75]
>
{ ... ELM with In, FunctionRef, Interval, Literal ... }

> :quit
Goodbye!
```

**What's happening:** The REPL reads lines until a blank line, then runs the full three-stage CQL compiler (parse → semantic analysis → ELM emission) and prints the resulting ELM JSON. Each blank-line submission is independent — you can experiment with individual `define` statements without writing a full library header.

`:compact` switches from pretty-printed to single-line JSON — useful when you want to copy-paste the output into another tool or just see the structure without scrolling. `:debug` adds annotations and source locators to every ELM node, showing exactly which part of the CQL source each node came from.

**What the output means:**

- `"type": "Literal"` — a constant value; no computation needed
- `"type": "GreaterOrEqual"` — a comparison operator node; its `operand` array holds the two sides
- `"type": "FunctionRef", "name": "AgeInYears"` — a call to a model function; the CQL model (FHIR) supplies the implementation
- `"type": "In"` — the `in` interval membership operator; used heavily in measure logic for date/age range checks
- `"locator": "1:1-1:22"` — source location (line:col-line:col); ties each ELM node back to its CQL source for tooling and debugging

**Good expressions to show live:**

| CQL snippet | What it demonstrates |
|---|---|
| `define "Hello": 'world'` | Simplest possible define — a string literal |
| `define "Adult": AgeInYears() >= 18` | Model function call + comparison |
| `define "Range": AgeInYearsAt(@2024-01-01) in Interval[18, 75]` | Interval membership — common measure pattern |
| `define "Flag": true and false` | Boolean logic |
| `define "Calc": (3 + 4) * 2` | Arithmetic — verify operator precedence |

**Why it matters:** CQL measures are typically edited in a text file, compiled by a Java service, and checked by running the full test suite. The REPL short-circuits that loop to milliseconds — type an expression, see the ELM immediately, catch type errors or structural mistakes before committing to a file. Particularly useful when learning ELM structure or debugging a tricky operator.
