import {
  evaluateExpression as evaluateFhirPath,
  initFhirPath,
  parseExpression as parseFhirPath,
  validateExpression as validateFhirPath,
  version as fhirPathVersion
} from "@reason-healthcare/fhirpath/web";
import {
  explainExpression as explainVcl,
  initVcl,
  translateExpression as translateVcl,
  validateExpression as validateVcl,
  version as vclVersion
} from "@reason-healthcare/vcl/web";
import {
  compile as compileCql,
  initCql,
  version as cqlVersion
} from "@reason-healthcare/cql/web";
import "./styles.css";
import {
  cqlSource,
  fhirPathExamples,
  formatJson,
  parseJson,
  patientResource,
  vclExamples
} from "./samples";

type Tool = "fhirpath" | "vcl" | "cql";
type OutputMode = "rendered" | "source";

interface PlaygroundResult<T = unknown> {
  success?: boolean;
  data?: string;
  error?: string;
  value?: T;
}

interface RenderOptions {
  title: string;
}

const state = {
  activeTool: "fhirpath" as Tool
};

const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("Missing app root");
}

app.innerHTML = `
  <section class="shell">
    <header class="topbar">
      <div>
        <p class="eyebrow">Reason Health</p>
        <h1>WASM Playground</h1>
      </div>
      <dl class="versions">
        <div><dt>FHIRPath</dt><dd id="fhirpath-version">...</dd></div>
        <div><dt>VCL</dt><dd id="vcl-version">...</dd></div>
        <div><dt>CQL</dt><dd id="cql-version">...</dd></div>
      </dl>
    </header>

    <nav class="tabs" aria-label="Tool">
      <button class="tab active" type="button" data-tool="fhirpath">FHIRPath</button>
      <button class="tab" type="button" data-tool="vcl">VCL</button>
      <button class="tab" type="button" data-tool="cql">CQL</button>
    </nav>

    <section class="tool active" data-panel="fhirpath">
      <div class="workspace two-pane">
        <form class="pane editor" id="fhirpath-form">
          <label for="fhirpath-expression">Expression</label>
          <input id="fhirpath-expression" value="${fhirPathExamples[0]}" />
          <div class="chips" id="fhirpath-examples"></div>
          <label for="fhirpath-resource">Resource JSON</label>
          <textarea id="fhirpath-resource" spellcheck="false">${formatJson(patientResource)}</textarea>
          <div class="actions">
            <button type="submit">Evaluate</button>
            <button type="button" class="secondary" id="fhirpath-parse">Parse</button>
            <button type="button" class="secondary" id="fhirpath-validate">Validate</button>
          </div>
        </form>
        <output class="pane result" id="fhirpath-output"></output>
      </div>
    </section>

    <section class="tool" data-panel="vcl">
      <div class="workspace two-pane">
        <form class="pane editor" id="vcl-form">
          <label for="vcl-expression">Expression</label>
          <textarea id="vcl-expression" class="short" spellcheck="false">${vclExamples[1]}</textarea>
          <div class="chips" id="vcl-examples"></div>
          <label for="vcl-default-system">Default System</label>
          <input id="vcl-default-system" value="http://snomed.info/sct" />
          <div class="actions">
            <button type="submit">Translate</button>
            <button type="button" class="secondary" id="vcl-explain">Explain</button>
            <button type="button" class="secondary" id="vcl-validate">Validate</button>
          </div>
        </form>
        <output class="pane result" id="vcl-output"></output>
      </div>
    </section>

    <section class="tool" data-panel="cql">
      <div class="workspace two-pane">
        <form class="pane editor" id="cql-form">
          <label for="cql-source">CQL Source</label>
          <textarea id="cql-source" spellcheck="false">${cqlSource}</textarea>
          <div class="actions">
            <button type="submit">Compile</button>
            <label class="toggle"><input type="checkbox" id="cql-source-map" /> Source map</label>
          </div>
        </form>
        <output class="pane result" id="cql-output"></output>
      </div>
    </section>
  </section>
`;

bindTabs();
bindExamples("fhirpath-examples", fhirPathExamples, "fhirpath-expression");
bindExamples("vcl-examples", vclExamples, "vcl-expression");
bindFhirPath();
bindVcl();
bindCql();
void initializeWasm();

async function initializeWasm(): Promise<void> {
  await Promise.all([initFhirPath(), initVcl(), initCql()]);
  renderVersions();
  evaluateFhirPathForm();
}

function bindTabs(): void {
  document.querySelectorAll<HTMLButtonElement>("[data-tool]").forEach((button) => {
    button.addEventListener("click", () => {
      const tool = button.dataset.tool as Tool;
      state.activeTool = tool;
      document.querySelectorAll("[data-tool]").forEach((tab) => {
        tab.classList.toggle("active", tab === button);
      });
      document.querySelectorAll<HTMLElement>("[data-panel]").forEach((panel) => {
        panel.classList.toggle("active", panel.dataset.panel === state.activeTool);
      });
    });
  });
}

function bindExamples(containerId: string, examples: readonly string[], targetId: string): void {
  const container = requiredElement<HTMLDivElement>(containerId);
  const target = requiredElement<HTMLInputElement | HTMLTextAreaElement>(targetId);
  container.replaceChildren(
    ...examples.map((example) => {
      const button = document.createElement("button");
      button.type = "button";
      button.className = "chip";
      button.textContent = example;
      button.addEventListener("click", () => {
        target.value = example;
      });
      return button;
    })
  );
}

function bindFhirPath(): void {
  requiredElement<HTMLFormElement>("fhirpath-form").addEventListener("submit", (event) => {
    event.preventDefault();
    evaluateFhirPathForm();
  });
  requiredElement<HTMLButtonElement>("fhirpath-parse").addEventListener("click", () => {
    renderResult("fhirpath-output", parseFhirPath(expressionValue("fhirpath-expression")), {
      title: "FHIRPath parse"
    });
  });
  requiredElement<HTMLButtonElement>("fhirpath-validate").addEventListener("click", () => {
    renderResult("fhirpath-output", validateFhirPath(expressionValue("fhirpath-expression")), {
      title: "FHIRPath validation"
    });
  });
}

function evaluateFhirPathForm(): void {
  try {
    const resource = parseJson(textValue("fhirpath-resource"));
    renderResult(
      "fhirpath-output",
      evaluateFhirPath(expressionValue("fhirpath-expression"), resource),
      { title: "FHIRPath result" }
    );
  } catch (error) {
    renderError("fhirpath-output", error);
  }
}

function bindVcl(): void {
  requiredElement<HTMLFormElement>("vcl-form").addEventListener("submit", (event) => {
    event.preventDefault();
    renderResult(
      "vcl-output",
      translateVcl(textValue("vcl-expression"), {
        defaultSystem: optionalValue("vcl-default-system")
      }),
      { title: "FHIR ValueSet compose" }
    );
  });
  requiredElement<HTMLButtonElement>("vcl-explain").addEventListener("click", () => {
    renderResult("vcl-output", explainVcl(textValue("vcl-expression"), optionalValue("vcl-default-system")), {
      title: "VCL explanation"
    });
  });
  requiredElement<HTMLButtonElement>("vcl-validate").addEventListener("click", () => {
    renderResult("vcl-output", validateVcl(textValue("vcl-expression")), {
      title: "VCL validation"
    });
  });
}

function bindCql(): void {
  requiredElement<HTMLFormElement>("cql-form").addEventListener("submit", (event) => {
    event.preventDefault();
    renderResult(
      "cql-output",
      compileCql(textValue("cql-source"), {
        sourceMap: requiredElement<HTMLInputElement>("cql-source-map").checked
      }),
      { title: "CQL ELM output" }
    );
  });
}

function renderVersions(): void {
  setText("fhirpath-version", fhirPathVersion());
  setText("vcl-version", vclVersion());
  setText("cql-version", cqlVersion());
}

function renderResult(targetId: string, value: unknown, options: RenderOptions): void {
  const target = requiredElement<HTMLOutputElement>(targetId);
  target.classList.remove("error");
  const source = formatJson(value);
  const rendered = renderOutputBody(value);
  target.innerHTML = [
    `<div class="result-toolbar">`,
    `<div>`,
    `<p class="result-eyebrow">Output</p>`,
    `<h2>${escapeHtml(options.title)}</h2>`,
    `</div>`,
    `<div class="view-toggle" role="group" aria-label="Output view">`,
    `<button type="button" class="active" data-output-view="rendered">Rendered</button>`,
    `<button type="button" data-output-view="source">Source</button>`,
    `</div>`,
    `</div>`,
    `<div class="result-view active" data-output-panel="rendered">${rendered}</div>`,
    `<div class="result-view" data-output-panel="source"><pre>${escapeHtml(source)}</pre></div>`
  ].join("");
  bindOutputToggle(target);
}

function renderError(targetId: string, error: unknown): void {
  const target = requiredElement<HTMLOutputElement>(targetId);
  target.classList.add("error");
  const message = error instanceof Error ? error.message : String(error);
  target.innerHTML = [
    `<div class="result-toolbar">`,
    `<div>`,
    `<p class="result-eyebrow">Error</p>`,
    `<h2>Unable to render</h2>`,
    `</div>`,
    `</div>`,
    `<div class="result-view active">`,
    `<section class="rendered-block danger">`,
    `<h3>Request failed</h3>`,
    `<p>${escapeHtml(message)}</p>`,
    `</section>`,
    `</div>`
  ].join("");
}

function bindOutputToggle(target: HTMLElement): void {
  const buttons = [...target.querySelectorAll<HTMLButtonElement>("[data-output-view]")];
  const panels = [...target.querySelectorAll<HTMLElement>("[data-output-panel]")];
  buttons.forEach((button) => {
    button.addEventListener("click", () => {
      const mode = button.dataset.outputView as OutputMode;
      buttons.forEach((candidate) => candidate.classList.toggle("active", candidate === button));
      panels.forEach((panel) => panel.classList.toggle("active", panel.dataset.outputPanel === mode));
    });
  });
}

function renderOutputBody(value: unknown): string {
  const result = asPlaygroundResult(value);
  const payload = result.value ?? parseMaybeJson(result.data) ?? value;
  const status = renderStatus(result);

  if (result.success === false) {
    return `${status}<section class="rendered-block danger"><h3>Error</h3><p>${escapeHtml(result.error ?? "The operation failed.")}</p></section>`;
  }

  return `${status}${renderPayload(payload)}`;
}

function renderStatus(result: PlaygroundResult): string {
  if (result.success === undefined) {
    return "";
  }
  const label = result.success ? "Success" : "Failed";
  const tone = result.success ? "success" : "danger";
  return `<div class="status-line ${tone}"><span>${label}</span>${result.error ? `<p>${escapeHtml(result.error)}</p>` : ""}</div>`;
}

function renderPayload(payload: unknown): string {
  if (isValidationPayload(payload)) {
    return renderValidationPayload(payload);
  }
  if (isElmLibraryPayload(payload)) {
    return renderElmLibrary(payload);
  }
  if (isValueSetComposePayload(payload)) {
    return renderValueSetCompose(payload);
  }
  if (Array.isArray(payload)) {
    return renderArrayPayload(payload);
  }
  if (isRecord(payload)) {
    return renderObjectPayload(payload);
  }
  return `<section class="rendered-block"><h3>Value</h3><p class="scalar">${escapeHtml(String(payload ?? ""))}</p></section>`;
}

function renderValidationPayload(payload: { valid: boolean; message?: string }): string {
  return `
    <section class="rendered-block ${payload.valid ? "success" : "danger"}">
      <h3>${payload.valid ? "Valid" : "Invalid"}</h3>
      <p>${escapeHtml(payload.message ?? (payload.valid ? "No issues found." : "Validation failed."))}</p>
    </section>
  `;
}

function renderElmLibrary(payload: { library: unknown }): string {
  const library = isRecord(payload.library) ? payload.library : {};
  const identifier = isRecord(library.identifier) ? library.identifier : {};
  const statements = isRecord(library.statements) && Array.isArray(library.statements.def)
    ? library.statements.def
    : [];
  const title = [identifier.id, identifier.version].filter(Boolean).join(" ") || "Anonymous library";
  const rows = statements.map((statement) => {
    const row = isRecord(statement) ? statement : {};
    const expression = isRecord(row.expression) ? row.expression : {};
    return `<tr><td>${escapeHtml(String(row.name ?? "(unnamed)"))}</td><td>${escapeHtml(String(expression.type ?? "expression"))}</td></tr>`;
  }).join("");

  return `
    <section class="rendered-block">
      <h3>${escapeHtml(title)}</h3>
      <p>${statements.length} definition${statements.length === 1 ? "" : "s"} emitted to ELM.</p>
      ${rows ? `<table><thead><tr><th>Definition</th><th>Expression</th></tr></thead><tbody>${rows}</tbody></table>` : ""}
    </section>
  `;
}

function renderValueSetCompose(payload: { include?: unknown[]; exclude?: unknown[] }): string {
  const include = Array.isArray(payload.include) ? payload.include : [];
  const exclude = Array.isArray(payload.exclude) ? payload.exclude : [];
  const rows = [...include.map((entry) => renderComposeRow("include", entry)), ...exclude.map((entry) => renderComposeRow("exclude", entry))].join("");
  return `
    <section class="rendered-block">
      <h3>ValueSet.compose</h3>
      <p>${include.length} include block${include.length === 1 ? "" : "s"}${exclude.length ? `, ${exclude.length} exclude block${exclude.length === 1 ? "" : "s"}` : ""}.</p>
      ${rows ? `<table><thead><tr><th>Mode</th><th>System</th><th>Filters / concepts</th></tr></thead><tbody>${rows}</tbody></table>` : ""}
    </section>
  `;
}

function renderComposeRow(mode: string, entry: unknown): string {
  const row = isRecord(entry) ? entry : {};
  const filters = Array.isArray(row.filter)
    ? row.filter.map((filter) => isRecord(filter) ? `${filter.property ?? "property"} ${filter.op ?? "="} ${filter.value ?? ""}` : String(filter))
    : [];
  const concepts = Array.isArray(row.concept)
    ? row.concept.map((concept) => isRecord(concept) ? String(concept.code ?? concept.display ?? "concept") : String(concept))
    : [];
  const details = [...filters, ...concepts].join("; ") || "All codes in system";
  return `<tr><td>${escapeHtml(mode)}</td><td>${escapeHtml(String(row.system ?? "(default)"))}</td><td>${escapeHtml(details)}</td></tr>`;
}

function renderArrayPayload(payload: unknown[]): string {
  if (payload.length === 0) {
    return `<section class="rendered-block"><h3>Collection</h3><p>No values returned.</p></section>`;
  }
  const items = payload.map((item) => `<li>${renderInlineValue(item)}</li>`).join("");
  return `<section class="rendered-block"><h3>Collection</h3><ul>${items}</ul></section>`;
}

function renderObjectPayload(payload: Record<string, unknown>): string {
  const entries = Object.entries(payload);
  if (entries.length === 0) {
    return `<section class="rendered-block"><h3>Object</h3><p>No fields returned.</p></section>`;
  }
  const rows = entries.map(([key, entry]) => `<tr><th>${escapeHtml(key)}</th><td>${renderInlineValue(entry)}</td></tr>`).join("");
  return `<section class="rendered-block"><h3>Rendered fields</h3><table><tbody>${rows}</tbody></table></section>`;
}

function renderInlineValue(value: unknown): string {
  if (value === null || value === undefined) {
    return `<span class="muted">null</span>`;
  }
  if (Array.isArray(value)) {
    return escapeHtml(value.map((item) => typeof item === "object" ? JSON.stringify(item) : String(item)).join(", "));
  }
  if (isRecord(value)) {
    const preferred = value.value ?? value.code ?? value.display ?? value.name ?? value.type;
    return escapeHtml(preferred === undefined ? JSON.stringify(value) : String(preferred));
  }
  return escapeHtml(String(value));
}

function asPlaygroundResult(value: unknown): PlaygroundResult {
  return isRecord(value) && ("success" in value || "data" in value || "error" in value)
    ? value as PlaygroundResult
    : {};
}

function parseMaybeJson(value: string | undefined): unknown {
  if (value === undefined) {
    return undefined;
  }
  try {
    return JSON.parse(value);
  } catch {
    return value;
  }
}

function isValidationPayload(value: unknown): value is { valid: boolean; message?: string } {
  return isRecord(value) && typeof value.valid === "boolean";
}

function isElmLibraryPayload(value: unknown): value is { library: unknown } {
  return isRecord(value) && isRecord(value.library);
}

function isValueSetComposePayload(value: unknown): value is { include?: unknown[]; exclude?: unknown[] } {
  return isRecord(value) && (Array.isArray(value.include) || Array.isArray(value.exclude));
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function expressionValue(id: string): string {
  return requiredElement<HTMLInputElement>(id).value.trim();
}

function textValue(id: string): string {
  return requiredElement<HTMLInputElement | HTMLTextAreaElement>(id).value;
}

function optionalValue(id: string): string | undefined {
  const value = textValue(id).trim();
  return value.length === 0 ? undefined : value;
}

function setText(id: string, value: string): void {
  requiredElement<HTMLElement>(id).textContent = value;
}

function requiredElement<T extends HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) {
    throw new Error(`Missing element #${id}`);
  }
  return element as T;
}

function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}
