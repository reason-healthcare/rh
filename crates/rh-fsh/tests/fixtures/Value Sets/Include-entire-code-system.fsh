// @Name: Include Entire Code System
// @Description: Value set comprised of an entire code system
ValueSet: SequenceVariantVS
Id: hgvs-vs
Title: "Human Genome Variation Society Sequence Variant Nomenclature Value Set"
Description: "HGVS nomenclature is used to report and exchange information regarding variants found in DNA, RNA, and protein sequences."
// The "include" in this rule is optional
// http://varnomen.hgvs.org can be replaced with an alias
* include codes from system http://varnomen.hgvs.org


// Value set illustrating various include rules
ValueSet: SurgicalProcedureVS
Id: surgical-procedure-vs
Title: "Surgical Procedure Value Set"
Description: "Includes surgical procedure codes from SNOMED CT, ICD-10-PCS and CPT."
* include codes from system http://www.ama-assn.org/go/cpt
* include codes from system http://www.cms.gov/Medicare/Coding/ICD10
* include codes from system http://snomed.info/sct where concept is-a #387713003 "Surgical procedure (procedure)"