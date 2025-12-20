import { useCallback, useEffect, useState } from "react";

type EngineModule = typeof import("aix_core");
type Engine = InstanceType<EngineModule["AixEngine"]>;

type UseAixEngineResult = {
  engine: Engine | null;
  dispatch: (actionJson: string) => Promise<void>;
  refreshTrigger: number;
};

export function useAixEngine(initialJson: string, containerId: string): UseAixEngineResult {
  const [engine, setEngine] = useState<Engine | null>(null);
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  useEffect(() => {
    let active = true;

    const load = async () => {
      try {
        const mod = await import("aix_core");
        if (!active) return;
        if (typeof mod.default === "function") {
          await mod.default();
        }
        if (!active) return;
        mod.init_panic_hook();
        const inst = new mod.AixEngine(initialJson);
        if (!active) return;
        await inst.rerender(containerId);
        setEngine(inst);
        setRefreshTrigger((v) => v + 1);
      } catch (err) {
        console.error("Failed to load AIX engine", err);
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
