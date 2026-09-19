import { cpSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";

const [crate, wasmName] = process.argv.slice(2);

if (!crate || !wasmName) {
  console.error("usage: build-wasm-package.mjs <crate> <wasm-name>");
  process.exit(2);
}

const repoRoot = resolve(import.meta.dirname, "../..");
const packageRoot = resolve(import.meta.dirname, "..", crate);
const crateRoot = resolve(repoRoot, "crates", `rh-${crate}`);

const targets = [
  ["node", "wasm-node"],
  ["web", "wasm"],
  ["bundler", "wasm-bundler"]
];

for (const [target, outDir] of targets) {
  const result = spawnSync("just", ["wasm-build", crate, target], {
    cwd: repoRoot,
    stdio: "inherit"
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }

  const source = resolve(crateRoot, "pkg", target === "node" ? "node" : target);
  const destination = resolve(packageRoot, outDir);
  rmSync(destination, { force: true, recursive: true });
  cpSync(source, destination, { recursive: true });
  rmSync(resolve(destination, ".gitignore"), { force: true });
  // wasm-pack's nodejs target is CommonJS. This nested package marker keeps
  // Node from inheriting the parent package's `type: module` declaration.
  if (target === "node") {
    writeFileSync(resolve(destination, "package.json"), '{"type":"commonjs"}\n');
  }
}

console.log(`Built ${wasmName} WASM artifacts for ${crate}`);
