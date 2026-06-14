// @Name: ObeysRules
// @Description: Profiles that use obeys rules to attach invariants

Invariant: pat-1
Description: "Patient must have a family name or a text name"
Expression: "name.family.exists() or name.text.exists()"
Severity: #error

Profile: ObeysRuleProfile
Id: obeys-rule-profile
Parent: Patient
Title: "Obeys Rule Profile"
Description: "A profile that attaches an invariant via an obeys rule."

* obeys pat-1
* name.family obeys pat-1
