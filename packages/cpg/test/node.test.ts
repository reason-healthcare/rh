import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { applyPlanDefinition } from "../dist/node.js";

describe("@reasonhealth/cpg node wrapper", () => {
  it("applies a PlanDefinition and returns a RequestGroup Bundle", () => {
    const planDefinition = JSON.parse(
      readFileSync(resolve(__dirname, "./fixtures/PlanDefinition-SimplePlanDefinition.json"), "utf8")
    );

    const result = applyPlanDefinition(planDefinition, "Patient/123", {
      resourceType: "Bundle",
      type: "collection",
      entry: []
    });

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      resourceType: "Bundle"
    });
    expect(result.value?.entry?.[0]?.resource).toMatchObject({
      resourceType: "RequestGroup"
    });
  });
});
