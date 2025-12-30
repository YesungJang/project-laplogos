import { readFile } from "fs/promises";
import { resolve } from "path";

let cachedWasm: Uint8Array | null = null;
let cachedWasmPath: string | null = null;

export async function initLaplogosWasm(mod: typeof import("laplogos_core")) {
  const wasmPath =
    process.env.LAPLOGOS_WASM_PATH ?? resolve(process.cwd(), "public/wasm/laplogos_core_bg.wasm");
  if (!cachedWasm || cachedWasmPath !== wasmPath) {
    cachedWasm = await readFile(wasmPath);
    cachedWasmPath = wasmPath;
  }
  const wasmBytes = cachedWasm;
  if (typeof mod.initSync === "function") {
    mod.initSync({ module: wasmBytes });
    return;
  }
  if (typeof mod.default === "function") {
    await mod.default(wasmBytes);
  }
}

export async function renderLaplogosHtml(page: unknown, state: unknown) {
  try {
    const mod = await import("laplogos_core");
    await initLaplogosWasm(mod);
    return mod.render_laplogos_to_string(page, state);
  } catch (err) {
    console.error("Failed to render laplogos SSR", err);
    const message = err instanceof Error ? err.message : "SSR failed";
    return renderErrorHtml(message);
  }
}

export function escapeJsonForScript(payload: unknown) {
  return JSON.stringify(payload).replace(/</g, "\\u003c");
}

export function renderErrorHtml(message: string) {
  const safe = escapeHtml(message);
  return [
    '<div class="rounded-md border border-rose-200 bg-rose-50 p-4 text-rose-900">',
    '<h3 class="text-base font-semibold">Laplogos Error</h3>',
    `<p class="mt-2 text-sm">${safe}</p>`,
    '<p class="mt-2 text-xs text-rose-700">Debug View を確認してください。</p>',
    "</div>",
  ].join("");
}

function escapeHtml(text: string) {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
