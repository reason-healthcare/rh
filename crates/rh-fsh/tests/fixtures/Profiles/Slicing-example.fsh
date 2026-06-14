// @Name: SlicingExample
// @Description: Profile demonstrating element slicing with discriminators

Profile: SlicedIdentifierProfile
Id: sliced-identifier-profile
Parent: Patient
Title: "Sliced Identifier Profile"
Description: "A profile that slices the identifier element by system."

* identifier ^slicing.discriminator.type = #value
* identifier ^slicing.discriminator.path = "system"
* identifier ^slicing.rules = #open

* identifier contains
    MRN 1..1 MS and
    SSN 0..1 and
    NPI 0..1

* identifier[MRN].system = "http://hospital.example.org/mrn"
* identifier[MRN].value 1..1 MS
* identifier[MRN] ^short = "Medical Record Number"

* identifier[SSN].system = "http://hl7.org/fhir/sid/us-ssn"
* identifier[SSN] ^short = "Social Security Number"

* identifier[NPI].system = "http://hl7.org/fhir/sid/us-npi"
* identifier[NPI] ^short = "National Provider Identifier"
