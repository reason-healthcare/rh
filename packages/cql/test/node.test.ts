import { describe, expect, it } from "vitest";
import { compile, evaluate, explainCompile, explainParse, version } from "../dist/node.js";

const source = "library Test version '1.0' define X: 1 + 2";

describe("@reasonhealth/cql node wrapper", () => {
  it("compiles CQL to ELM JSON", () => {
    const result = compile(source);

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      library: {
        identifier: {
          id: "Test",
          version: "1.0"
        }
      }
    });
  });

  it("evaluates a named ELM expression", () => {
    const compiled = compile(source);
    expect(compiled.success).toBe(true);

    const result = evaluate(compiled.data ?? "", { expression: "X" });

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      expression: "X",
      result: 3,
      type: "integer"
    });
  });

  it("explains CQL parse output", () => {
    const result = explainParse(source);

    expect(result.success).toBe(true);
    expect(result.data).toContain("ExpressionDef(X)");
  });

  it("explains CQL compile output", () => {
    const result = explainCompile(source);

    expect(result.success).toBe(true);
    expect(result.data).toContain("ExpressionDef(X)");
  });

  it("exposes the crate version", () => {
    expect(version()).toMatch(/^\d+\.\d+\.\d+/);
  });
});
