'use client';

import { useEffect, useState } from "react";

type EngineModule = typeof import("laplogos_core");

type Engine = InstanceType<EngineModule["LaplogosEngine"]>;

type Props = {
  engine: Engine | null;
  refreshTrigger: number;
  renderSource: string;
};

const RUNTIME_VERSION = process.env.NEXT_PUBLIC_LAPLOGOS_VERSION ?? "dev";

function prettyJson(text: string) {
  try {
    return JSON.stringify(JSON.parse(text), null, 2);
  } catch {
    return text;
  }
}

function parseJson(text: string) {
  try {
    return JSON.parse(text);
  } catch {
    return text;
  }
}

function parseJsonArray(text: string) {
  try {
    const parsed = JSON.parse(text);
    return Array.isArray(parsed) ? parsed : [parsed];
  } catch {
    return [text];
  }
}

export function DebugView({ engine, refreshTrigger, renderSource }: Props) {
  const [stateText, setStateText] = useState<string>("{}");
  const [actionLogText, setActionLogText] = useState<string>("");
  const [errorLogText, setErrorLogText] = useState<string>("");
  const [pageText, setPageText] = useState<string>("{}");
  const [metricsText, setMetricsText] = useState<string>("{}");
  const [copyStatus, setCopyStatus] = useState<string>("");

  useEffect(() => {
    if (!engine) return;
    try {
      setStateText(engine.get_state_json());
      setActionLogText(engine.get_action_log_json());
      setErrorLogText(engine.get_error_log_json());
      setPageText(engine.get_page_json());
      setMetricsText(engine.get_metrics_json());
    } catch (e) {
      console.error("Failed to read state", e);
    }
  }, [engine, refreshTrigger]);

  const errorItems = parseJsonArray(errorLogText);
  const hasContainerIdMissing = errorItems.some(
    (item) => typeof item === "string" && item.includes("container id is not set")
  );

  const buildReproPackage = () => {
    if (!engine) return "{}";
    const inputPage = parseJson(engine.get_input_page_json());
    const normalizedPage = parseJson(engine.get_page_json());
    const state = parseJson(engine.get_state_json());
    const actionLog = parseJsonArray(engine.get_action_log_json());
    const errorLog = parseJsonArray(engine.get_error_log_json());
    const schemaVersion =
      typeof normalizedPage === "object" && normalizedPage && "v" in normalizedPage
        ? (normalizedPage as { v?: number }).v ?? 1
        : 1;

    const payload = {
      schema_v: schemaVersion,
      runtime_version: RUNTIME_VERSION,
      page_identifier: window.location.pathname,
      user_agent: navigator.userAgent,
      render_source: renderSource,
      input_page: inputPage,
      normalized_page: normalizedPage,
      state,
      action_log: actionLog,
      error_log: errorLog,
      metrics: parseJson(engine.get_metrics_json()),
    };

    return JSON.stringify(payload, null, 2);
  };

  const handleCopy = async () => {
    if (!engine) return;
    const payload = buildReproPackage();
    try {
      await navigator.clipboard.writeText(payload);
      setCopyStatus("Copied");
    } catch {
      setCopyStatus("Copy failed");
    }
  };

  const handleDownload = () => {
    if (!engine) return;
    const payload = buildReproPackage();
    const blob = new Blob([payload], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "laplogos-repro.json";
    link.click();
    URL.revokeObjectURL(url);
  };

  return (
    <aside className="w-full max-w-md rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-slate-900">Debug View</h2>
        <span className="text-xs text-slate-500">{renderSource}</span>
      </div>
      <div className="mt-3 flex flex-wrap gap-2 text-xs text-slate-700">
        <button
          className="rounded border border-slate-200 px-2 py-1 hover:bg-slate-50"
          onClick={handleCopy}
          disabled={!engine}
        >
          Copy repro
        </button>
        <button
          className="rounded border border-slate-200 px-2 py-1 hover:bg-slate-50"
          onClick={handleDownload}
          disabled={!engine}
        >
          Download
        </button>
        {copyStatus ? <span className="text-slate-500">{copyStatus}</span> : null}
      </div>
      <div className="mt-4 space-y-4">
        {hasContainerIdMissing ? (
          <div className="rounded border border-amber-200 bg-amber-50 p-2 text-xs text-amber-800">
            container id が未設定です。埋め込み側で container_id を設定してください。
          </div>
        ) : null}
        <section>
          <h3 className="text-sm font-semibold text-slate-900">State</h3>
          <pre className="mt-2 max-h-40 overflow-auto text-xs text-slate-800">
            {prettyJson(stateText)}
          </pre>
        </section>
        <section>
          <h3 className="text-sm font-semibold text-slate-900">Schema Tree</h3>
          <pre className="mt-2 max-h-40 overflow-auto text-xs text-slate-800">
            {prettyJson(pageText)}
          </pre>
        </section>
        <section>
          <h3 className="text-sm font-semibold text-slate-900">Action Log</h3>
          <pre className="mt-2 max-h-32 overflow-auto text-xs text-slate-800">
            {parseJsonArray(actionLogText).join("\n")}
          </pre>
        </section>
        <section>
          <h3 className="text-sm font-semibold text-slate-900">Error Log</h3>
          <pre className="mt-2 max-h-32 overflow-auto text-xs text-slate-800">
            {errorItems.join("\n")}
          </pre>
        </section>
        <section>
          <h3 className="text-sm font-semibold text-slate-900">Perf</h3>
          <pre className="mt-2 max-h-24 overflow-auto text-xs text-slate-800">
            {prettyJson(metricsText)}
          </pre>
        </section>
      </div>
    </aside>
  );
}
