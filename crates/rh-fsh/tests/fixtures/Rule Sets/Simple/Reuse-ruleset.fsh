// @Name: ReuseRuleSet
// @Description: Multiple profiles all sharing the same non-parameterized RuleSet

RuleSet: CommonObservationRules
* status 1..1 MS
* code 1..1 MS
* subject 1..1 MS
* subject only Reference(Patient)
* effective[x] only dateTime
* obeys us-core-2

Invariant: us-core-2
Description: "If there is no component or hasMember element then either a value[x] or a data absent reason must be present."
Expression: "(component | hasMember).empty() implies (dataAbsentReason.exists() or value.exists())"
Severity: #error

Profile: BloodPressureProfile
Id: blood-pressure-profile
Parent: Observation
Title: "Blood Pressure Profile"
Description: "Profile for blood pressure observations."
* insert CommonObservationRules
* code = http://loinc.org#55284-4

Profile: BodyWeightProfile
Id: body-weight-profile
Parent: Observation
Title: "Body Weight Profile"
Description: "Profile for body weight observations."
* insert CommonObservationRules
* code = http://loinc.org#29463-7
