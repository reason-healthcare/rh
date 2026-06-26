use rh_validator::TerminologyConfig;

#[test]
#[ignore = "requires network access to https://tx.fhir.org/r4"]
fn validates_code_against_tx_fhir_org() {
    let service = TerminologyConfig::fhir_tx()
        .build()
        .expect("tx.fhir.org terminology service should be configured");

    let result = service
        .validate_code_in_codesystem(
            "http://hl7.org/fhir/administrative-gender",
            "male",
            Some("Male"),
        )
        .expect("tx.fhir.org should validate AdministrativeGender male");

    assert!(result.result, "expected tx.fhir.org to accept male");
    assert_eq!(result.display.as_deref(), Some("Male"));
}
