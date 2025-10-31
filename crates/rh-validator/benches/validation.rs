use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rh_validator::FhirValidator;
use serde_json::json;

fn setup_validator() -> Option<FhirValidator> {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        return None;
    }

    FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .ok()
}

fn create_simple_patient() -> serde_json::Value {
    json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    })
}

fn create_complex_patient() -> serde_json::Value {
    json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2106-3",
                            "display": "White"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "White"
                    }
                ]
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2186-5",
                            "display": "Not Hispanic or Latino"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "Not Hispanic or Latino"
                    }
                ]
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
                "valueCode": "M"
            }
        ],
        "identifier": [
            {
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            },
            {
                "system": "http://hl7.org/fhir/sid/us-ssn",
                "value": "123-45-6789"
            }
        ],
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "William"]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234",
                "use": "home"
            },
            {
                "system": "email",
                "value": "john.doe@example.com"
            }
        ],
        "gender": "male",
        "birthDate": "1970-01-01",
        "address": [
            {
                "use": "home",
                "line": ["123 Main St", "Apt 4B"],
                "city": "Boston",
                "state": "MA",
                "postalCode": "02101",
                "country": "US"
            }
        ]
    })
}

fn bench_simple_patient_validation(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let patient = create_simple_patient();
    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    c.bench_function("validate_simple_patient", |b| {
        b.iter(|| {
            validator
                .validate_with_profile(black_box(&patient), black_box(profile_url))
                .unwrap()
        });
    });
}

fn bench_complex_patient_validation(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let patient = create_complex_patient();
    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    c.bench_function("validate_complex_patient", |b| {
        b.iter(|| {
            validator
                .validate_with_profile(black_box(&patient), black_box(profile_url))
                .unwrap()
        });
    });
}

fn bench_auto_detect_validation(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let patient = create_simple_patient();

    c.bench_function("validate_auto_detect", |b| {
        b.iter(|| validator.validate_auto(black_box(&patient)).unwrap());
    });
}

fn bench_batch_validation(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    let mut group = c.benchmark_group("batch_validation");
    for batch_size in [10, 50, 100, 500].iter() {
        let patients: Vec<_> = (0..*batch_size).map(|_| create_simple_patient()).collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, _| {
                b.iter(|| {
                    for patient in &patients {
                        validator
                            .validate_with_profile(black_box(patient), black_box(profile_url))
                            .unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_cache_performance(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let patient = create_simple_patient();
    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    validator
        .validate_with_profile(&patient, profile_url)
        .unwrap();

    let (snapshot_cache, rule_cache) = validator.cache_stats();
    println!(
        "Cache stats before bench: Snapshot cache: {}/{}, Rule cache: {}/{}",
        snapshot_cache.0, snapshot_cache.1, rule_cache.0, rule_cache.1
    );

    validator.reset_cache_metrics();

    c.bench_function("validate_cached_profile", |b| {
        b.iter(|| {
            validator
                .validate_with_profile(black_box(&patient), black_box(profile_url))
                .unwrap()
        });
    });

    let (prof_hits, prof_misses, prof_rate, rule_hits, rule_misses, rule_rate) =
        validator.cache_metrics();
    println!("\nCache Performance Metrics:");
    println!(
        "  Profile Cache: {}/{} hits ({:.1}% hit rate)",
        prof_hits,
        prof_hits + prof_misses,
        prof_rate * 100.0
    );
    println!(
        "  Rule Cache: {}/{} hits ({:.1}% hit rate)",
        rule_hits,
        rule_hits + rule_misses,
        rule_rate * 100.0
    );
}

criterion_group!(
    benches,
    bench_simple_patient_validation,
    bench_complex_patient_validation,
    bench_auto_detect_validation,
    bench_batch_validation,
    bench_cache_performance
);
criterion_main!(benches);
