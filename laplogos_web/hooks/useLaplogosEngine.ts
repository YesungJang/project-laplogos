import { useCallback, useEffect, useState } from "react";
import { usePathname, useRouter } from "next/navigation";
import { normalizePagePath } from "../lib/laplogos-path";
import { loadClientConfig } from "../lib/laplogos-config";

type EngineModule = typeof import("laplogos_core");
type Engine = InstanceType<EngineModule["LaplogosEngine"]> & {
  init: (json: string) => void;
  set_on_nav: (cb: (to: string) => void) => void;
  set_page_json: (json: string) => void;
  set_state_json: (json: string) => void;
  hydrate: (containerId: string) => void;
  rerender: (containerId: string) => void;
  dispatch: (actionJson: string) => void;
};

type BootstrapPayload = {
  page?: unknown;
  state?: unknown;
};

const WASM_PATH = "/wasm/laplogos_core_bg.wasm";

type UseLaplogosEngineResult = {
  engine: Engine | null;
  dispatch: (actionJson: string) => Promise<void>;
  refreshTrigger: number;
  error: string | null;
  renderSource: string;
};

function isExternalUrl(to: string) {
  return /^https?:\/\//i.test(to);
}

function shouldForceHydrationFail() {
  if (process.env.NODE_ENV === "production") return false;
  try {
    const params = new URLSearchParams(window.location.search);
    return params.get("laplogosHydrationFail") === "1";
  } catch {
    return false;
  }
}

function forceHydrationMismatch(containerId: string) {
  const container = document.getElementById(containerId);
  const root = container?.firstElementChild;
  if (root) {
    root.remove();
  }
}

async function fetchPageJson(pathname: string, fallbackJson: string) {
  try {
    const pagePath = normalizePagePath(pathname);
    const res = await fetch(`/api/laplogos?page=${encodeURIComponent(pagePath)}`, {
      headers: { Accept: "application/json" },
    });
    if (!res.ok) return fallbackJson;
    const data = await res.json();
    return JSON.stringify(data);
  } catch {
    return fallbackJson;
  }
}

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
  const [error, setError] = useState<string | null>(null);
  const [renderSource, setRenderSource] = useState<string>("CSR");
  const router = useRouter();
  const pathname = usePathname();

  const setErrorFrom = useCallback((err: unknown, fallback: string) => {
    if (err instanceof Error) {
      setError(err.message);
      return;
    }
    if (typeof err === "string") {
      setError(err);
      return;
    }
    setError(fallback);
  }, []);

  const onNav = useCallback(
    (to: string) => {
      if (isExternalUrl(to)) {
        window.location.assign(to);
        return;
      }
      router.push(to);
    },
    [router]
  );

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
          await mod.default(WASM_PATH);
        }
        if (!active) return;
        mod.init_panic_hook();
        const inst = new mod.LaplogosEngine(pageJson);
        if (!active) return;
        const config = await loadClientConfig();
        inst.init(JSON.stringify(config));
        inst.set_on_nav(onNav);
        inst.set_state_json(stateJson);
        if (bootstrap) {
          if (shouldForceHydrationFail()) {
            forceHydrationMismatch(containerId);
          }
          await inst.hydrate(containerId);
          setRenderSource("SSR+Hydration");
        } else {
          await inst.rerender(containerId);
          setRenderSource("CSR");
        }
        setEngine(inst);
        setRefreshTrigger((v) => v + 1);
      } catch (err) {
        console.error("Failed to load laplogos engine", err);
        if (!active) return;
        setErrorFrom(err, "Failed to load laplogos engine");
      }
    };

    load();

    return () => {
      active = false;
    };
  }, [initialJson, containerId, onNav, setErrorFrom]);

  useEffect(() => {
    if (!engine || error) return;
    let active = true;

    const update = async () => {
      try {
        const pageJson = await fetchPageJson(pathname, initialJson);
        if (!active) return;
        engine.set_page_json(pageJson);
        await engine.rerender(containerId);
        setRenderSource("CSR Update");
        setRefreshTrigger((v) => v + 1);
      } catch (err) {
        console.error("Failed to update page", err);
        if (!active) return;
        setErrorFrom(err, "Failed to update page");
      }
    };

    update();

    return () => {
      active = false;
    };
  }, [engine, pathname, initialJson, containerId, error, setErrorFrom]);

  const dispatch = useCallback(
    async (actionJson: string) => {
      if (!engine || error) return;
      try {
        engine.dispatch(actionJson);
        setRefreshTrigger((v) => v + 1);
      } catch (err) {
        console.error("Failed to dispatch action", err);
        setErrorFrom(err, "Failed to dispatch action");
      }
    },
    [engine, containerId, error, setErrorFrom]
  );

  return { engine, dispatch, refreshTrigger, error, renderSource };
}
