## ADDED Requirements

### Requirement: Guide document exists
The `crates/rh-packager/` directory SHALL contain a `GUIDE.md` file providing a step-by-step
walkthrough for assembling a FHIR Package using `rh-packager`.

#### Scenario: File is present
- **WHEN** a user navigates to `crates/rh-packager/`
- **THEN** a `GUIDE.md` file is present alongside `README.md`

#### Scenario: README links to guide
- **WHEN** a user reads `crates/rh-packager/README.md`
- **THEN** there is a link to `GUIDE.md` in the Getting Started or introductory section

---

### Requirement: Guide covers project setup
The guide SHALL describe how to create the required project files (`package.json`,
`ImplementationGuide.json`, and `packager.toml`) before running any processor.

#### Scenario: package.json structure shown
- **WHEN** a user reads the project setup section
- **THEN** a complete example `package.json` is shown with all required fields
  (`name`, `version`, `fhirVersions`, `dependencies`)

#### Scenario: packager.toml with all processors shown
- **WHEN** a user reads the project setup section
- **THEN** a complete `packager.toml` is shown wiring all built-in processors into
  `[hooks]` in the order: `fsh` → `snapshot` → `validate` → `cql` → `shell`

---

### Requirement: Guide covers the FSH processor step
The guide SHALL demonstrate authoring a FHIR Shorthand (`.fsh`) profile and running the `fsh`
built-in processor to compile it into a JSON `StructureDefinition`.

#### Scenario: FSH source file shown
- **WHEN** a user reads the FSH step
- **THEN** an example `.fsh` file defining a profile is shown inline

#### Scenario: FSH processor output described
- **WHEN** a user reads the FSH step
- **THEN** the guide explains that `fsh` compiles `*.fsh` files into FHIR JSON resources
  that become available to subsequent pipeline stages

---

### Requirement: Guide covers the snapshot processor step
The guide SHALL demonstrate running the `snapshot` built-in processor to expand a differential
profile into a full snapshot.

#### Scenario: Snapshot expansion explained
- **WHEN** a user reads the snapshot step
- **THEN** the guide explains that `snapshot` expands StructureDefinition differentials into
  full snapshots required by validators and tooling

#### Scenario: Snapshot configuration shown
- **WHEN** a user reads the snapshot step
- **THEN** any required `[snapshot]` configuration (e.g., `packages_dir`) is shown

---

### Requirement: Guide covers the validate processor step
The guide SHALL demonstrate running the `validate` built-in processor to check resources for
FHIR conformance errors.

#### Scenario: Validate step described
- **WHEN** a user reads the validate step
- **THEN** the guide explains that `validate` checks all package resources against their
  declared profiles and the base FHIR specification

#### Scenario: Validation failure handling mentioned
- **WHEN** a user reads the validate step
- **THEN** the guide describes what happens when validation fails (pipeline halts with errors)

---

### Requirement: Guide covers the CQL processor step
The guide SHALL demonstrate authoring a CQL library and running the `cql` built-in processor to
compile it and embed the ELM output into a FHIR `Library` resource.

#### Scenario: CQL source shown
- **WHEN** a user reads the CQL step
- **THEN** an example `.cql` file is shown inline, referencing the profile from the FSH step

#### Scenario: CQL output described
- **WHEN** a user reads the CQL step
- **THEN** the guide explains that `cql` compiles CQL to ELM and upserts a `Library` resource
  into the package

---

### Requirement: Guide covers the shell processor step
The guide SHALL demonstrate configuring a `[processors.<name>]` shell processor in `packager.toml`
and show it executing a custom command as part of the pipeline.

#### Scenario: Shell processor config shown
- **WHEN** a user reads the shell processor step
- **THEN** a complete `[processors.my-step]` entry is shown in `packager.toml` with a `command`
  and hook stage

#### Scenario: Environment variables documented
- **WHEN** a user reads the shell processor step
- **THEN** the guide lists the key environment variables available to the shell command
  (`PACKAGER_SOURCE_DIR`, `PACKAGER_OUTPUT_DIR`, `PACKAGER_WORKDIR`, `PACKAGER_PACKAGE_NAME`)

---

### Requirement: Guide includes an extended example with Python shell hooks
The guide SHALL include an "Extending the Pipeline" section after the basic flow that demonstrates
two Python-based shell processors: one that stamps a build date onto resources, and one that
validates the date is present.

#### Scenario: Python stamp hook shown
- **WHEN** a user reads the extended example section
- **THEN** a complete Python script (`stamp_date.py`) is shown that reads resources from
  `$PACKAGER_WORKDIR/resources/`, adds a `meta.tag` build-date entry to each, and writes
  them back

#### Scenario: Python validation hook shown
- **WHEN** a user reads the extended example section
- **THEN** a complete Python script (`check_date.py`) is shown that reads resources from
  `$PACKAGER_WORKDIR/resources/` and exits non-zero if any resource is missing the
  build-date tag written by the stamp hook

#### Scenario: packager.toml wiring shown for Python hooks
- **WHEN** a user reads the extended example section
- **THEN** the `packager.toml` snippet registers both scripts as `[processors.stamp-date]` and
  `[processors.check-date]`, with `stamp-date` in `before_build` and `check-date` in
  `after_build` (or both in a logical stage order)

#### Scenario: Exit-code contract explained
- **WHEN** a user reads the extended example section
- **THEN** the guide explains that a non-zero exit code from any shell processor aborts the
  pipeline, and shows what the error output looks like when `check_date.py` finds a resource
  missing the tag

---

### Requirement: Guide shows the final package output
The guide SHALL show the resulting FHIR Package directory layout and `.tgz` artifact after
running `rh package build`.

#### Scenario: Output directory layout shown
- **WHEN** a user reads the final output section
- **THEN** a directory tree showing `output/package/` contents is shown, including compiled
  resources, narrative, and metadata files

#### Scenario: Build command shown
- **WHEN** a user reads the final output section
- **THEN** the exact `rh package build <dir>` command is shown and the resulting `.tgz`
  filename is described
