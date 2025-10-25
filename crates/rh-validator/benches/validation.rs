use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rh_validator::{FhirValidator, IssueCode, Severity, ValidationIssue, ValidationResult};

fn benchmark_validation_result_creation(c: &mut Criterion) {
    c.bench_function("validation_result_creation", |b| {
        b.iter(|| {
            let mut result = ValidationResult::new(black_box("Patient"));
            result.add_issue(ValidationIssue::new(
                Severity::Error,
                IssueCode::Required,
                "Missing field",
            ));
            black_box(result)
        });
    });
}

fn benchmark_validation_issue_creation(c: &mut Criterion) {
    c.bench_function("validation_issue_creation", |b| {
        b.iter(|| {
            ValidationIssue::new(
                black_box(Severity::Error),
                black_box(IssueCode::Invariant),
                black_box("Test error"),
            )
            .with_expression("Patient.name.exists()")
            .with_location("Patient.name")
        });
    });
}

fn generate_patient_json(id: usize) -> String {
    // Generate a patient with contact information to trigger FHIRPath invariants
    // Patient.contact requires: name.exists() or telecom.exists() or address.exists() or organization.exists()
    format!(
        r#"{{"resourceType":"Patient","id":"patient-{id}","name":[{{"family":"Doe","given":["John"]}}],"gender":"male","birthDate":"1980-01-01","contact":[{{"name":{{"family":"Smith","given":["Jane"]}},"telecom":[{{"system":"phone","value":"555-1234"}}],"address":{{"line":["123 Main St"],"city":"Springfield","state":"IL","postalCode":"62701"}},"relationship":[{{"coding":[{{"system":"http://terminology.hl7.org/CodeSystem/v2-0131","code":"N"}}]}}]}}]}}"#
    )
}

fn generate_ndjson_patients(count: usize) -> String {
    (0..count)
        .map(generate_patient_json)
        .collect::<Vec<_>>()
        .join("\n")
}

fn benchmark_single_patient_validation(c: &mut Criterion) {
    let validator = FhirValidator::new().unwrap();
    let patient_json = generate_patient_json(1);

    c.bench_function("validate_single_patient", |b| {
        b.iter(|| {
            validator
                .validate_any_resource(black_box(&patient_json))
                .unwrap()
        });
    });
}

fn benchmark_batch_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_validation");

    for size in [10, 100, 1000, 10000] {
        let validator = FhirValidator::new().unwrap();
        let ndjson = generate_ndjson_patients(size);

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| validator.validate_ndjson_any(black_box(&ndjson)).unwrap());
        });
    }

    group.finish();
}

fn benchmark_structural_validation(c: &mut Criterion) {
    let validator = FhirValidator::new().unwrap();
    let patient_json = generate_patient_json(1);

    c.bench_function("validate_json_structural", |b| {
        b.iter(|| {
            validator
                .validate_json::<serde_json::Value>(black_box(&patient_json))
                .unwrap()
        });
    });
}

fn benchmark_multi_resource_types(c: &mut Criterion) {
    let validator = FhirValidator::new().unwrap();

    let resources = [
        r#"{"resourceType":"Patient","id":"p1","gender":"male"}"#,
        r#"{"resourceType":"Observation","id":"o1","status":"final","code":{"coding":[{"system":"http://loinc.org","code":"15074-8"}]}}"#,
        r#"{"resourceType":"Encounter","id":"e1","status":"finished","class":{"system":"http://terminology.hl7.org/CodeSystem/v3-ActCode","code":"AMB"}}"#,
        r#"{"resourceType":"Procedure","id":"pr1","status":"completed","subject":{"reference":"Patient/p1"},"code":{"coding":[{"system":"http://snomed.info/sct","code":"80146002"}]}}"#,
    ];

    let ndjson = resources.join("\n");

    c.bench_function("validate_mixed_resource_types", |b| {
        b.iter(|| validator.validate_ndjson_any(black_box(&ndjson)).unwrap());
    });
}

criterion_group!(
    benches,
    benchmark_validation_result_creation,
    benchmark_validation_issue_creation,
    benchmark_single_patient_validation,
    benchmark_structural_validation,
    benchmark_batch_validation,
    benchmark_multi_resource_types,
);
criterion_main!(benches);
