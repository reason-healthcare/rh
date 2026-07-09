import { describe, expect, it } from "vitest";
import {
  compile,
  dataRequirements,
  emitSql,
  emitSqlQueryLibrary,
  emitViewDefinitions,
  evaluate,
  explainCompile,
  explainParse,
  inspect,
  lowerCheck,
  relationalPlan,
  version
} from "../dist/node.js";

const source = "library Test version '1.0' define X: 1 + 2";
const retrieveSource = `library DiabetesMeasure version '1.0.0'
using FHIR version '4.0.1'
valueset "Diabetes": 'http://example.org/fhir/ValueSet/diabetes'
context Patient
define "Diabetes Conditions":
  [Condition: "Diabetes"]
define "Has Diabetes":
  exists "Diabetes Conditions"
`;

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

  it("inspects compiled ELM", () => {
    const result = inspect(retrieveSource);

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      library: "DiabetesMeasure",
      retrieves: [{ resource: "Condition" }]
    });
  });

  it("extracts CQL data requirements", () => {
    const result = dataRequirements(retrieveSource);

    expect(result.success).toBe(true);
    expect(result.value).toMatchObject({
      library: "DiabetesMeasure",
      resources: ["Condition"]
    });
  });

  it("builds a relational plan and lower check", () => {
    const plan = relationalPlan(retrieveSource);
    const report = lowerCheck(retrieveSource);

    expect(plan.success).toBe(true);
    expect(plan.value).toMatchObject({
      library: "DiabetesMeasure",
      target: "sql-on-fhir"
    });
    expect(report.success).toBe(true);
    expect(report.value).toMatchObject({
      target: "sql-on-fhir",
      supported: true
    });
  });

  it("emits SQL-on-FHIR artifacts", () => {
    const views = emitViewDefinitions(retrieveSource);
    const sql = emitSql(retrieveSource);
    const sqlLibrary = emitSqlQueryLibrary(retrieveSource);

    expect(views.success).toBe(true);
    expect(views.value).toMatchObject({
      views: [{ name: "condition_view", resource: "Condition" }]
    });
    expect(sql.success).toBe(true);
    expect(sql.value).toMatchObject({
      views: [{ name: "condition_view" }]
    });
    expect(JSON.stringify(sql.value)).toContain("condition_view");
    expect(sqlLibrary.success).toBe(true);
    expect(sqlLibrary.value).toMatchObject({
      library: {
        resourceType: "Library",
        type: {
          coding: [{ code: "sql-query" }]
        }
      }
    });
  });
});
