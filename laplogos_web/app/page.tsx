import { readFile } from "fs/promises";
import { resolve } from "path";
import { LaplogosApp } from "../components/LaplogosApp";
import { getPageSchema } from "./laplogos-schema";

export const runtime = "nodejs";

const initialPage = getPageSchema("/");

const initialState = {};

async function initLaplogosWasm(mod: typeof import("laplogos_core")) {
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

export default async function Home() {
  const renderedHtml = await renderLaplogosHtml(initialPage, initialState);
  const bootstrapJson = JSON.stringify({ page: initialPage, state: initialState }).replace(
    /</g,
    "\\u003c"
  );
  const initialJson = JSON.stringify(initialPage);

  return (
    <LaplogosApp
      initialJson={initialJson}
      bootstrapJson={bootstrapJson}
      renderedHtml={renderedHtml}
    />
  );
}
