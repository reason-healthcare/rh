# rh-fsh Project Conformance

This harness compares `rh-fsh` output against the reference `fsh-sushi`
compiler for real implementation guide projects.

Default projects:

- HL7 CARIN Blue Button: `https://github.com/HL7/carin-bb`
- HL7 mCODE: `https://github.com/HL7/fhir-mCODE-ig`
- Da Vinci CRD: `https://github.com/HL7/davinci-crd`
- Da Vinci DTR: `https://github.com/HL7/davinci-dtr`
- Da Vinci PAS: `https://github.com/HL7/davinci-pas`
- HL7 IPS: `https://github.com/HL7/fhir-ips`

## Usage

```bash
# Build the rh CLI in an isolated target directory.
CARGO_TARGET_DIR=target/conformance cargo build -p rh-cli

# Run all default projects. Repositories are cloned under conformance/projects.
python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py

# Smoke-test only the first N FSH files per project.
python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py --limit-files 10

# Restore already-cloned repositories to the revisions in projects.json.
python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py --update-projects
```

Reports are written to `crates/rh-fsh/conformance/results/latest-summary.json`
and `crates/rh-fsh/conformance/results/latest-summary.md`.

The runner normalizes fields that are expected to differ across tools or runs
(`text`, `meta`, generated package metadata, and publisher-generated index
files), then compares resources by `(resourceType, id)`.

SUSHI is pinned to 3.19.0. Project repository URLs and revisions are pinned in
`projects.json`; the runner refuses to use a different revision unless
`--update-projects` is supplied to restore the checked-in revision.
