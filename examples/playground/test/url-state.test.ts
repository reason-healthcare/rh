import { describe, expect, it } from "vitest";
import { parseShareableState, serializeShareableState, type ShareableState } from "../src/url-state";

const baseState: ShareableState = {
  tool: "fhirpath",
  fhirpathExpression: "Patient.name.given",
  fhirpathResource: '{\n  "resourceType": "Patient"\n}',
  vclExpression: "<< 123 |Example|",
  vclDefaultSystem: "http://snomed.info/sct",
  cqlSource: "library Demo version '1.0.0'\n\ndefine X: true",
  cqlSourceMap: false
};

describe("playground URL state", () => {
  it("serializes and parses FHIRPath input", () => {
    const search = serializeShareableState(baseState);

    expect(search).toContain("tool=fhirpath");
    expect(parseShareableState(search)).toMatchObject({
      tool: "fhirpath",
      fhirpathExpression: baseState.fhirpathExpression,
      fhirpathResource: baseState.fhirpathResource
    });
  });

  it("serializes and parses VCL source", () => {
    const state = { ...baseState, tool: "vcl" as const };
    const parsed = parseShareableState(serializeShareableState(state));

    expect(parsed).toMatchObject({
      tool: "vcl",
      vclExpression: baseState.vclExpression,
      vclDefaultSystem: baseState.vclDefaultSystem
    });
  });

  it("serializes and parses multiline CQL source with options", () => {
    const state = { ...baseState, tool: "cql" as const, cqlSourceMap: true };
    const parsed = parseShareableState(serializeShareableState(state));

    expect(parsed).toMatchObject({
      tool: "cql",
      cqlSource: baseState.cqlSource,
      cqlSourceMap: true
    });
  });

  it("ignores unknown tools", () => {
    expect(parseShareableState("?tool=unknown&cql=define%20X%3A%20true")).toEqual({
      cqlSource: "define X: true"
    });
  });
});
