// @Name: AddElementRules
// @Description: Logical model using AddElement rules to define custom elements

Logical: MyLogicalModel
Id: my-logical-model
Title: "My Logical Model"
Description: "A logical model with custom elements defined using add element rules."

* identifier 0..* SU Identifier "Patient identifiers" "The identifiers for this patient."
* name 1..* MS HumanName "Patient name" "The patient's full name."
* birthDate 0..1 date "Date of birth" "The patient's date of birth."
* active 1..1 boolean "Whether active" "Whether this is an active record."
