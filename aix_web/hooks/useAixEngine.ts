import { useEffect, useState } from "react";

type EngineModule = typeof import("aix_core");

export function useAixEngine() {
  const [engine, setEngine] = useState<EngineModule | null>(null);

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
        setEngine(mod);
      } catch (err) {
        console.error("Failed to load AIX engine", err);
      }
    };

    load();

    return () => {
      active = false;
    };
  }, []);

  return engine;
}
