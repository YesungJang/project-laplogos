import { LaplogosApp } from "../../components/LaplogosApp";
import { getPageSchema } from "../laplogos-schema";
import { normalizePagePath } from "../../lib/laplogos-path";
import { escapeJsonForScript, renderLaplogosHtml } from "../../lib/laplogos-ssr";

export const runtime = "nodejs";

const initialState = {};

type Props = {
  params: { path?: string[] };
};

export default async function LaplogosDynamicPage({ params }: Props) {
  const rawPath = params.path?.join("/") ?? "/";
  const pagePath = normalizePagePath(rawPath);
  const initialPage = getPageSchema(pagePath);
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
