import { describe, expect, it } from "vitest";
import { evaluateExpression, parseExpression, validateExpression, version } from "../dist/node.js";

describe("@reason-healthcare/fhirpath node wrapper", () => {
  it("parses a FHIRPath expression", () => {
    const result = parseExpression("Patient.name.given");

    expect(result.success).toBe(true);
    expect(result.data).toBeTypeOf("string");
  });

  it("evaluates an expression against JSON resource input", () => {
    const result = evaluateExpression("name.given", {
      resourceType: "Patient",
      name: [{ given: ["Ada"] }]
    });

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      count: 1,
      type: "single_value",
      result: "Ada"
    });
  });

  it("validates expressions and exposes the crate version", () => {
    expect(validateExpression("name.exists()").success).toBe(true);
    expect(version()).toMatch(/^\d+\.\d+\.\d+/);
  });
});
