use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rh_fsh::{
    build_definition_index, compile_fsh, CompiledSchema, DependencyDefinitionSet, FhirDefs,
    FshConfig, FshParser, FshTank, SemanticProgram,
};
use rh_hl7_fhir_r4_core::metadata::get_field_info;

fn make_fsh_input(n: usize) -> String {
    let mut buf = String::new();
    for i in 0..n {
        buf.push_str(&format!(
            "Profile: MyProfile{i}\nParent: Observation\nTitle: \"Profile {i}\"\n\n"
        ));
    }
    buf
}

fn bench_small(c: &mut Criterion) {
    let input = make_fsh_input(10);
    c.bench_function("compile_small", |b| {
        b.iter(|| compile_fsh(black_box(&input), "bench").ok())
    });
}

fn bench_medium(c: &mut Criterion) {
    let input = make_fsh_input(100);
    c.bench_function("compile_medium", |b| {
        b.iter(|| compile_fsh(black_box(&input), "bench").ok())
    });
}

fn bench_large(c: &mut Criterion) {
    let input = make_fsh_input(1000);
    c.bench_function("compile_large", |b| {
        b.iter(|| compile_fsh(black_box(&input), "bench").ok())
    });
}

fn bench_schema_lookup(c: &mut Criterion) {
    let document = FshParser::parse(
        "Instance: example\nInstanceOf: Patient\n* name[0].given[0] = \"Ada\"\n",
        "bench-schema.fsh",
    )
    .expect("benchmark FSH parses");
    let mut tank = FshTank::new();
    tank.add_document(document).expect("benchmark tank builds");
    let defs = FhirDefs::r4();
    let definitions = build_definition_index(
        &tank,
        &FshConfig::default(),
        &DependencyDefinitionSet::default(),
    );
    let schema = CompiledSchema::compile(&tank, defs.as_ref(), &definitions);

    c.bench_function("schema_core_field_lookup", |b| {
        b.iter(|| black_box(schema.field(black_box("HumanName"), black_box("given"))))
    });
    c.bench_function("compiled_choice_field_lookup", |b| {
        b.iter(|| black_box(schema.field(black_box("Observation"), black_box("valueString"))))
    });
    c.bench_function("generated_core_field_lookup", |b| {
        b.iter(|| black_box(get_field_info(black_box("HumanName"), black_box("given"))))
    });
}

fn bench_semantic_lowering(c: &mut Criterion) {
    let mut input = String::from("Instance: example\nInstanceOf: Patient\n");
    for _ in 0..1000 {
        input.push_str("* active = true\n");
    }
    let document = FshParser::parse(&input, "bench-semantic.fsh").expect("benchmark FSH parses");
    let mut tank = FshTank::new();
    tank.add_document(document).expect("benchmark tank builds");
    let rules = &tank
        .instances
        .get("example")
        .expect("benchmark instance exists")
        .rules;

    c.bench_function("semantic_lowering_1000_assignments", |b| {
        b.iter(|| {
            black_box(SemanticProgram::lower_instance(black_box(rules), |_| {
                serde_json::Value::Bool(true)
            }))
        })
    });
}

criterion_group!(
    benches,
    bench_small,
    bench_medium,
    bench_large,
    bench_schema_lookup,
    bench_semantic_lowering
);
criterion_main!(benches);
