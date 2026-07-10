// @Name: MultipleProfiles
// @Description: Multiple profile definitions in a single FSH file

Profile: BasePatientProfile
Id: base-patient-profile
Parent: Patient
Title: "Base Patient Profile"
Description: "A base profile for all patients."
* name 1..* MS
* birthDate 1..1 MS

Profile: PediatricPatientProfile
Id: pediatric-patient-profile
Parent: base-patient-profile
Title: "Pediatric Patient Profile"
Description: "A profile for pediatric patients, derived from the base profile."
* deceased[x] MS
* ^url = "http://example.org/StructureDefinition/PediatricPatientProfile"
