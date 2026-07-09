import {
  evaluateExpression as evaluateFhirPath,
  initFhirPath,
  parseExpression as parseFhirPath,
  validateExpression as validateFhirPath,
  version as fhirPathVersion
} from "@reasonhealth/fhirpath/web";
import {
  explainExpression as explainVcl,
  initVcl,
  translateExpression as translateVcl,
  validateExpression as validateVcl,
  version as vclVersion
} from "@reasonhealth/vcl/web";
import {
  compile as compileCql,
  dataRequirements as cqlDataRequirements,
  emitSql as cqlEmitSql,
  emitSqlQueryLibrary as cqlEmitSqlQueryLibrary,
  emitViewDefinitions as cqlEmitViewDefinitions,
  explainCompile as explainCqlCompile,
  explainParse as explainCqlParse,
  initCql,
  inspect as inspectCql,
  lowerCheck as cqlLowerCheck,
  relationalPlan as cqlRelationalPlan,
  version as cqlVersion
} from "@reasonhealth/cql/web";
import "./styles.css";
import {
  cqlSource,
  fhirPathExamples,
  formatJson,
  parseJson,
  patientResource,
  vclExamples
} from "./samples";
import {
  parseShareableState,
  serializeShareableState,
  type ShareableState,
  type Tool
} from "./url-state";

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
let shareStatusTimeout: number | undefined;

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
      <div class="topbar-tools">
        <dl class="versions">
          <div><dt>FHIRPath</dt><dd id="fhirpath-version">...</dd></div>
          <div><dt>VCL</dt><dd id="vcl-version">...</dd></div>
          <div><dt>CQL</dt><dd id="cql-version">...</dd></div>
        </dl>
        <button class="share-button" type="button" id="share-link">Share link</button>
      </div>
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
<button type="button" class="secondary" id="cql-explain-compile">Explain Compile</button>
<button type="button" class="secondary" id="cql-explain-parse">Explain Parse</button>
<button type="button" class="secondary" id="cql-inspect">Inspect</button>
<button type="button" class="secondary" id="cql-data-requirements">Data Requirements</button>
<button type="button" class="secondary" id="cql-plan">Plan</button>
<button type="button" class="secondary" id="cql-lower-check">Lower Check</button>
<button type="button" class="secondary" id="cql-emit-views">Emit Views</button>
<button type="button" class="secondary" id="cql-emit-sql">Emit SQL</button>
<button type="button" class="secondary" id="cql-emit-sql-library">SQL Library</button>
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
bindShareLink();
bindShareableInputs();
applyShareableStateFromUrl();
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
      activateTool(tool);
      syncShareUrl();
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
        syncShareUrl();
      });
      return button;
    })
  );
}

function bindShareLink(): void {
  requiredElement<HTMLButtonElement>("share-link").addEventListener("click", async () => {
    const url = syncShareUrl();
    try {
      await navigator.clipboard.writeText(url);
      showShareStatus("Copied");
    } catch {
      showShareStatus("Link ready");
      window.prompt("Share link", url);
    }
  });

  window.addEventListener("popstate", () => {
    applyShareableStateFromUrl();
  });
}

function bindShareableInputs(): void {
  [
    "fhirpath-expression",
    "fhirpath-resource",
    "vcl-expression",
    "vcl-default-system",
    "cql-source",
    "cql-source-map"
  ].forEach((id) => {
    requiredElement<HTMLInputElement | HTMLTextAreaElement>(id).addEventListener("input", () => {
      syncShareUrl();
    });
  });
}

function activateTool(tool: Tool): void {
  state.activeTool = tool;
  document.querySelectorAll<HTMLButtonElement>("[data-tool]").forEach((tab) => {
    tab.classList.toggle("active", tab.dataset.tool === state.activeTool);
  });
  document.querySelectorAll<HTMLElement>("[data-panel]").forEach((panel) => {
    panel.classList.toggle("active", panel.dataset.panel === state.activeTool);
  });
}

function applyShareableStateFromUrl(): void {
  const shared = parseShareableState(window.location.search);
  if (shared.fhirpathExpression !== undefined) {
    requiredElement<HTMLInputElement>("fhirpath-expression").value = shared.fhirpathExpression;
  }
  if (shared.fhirpathResource !== undefined) {
    requiredElement<HTMLTextAreaElement>("fhirpath-resource").value = shared.fhirpathResource;
  }
  if (shared.vclExpression !== undefined) {
    requiredElement<HTMLTextAreaElement>("vcl-expression").value = shared.vclExpression;
  }
  if (shared.vclDefaultSystem !== undefined) {
    requiredElement<HTMLInputElement>("vcl-default-system").value = shared.vclDefaultSystem;
  }
  if (shared.cqlSource !== undefined) {
    requiredElement<HTMLTextAreaElement>("cql-source").value = shared.cqlSource;
  }
  if (shared.cqlSourceMap !== undefined) {
    requiredElement<HTMLInputElement>("cql-source-map").checked = shared.cqlSourceMap;
  }
  activateTool(shared.tool ?? state.activeTool);
}

function syncShareUrl(): string {
  const search = serializeShareableState(currentShareableState());
  const url = `${window.location.pathname}${search}${window.location.hash}`;
  window.history.replaceState(null, "", url);
  return window.location.href;
}

function currentShareableState(): ShareableState {
  return {
    tool: state.activeTool,
    fhirpathExpression: requiredElement<HTMLInputElement>("fhirpath-expression").value,
    fhirpathResource: requiredElement<HTMLTextAreaElement>("fhirpath-resource").value,
    vclExpression: requiredElement<HTMLTextAreaElement>("vcl-expression").value,
    vclDefaultSystem: requiredElement<HTMLInputElement>("vcl-default-system").value,
    cqlSource: requiredElement<HTMLTextAreaElement>("cql-source").value,
    cqlSourceMap: requiredElement<HTMLInputElement>("cql-source-map").checked
  };
}

function showShareStatus(label: string): void {
  const button = requiredElement<HTMLButtonElement>("share-link");
  button.textContent = label;
  window.clearTimeout(shareStatusTimeout);
  shareStatusTimeout = window.setTimeout(() => {
    button.textContent = "Share link";
  }, 1400);
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
  const cqlSourceText = () => textValue("cql-source");
  const compileOptions = () => ({
    sourceMap: requiredElement<HTMLInputElement>("cql-source-map").checked
  });

  requiredElement<HTMLFormElement>("cql-form").addEventListener("submit", (event) => {
    event.preventDefault();
    renderResult(
      "cql-output",
      compileCql(cqlSourceText(), compileOptions()),
      { title: "CQL ELM output" }
    );
  });
  requiredElement<HTMLButtonElement>("cql-explain-compile").addEventListener("click", () => {
    renderResult(
      "cql-output",
      explainCqlCompile(cqlSourceText(), compileOptions()),
      { title: "CQL compile explanation" }
    );
  });
  requiredElement<HTMLButtonElement>("cql-explain-parse").addEventListener("click", () => {
    renderResult("cql-output", explainCqlParse(cqlSourceText()), {
      title: "CQL parse explanation"
    });
  });
  requiredElement<HTMLButtonElement>("cql-inspect").addEventListener("click", () => {
    renderResult("cql-output", inspectCql(cqlSourceText()), {
      title: "CQL inspection"
    });
  });
  requiredElement<HTMLButtonElement>("cql-data-requirements").addEventListener("click", () => {
    renderResult("cql-output", cqlDataRequirements(cqlSourceText()), {
      title: "CQL data requirements"
    });
  });
  requiredElement<HTMLButtonElement>("cql-plan").addEventListener("click", () => {
    renderResult("cql-output", cqlRelationalPlan(cqlSourceText()), {
      title: "CQL relational plan"
    });
  });
  requiredElement<HTMLButtonElement>("cql-lower-check").addEventListener("click", () => {
    renderResult("cql-output", cqlLowerCheck(cqlSourceText()), {
      title: "SQL-on-FHIR lower check"
    });
  });
  requiredElement<HTMLButtonElement>("cql-emit-views").addEventListener("click", () => {
    renderResult("cql-output", cqlEmitViewDefinitions(cqlSourceText()), {
      title: "SQL-on-FHIR ViewDefinitions"
    });
  });
  requiredElement<HTMLButtonElement>("cql-emit-sql").addEventListener("click", () => {
    renderResult("cql-output", cqlEmitSql(cqlSourceText()), {
      title: "SQL-on-FHIR SQL"
    });
  });
  requiredElement<HTMLButtonElement>("cql-emit-sql-library").addEventListener("click", () => {
    renderResult("cql-output", cqlEmitSqlQueryLibrary(cqlSourceText()), {
      title: "SQLQuery Library"
    });
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
  if (isCqlSourceMapPayload(payload)) {
    return renderCqlSourceMapPayload(payload);
  }
  if (isElmLibraryPayload(payload)) {
    return renderElmLibrary(payload);
  }
  if (isSqlTextPayload(payload)) {
    return renderSqlTextPayload(payload);
  }
  if (isLowerCheckPayload(payload)) {
    return renderLowerCheckPayload(payload);
  }
  if (isViewGenerationPayload(payload)) {
    return renderViewGenerationPayload(payload);
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
  if (typeof payload === "string" && payload.includes("\n")) {
    return renderTextPayload(payload);
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

function renderCqlSourceMapPayload(payload: { library: unknown; sourceMap: unknown }): string {
  const library = isRecord(payload.library) ? payload.library : {};
  const identifier = isRecord(library.identifier) ? library.identifier : {};
  const sourceMap = isRecord(payload.sourceMap) ? payload.sourceMap : {};
  const sourceDocuments = Array.isArray(sourceMap.source_documents) ? sourceMap.source_documents : [];
  const mappings = Array.isArray(sourceMap.mappings) ? sourceMap.mappings : [];
  const nodeMetas = Array.isArray(sourceMap.elm_node_metas) ? sourceMap.elm_node_metas : [];
  const record = payload as Record<string, unknown>;
  const diagnostics = ["errors", "warnings", "messages"].map((key) => {
    const values = Array.isArray(record[key]) ? record[key] : [];
    return { key, count: values.length };
  });
  const rows = sourceDocuments.map((entry) => {
    const doc = isRecord(entry) ? entry : {};
    return `<tr><td>${escapeHtml(String(doc.doc_id ?? "(document)"))}</td><td>${escapeHtml(String(doc.uri ?? "(inline source)"))}</td></tr>`;
  }).join("");
  const title = [identifier.id, identifier.version].filter(Boolean).join(" ") || "Anonymous library";
  const diagnosticText = diagnostics.map(({ key, count }) => `${count} ${key}`).join(", ");

  return [
    `<section class="rendered-block">`,
    `<h3>${escapeHtml(title)} source map</h3>`,
    `<div class="metric-grid">`,
    `<div><span>${sourceDocuments.length}</span><p>source document${sourceDocuments.length === 1 ? "" : "s"}</p></div>`,
    `<div><span>${mappings.length}</span><p>mapping${mappings.length === 1 ? "" : "s"}</p></div>`,
    `<div><span>${nodeMetas.length}</span><p>ELM node${nodeMetas.length === 1 ? "" : "s"}</p></div>`,
    `</div>`,
    `<p>${escapeHtml(diagnosticText)}.</p>`,
    rows ? `<table><thead><tr><th>Document</th><th>URI</th></tr></thead><tbody>${rows}</tbody></table>` : "",
    `</section>`
  ].join("");
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

function renderLowerCheckPayload(payload: { target: string; supported: boolean; supportedNodes?: unknown[]; unsupportedNodes?: unknown[]; notes?: unknown[] }): string {
  const unsupported = Array.isArray(payload.unsupportedNodes) ? payload.unsupportedNodes : [];
  const supported = Array.isArray(payload.supportedNodes) ? payload.supportedNodes : [];
  const notes = Array.isArray(payload.notes) ? payload.notes : [];
  const rows = [...supported.map((node) => renderSupportRow("supported", node)), ...unsupported.map((node) => renderSupportRow("unsupported", node))].join("");

  return `
    <section class="rendered-block ${payload.supported ? "success" : "danger"}">
      <h3>${escapeHtml(payload.target)} lower check</h3>
      <p>${payload.supported ? "All encountered node kinds are supported by the first-pass lowerer." : `${unsupported.length} unsupported node kind${unsupported.length === 1 ? "" : "s"} found.`}</p>
      ${rows ? `<table><thead><tr><th>Status</th><th>ELM node</th><th>Count</th></tr></thead><tbody>${rows}</tbody></table>` : ""}
      ${notes.length ? `<ul>${notes.map((note) => `<li>${escapeHtml(String(note))}</li>`).join("")}</ul>` : ""}
    </section>
  `;
}

function renderSupportRow(status: string, node: unknown): string {
  const row = isRecord(node) ? node : {};
  return `<tr><td>${escapeHtml(status)}</td><td>${escapeHtml(String(row.nodeType ?? ""))}</td><td>${escapeHtml(String(row.count ?? ""))}</td></tr>`;
}

function renderViewGenerationPayload(payload: { views: unknown[] }): string {
  const rows = payload.views.map((view) => {
    const row = isRecord(view) ? view : {};
    return `<tr><td>${escapeHtml(String(row.name ?? ""))}</td><td>${escapeHtml(String(row.resource ?? ""))}</td><td>${escapeHtml(String(row.url ?? ""))}</td></tr>`;
  }).join("");

  return `
    <section class="rendered-block">
      <h3>SQL-on-FHIR ViewDefinitions</h3>
      <p>${payload.views.length} view${payload.views.length === 1 ? "" : "s"} generated from CQL retrieves.</p>
      ${rows ? `<table><thead><tr><th>Name</th><th>Resource</th><th>Canonical URL</th></tr></thead><tbody>${rows}</tbody></table>` : ""}
    </section>
  `;
}

function renderSqlTextPayload(payload: { sql: string; views?: unknown[]; library?: unknown }): string {
  const views = Array.isArray(payload.views) ? payload.views : [];
  const title = isRecord(payload.library) ? "SQLQuery Library" : "SQL-on-FHIR SQL";
  return `
    <section class="rendered-block">
      <h3>${title}</h3>
      <p>${views.length} ViewDefinition dependenc${views.length === 1 ? "y" : "ies"}.</p>
      <pre class="explanation-pre">${escapeHtml(payload.sql)}</pre>
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

function renderTextPayload(payload: string): string {
  return `<section class="rendered-block"><h3>Explanation</h3><pre class="explanation-pre">${escapeHtml(payload)}</pre></section>`;
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

function isCqlSourceMapPayload(value: unknown): value is { library: unknown; sourceMap: unknown } {
  return isRecord(value) && isRecord(value.library) && isRecord(value.sourceMap);
}

function isValueSetComposePayload(value: unknown): value is { include?: unknown[]; exclude?: unknown[] } {
  return isRecord(value) && (Array.isArray(value.include) || Array.isArray(value.exclude));
}

function isLowerCheckPayload(value: unknown): value is { target: string; supported: boolean; supportedNodes?: unknown[]; unsupportedNodes?: unknown[]; notes?: unknown[] } {
  return isRecord(value) && typeof value.target === "string" && typeof value.supported === "boolean";
}

function isViewGenerationPayload(value: unknown): value is { views: unknown[] } {
  return isRecord(value) && Array.isArray(value.views) && !("sql" in value);
}

function isSqlTextPayload(value: unknown): value is { sql: string; views?: unknown[]; library?: unknown } {
  return isRecord(value) && typeof value.sql === "string";
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
