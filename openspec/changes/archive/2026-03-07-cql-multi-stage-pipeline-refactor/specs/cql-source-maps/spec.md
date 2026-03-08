## ADDED Requirements

### Requirement: SourceMap correlates CQL spans to ELM nodes

The source-map infrastructure SHALL produce a `SourceMap` struct containing:
- `schema_version`: Version string in format `rh-cql-sourcemap/v1`
- `options_fingerprint`: Hash of compiler options used during compilation
- `source_documents`: List of `SourceDocument` entries (one per CQL source file in the compilation)
- `elm_node_metas`: List of `ElmNodeMeta` entries describing each ELM node produced
- `mappings`: List of `SourceElmMapping` entries correlating source spans to ELM nodes

#### Scenario: Single-library source map
- **WHEN** a single CQL library is compiled with source-map enabled
- **THEN** the `SourceMap` contains one `SourceDocument`, multiple `ElmNodeMeta` entries (one per ELM node), and mappings connecting CQL spans to those ELM nodes

#### Scenario: Schema version present
- **WHEN** a source map is produced
- **THEN** the `schema_version` field is `"rh-cql-sourcemap/v1"`

### Requirement: ElmNodeMeta describes each ELM node

Each `ElmNodeMeta` entry SHALL contain:
- `elm_node_id`: A stable identifier for the ELM node
- `elm_path`: A JSON pointer-like path to the node within the ELM library (e.g., `statements/def[0]/expression/operand[0]`)
- `elm_kind`: The ELM expression type name (e.g., `Add`, `Literal`, `Retrieve`)
- `parent_id`: The `elm_node_id` of the parent node (None for root)

#### Scenario: ELM node metadata for addition
- **WHEN** a CQL expression `2 + 3` is compiled
- **THEN** the source map contains an `ElmNodeMeta` with `elm_kind` of `"Add"` and two child `ElmNodeMeta` entries with `elm_kind` of `"Literal"` whose `parent_id` references the `Add` node

### Requirement: SourceElmMapping connects spans to nodes with role

Each `SourceElmMapping` entry SHALL contain:
- `doc_id`: Reference to the source document
- `span`: The CQL source span (start line/column/byte, end line/column/byte)
- `role`: One of `direct`, `implicit-conversion`, `desugared`, `synthetic`
- `elm_node_ids`: List of ELM node IDs produced from this span
- `confidence`: One of `exact` or `approximate`

#### Scenario: Direct mapping for literal
- **WHEN** a CQL literal `42` at line 3, column 15 is compiled
- **THEN** a mapping exists with `role: "direct"`, `confidence: "exact"`, and `elm_node_ids` containing the `Literal` ELM node

#### Scenario: Implicit conversion mapping
- **WHEN** an implicit `ToDecimal` conversion is applied to an integer operand
- **THEN** a mapping exists with `role: "implicit-conversion"` pointing to the `ToDecimal` ELM node generated for the conversion

#### Scenario: One CQL span maps to multiple ELM nodes
- **WHEN** a CQL expression desugars into multiple ELM nodes
- **THEN** the mapping's `elm_node_ids` list contains all generated ELM node IDs

### Requirement: Stable hash-based IDs

The `doc_id` SHALL be derived from a hash of the library identifier, version, and source URI. The `elm_node_id` SHALL be derived from a hash of the ELM node's kind, path, and signature. IDs SHALL be deterministic — identical input produces identical IDs.

#### Scenario: Deterministic node IDs
- **WHEN** the same CQL library is compiled twice
- **THEN** the `elm_node_id` values in both source maps are identical

### Requirement: Source map serialized as sidecar JSON

The source map SHALL be serializable to JSON as a sidecar file with extension `.elm.sourcemap.json`. The `compile_to_elm_with_sourcemap()` API SHALL return both the ELM library and the source map.

#### Scenario: Sidecar JSON output
- **WHEN** `compile_to_elm_with_sourcemap()` is called
- **THEN** the result includes an `elm::Library` and a `SourceMap` that serializes to valid JSON matching the schema

#### Scenario: Source map round-trips through JSON
- **WHEN** a `SourceMap` is serialized to JSON and deserialized back
- **THEN** the deserialized `SourceMap` is identical to the original
