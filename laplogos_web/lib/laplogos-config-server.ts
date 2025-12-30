import { readFile } from "fs/promises";
import { resolve } from "path";

export type LaplogosClientConfig = {
  allowed_actions: string[];
  allowed_hosts: string[];
};

const DEFAULT_ACTIONS = ["set", "inc", "fetch", "nav"];
const CONFIG_PATH =
  process.env.LAPLOGOS_CONFIG_PATH ?? resolve(process.cwd(), "public/laplogos-config.json");

let cachedConfig: LaplogosClientConfig | null = null;
let cachedConfigPath: string | null = null;

function parseList(value: string | undefined) {
  if (!value) return [];
  return value
    .split(",")
    .map((item) => item.trim())
    .filter((item) => item.length > 0)
    .map((item) => item.toLowerCase());
}

function normalizeConfig(
  input: unknown,
  fallback: LaplogosClientConfig
): LaplogosClientConfig {
  if (!input || typeof input !== "object") return fallback;
  const record = input as Record<string, unknown>;
  const allowed_actions = Array.isArray(record.allowed_actions)
    ? record.allowed_actions.filter((v) => typeof v === "string").map((v) => v.toLowerCase())
    : fallback.allowed_actions;
  const allowed_hosts = Array.isArray(record.allowed_hosts)
    ? record.allowed_hosts.filter((v) => typeof v === "string").map((v) => v.toLowerCase())
    : fallback.allowed_hosts;
  return { allowed_actions, allowed_hosts };
}

export async function loadServerConfig(): Promise<LaplogosClientConfig> {
  const envActions = parseList(process.env.NEXT_PUBLIC_LAPLOGOS_ALLOWED_ACTIONS);
  const envHosts = parseList(process.env.NEXT_PUBLIC_LAPLOGOS_ALLOWED_HOSTS);
  const envConfig: LaplogosClientConfig = {
    allowed_actions: envActions.length > 0 ? envActions : DEFAULT_ACTIONS,
    allowed_hosts: envHosts,
  };
  const envOverrides = {
    actions: envActions.length > 0,
    hosts: envHosts.length > 0,
  };

  if (cachedConfig && cachedConfigPath === CONFIG_PATH) {
    return cachedConfig;
  }

  try {
    const raw = await readFile(CONFIG_PATH, "utf8");
    const data = JSON.parse(raw);
    const fileConfig = normalizeConfig(data, envConfig);
    cachedConfig = {
      allowed_actions: envOverrides.actions ? envConfig.allowed_actions : fileConfig.allowed_actions,
      allowed_hosts: envOverrides.hosts ? envConfig.allowed_hosts : fileConfig.allowed_hosts,
    };
    cachedConfigPath = CONFIG_PATH;
    return cachedConfig;
  } catch {
    cachedConfig = envConfig;
    cachedConfigPath = CONFIG_PATH;
    return cachedConfig;
  }
}
