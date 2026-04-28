// @Name: Observation Component Slicing
// @Description: RuleSet for Slicing Observation.component

// Slice the component element on the component.code element
RuleSet: ObservationComponentSlicingRules
* component ^slicing.discriminator.type = #pattern
* component ^slicing.discriminator.path = "code"
* component ^slicing.rules = #open
* component ^slicing.description = "Slice based on the component.code pattern"

// Put the slicing rules rules into action
Profile: TumorSize
Parent:  Observation
Id:      tumor-size
Title:   "Tumor Size"
Description:  "Records the dimensions of a tumor"
* insert ObservationComponentSlicingRules
// Require 1 dimension; up to two additional dimensions are optional
* component contains maxDimension 1..1 MS and otherDimension 0..2 MS
* component[maxDimension].code = LNC#33728-7
* component[maxDimension] ^short = "Maximum dimension of tumor"
* component[maxDimension] ^definition = "The longest tumor dimension"
* component[otherDimension].code = LNC#33729-5
* component[otherDimension] ^short = "Other tumor dimension"
* component[otherDimension] ^definition = "Additional tumor dimensions should be ordered from largest to smallest"

Alias:   LNC = http://loinc.org

