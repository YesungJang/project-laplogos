import { readFile } from "fs/promises";
import { resolve } from "path";

export async function initLaplogosWasm(mod: typeof import("laplogos_core")) {
  if (typeof mod.initSync === "function") {
    const wasmPath = resolve(process.cwd(), "../laplogos_core/pkg/laplogos_core_bg.wasm");
    const wasmBytes = await readFile(wasmPath);
    mod.initSync({ module: wasmBytes });
    return;
  }
  if (typeof mod.default === "function") {
    await mod.default();
  }
}

export async function renderLaplogosHtml(page: unknown, state: unknown) {
  try {
    const mod = await import("laplogos_core");
    await initLaplogosWasm(mod);
    return mod.render_laplogos_to_string(page, state);
  } catch (err) {
    console.error("Failed to render laplogos SSR", err);
    return "";
  }
}

export function escapeJsonForScript(payload: unknown) {
  return JSON.stringify(payload).replace(/</g, "\\u003c");
}
