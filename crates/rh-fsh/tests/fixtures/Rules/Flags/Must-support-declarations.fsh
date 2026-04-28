// @Name: Must Support Declarations
// @Description: Basic Must Support declarations

// MS flags can be added in several different ways, as illustrated by the following examples

Profile: MyObservation
Parent: Observation
// on various paths
* subject MS
* component and extension MS
* status and focus and effective[x] MS
// In a contains rule
* extension contains RelatedConditionExtension named relatedCondition 0..1 MS
* extension[relatedCondition] and extension[relatedCondition].valueReference MS
// combined with cardinality
* category 1..1 MS

Extension: RelatedConditionExtension
Id: related-condition-extension
Description: "Example extension for must-support declarations."
* value[x] only Reference(Condition)


