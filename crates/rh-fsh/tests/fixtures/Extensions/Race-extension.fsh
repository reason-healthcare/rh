// @Name: RaceExtension
// @Description: A complex multi-level extension with sub-extensions and slicing

Extension: RaceExt
Id: race-ext
Title: "Race Extension"
Description: "An extension for capturing detailed race information with sub-extensions."

* extension contains
    ombCategory 0..5 MS and
    detailed 0..* and
    text 1..1 MS

* extension[ombCategory] ^short = "OMB race category"
* extension[ombCategory].value[x] only Coding
* extension[ombCategory].valueCoding from http://hl7.org/fhir/us/core/ValueSet/omb-race-category (required)

* extension[detailed] ^short = "Detailed race codes"
* extension[detailed].value[x] only Coding

* extension[text] ^short = "Race text"
* extension[text].value[x] only string
