'use client';

import { useEffect } from "react";
import { useAixEngine } from "../hooks/useAixEngine";

const testJson =
  '{"v":1,"root":{"i":"root","t":"Box","p":{"pad":10},"c":[{"i":"t1","t":"Txt","p":{"val":"Hello AIX"}}]}}';

export default function Home() {
  const engine = useAixEngine();

  useEffect(() => {
    if (!engine) return;
    engine.render_aix(testJson, "aix-root");
  }, [engine]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-slate-50 p-8 text-slate-900">
      <h1 className="mb-4 text-2xl font-semibold">AIX Render Demo</h1>
      <div
        id="aix-root"
        className="w-full max-w-xl rounded-lg border border-slate-200 bg-white p-4 shadow-sm"
      />
    </main>
  );
}
