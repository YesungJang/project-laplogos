import { useEffect, useState } from "react";

type EngineModule = typeof import("../../aix_core/pkg");

export function useAixEngine() {
  const [engine, setEngine] = useState<EngineModule | null>(null);

  useEffect(() => {
    let active = true;
    import("aix_core")
      .then((mod) => {
        if (!active) return;
        mod.init_panic_hook();
        setEngine(mod);
      })
      .catch((err) => {
        console.error("Failed to load AIX engine", err);
      });

    return () => {
      active = false;
    };
  }, []);

  return engine;
}
