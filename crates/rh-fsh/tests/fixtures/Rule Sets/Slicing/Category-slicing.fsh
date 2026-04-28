// @Name: Category Slicing
// @Description: Slice the category element so a certain category is required

RuleSet: CategorySlicingRules
* category ^slicing.discriminator.type = #pattern
* category ^slicing.discriminator.path = "$this"
* category ^slicing.rules = #open
* category ^slicing.description = "Slicing requires the given value but allows additional categories"


Profile: KarnofskyPerformanceStatus
Parent:  Observation
Id:      karnofsky-performance-status
Title:   "Karnofsky Performance Status"
Description: "The Karnofsky Performance Status (KPS) is a tool used to measure a patient's functional status. The KPS score ranges from 0 to 100 in intervals of 10."
// use the rule set defined above
* insert CategorySlicingRules
// The category "survey" is required to conform to this profile
* category = http://terminology.hl7.org/CodeSystem/observation-category#survey
* code = LNC#89243-0 //"Karnofsky Performance Status score"
* value[x] only integer
