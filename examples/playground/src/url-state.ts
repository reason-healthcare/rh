export type Tool = "fhirpath" | "vcl" | "cql";

export interface ShareableState {
  tool: Tool;
  fhirpathExpression: string;
  fhirpathResource: string;
  vclExpression: string;
  vclDefaultSystem: string;
  cqlSource: string;
  cqlSourceMap: boolean;
}

const tools = new Set<Tool>(["fhirpath", "vcl", "cql"]);

export function parseShareableState(search: string): Partial<ShareableState> {
  const params = new URLSearchParams(search.startsWith("?") ? search.slice(1) : search);
  const tool = params.get("tool");

  return {
    ...(isTool(tool) ? { tool } : {}),
    ...optionalParam(params, "fp", "fhirpathExpression"),
    ...optionalParam(params, "resource", "fhirpathResource"),
    ...optionalParam(params, "vcl", "vclExpression"),
    ...optionalParam(params, "system", "vclDefaultSystem"),
    ...optionalParam(params, "cql", "cqlSource"),
    ...(params.has("sourceMap") ? { cqlSourceMap: params.get("sourceMap") === "1" } : {})
  };
}

export function serializeShareableState(state: ShareableState): string {
  const params = new URLSearchParams();
  params.set("tool", state.tool);

  if (state.tool === "fhirpath") {
    params.set("fp", state.fhirpathExpression);
    params.set("resource", state.fhirpathResource);
  }

  if (state.tool === "vcl") {
    params.set("vcl", state.vclExpression);
    params.set("system", state.vclDefaultSystem);
  }

  if (state.tool === "cql") {
    params.set("cql", state.cqlSource);
    if (state.cqlSourceMap) {
      params.set("sourceMap", "1");
    }
  }

  return `?${params.toString()}`;
}

function optionalParam<K extends keyof ShareableState>(
  params: URLSearchParams,
  paramName: string,
  stateKey: K
): Pick<ShareableState, K> | Record<string, never> {
  const value = params.get(paramName);
  return value === null ? {} : { [stateKey]: value } as Pick<ShareableState, K>;
}

function isTool(value: string | null): value is Tool {
  return value !== null && tools.has(value as Tool);
}
