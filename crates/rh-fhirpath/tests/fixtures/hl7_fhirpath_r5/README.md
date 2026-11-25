# Pinned FHIR R5 suite

The XML suite and public JSON examples are from FHIR/fhir-test-cases commit
`469d90ab58663266d5d7ff82ba3587257fd8a4be` under Apache-2.0.
HL7 links to the R5 suite from the FHIRPath 3.0.0 tests page. The suite continues
to carry historical expectations and tests for additional hosts. [Upstream
suite and fixtures](https://github.com/FHIR/fhir-test-cases/tree/469d90ab58663266d5d7ff82ba3587257fd8a4be/r5) ·
[HL7 test guidance](https://hl7.org/fhirpath/STU3/en/tests.html) · [License](LICENSE.txt)

The five `parameters-*` files without an upstream JSON example are derived
from the same revision's XML. The conversion uses the primitive `value`
attribute, preserves decimal and exponent lexemes as JSON numbers, and
preserves the order of the repeating `parameter` elements. The XHTML tests
use the literal strings provided in the XML. The CDA input requires a separate
logical model and falls outside the FHIR model runs.

`known-differences.tsv` records the case ID, outcome, and rationale for each
scored difference. The tests reject new or changed differences and stale
entries. The R4 fixture set also uses the upstream `patient-name-extensions.json`
from this revision. [R4 primitive fixture](https://github.com/FHIR/fhir-test-cases/blob/469d90ab58663266d5d7ff82ba3587257fd8a4be/r4/patient-name-extensions.json)
