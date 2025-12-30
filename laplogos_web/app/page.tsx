import { LaplogosApp } from "../components/LaplogosApp";
import { getPageSchema } from "./laplogos-schema";
import { normalizePagePath } from "../lib/laplogos-path";
import { escapeJsonForScript, renderLaplogosHtml } from "../lib/laplogos-ssr";

export const runtime = "nodejs";

const initialPage = getPageSchema(normalizePagePath("/"));

const initialState = {};

async function initLaplogosWasm(mod: typeof import("laplogos_core")) {
  const wasmPath =
    process.env.LAPLOGOS_WASM_PATH ??
    resolve(process.cwd(), "public/wasm/laplogos_core_bg.wasm");
  const wasmBytes = await readFile(wasmPath);
  if (typeof mod.initSync === "function") {
    mod.initSync({ module: wasmBytes });
    return;
  }
  if (typeof mod.default === "function") {
    await mod.default(wasmBytes);
  }
}

async function renderLaplogosHtml(page: unknown, state: unknown) {
  try {
    const mod = await import("laplogos_core");
    await initLaplogosWasm(mod);
    return mod.render_laplogos_to_string(page, state);
  } catch (err) {
    console.error("Failed to render laplogos SSR", err);
    return "";
  }
}

function escapeJsonForScript(payload: unknown) {
  return JSON.stringify(payload).replace(/</g, "\\u003c");
}

export default async function Home() {
  const renderedHtml = await renderLaplogosHtml(initialPage, initialState);
  const bootstrapJson = escapeJsonForScript({ page: initialPage, state: initialState });
  const initialJson = JSON.stringify(initialPage);

  return (
    <LaplogosApp
      initialJson={initialJson}
      bootstrapJson={bootstrapJson}
      renderedHtml={renderedHtml}
    />
  );
}
