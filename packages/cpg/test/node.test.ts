import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  applyPlanDefinition,
  assembleQuestionnaire,
  evaluateMeasure,
  populateQuestionnaire,
  validateQuestionnaireResponse
} from "../dist/node.js";

describe("@reasonhealth/cpg node wrapper", () => {
  it("applies a PlanDefinition and returns a RequestGroup Bundle", () => {
    const planDefinition = JSON.parse(
      readFileSync(resolve(fileURLToPath(import.meta.url), "../fixtures/PlanDefinition-SimplePlanDefinition.json"), "utf8")
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

describe("@reasonhealth/cpg measure and questionnaire wrappers", () => {
  it("evaluates a Measure with a FHIRPath population criteria", () => {
    const measure = {
      resourceType: "Measure",
      url: "http://example.org/Measure/TestMeasure",
      library: ["http://example.org/Library/Test"],
      group: [
        {
          population: [
            {
              code: { coding: [{ code: "initial-population" }] },
              criteria: { language: "text/fhirpath", expression: "url" }
            }
          ]
        }
      ]
    };

    const result = evaluateMeasure(measure, "Patient/123", {
      resourceType: "Bundle",
      type: "collection",
      entry: []
    });

    expect(result.success).toBe(true);
    expect(result.value?.resourceType).toBe("MeasureReport");
    expect(result.value?.type).toBe("individual");
    expect(result.value?.group?.[0]?.population?.[0]?.count).toBe(1);
  });

  it("populates a Questionnaire from initial values", () => {
    const questionnaire = {
      resourceType: "Questionnaire",
      url: "http://example.org/Questionnaire/Test",
      item: [
        {
          linkId: "q1",
          text: "Name",
          type: "string",
          initial: [{ valueString: "Alice" }]
        }
      ]
    };

    const result = populateQuestionnaire(questionnaire, "Patient/example", {
      resourceType: "Bundle",
      type: "collection",
      entry: []
    });

    expect(result.success).toBe(true);
    expect(result.value?.resourceType).toBe("QuestionnaireResponse");
    expect(result.value?.item?.[0]?.linkId).toBe("q1");
    expect(result.value?.item?.[0]?.answer?.[0]?.valueString).toBe("Alice");
  });

  it("assembles a referenced subQuestionnaire", () => {
    const root = {
      resourceType: "Questionnaire",
      url: "http://example.org/Questionnaire/root",
      version: "1.0",
      item: [
        { linkId: "normal", type: "string", text: "Normal" },
        {
          linkId: "sub",
          type: "display",
          extension: [
            {
              url: "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-subQuestionnaire",
              valueCanonical: "http://example.org/Questionnaire/sub"
            }
          ]
        }
      ]
    };

    const result = assembleQuestionnaire(root, {
      resourceType: "Bundle",
      type: "collection",
      entry: [
        {
          resource: {
            resourceType: "Questionnaire",
            url: "http://example.org/Questionnaire/sub",
            item: [{ linkId: "sub-item", type: "string", text: "Sub item" }]
          }
        }
      ]
    });

    expect(result.success).toBe(true);
    expect(result.value?.version?.endsWith("-assembled")).toBe(true);
    expect(result.value?.item?.map((item) => item.linkId)).toContain("sub-item");
  });

  it("reports missing required QuestionnaireResponse answers", () => {
    const result = validateQuestionnaireResponse(
      {
        resourceType: "Questionnaire",
        item: [{ linkId: "required", type: "string", required: true }]
      },
      {
        resourceType: "QuestionnaireResponse",
        item: [{ linkId: "required" }]
      }
    );

    expect(result.success).toBe(true);
    expect(result.value?.issues.length).toBeGreaterThanOrEqual(1);
  });
});
