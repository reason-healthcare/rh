library Golden version '1.0'
using FHIR version '4.0.1'

codesystem ExampleCodes: 'https://example.test/CodeSystem/codes' version '2026'
valueset Codes: 'https://example.test/ValueSet/codes' version 'fixture-1' codesystems { ExampleCodes }

context Patient

define CodeAInSet: Code 'a' from ExampleCodes in Codes
define CodeZInSet: Code 'z' from ExampleCodes in Codes
