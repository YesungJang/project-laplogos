import { LaplogosApp } from "../components/LaplogosApp";
import { getPageSchema } from "./laplogos-schema";
import { normalizePagePath } from "../lib/laplogos-path";
import { escapeJsonForScript, renderLaplogosHtml } from "../lib/laplogos-ssr";

export const runtime = "nodejs";

const initialPage = getPageSchema(normalizePagePath("/"));

const initialState = {};

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
