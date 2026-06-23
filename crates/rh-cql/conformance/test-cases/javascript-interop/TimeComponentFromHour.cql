library TimeComponentFromHour

/*
Representative Workstream 5 JavaScript cql-execution value mismatch.

HL7 source:
  CqlDateTimeOperatorsTest.xml::DateTimeComponentFrom::TimeComponentFromHour

Expression:
  hour from @T23:20:15.555

Expected:
  23

Observed in the JavaScript matrix:
  rh-cql ELM evaluated by cql-execution returns an empty/null-like result.
  Java ELM evaluated by cql-execution has the same value mismatch, so this is
  currently categorized as JavaScript engine or expected-output review rather
  than an RH-only ELM shape issue.
*/
define Result: hour from @T23:20:15.555
