'use client';

import { useEffect, useState } from "react";

type Engine = Awaited<ReturnType<typeof import("laplogos_core")>>["LaplogosEngine"];

type Props = {
  engine: Engine | null;
  refreshTrigger: number;
};

export function DebugView({ engine, refreshTrigger }: Props) {
  const [stateText, setStateText] = useState<string>("{}");

  useEffect(() => {
    if (!engine) return;
    try {
      setStateText(engine.get_state_json());
    } catch (e) {
      console.error("Failed to read state", e);
    }
  }, [engine, refreshTrigger]);

  return (
    <aside className="w-full max-w-md rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
      <h2 className="text-lg font-semibold text-slate-900">State Inspector</h2>
      <pre className="mt-2 max-h-64 overflow-auto text-sm text-slate-800">
        {stateText}
      </pre>
    </aside>
  );
}
