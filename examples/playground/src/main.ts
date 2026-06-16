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
    renderResult("fhirpath-output", parseFhirPath(expressionValue("fhirpath-expression")));
  });
  requiredElement<HTMLButtonElement>("fhirpath-validate").addEventListener("click", () => {
    renderResult("fhirpath-output", validateFhirPath(expressionValue("fhirpath-expression")));
  });
}

function evaluateFhirPathForm(): void {
  try {
    const resource = parseJson(textValue("fhirpath-resource"));
    renderResult(
      "fhirpath-output",
      evaluateFhirPath(expressionValue("fhirpath-expression"), resource)
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
      })
    );
  });
  requiredElement<HTMLButtonElement>("vcl-explain").addEventListener("click", () => {
    renderResult("vcl-output", explainVcl(textValue("vcl-expression"), optionalValue("vcl-default-system")));
  });
  requiredElement<HTMLButtonElement>("vcl-validate").addEventListener("click", () => {
    renderResult("vcl-output", validateVcl(textValue("vcl-expression")));
  });
}

function bindCql(): void {
  requiredElement<HTMLFormElement>("cql-form").addEventListener("submit", (event) => {
    event.preventDefault();
    renderResult(
      "cql-output",
      compileCql(textValue("cql-source"), {
        sourceMap: requiredElement<HTMLInputElement>("cql-source-map").checked
      })
    );
  });
}

function renderVersions(): void {
  setText("fhirpath-version", fhirPathVersion());
  setText("vcl-version", vclVersion());
  setText("cql-version", cqlVersion());
}

function renderResult(targetId: string, value: unknown): void {
  const target = requiredElement<HTMLOutputElement>(targetId);
  target.classList.remove("error");
  target.innerHTML = `<pre>${escapeHtml(formatJson(value))}</pre>`;
}

function renderError(targetId: string, error: unknown): void {
  const target = requiredElement<HTMLOutputElement>(targetId);
  target.classList.add("error");
  target.innerHTML = `<pre>${escapeHtml(error instanceof Error ? error.message : String(error))}</pre>`;
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
