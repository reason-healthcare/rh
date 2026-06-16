export const patientResource = {
  resourceType: "Patient",
  id: "example",
  name: [
    {
      use: "official",
      family: "Shaw",
      given: ["Maya", "Lee"]
    }
  ],
  gender: "female",
  birthDate: "1988-02-14",
  telecom: [
    {
      system: "email",
      value: "maya.shaw@example.org"
    }
  ],
  address: [
    {
      city: "Madison",
      state: "WI"
    }
  ]
};

export const fhirPathExamples = [
  "Patient.name.given",
  "Patient.telecom.where(system = 'email').value",
  "Patient.birthDate.toString()"
] as const;

export const vclExamples = [
  "123456",
  "(http://loinc.org)12345-6 ; concept <<123455",
  "(http://loinc.org)(parent^{LP46821-2,LP259418-4}) ; concept <<123455 ; 123"
] as const;

export const cqlSource = `library Playground version '1.0.0'

define "Greeting":
  'hello'

define "Score":
  40 + 2
`;

export function formatJson(value: unknown): string {
  return JSON.stringify(value, null, 2);
}

export function parseJson(text: string): unknown {
  return JSON.parse(text);
}
