import { describe, expect, it } from "vitest";
import { explainExpression, parseExpression, translateExpression, validateExpression, version } from "../dist/node.js";

describe("@reasonhealth/vcl node wrapper", () => {
  it("parses a VCL expression", () => {
    const result = parseExpression("123456");

    expect(result.success).toBe(true);
    expect(result.data).toBeTypeOf("string");
  });

  it("translates VCL to ValueSet.compose JSON", () => {
    const result = translateExpression("123456", {
      defaultSystem: "http://snomed.info/sct"
    });

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      include: [
        {
          system: "http://snomed.info/sct",
          concept: [{ code: "123456" }]
        }
      ]
    });
  });

  it("validates, explains, and exposes the crate version", () => {
    expect(validateExpression("123456").success).toBe(true);
    expect(explainExpression("123456").success).toBe(true);
    expect(version()).toMatch(/^\d+\.\d+\.\d+/);
  });
});
