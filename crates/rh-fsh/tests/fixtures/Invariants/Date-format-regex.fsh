// @Name: Date Format Regex
// @Description: Example of regular expression that limits dateTime element to Dates (YYYY-MM-DD only)
Invariant:   date-invariant
Description: "All timestamps SHOULD be represented as Dates (YYYY-MM-DD only)."
Expression:  "$this.toString().matches('^[0-9]{4}-[0-9]{2}-[0-9]{2}$')"
Severity:    #warning