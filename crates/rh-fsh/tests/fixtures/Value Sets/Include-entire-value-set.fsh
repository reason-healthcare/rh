// @Name: Include Entire Value Set
// @Description: A value set containing other value sets
Alias: $PrimaryCancerDisorderVS = http://example.org/fhir/ValueSet/primary-cancer-disorder
Alias: $SecondaryCancerDisorderVS = http://example.org/fhir/ValueSet/secondary-cancer-disorder

ValueSet: PrimaryAndSecondaryCancerDisorderVS
Id: primary-and-secondard-cancer-disorder-vs
Title: "Cancer Disorder Value Set"
Description:   "A value set containing both primary and secondary tumor types."
* include codes from valueset $PrimaryCancerDisorderVS
* include codes from valueset $SecondaryCancerDisorderVS
