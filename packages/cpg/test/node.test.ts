import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  applyPlanDefinition,
  assembleQuestionnaire,
  evaluateMeasure,
  extractQuestionnaireObservations,
  populateQuestionnaire,
  reconcileExtractedObservations,
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
  it("extracts constrained SDC Boolean Observations and replaces exact QR provenance", () => {
    const questionnaire = {
      resourceType: "Questionnaire",
      url: "http://example.org/Questionnaire/fall-screen",
      version: "1.0.0",
      meta: {
        profile: ["http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-extr-obsn"]
      },
      extension: [{
        url: "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-observationExtract",
        valueBoolean: true
      }],
      item: [
        { linkId: "one", type: "boolean", required: true, code: [{ system: "http://loinc.org", version: "2.81", code: "one" }] },
        { linkId: "two", type: "boolean", required: true, code: [{ system: "http://loinc.org", version: "2.81", code: "two" }] },
        { linkId: "three", type: "boolean", required: true, code: [{ system: "http://loinc.org", version: "2.81", code: "three" }] }
      ]
    };
    const response = {
      resourceType: "QuestionnaireResponse",
      id: "response-1",
      status: "completed",
      questionnaire: "http://example.org/Questionnaire/fall-screen|1.0.0",
      subject: { reference: "Patient/1" },
      encounter: { reference: "Encounter/1" },
      author: { reference: "Patient/1" },
      authored: "2026-06-15T09:20:00Z",
      item: [
        { linkId: "one", answer: [{ valueBoolean: true }] },
        { linkId: "two", answer: [{ valueBoolean: false }] },
        { linkId: "three", answer: [{ valueBoolean: false }] }
      ]
    };

    const result = extractQuestionnaireObservations(questionnaire, response, "Patient/1", {
      encounter: "Encounter/1"
    });
    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({ status: "extracted" });
    expect(result.value?.observations).toHaveLength(3);

    const data = {
      resourceType: "Bundle",
      type: "collection",
      entry: [
        { resource: { resourceType: "Observation", id: "old", derivedFrom: [{ reference: "QuestionnaireResponse/response-1" }] } },
        { resource: { resourceType: "Observation", id: "aggregate", derivedFrom: [{ reference: "QuestionnaireResponse/response-1" }, { reference: "QuestionnaireResponse/other" }] } },
        { resource: { resourceType: "Observation", id: "other", derivedFrom: [{ reference: "QuestionnaireResponse/other" }] } },
        { resource: { resourceType: "Condition", id: "preserved" } }
      ]
    };
    const reconciled = reconcileExtractedObservations(
      data,
      "QuestionnaireResponse/response-1",
      result.value?.transaction
    );
    const resources = reconciled.entry as Array<{ resource: { resourceType: string; id?: string } }>;
    expect(resources.some(({ resource }) => resource.id === "old")).toBe(false);
    expect(resources.some(({ resource }) => resource.id === "aggregate")).toBe(true);
    expect(resources.some(({ resource }) => resource.id === "other")).toBe(true);
    expect(resources.filter(({ resource }) => resource.resourceType === "Observation")).toHaveLength(5);
    expect(resources.filter(({ resource }) => resource.resourceType === "Observation").every((entry) =>
      entry.resource.id !== undefined || typeof (entry as { fullUrl?: unknown }).fullUrl === "string"
    )).toBe(true);

    const cleared = reconcileExtractedObservations(data, "QuestionnaireResponse/response-1");
    expect((cleared.entry as Array<{ resource: { id?: string } }>).some(({ resource }) => resource.id === "old")).toBe(false);
  });

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
              criteria: { language: "text/fhirpath", expression: "active = true" }
            }
          ]
        }
      ]
    };

    const result = evaluateMeasure(measure, "Patient/123", {
      resourceType: "Bundle",
      type: "collection",
      entry: []
    }, {
      data: {
        resourceType: "Bundle",
        type: "collection",
        entry: [
          { resource: { resourceType: "Patient", id: "123", active: true } }
        ]
      }
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

  it("keeps the authored versioned canonical in an assembled response", () => {
    const root = {
      resourceType: "Questionnaire",
      url: "http://example.org/Questionnaire/root",
      version: "0.2.0",
      item: [{ linkId: "screen", type: "boolean" }]
    };
    const content = { resourceType: "Bundle", type: "collection", entry: [] };

    const assembled = assembleQuestionnaire(root, content);
    expect(assembled.success).toBe(true);
    expect(assembled.value?.version).toBe("0.2.0-assembled");

    const populated = populateQuestionnaire(assembled.value, "Patient/example", content, {
      encounter: "Encounter/selected"
    });
    expect(populated.success).toBe(true);
    expect(populated.value?.questionnaire).toBe("http://example.org/Questionnaire/root|0.2.0");
    expect(populated.value?.encounter).toEqual({ reference: "Encounter/selected" });
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
