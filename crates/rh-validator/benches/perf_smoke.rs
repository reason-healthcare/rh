use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rh_validator::FhirValidator;
use serde_json::json;

const US_CORE_PATIENT_PROFILE: &str =
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

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
            "profile": [US_CORE_PATIENT_PROFILE]
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
            "profile": [US_CORE_PATIENT_PROFILE]
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

fn warm_profile_validation(validator: &FhirValidator, patient: &serde_json::Value) {
    validator
        .validate_with_profile(patient, US_CORE_PATIENT_PROFILE)
        .unwrap();
}

fn print_warm_cache_metrics_once(validator: &FhirValidator, patient: &serde_json::Value) {
    warm_profile_validation(validator, patient);
    validator.reset_cache_metrics();
    warm_profile_validation(validator, patient);

    let (prof_hits, prof_misses, prof_rate, rule_hits, rule_misses, rule_rate) =
        validator.cache_metrics();
    println!("\nperf_smoke warm cache metrics:");
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

fn bench_simple_patient_validation(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let patient = create_simple_patient();
    warm_profile_validation(&validator, &patient);

    c.bench_function("validate_simple_patient", |b| {
        b.iter(|| {
            validator
                .validate_with_profile(black_box(&patient), black_box(US_CORE_PATIENT_PROFILE))
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
    validator.validate_auto(&patient).unwrap();

    c.bench_function("validate_auto_detect", |b| {
        b.iter(|| validator.validate_auto(black_box(&patient)).unwrap());
    });
}

fn bench_complex_patient_validation(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let patient = create_complex_patient();
    warm_profile_validation(&validator, &patient);

    c.bench_function("validate_complex_patient", |b| {
        b.iter(|| {
            validator
                .validate_with_profile(black_box(&patient), black_box(US_CORE_PATIENT_PROFILE))
                .unwrap()
        });
    });
}

fn bench_warmed_batch_validation(c: &mut Criterion) {
    let Some(validator) = setup_validator() else {
        println!("Skipping benchmark: US Core package not found");
        return;
    };

    let patient = create_simple_patient();
    print_warm_cache_metrics_once(&validator, &patient);

    let mut group = c.benchmark_group("warmed_batch_validation");
    for batch_size in [10, 50] {
        let patients: Vec<_> = (0..batch_size).map(|_| create_simple_patient()).collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            &batch_size,
            |b, _| {
                b.iter(|| {
                    for patient in &patients {
                        validator
                            .validate_with_profile(
                                black_box(patient),
                                black_box(US_CORE_PATIENT_PROFILE),
                            )
                            .unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =
        bench_simple_patient_validation,
        bench_auto_detect_validation,
        bench_complex_patient_validation,
        bench_warmed_batch_validation
}
criterion_main!(benches);
