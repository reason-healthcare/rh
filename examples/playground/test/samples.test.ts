import { describe, expect, it } from "vitest";
import { formatJson, parseJson, patientResource } from "../src/samples";

describe("playground samples", () => {
  it("ship parseable patient JSON", () => {
    expect(parseJson(formatJson(patientResource))).toMatchObject({
      resourceType: "Patient",
      id: "example"
    });
  });
});
