# Native CQL ValueSet membership example

This small synthetic example evaluates two CQL code literals against the
checked-in golden terminology snapshot. The snapshot binds `Golden|1.0|Codes`
to ValueSet version `fixture-1` and CodeSystem release `2026`. Its two codes are
test data, not a clinical expansion or terminology release.

The native golden-snapshot commands below use this checkout's fixtures and the
RH CLI. Once 0.3.0 is
available, use the installed binary on `PATH` or set `RH_BIN` to its explicit
path. To use a source build, run `cargo build -p rh-cli --bin rh` and set
`RH_BIN=./target/debug/rh`. The CQL source is unchanged by the commands.

From the RH repository root:

```bash
RH_BIN="${RH_BIN:-rh}"
SNAPSHOT=fixtures/terminology/golden-snapshot
EXAMPLE=examples/value-set-membership
mkdir -p target

"$RH_BIN" cql terminology-requirements "$EXAMPLE/measure.cql" \
  --output target/value-set-membership-requirements.json
"$RH_BIN" cql eval "$EXAMPLE/measure.cql" CodeAInSet \
  --data "$EXAMPLE/patient.json" --terminology-snapshot "$SNAPSHOT" \
  --allow-fixture
# true
"$RH_BIN" cql eval "$EXAMPLE/measure.cql" CodeZInSet \
  --data "$EXAMPLE/patient.json" --terminology-snapshot "$SNAPSHOT" \
  --allow-fixture
# false
```

Fixture scope is visible in the snapshot lock and requires `--allow-fixture`.
The same commands without that flag fail before evaluating. The CQL source is
read as supplied and is not rewritten; requirements and other generated files
are separate outputs. Evaluation reads the local snapshot and does not contact
a terminology server.

To prepare a fresh snapshot instead, build `rh-analytics` from the separate,
private `reasonhealth-analytics` checkout (repository access required). Set
`RH_BIN` or `ANALYTICS_BIN` to absolute
executable paths when using binaries built in another worktree or target
directory. The Analytics CLI accepts local complete FHIR
ValueSet expansions and can also resolve them from a configured `$expand`
endpoint during preparation:

```bash
cargo build --manifest-path ../reasonhealth-analytics/Cargo.toml --bin rh-analytics
ANALYTICS_BIN="${ANALYTICS_BIN:-../reasonhealth-analytics/target/debug/rh-analytics}"
"$ANALYTICS_BIN" terminology prepare \
  --requirements target/value-set-membership-requirements.json \
  --terminology "$SNAPSHOT/expansions/golden.json" \
  --output target/value-set-membership-snapshot \
  --fixture-justification 'Synthetic native membership example only.'
"$RH_BIN" cql eval "$EXAMPLE/measure.cql" CodeAInSet \
  --data "$EXAMPLE/patient.json" \
  --terminology-snapshot target/value-set-membership-snapshot --allow-fixture
```

ValueSet and CodeSystem versions select an expansion. Membership compares
system and code, ignoring display and the patient code's `version`. The
snapshot is immutable: identical preparation inputs reuse it; changed or
refreshed inputs need a new output directory. Compare two snapshots with
`rh-analytics terminology diff OLD NEW --allow-fixture` when they are fixture
scoped. For the two-repository example that also emits SQL and runs DataFusion,
see the [Analytics membership example in the private repository](https://github.com/Vermonster/reasonhealth-analytics/tree/main/examples/value-set-membership).
Neither example establishes full clinical measure conformance.
