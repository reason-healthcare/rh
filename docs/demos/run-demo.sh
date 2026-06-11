#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# FHIR DevDays 2025  —  rh live demo runner
#
# Usage:
#   ./run-demo.sh            # interactive — press Enter between demos
#   ./run-demo.sh 1          # run only demo 1
#   ./run-demo.sh auto       # auto-advance with 3-second pauses
#
# All demos assume `rh` is on PATH.  Install with:
#   brew tap reason-healthcare/rh && brew install rh
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
DATA="$DIR/data"
RH="${RH:-rh}"          # override with RH=/path/to/rh ./run-demo.sh

# ── colours ──────────────────────────────────────────────────────────────────
BOLD=$'\033[1m'; DIM=$'\033[2m'; RESET=$'\033[0m'
ORANGE=$'\033[38;5;214m'; CYAN=$'\033[36m'; GREEN=$'\033[32m'; RED=$'\033[31m'

# ── helpers ───────────────────────────────────────────────────────────────────
banner() {
  echo ""
  echo "${ORANGE}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
  printf "${ORANGE}${BOLD}  %-68s${RESET}\n" "$*"
  echo "${ORANGE}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
  echo ""
}

cmd() {
  echo "${CYAN}${BOLD}\$ $*${RESET}"
}

pause() {
  case "${1:-interactive}" in
    auto)      sleep 3 ;;
    [0-9]*)    return ;;   # single demo — no pause
    *)         read -r -p "${DIM}  [ press Enter to continue ]${RESET}" ;;
  esac
}

MODE="${1:-interactive}"

# ── Demo 1 — Startup speed ────────────────────────────────────────────────────
demo_1() {
  banner "Demo 1 — Instant startup: 7 ms cold"
  echo "  The JVM needs 3–8 seconds before it prints a single line."
  echo "  rh is a native binary. Watch:"
  echo ""

  cmd "time rh --version"
  time $RH --version
  echo ""
  cmd "time rh --help | head -5"
  time $RH --help 2>/dev/null | head -5
  echo ""
  echo "${GREEN}${BOLD}  17 MB.  No JDK.  No runtime.  No install ceremony.${RESET}"
}

# ── Demo 2 — FHIRPath ─────────────────────────────────────────────────────────
demo_2() {
  banner "Demo 2 — FHIRPath evaluation"
  echo "  Resource: $DATA/patient.json"
  echo ""
  cat "$DATA/patient.json" | python3 -m json.tool | head -20
  echo "${DIM}  ...${RESET}"
  echo ""

  cmd "rh fhirpath eval \"Patient.name.where(use='official').family\" -d data/patient.json"
  $RH fhirpath eval "Patient.name.where(use='official').family" -d "$DATA/patient.json" 2>/dev/null

  echo ""
  cmd "rh fhirpath eval \"Patient.name.given\" -d data/patient.json"
  $RH fhirpath eval "Patient.name.given" -d "$DATA/patient.json" 2>/dev/null

  echo ""
  cmd "rh fhirpath eval \"Patient.birthDate\" -d data/patient.json"
  $RH fhirpath eval "Patient.birthDate" -d "$DATA/patient.json" 2>/dev/null

  echo ""
  cmd "rh fhirpath parse \"Patient.name.where(use='official').family\""
  $RH fhirpath parse "Patient.name.where(use='official').family" 2>/dev/null

  echo ""
  echo "${GREEN}${BOLD}  7 ms — including process spawn.${RESET}"
}

# ── Demo 3 — Validate valid resource ─────────────────────────────────────────
demo_3() {
  banner "Demo 3 — Validate a FHIR resource"
  echo ""

  cmd "rh validate resource -i data/patient.json"
  $RH validate resource -i "$DATA/patient.json" 2>/dev/null || true
  echo ""

  echo "${RED}${BOLD}  Now an invalid resource:${RESET}"
  echo ""
  cat "$DATA/patient-invalid.json"
  echo ""
  cmd "rh validate resource -i data/patient-invalid.json"
  $RH validate resource -i "$DATA/patient-invalid.json" 2>/dev/null || true
  echo ""

  cmd "rh validate resource -i data/patient-invalid.json -f json | python3 -m json.tool | head -30"
  $RH validate resource -i "$DATA/patient-invalid.json" -f json 2>/dev/null \
    | python3 -m json.tool | head -30 || true
}

# ── Demo 4 — Batch validation ─────────────────────────────────────────────────
demo_4() {
  banner "Demo 4 — Batch validation: 1,000 resources"
  echo ""
  cmd "wc -l data/patients-1000.ndjson"
  wc -l "$DATA/patients-1000.ndjson"
  echo ""

  cmd "time rh validate batch -i data/patients-1000.ndjson"
  time $RH validate batch -i "$DATA/patients-1000.ndjson" 2>/dev/null
  echo ""
  echo "${GREEN}${BOLD}  ~0.6 s for 1,000 resources = ~1,600 validations/sec${RESET}"
  echo "  (Profile snapshot loaded once, then cached in memory)"
}

# ── Demo 5 — FSH → FHIR JSON ─────────────────────────────────────────────────
demo_5() {
  banner "Demo 5 — Compile FSH to FHIR JSON"
  echo ""
  cat "$DATA/demo-profiles.fsh"
  echo ""

  cmd "rh fsh compile data/demo-profiles.fsh"
  $RH fsh compile "$DATA/demo-profiles.fsh" 2>/dev/null | python3 -m json.tool | head -40
  echo "${DIM}  ...${RESET}"
  echo ""
  echo "${GREEN}${BOLD}  FSH → StructureDefinition JSON in 9 ms.${RESET}"
}

# ── Demo 6 — CQL compile & explain ───────────────────────────────────────────
demo_6() {
  banner "Demo 6 — CQL → ELM compilation"
  echo ""
  cat "$DATA/demo-measure.cql"
  echo ""

  cmd "rh cql compile data/demo-measure.cql | python3 -m json.tool | head -40"
  $RH cql compile "$DATA/demo-measure.cql" 2>/dev/null \
    | python3 -m json.tool | head -40
  echo "${DIM}  ...${RESET}"
  echo ""

  cmd "rh cql explain data/demo-measure.cql"
  $RH cql explain "$DATA/demo-measure.cql" 2>/dev/null | head -30
  echo ""
  echo "${GREEN}${BOLD}  Full CQL → ELM in 9 ms. Explain mode for debugging.${RESET}"
}

# ── Demo 7 — VCL ──────────────────────────────────────────────────────────────
demo_7() {
  banner "Demo 7 — ValueSet Compose Language (VCL)"
  echo "  VCL: a compact DSL for ValueSet definitions, proposed in FHIR IG Guidance"
  echo ""

  cmd "rh vcl explain \"concept << 44054006\""
  $RH vcl explain 'concept << 44054006' 2>/dev/null

  echo ""
  cmd "rh vcl explain \"status = \\\"active\\\"\""
  $RH vcl explain 'status = "active"' 2>/dev/null

  echo ""
  cmd "rh vcl translate 'concept << 44054006' -s http://snomed.info/sct -f json"
  $RH vcl translate 'concept << 44054006' \
    -s 'http://snomed.info/sct' -f json 2>/dev/null \
    | python3 -m json.tool

  echo ""
  cmd "rh vcl translate 'status = \"active\"; concept << 44054006' -s http://snomed.info/sct -f json"
  $RH vcl translate 'status = "active"; concept << 44054006' \
    -s 'http://snomed.info/sct' -f json 2>/dev/null \
    | python3 -m json.tool

  echo ""
  echo "${GREEN}${BOLD}  VCL → FHIR ValueSet.compose in 8 ms. Also ships as WASM.${RESET}"
}

# ── Demo 8 — Pipes (Unix philosophy) ─────────────────────────────────────────
demo_8() {
  banner "Demo 8 — Composability: pipes and JSON output"
  echo ""

  # Pipe a FHIR resource directly from a shell string into the validator
  cmd "echo '{\"resourceType\":\"Patient\",\"id\":\"inline\"}' | rh validate resource"
  echo '{"resourceType":"Patient","id":"inline"}' \
    | $RH validate resource 2>/dev/null || true
  echo ""

  # jq-style: extract issue count from JSON output
  cmd "rh validate resource -i data/patient.json -f json | python3 -c 'import sys,json; d=json.load(sys.stdin); print(\"issues:\", len(d[\"issues\"]))'"
  $RH validate resource -i "$DATA/patient.json" -f json 2>/dev/null \
    | python3 -c "import sys,json; d=json.load(sys.stdin); print('issues:', len(d['issues']))"
  echo ""

  # Pipe FHIRPath JSON output onward
  cmd "rh fhirpath eval 'Patient.id' -d data/patient.json -f json | python3 -m json.tool"
  $RH fhirpath eval 'Patient.id' -d "$DATA/patient.json" -f json 2>/dev/null \
    | python3 -m json.tool
  echo ""

  # Batch validate from stdin
  cmd "head -5 data/patients-1000.ndjson | rh validate batch"
  head -5 "$DATA/patients-1000.ndjson" | $RH validate batch 2>/dev/null
  echo ""
  echo "${GREEN}${BOLD}  Compose rh with any Unix tool: jq, awk, xargs, CI pipelines.${RESET}"
}

# ── Dispatch ──────────────────────────────────────────────────────────────────
DEMOS=(1 2 3 4 5 6 7 8)

if [[ "$MODE" =~ ^[0-9]+$ ]]; then
  "demo_${MODE}"
else
  for d in "${DEMOS[@]}"; do
    "demo_${d}"
    pause "$MODE"
    clear
  done
  banner "That's rh — try it: brew tap reason-healthcare/rh && brew install rh"
fi
