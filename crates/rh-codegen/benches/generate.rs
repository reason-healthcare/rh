use criterion::{criterion_group, criterion_main, Criterion};
use rh_codegen::{CodeGenerator, CodegenConfig, StructureDefinition};

fn make_structure_defs() -> Vec<StructureDefinition> {
    let base_defs = vec![
        ("Patient", "resource", "DomainResource"),
        ("Observation", "resource", "DomainResource"),
        ("Condition", "resource", "DomainResource"),
        ("Practitioner", "resource", "DomainResource"),
        ("Organization", "resource", "DomainResource"),
        ("Identifier", "complex-type", "Element"),
        ("HumanName", "complex-type", "Element"),
        ("Address", "complex-type", "Element"),
        ("CodeableConcept", "complex-type", "Element"),
        ("Quantity", "complex-type", "Element"),
        ("string", "primitive-type", "Element"),
        ("integer", "primitive-type", "Element"),
        ("boolean", "primitive-type", "Element"),
        ("decimal", "primitive-type", "Element"),
        ("uri", "primitive-type", "Element"),
    ];

    base_defs
        .into_iter()
        .map(|(name, kind, base)| StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: name.to_string(),
            url: format!("http://hl7.org/fhir/StructureDefinition/{name}"),
            name: name.to_string(),
            title: Some(name.to_string()),
            status: "active".to_string(),
            kind: kind.to_string(),
            is_abstract: false,
            description: Some(format!("FHIR {name}")),
            purpose: None,
            base_type: base.to_string(),
            base_definition: Some(format!("http://hl7.org/fhir/StructureDefinition/{base}")),
            version: None,
            differential: None,
            snapshot: None,
        })
        .collect()
}

fn bench_generate_single_struct(c: &mut Criterion) {
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    let patient_structure = StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Patient".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        title: Some("Patient".to_string()),
        status: "active".to_string(),
        kind: "resource".to_string(),
        is_abstract: false,
        description: Some("A patient resource".to_string()),
        purpose: None,
        base_type: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        version: None,
        differential: None,
        snapshot: None,
    };

    rh_codegen::generators::type_registry::TypeRegistry::register_from_structure_definition(
        &patient_structure,
    );

    c.bench_function("generate_struct_patient", |b| {
        b.iter(|| {
            let _ = generator.generate_struct(&patient_structure);
        })
    });
}

fn bench_generate_structs_sequential(c: &mut Criterion) {
    let structure_defs = make_structure_defs();
    for sd in &structure_defs {
        rh_codegen::generators::type_registry::TypeRegistry::register_from_structure_definition(sd);
    }

    c.bench_function("generate_structs_sequential_14_defs", |b| {
        b.iter(|| {
            let config = CodegenConfig::default();
            let mut generator = CodeGenerator::new(config);
            for sd in &structure_defs {
                let _ = generator.generate_struct(sd);
            }
        })
    });
}

fn bench_generate_structs_parallel(c: &mut Criterion) {
    let structure_defs = make_structure_defs();
    for sd in &structure_defs {
        rh_codegen::generators::type_registry::TypeRegistry::register_from_structure_definition(sd);
    }

    c.bench_function("generate_structs_parallel_14_defs", |b| {
        b.iter(|| {
            let config = CodegenConfig::default();
            let generator = CodeGenerator::new(config);
            let _ = generator.generate_structs_parallel(&structure_defs);
        })
    });
}

fn bench_type_registry_registration(c: &mut Criterion) {
    let patient_structure = StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Patient".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        title: Some("Patient".to_string()),
        status: "active".to_string(),
        kind: "resource".to_string(),
        is_abstract: false,
        description: Some("A patient resource".to_string()),
        purpose: None,
        base_type: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        version: None,
        differential: None,
        snapshot: None,
    };

    c.bench_function("type_registry_register", |b| {
        b.iter(|| {
            rh_codegen::generators::type_registry::TypeRegistry::register_from_structure_definition(
                &patient_structure,
            );
        })
    });
}

criterion_group!(
    benches,
    bench_generate_single_struct,
    bench_generate_structs_sequential,
    bench_generate_structs_parallel,
    bench_type_registry_registration
);
criterion_main!(benches);
