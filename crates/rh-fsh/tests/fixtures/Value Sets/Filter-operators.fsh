// @Name: FilterOperators
// @Description: ValueSets using various filter operators (is-a, descendent-of, regex)

ValueSet: SnomedFindingsVS
Id: snomed-findings-vs
Title: "SNOMED Clinical Findings"
Description: "ValueSet containing SNOMED CT clinical findings using is-a filtering."

* include codes from system http://snomed.info/sct
    where concept is-a #404684003

ValueSet: LOINCVitalSignsVS
Id: loinc-vitals-vs
Title: "LOINC Vital Signs"
Description: "ValueSet containing LOINC vital sign codes using regex filtering."

* include codes from system http://loinc.org
    where code regex "^[0-9]{5}-[0-9]$"
