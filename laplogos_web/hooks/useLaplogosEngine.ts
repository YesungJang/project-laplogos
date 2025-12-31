import { useCallback, useEffect, useRef, useState } from "react";
import { usePathname } from "next/navigation";
import { normalizePagePath } from "../lib/laplogos-path";
import { envClientConfig, loadClientConfig } from "../lib/laplogos-config";

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
  config?: unknown;
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
    if (!res.ok) return fallbackPageJson(pagePath, fallbackJson);
    const data = await res.json();
    return JSON.stringify(data);
  } catch {
    return fallbackPageJson(normalizePagePath(pathname), fallbackJson);
  }
}

async function fallbackPageJson(pathname: string, fallbackJson: string) {
  if (process.env.NODE_ENV === "production") return fallbackJson;
  try {
    const mod = await import("../app/laplogos-schema");
    return JSON.stringify(mod.getPageSchema(pathname));
  } catch {
    return fallbackJson;
  }
}

function readBootstrap(): { pageJson: string; stateJson: string; config?: unknown } | null {
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
      config: parsed.config,
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
  const pathname = usePathname();
  const [currentPath, setCurrentPath] = useState(() => {
    if (typeof window === "undefined") return pathname;
    return window.location.pathname;
  });
  const initialJsonRef = useRef(initialJson);
  const fallbackJsonRef = useRef(initialJson);
  const engineRef = useRef<Engine | null>(null);
  const skipNextUpdateRef = useRef(false);

  useEffect(() => {
    fallbackJsonRef.current = initialJson;
  }, [initialJson]);

  useEffect(() => {
    engineRef.current = engine;
  }, [engine]);

  useEffect(() => {
    const handlePopState = () => {
      setCurrentPath(window.location.pathname);
    };
    window.addEventListener("popstate", handlePopState);
    return () => {
      window.removeEventListener("popstate", handlePopState);
    };
  }, []);

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

  const updatePage = useCallback(
    async (path: string) => {
      const currentEngine = engineRef.current;
      if (!currentEngine || error) return;
      try {
        const pageJson = await fetchPageJson(path, fallbackJsonRef.current);
        currentEngine.set_page_json(pageJson);
        await currentEngine.rerender(containerId);
        setRenderSource("CSR Update");
        setRefreshTrigger((v) => v + 1);
      } catch (err) {
        console.error("Failed to update page", err);
        setErrorFrom(err, "Failed to update page");
      }
    },
    [containerId, error, setErrorFrom]
  );

  const onNav = useCallback(
    (to: string) => {
      if (isExternalUrl(to)) {
        window.location.assign(to);
        return;
      }
      try {
        const nextUrl = new URL(to, window.location.origin);
        skipNextUpdateRef.current = true;
        void (async () => {
          await updatePage(nextUrl.pathname);
          window.history.pushState({}, "", nextUrl);
          setCurrentPath(nextUrl.pathname);
        })();
      } catch {
        window.location.assign(to);
      }
    },
    [updatePage]
  );

  useEffect(() => {
    if (engine) return;
    let active = true;

    const load = async () => {
      try {
        const bootstrap = readBootstrap();
        const pageJson = bootstrap?.pageJson ?? initialJsonRef.current;
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
        engineRef.current = inst;
        const bootstrapConfig =
          bootstrap?.config && typeof bootstrap.config === "object" ? bootstrap.config : null;
        const config = bootstrapConfig ?? envClientConfig();
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
        if (!bootstrapConfig) {
          loadClientConfig()
            .then((fileConfig) => {
              if (!active) return;
              inst.init(JSON.stringify(fileConfig));
            })
            .catch(() => {});
        }
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
  }, [containerId, onNav, setErrorFrom, engine]);

  useEffect(() => {
    if (!engine || error) return;
    let active = true;

    const update = async () => {
      try {
        if (skipNextUpdateRef.current) {
          skipNextUpdateRef.current = false;
          return;
        }
        await updatePage(currentPath);
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
  }, [engine, currentPath, error, setErrorFrom, updatePage]);

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
    [engine, error, setErrorFrom]
  );

  return { engine, dispatch, refreshTrigger, error, renderSource };
}
