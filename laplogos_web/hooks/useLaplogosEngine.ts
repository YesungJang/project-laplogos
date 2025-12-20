import { useCallback, useEffect, useState } from "react";

type EngineModule = typeof import("laplogos_core");
type Engine = InstanceType<EngineModule["LaplogosEngine"]>;

type BootstrapPayload = {
  page?: unknown;
  state?: unknown;
};

type UseLaplogosEngineResult = {
  engine: Engine | null;
  dispatch: (actionJson: string) => Promise<void>;
  refreshTrigger: number;
};

function readBootstrap(): { pageJson: string; stateJson: string } | null {
  const el = document.getElementById("laplogos-bootstrap");
  const text = el?.textContent?.trim();
  if (!text) return null;
  try {
    const parsed = JSON.parse(text) as BootstrapPayload;
    if (!parsed || typeof parsed !== "object" || !parsed.page) return null;
    const state = parsed.state ?? {};
    return {
      pageJson: JSON.stringify(parsed.page),
      stateJson: JSON.stringify(state),
    };
  } catch {
    return null;
  }
}

export function useLaplogosEngine(initialJson: string, containerId: string): UseLaplogosEngineResult {
  const [engine, setEngine] = useState<Engine | null>(null);
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  useEffect(() => {
    let active = true;

    const load = async () => {
      try {
        const bootstrap = readBootstrap();
        const pageJson = bootstrap?.pageJson ?? initialJson;
        const stateJson = bootstrap?.stateJson ?? "{}";
        const mod = await import("laplogos_core");
        if (!active) return;
        if (typeof mod.default === "function") {
          await mod.default();
        }
        if (!active) return;
        mod.init_panic_hook();
        const inst = new mod.LaplogosEngine(pageJson);
        if (!active) return;
        inst.set_state_json(stateJson);
        if (bootstrap) {
          await inst.hydrate(containerId);
        } else {
          await inst.rerender(containerId);
        }
        setEngine(inst);
        setRefreshTrigger((v) => v + 1);
      } catch (err) {
        console.error("Failed to load laplogos engine", err);
      }
    };

    load();

    return () => {
      active = false;
    };
  }, [initialJson, containerId]);

  const dispatch = useCallback(
    async (actionJson: string) => {
      if (!engine) return;
      try {
        engine.dispatch(actionJson);
        await engine.rerender(containerId);
        setRefreshTrigger((v) => v + 1);
      } catch (err) {
        console.error("Failed to dispatch action", err);
      }
    },
    [engine, containerId]
  );

  return { engine, dispatch, refreshTrigger };
}
