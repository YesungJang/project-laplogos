'use client';

import { useMemo } from "react";
import { DebugView } from "../components/DebugView";
import { useAixEngine } from "../hooks/useAixEngine";

const containerId = "aix-root";

const initialJson = JSON.stringify({
  v: 1,
  root: {
    i: "root",
    t: "Box",
    p: { pad: 10 },
    c: [
      { i: "t1", t: "Txt", p: { val: "Hello AIX" } },
      {
        i: "btn1",
        t: "Btn",
        p: { label: "Set random count" },
        on: { click: ["Set", "count", "demo"] },
      },
    ],
  },
});

export default function Home() {
  const { engine, dispatch, refreshTrigger } = useAixEngine(initialJson, containerId);

  const sendRandom = useMemo(
    () => () => {
      const val = Math.random().toString(36).slice(2, 7);
      dispatch(JSON.stringify({ type: "Set", key: "count", val }));
    },
    [dispatch]
  );

  return (
    <main className="min-h-screen bg-slate-50 p-8 text-slate-900">
      <h1 className="mb-6 text-2xl font-semibold">AIX Render Demo</h1>
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
          />
        </section>
        <DebugView engine={engine} refreshTrigger={refreshTrigger} />
      </div>
    </main>
  );
}
