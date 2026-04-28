// @Name: Shared Properties with Rule Sets
// @Description: This is an alternative approach to using abstract profiles to define constraints that are common to multiple profiles using a FHIR Shorthand feature called "Rule Sets" (https://build.fhir.org/ig/HL7/fhir-shorthand/reference.html#defining-rule-sets). See the "Abstract Profile" example for more information on abstract profiles.

/*
Abstract profiles and rule sets can both be used to achieve the same goal: defining a set of constraints used in multiple profiles without having to define those constraints more than once in FHIR Shorthand.

There are a few practical differences between these approaches:

1) Using an abstract profile will create a StructureDefinition resource for the abstract profile that will appear as part of the built Implementation Guide -- this makes it possible for other IGs to also create profiles inheriting from this abstract profile. A Rule Set in FHIR Shorthand is applied upstream of the IG build process by SUSHI, so it will **not** appear anywhere in the built IG; instead, in the built IG the common constraints will appear in the "child" profiles as if they were directly defined there, and the "child" profiles will inherit from the base resource (or another profile) rather than from a common abstract profile. (Note that "child" is in quotes here because the Rule Set does not actually establish an inheritance relationship.) Other IGs will therefore not be able to create profiles that are "children" of the Rule Set.

2) The differential of a child of an abstract profile will show the differences between the child and the abstract profile. The differential of a "child" profile using the Rule Set approach will compare against the parent, which may be a base FHIR resource (like Patient or Observation). Profilers should consider which of these differentials is likely to be of most use to implementers when deciding which approach to use. Both can be easily tested by moving all the constraints of in the abstract profile into a Rule Set, and comparing the built IG with the rule set inserted into the abstract profile vs. directly into the child profiles.

3) Profiles can have only a single parent, but multiple Rule Sets can be inserted into a profile. The latter effectively allows for multiple inheritance, but profilers should be aware that this creates complexity which may make FSH hard to read or introduce conflicting constraints. Note that running SUSHI with the `--preprocessed` flag will save a version of the FSH that has the Rule Sets inserted. See https://fshschool.org/docs/sushi/running/#preprocessed-fsh for more information.
*/

RuleSet: CommonPatientConstraints

// Require at least one value inside the `name` element
* name 1..*

// Mark elements as MustSupport
* name and name.given and name.family MS

* deceased[x] 1..1



// The profiles below will inherit the constraints from the above parent abstract profile

Profile:     ExampleLivingPatientProfile
Id:          example-living-patient-profile
Parent:      Patient
Title:       "Example Living Patient Patient Profile"
Description: "Example of a profile for living patients sharing common constraints using a FSH Rule Set"

* insert CommonPatientConstraints
* deceased[x] only boolean
* deceasedBoolean = false



Profile:     ExampleDeceasedPatientProfile
Id:          example-deceased-patient-profile
Parent:      Patient
Title:       "Example Deceased Patient Patient Profile"
Description: "Example of a profile for deceased patients sharing common constraints using a FSH Rule Set"

* insert CommonPatientConstraints
* deceased[x] only dateTime