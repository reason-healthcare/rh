// @Name: Include Rules
// @Description: Value set illustrating various include rules
ValueSet: SurgicalProcedureVS
Id: surgical-procedure-vs
Title: "Surgical Procedure Value Set"
Description: "Includes surgical procedure codes from SNOMED CT, ICD-10-PCS and CPT."
* include codes from system http://snomed.info/sct where concept is-a #387713003 "Surgical procedure (procedure)"
* include codes from system http://www.ama-assn.org/go/cpt
* include codes from system http://www.cms.gov/Medicare/Coding/ICD10