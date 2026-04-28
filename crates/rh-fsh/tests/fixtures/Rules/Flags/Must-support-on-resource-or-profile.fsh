// @Name: Must Support on Resource or Profile
// @Description: Put a MS flag on a profile (as opposed to an element in the profile)

// Require the user to implement the Person resource
Profile: MustImplementThisResource
Parent: Person
* . MS

// Require the user to implement this Medication Profile
Profile: MustImplementThisProfile
Parent: Medication
* . MS
* code from http://www.nlm.nih.gov/research/umls/rxnorm (required)