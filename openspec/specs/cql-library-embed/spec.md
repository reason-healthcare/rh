# cql-library-embed

## Purpose

Defines the behaviour of the `cql` lifecycle hook processor, which validates, compiles CQL files, and embeds the CQL source and ELM JSON into matching Library FHIR resources during a publish pipeline.

## Requirements

### Requirement: cql processor validates and compiles all CQL files in the source directory

When the `cql` processor is invoked as a lifecycle hook, it SHALL find all `*.cql` files in the
source directory, validate each for syntax and semantic correctness using `rh-cql`, and compile
each to ELM JSON. Any validation or compilation error SHALL fail the pipeline.

#### Scenario: Valid CQL file compiles successfully
- **WHEN** the `cql` processor runs and a `*.cql` file is syntactically and semantically valid
- **THEN** the file is compiled to ELM JSON without error

#### Scenario: CQL syntax error fails the pipeline
- **WHEN** the `cql` processor runs and a `*.cql` file has a syntax error
- **THEN** the processor returns an error with the file path and error details, aborting the pipeline

#### Scenario: CQL semantic error fails the pipeline
- **WHEN** the `cql` processor runs and a `*.cql` file references an undefined identifier
- **THEN** the processor returns an error with the file path and semantic error details, aborting the pipeline

### Requirement: cql processor embeds CQL source and ELM into matching Library resources

For each compiled CQL file, the `cql` processor SHALL locate the matching `Library` resource in
the in-memory resource map by CQL library name (file stem → `Library-<name>.json`) and embed
the CQL source and ELM JSON as base64-encoded `content[]` entries per the CQL IG guidance.

#### Scenario: CQL source is embedded as text/cql attachment
- **WHEN** `MyLibrary.cql` is compiled and `Library-MyLibrary.json` exists in the resource map
- **THEN** the Library's `content` array contains an entry `{ "contentType": "text/cql", "data": "<base64 CQL source>" }`

#### Scenario: ELM JSON is embedded as application/elm+json attachment
- **WHEN** `MyLibrary.cql` is compiled successfully
- **THEN** the Library's `content` array contains an entry `{ "contentType": "application/elm+json", "data": "<base64 ELM JSON>" }`

#### Scenario: Existing content entries are replaced
- **WHEN** a Library resource already has `text/cql` or `application/elm+json` content entries
- **THEN** those entries are replaced with the freshly compiled content

### Requirement: cql processor creates a minimal Library resource when none exists

If no matching `Library-<name>.json` exists in the resource map for a given CQL file, the `cql`
processor SHALL create a minimal conformant `Library` resource, add it to the in-memory resource
map, and emit an informational message recommending the author add the resource to the source
directory.

#### Scenario: Auto-created Library has required fields
- **WHEN** `MyLibrary.cql` is compiled and no `Library-MyLibrary.json` exists
- **THEN** a new Library resource is added to the resource map with `resourceType: "Library"`, `id: "MyLibrary"`, `status: "draft"`, `type.coding` containing `library-type#logic-library`, and the compiled content entries

#### Scenario: Author is notified of auto-created Library
- **WHEN** the `cql` processor auto-creates a Library resource
- **THEN** an informational message is printed recommending the author add `Library-<name>.json` to the source directory

### Requirement: cql processor sets Library type to logic-library when absent

If a matched or auto-created Library resource does not have a `type` element, the `cql`
processor SHALL set it to the `logic-library` code from the `library-type` CodeSystem.

#### Scenario: Missing type is set to logic-library
- **WHEN** `Library-MyLibrary.json` exists with no `type` element and a matching CQL file is compiled
- **THEN** the Library in the resource map has `type.coding[0]` = `{ "system": "http://terminology.hl7.org/CodeSystem/library-type", "code": "logic-library" }`

#### Scenario: Existing type is preserved
- **WHEN** `Library-MyLibrary.json` exists with `type` already set
- **THEN** the `type` element is not modified by the `cql` processor

### Requirement: cql processor resolves CQL library dependencies from packages

The `cql` processor SHALL support CQL `include` statements by resolving included libraries
from both the source directory and from dependency packages loaded from `~/.fhir/packages/`,
using the existing `FileLibrarySourceProvider` and `PackageLibrarySourceProvider` from `rh-cql`.

#### Scenario: Included CQL library in source directory is resolved
- **WHEN** `MyLibrary.cql` contains `include Helpers version '1.0.0'` and `Helpers.cql` exists in the source directory
- **THEN** compilation succeeds with the included library resolved

#### Scenario: Included CQL library in dependency package is resolved
- **WHEN** `MyLibrary.cql` includes a library that exists in a loaded dependency FHIR package
- **THEN** compilation succeeds with the library resolved from the package

### Requirement: cql processor is configurable via publisher.toml

The `cql` processor SHALL read a `[cql]` configuration block from `publisher.toml`.

#### Scenario: Custom packages_dir is used for library resolution
- **WHEN** `publisher.toml` contains `[cql]\npackages_dir = "/custom/packages"`
- **THEN** the `cql` processor loads FHIR packages from that directory for library resolution
