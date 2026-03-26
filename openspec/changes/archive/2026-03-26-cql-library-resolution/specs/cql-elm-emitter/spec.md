## ADDED Requirements

### Requirement: ELM includes field populated from TypedLibrary

The `ElmEmitter` SHALL populate the `elm::Library.includes` field from the `TypedLibrary.includes` collection. For each include declaration, the emitter SHALL produce an `IncludeDef` with `path`, `version`, and `localIdentifier` (alias) matching the CQL source. This field SHALL NOT be hardcoded to `None`.

#### Scenario: Single include emits IncludeDef
- **WHEN** a `TypedLibrary` has one include `include FHIRHelpers version '4.0.1' called FHIRHelpers`
- **THEN** the emitted `elm::Library.includes` contains one `IncludeDef` with `path: "FHIRHelpers"`, `version: "4.0.1"`, `localIdentifier: "FHIRHelpers"`

#### Scenario: Include with custom alias preserved
- **WHEN** a `TypedLibrary` has one include `include SomeLib called CaseLogic`
- **THEN** the emitted `IncludeDef` has `localIdentifier: "CaseLogic"` and `path: "SomeLib"`

#### Scenario: No includes emits empty list not None
- **WHEN** a `TypedLibrary` has no include declarations
- **THEN** `elm::Library.includes` is an empty list (not `None`)
