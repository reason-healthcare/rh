import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { dataRequirements, initCql, inspect } from "../dist/web.js";

const retrieveSource = `library DiabetesMeasure version '1.0.0'
using FHIR version '4.0.1'
valueset "Diabetes": 'http://example.org/fhir/ValueSet/diabetes'
context Patient
define "Diabetes Conditions":
  [Condition: "Diabetes"]
define "Has Diabetes":
  exists "Diabetes Conditions"
`;

let initialized = false;

async function initWebWasm(): Promise<void> {
  if (initialized) {
    return;
  }

  const wasmBytes = readFileSync(new URL("../wasm/rh_cql_bg.wasm", import.meta.url));
  await initCql(wasmBytes);
  initialized = true;
}

describe.sequential("@reasonhealth/cql web wrapper", () => {
  it("requires initialization before calling helpers", () => {
    expect(() => inspect(retrieveSource)).toThrow();
  });

  it("runs analytics helpers after initCql", async () => {
    await initWebWasm();

    const result = dataRequirements(retrieveSource);

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      library: "DiabetesMeasure",
      resources: ["Condition"]
    });
  });

  it("surfaces invalid CQL as a wrapper error", async () => {
    await initWebWasm();

    const result = dataRequirements("library Broken version '1.0' define X");

    expect(result.success).toBe(false);
    expect(result.error).toContain("Failed to compile CQL");
    expect(result.value).toBeUndefined();
  });
});
