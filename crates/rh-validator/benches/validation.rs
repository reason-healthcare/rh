use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rh_validator::{IssueCode, Severity, ValidationIssue, ValidationResult};

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

criterion_group!(
    benches,
    benchmark_validation_result_creation,
    benchmark_validation_issue_creation
);
criterion_main!(benches);
