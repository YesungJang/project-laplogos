'use client';

import { memo, useMemo, useRef } from "react";
import { DebugView } from "./DebugView";
import { ErrorView } from "./ErrorView";
import { useLaplogosEngine } from "../hooks/useLaplogosEngine";

type Props = {
  initialJson: string;
  bootstrapJson: string;
  renderedHtml: string;
};

const containerId = "laplogos-root";

const LaplogosContainer = memo(
  function LaplogosContainer({ html }: { html: string }) {
    return (
      <div
        id={containerId}
        className="min-h-[200px] rounded-md border border-slate-200 bg-slate-50 p-3"
        dangerouslySetInnerHTML={{ __html: html }}
        suppressHydrationWarning
      />
    );
  },
  () => true
);

export function LaplogosApp({ initialJson, bootstrapJson, renderedHtml }: Props) {
  const renderedHtmlRef = useRef(renderedHtml);
  const bootstrapJsonRef = useRef(bootstrapJson);
  const { engine, dispatch, refreshTrigger, error, renderSource } = useLaplogosEngine(
    initialJson,
    containerId
  );

  const sendRandom = useMemo(
    () => () => {
      const val = Math.random().toString(36).slice(2, 7);
      dispatch(JSON.stringify({ t: "set", d: { key: "count", val } }));
    },
    [dispatch]
  );
  const isReady = !!engine && !error;

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
          {error ? (
            <ErrorView message={error} />
          ) : (
            <div className="relative">
              <LaplogosContainer html={renderedHtmlRef.current} />
              {!isReady && (
                <div
                  className="absolute inset-0 z-10 rounded-md bg-white/60 backdrop-blur-[1px]"
                  aria-hidden="true"
                />
              )}
            </div>
          )}
        </section>
        <DebugView
          engine={engine}
          refreshTrigger={refreshTrigger}
          renderSource={renderSource}
        />
      </div>
      <script
        id="laplogos-bootstrap"
        type="application/json"
        dangerouslySetInnerHTML={{ __html: bootstrapJsonRef.current }}
      />
    </main>
  );
}
