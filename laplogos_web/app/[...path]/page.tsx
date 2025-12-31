import { LaplogosApp } from "../../components/LaplogosApp";
import { getPageSchema } from "../laplogos-schema";
import { normalizePagePath } from "../../lib/laplogos-path";
import { escapeJsonForScript, renderLaplogosHtml } from "../../lib/laplogos-ssr";
import { loadServerConfig } from "../../lib/laplogos-config-server";

export const runtime = "nodejs";

const initialState = {};

type Params = {
  path?: string[];
};

type Props = {
  params: Promise<Params> | Params;
};

export default async function LaplogosDynamicPage({ params }: Props) {
  const { path } = await params;
  const rawPath = path?.join("/") ?? "/";
  const pagePath = normalizePagePath(rawPath);
  const initialPage = getPageSchema(pagePath);
  const renderedHtml = await renderLaplogosHtml(initialPage, initialState);
  const config = await loadServerConfig();
  const bootstrapJson = escapeJsonForScript({ page: initialPage, state: initialState, config });
  const initialJson = JSON.stringify(initialPage);

  return (
    <LaplogosApp
      initialJson={initialJson}
      bootstrapJson={bootstrapJson}
      renderedHtml={renderedHtml}
    />
  );
}
