// @Name: String Length Invariant
// @Description: Limit string length invariant
Invariant:   should-be-under-50-chars
Description: "Length SHOULD be <50 for data minimization."
Expression:  "$this.length() < 50')"
Severity:    #warning
