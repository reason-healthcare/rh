//! Benchmarks for snapshot generation: cold generation (element merging) and
//! cache-hit retrieval on a 600-element profile.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use rh_foundation::snapshot::{ElementDefinition, SnapshotGenerator, StructureDefinition};

const ELEMENT_COUNT: usize = 600;
const BASE_URL: &str = "http://example.org/StructureDefinition/Base";
const PROFILE_URL: &str = "http://example.org/StructureDefinition/Profile";

fn element(path: &str, min: u32, max: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(min),
        max: Some(max.to_string()),
        type_: None,
        binding: None,
        constraint: None,
        definition: Some(format!("Definition of {path}")),
        short: Some(format!("Short for {path}")),
        comment: None,
        requirements: None,
        must_support: None,
        is_summary: None,
        is_modifier: None,
        is_modifier_reason: None,
        slicing: None,
        slice_name: None,
        additional: std::collections::HashMap::new(),
    }
}

fn fixture_definitions() -> Vec<StructureDefinition> {
    let base_elements: Vec<ElementDefinition> = std::iter::once(element("Patient", 0, "*"))
        .chain((0..ELEMENT_COUNT).map(|i| element(&format!("Patient.field{i:04}"), 0, "*")))
        .collect();

    // Differential narrows every 6th element (~100 of 600).
    let differential_elements: Vec<ElementDefinition> = (0..ELEMENT_COUNT)
        .step_by(6)
        .map(|i| element(&format!("Patient.field{i:04}"), 1, "1"))
        .collect();

    vec![
        StructureDefinition {
            url: BASE_URL.to_string(),
            name: "Base".to_string(),
            type_: "Patient".to_string(),
            base_definition: None,
            differential: None,
            snapshot: Some(rh_foundation::snapshot::Snapshot {
                element: base_elements,
            }),
        },
        StructureDefinition {
            url: PROFILE_URL.to_string(),
            name: "Profile".to_string(),
            type_: "Patient".to_string(),
            base_definition: Some(BASE_URL.to_string()),
            differential: Some(rh_foundation::snapshot::types::Differential {
                element: differential_elements,
            }),
            snapshot: None,
        },
    ]
}

fn bench_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("snapshot");

    group.bench_function("generate_cold_600_elements", |b| {
        let mut generator = SnapshotGenerator::new();
        generator.load_structure_definitions(fixture_definitions());
        b.iter(|| {
            generator.clear_cache();
            black_box(generator.generate_snapshot(PROFILE_URL).expect("snapshot"))
        });
    });

    group.bench_function("generate_cache_hit", |b| {
        let mut generator = SnapshotGenerator::new();
        generator.load_structure_definitions(fixture_definitions());
        let _warm = generator.generate_snapshot(PROFILE_URL).expect("snapshot");
        b.iter(|| black_box(generator.generate_snapshot(PROFILE_URL).expect("snapshot")));
    });

    group.finish();
}

criterion_group!(benches, bench_snapshot);
criterion_main!(benches);
