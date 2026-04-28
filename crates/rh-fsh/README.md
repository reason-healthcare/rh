# rh-fsh

FHIR Shorthand (FSH) compiler for the `rh` toolkit.

Transforms FSH source files into FHIR R4 JSON packages including StructureDefinitions,
ValueSets, CodeSystems, and Instances.

## Usage

```rust
use rh_fsh::compile_fsh;

let fsh = r#"
Profile: MyPatient
Parent: Patient
Title: "My Patient Profile"
Description: "A custom patient profile"
* name 1..*
"#;

let package = compile_fsh(fsh, "example.fsh")?;
for resource in &package.resources {
    println!("{}", serde_json::to_string_pretty(resource)?);
}
```

## FSH Entities Supported

- `Profile` — StructureDefinition with derivation=constraint
- `Extension` — Extension StructureDefinition
- `Logical` — Logical model StructureDefinition
- `Resource` — Resource StructureDefinition
- `Instance` — FHIR resource instance
- `ValueSet` — FHIR ValueSet
- `CodeSystem` — FHIR CodeSystem
- `Invariant` — FHIRPath invariant
- `Mapping` — FHIR Mapping / ConceptMap
- `RuleSet` — Reusable rule set
- `Alias` — URL alias

## License

MIT OR Apache-2.0
