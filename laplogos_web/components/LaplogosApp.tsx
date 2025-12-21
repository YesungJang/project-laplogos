'use client';

import { useMemo } from "react";
import { DebugView } from "./DebugView";
import { useLaplogosEngine } from "../hooks/useLaplogosEngine";

type Props = {
  initialJson: string;
  bootstrapJson: string;
  renderedHtml: string;
};

const containerId = "laplogos-root";

export function LaplogosApp({ initialJson, bootstrapJson, renderedHtml }: Props) {
  const { engine, dispatch, refreshTrigger } = useLaplogosEngine(initialJson, containerId);

  const sendRandom = useMemo(
    () => () => {
      const val = Math.random().toString(36).slice(2, 7);
      dispatch(JSON.stringify({ t: "set", d: { key: "count", val } }));
    },
    [dispatch]
  );

  return (
    <main className="min-h-screen bg-slate-50 p-8 text-slate-900">
      <h1 className="mb-6 text-2xl font-semibold">laplogos Render Demo</h1>
      <div className="grid gap-6 lg:grid-cols-[2fr_1fr]">
        <section className="rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-lg font-semibold">App</h2>
            <button
              onClick={sendRandom}
              className="rounded-md bg-slate-900 px-3 py-2 text-sm font-medium text-white hover:bg-slate-800"
              disabled={!engine}
            >
              Dispatch random count
            </button>
          </div>
          <div
            id={containerId}
            className="min-h-[200px] rounded-md border border-slate-200 bg-slate-50 p-3"
            dangerouslySetInnerHTML={{ __html: renderedHtml }}
            suppressHydrationWarning
          />
        </section>
        <DebugView engine={engine} refreshTrigger={refreshTrigger} />
      </div>
      <script
        id="laplogos-bootstrap"
        type="application/json"
        dangerouslySetInnerHTML={{ __html: bootstrapJson }}
      />
    </main>
  );
}
