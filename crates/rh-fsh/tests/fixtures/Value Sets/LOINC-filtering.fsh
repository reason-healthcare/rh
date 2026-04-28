// @Name: LOINC Filtering
// @Description: Value set example filtering on LOINC Part (LP) codes
Alias:  LNC = http://loinc.org

ValueSet: PSATestVS
Id: psa-test-vs
Title: "Tumor Marker Test Value Set"
Description:  "Codes representing tests for prostate specific antigen."
* ^copyright = "This material contains content from LOINC (http://loinc.org). LOINC is copyright © 1995-2020, Regenstrief Institute, Inc. and the Logical Observation Identifiers Names and Codes (LOINC) Committee and is available at no cost under the license at http://loinc.org/license. LOINC® is a registered United States trademark of Regenstrief Institute, Inc"
// LOINC Part (LP) codes are found in https://loinc.org/download/loinc-part-file/
* include codes from system LNC where ancestor = "LP186220-2"
* include codes from system LNC where ancestor = "LP270129-2"
* include codes from system LNC where ancestor = "LP420658-9"
