## ADDED Requirements

### Requirement: Resource-specific markdown is embedded as FHIR narrative

For each `<stem>.md` file in the source directory where a matching `<stem>.json` FHIR resource
exists (matched by filename stem), the publisher SHALL convert the markdown to XHTML and embed
it as the resource's `text` element with `status` set to `"generated"`.

#### Scenario: Matching markdown file is embedded
- **WHEN** `ValueSet-myVS.json` and `ValueSet-myVS.md` both exist in the source directory
- **THEN** the output `ValueSet-myVS.json` contains `"text": { "status": "generated", "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">...</div>" }` with the converted markdown content

#### Scenario: Resource without matching markdown has no text injected
- **WHEN** `StructureDefinition-foo.json` exists but `StructureDefinition-foo.md` does not
- **THEN** the output resource's `text` field is unchanged from the source

#### Scenario: Existing text is overwritten
- **WHEN** a source resource already has a `text` element and a matching `.md` file exists
- **THEN** the existing `text` is replaced with the markdown-derived narrative

### Requirement: XHTML narrative div is namespace-qualified

The converted markdown SHALL be wrapped in `<div xmlns="http://www.w3.org/1999/xhtml">` as
required by the FHIR narrative data type.

#### Scenario: Output div has XHTML namespace
- **WHEN** any markdown file is converted to narrative
- **THEN** the `div` value begins with `<div xmlns="http://www.w3.org/1999/xhtml">`

### Requirement: Unmatched markdown files are included in the package as-is

Markdown files with no matching FHIR resource SHALL be copied to `package/other/<filename>.md`
in the output package.

#### Scenario: Standalone markdown is preserved in output
- **WHEN** `overview.md` exists in the source directory and no `overview.json` exists
- **THEN** `overview.md` is present at `package/other/overview.md` in the output

#### Scenario: Standalone markdown is not treated as a resource
- **WHEN** a standalone markdown file is copied to `package/other/`
- **THEN** it is not included in `.index.json`

### Requirement: Markdown is converted using CommonMark

The publisher SHALL use CommonMark-compliant markdown parsing for narrative conversion.

#### Scenario: CommonMark headings render as HTML headings
- **WHEN** a markdown file contains `## Section Title`
- **THEN** the embedded XHTML contains `<h2>Section Title</h2>`

#### Scenario: CommonMark paragraphs render as HTML paragraphs
- **WHEN** a markdown file contains a plain paragraph
- **THEN** the embedded XHTML contains a `<p>` element with the paragraph text
