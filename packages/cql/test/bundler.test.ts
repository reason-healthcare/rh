import { execFileSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import { describe, expect, it } from "vitest";

const retrieveSource = `library DiabetesMeasure version '1.0.0'
using FHIR version '4.0.1'
valueset "Diabetes": 'http://example.org/fhir/ValueSet/diabetes'
context Patient
define "Diabetes Conditions":
  [Condition: "Diabetes"]
define "Has Diabetes":
  exists "Diabetes Conditions"
`;

describe("@reasonhealth/cql bundler wrapper", () => {
  it("runs analytics helpers from the bundler dist output", () => {
    const entry = pathToFileURL(new URL("../dist/bundler.js", import.meta.url).pathname).href;
    const script = `
      const cql = await import(${JSON.stringify(entry)});
      const result = cql.dataRequirements(${JSON.stringify(retrieveSource)});
      console.log(JSON.stringify({
        success: result.success,
        resources: result.value?.resources
      }));
    `;

    const stdout = execFileSync(
      process.execPath,
      ["--experimental-wasm-modules", "--input-type=module", "--eval", script],
      {
        encoding: "utf8"
      }
    );
    const result = JSON.parse(stdout);

    expect(result).toMatchObject({
      success: true,
      resources: ["Condition"]
    });
  });
});
