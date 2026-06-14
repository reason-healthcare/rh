// @Name: ComplexParams
// @Description: Parameterized rule sets with multiple parameters and insertion

RuleSet: RequiredElement(path, min, max)
* {path} {min}..{max} MS
* {path} ^short = "Required element: {path}"

RuleSet: BoundCode(path, valueSet, strength)
* {path} from {valueSet} ({strength})

Profile: ParamRuleSetProfile
Id: param-ruleset-profile
Parent: Observation
Title: "Parameterized RuleSet Profile"
Description: "Profile demonstrating parameterized rule sets."

* insert RequiredElement(status, 1, 1)
* insert RequiredElement(code, 1, 1)
* insert BoundCode(code, http://loinc.org/vs/LL715-4, required)
* insert RequiredElement(subject, 0, 1)
