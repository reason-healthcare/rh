/// Test to check what our validator reports for mr-covid-m.json

#[test]
fn test_mr_covid_m_validation() {
    let resource_json = r#"{
  "resourceType" : "Measure",
  "id" : "ebuqHUPORnS7mV61F6MCnFVLAo080vcLScnKBlZ5Os7A3",
  "url" : "http://open.epic.com/Measure/ICU-Total-Ventilators-Facility",
  "name" : "ICU Total Ventilators",
  "title" : "Total Ventilators",
  "status" : "draft",
  "subjectCodeableConcept" : {
    "coding" : [
      {
        "code" : "Location",
        "display" : "Location"
      }
    ]
  },
  "date" : "2020-03-30T23:38:55Z",
  "publisher" : "Epic",
  "contact" : [
    {
      "name" : "Michael Donnelly",
      "telecom" : [
        {
          "system" : "url",
          "value" : "https://chat.fhir.org/#narrow/stream/226195-Covid-19-Response/topic/Example.20Measure.20definition",
          "rank" : 1
        },
        {
          "system" : "email",
          "value" : "michael.donnelly@epic.com",
          "rank" : 2
        }
      ]
    }
  ],
  "description" : "The number of ventilators available for use.",
  "useContext" : [
    {
      "code" : {
        "system" : "http://terminology.hl7.org/CodeSystem/usage-context-type",
        "code" : "focus"
      },
      "valueCodeableConcept" : {
        "coding" : [
          {
            "system" : "http://snomed.info/sct",
            "code" : "840535000",
            "display" : "COVID-19"
          }
        ],
        "text" : "COVID-19"
      }
    }
  ],
  "scoring" : {
    "coding" : [
      {
        "code" : "continuous-variable",
        "display" : "Continuous Variable"
      }
    ]
  },
  "group" : [
    {
      "population" : [
        {
          "code" : {
            "coding" : [
              {
                "system" : "http://terminology.hl7.org/CodeSystem/measure-population",
                "code" : "measure-population"
              }
            ]
          },
          "criteria" : {
            "description" : "ICU Location"
          }
        },
        {
          "code" : {
            "coding" : [
              {
                "system" : "http://terminology.hl7.org/CodeSystem/measure-population",
                "code" : "measure-observation"
              }
            ]
          },
          "criteria" : {
            "description" : "Number of ventilators available for use",
            "language" : "text/cql",
            "expression" : "Count([Device: Ventilators])"
          }
        }
      ]
    }
  ]
}"#;

    let resource: serde_json::Value =
        serde_json::from_str(resource_json).expect("Failed to parse JSON");

    let validator = rh_validator::FhirValidator::new(rh_validator::FhirVersion::R4, None)
        .expect("Failed to create validator");

    let result = validator
        .validate_auto(&resource)
        .expect("Failed to validate");

    println!("\n========================================");
    println!("RH Validator Results for mr-covid-m");
    println!("========================================");
    println!("Total issues: {}", result.issues.len());
    println!();

    let errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .collect();
    let warnings: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Warning)
        .collect();
    let info: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Information)
        .collect();

    println!("Errors: {}", errors.len());
    println!("Warnings: {}", warnings.len());
    println!("Info: {}", info.len());
    println!();

    if !errors.is_empty() {
        println!("ERROR ISSUES:");
        for (i, issue) in errors.iter().enumerate() {
            println!("  {}. [{:?}] {}", i + 1, issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("     Path: {path}");
            }
        }
        println!();
    }

    if !warnings.is_empty() {
        println!("WARNING ISSUES:");
        for (i, issue) in warnings.iter().enumerate() {
            println!("  {}. [{:?}] {}", i + 1, issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("     Path: {path}");
            }
        }
        println!();
    }

    if !info.is_empty() {
        println!("INFO ISSUES:");
        for (i, issue) in info.iter().enumerate() {
            println!("  {}. [{:?}] {}", i + 1, issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("     Path: {path}");
            }
        }
        println!();
    }

    println!("\n========================================");
    println!("Java Validator Expected Issues:");
    println!("========================================");
    println!("Errors: 3");
    println!("  1. Wrong Display Name 'COVID-19' for SNOMED code 840535000");
    println!("     Path: Measure.useContext[0].value.ofType(CodeableConcept).coding[0].display");
    println!("  2. Expression.language: minimum required = 1, but only found 0");
    println!("     Path: Measure.group[0].population[0].criteria");
    println!("  3. Constraint failed: exp-1: 'An expression or a reference must be provided'");
    println!("     Path: Measure.group[0].population[0].criteria");
    println!();
    println!("Warnings: 4");
    println!("  1. Coding has no system (subject.coding[0])");
    println!("  2. Coding has no system (scoring.coding[0])");
    println!("  3. Code not in value set 'MeasureScoring'");
    println!("  4. Constraint failed: mea-0 (Name should be usable as identifier)");
    println!();

    println!("========================================");
    println!("Comparison:");
    println!("========================================");
    println!("Java expects INVALID (3 errors + 4 warnings)");
    println!(
        "RH reports {} ({} errors + {} warnings)",
        if errors.is_empty() {
            "VALID"
        } else {
            "INVALID"
        },
        errors.len(),
        warnings.len()
    );
    println!("========================================\n");
}
