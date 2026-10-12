import { readFile } from "fs/promises";
import { resolve } from "path";
import {
  ERROR_VIEW_CONTAINER_CLASS,
  ERROR_VIEW_DEBUG_TEXT,
  ERROR_VIEW_HINT_CLASS,
  ERROR_VIEW_MESSAGE_CLASS,
  ERROR_VIEW_RECOVERY_TEXT,
  ERROR_VIEW_TITLE,
  ERROR_VIEW_TITLE_CLASS,
  toErrorViewMessage,
} from "./laplogos-error-view";

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

/**
 * SSR counterpart of `components/ErrorView.tsx`.
 *
 * Both paths render the same alert: the message stays escaped plain text and
 * the recovery instruction is shown next to the Debug View guidance.
 */
export function renderErrorHtml(message: string) {
  const safe = escapeHtml(toErrorViewMessage(message));
  return [
    `<div role="alert" aria-live="assertive" aria-atomic="true" class="${ERROR_VIEW_CONTAINER_CLASS}">`,
    `<h3 class="${ERROR_VIEW_TITLE_CLASS}">${ERROR_VIEW_TITLE}</h3>`,
    `<p class="${ERROR_VIEW_MESSAGE_CLASS}">${safe}</p>`,
    `<p class="${ERROR_VIEW_HINT_CLASS}">${ERROR_VIEW_RECOVERY_TEXT}</p>`,
    `<p class="${ERROR_VIEW_HINT_CLASS}">${ERROR_VIEW_DEBUG_TEXT}</p>`,
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
