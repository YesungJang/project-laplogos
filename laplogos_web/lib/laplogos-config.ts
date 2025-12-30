export type LaplogosClientConfig = {
  allowed_actions: string[];
  allowed_hosts: string[];
};

const DEFAULT_ACTIONS = ["set", "inc", "fetch", "nav"];
const CONFIG_PATH = process.env.NEXT_PUBLIC_LAPLOGOS_CONFIG_PATH ?? "/laplogos-config.json";

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

export function envClientConfig(): LaplogosClientConfig {
  const envActions = parseList(process.env.NEXT_PUBLIC_LAPLOGOS_ALLOWED_ACTIONS);
  const envHosts = parseList(process.env.NEXT_PUBLIC_LAPLOGOS_ALLOWED_HOSTS);
  return {
    allowed_actions: envActions.length > 0 ? envActions : DEFAULT_ACTIONS,
    allowed_hosts: envHosts,
  };
}

export async function loadClientConfig(): Promise<LaplogosClientConfig> {
  const envConfig = envClientConfig();
  const envActionsRaw = process.env.NEXT_PUBLIC_LAPLOGOS_ALLOWED_ACTIONS;
  const envHostsRaw = process.env.NEXT_PUBLIC_LAPLOGOS_ALLOWED_HOSTS;
  const envOverrides = {
    actions: !!envActionsRaw && envActionsRaw.trim().length > 0,
    hosts: !!envHostsRaw && envHostsRaw.trim().length > 0,
  };

  try {
    const res = await fetch(CONFIG_PATH, { cache: "no-store" });
    if (!res.ok) return envConfig;
    const data = await res.json();
    const fileConfig = normalizeConfig(data, envConfig);
    return {
      allowed_actions: envOverrides.actions ? envConfig.allowed_actions : fileConfig.allowed_actions,
      allowed_hosts: envOverrides.hosts ? envConfig.allowed_hosts : fileConfig.allowed_hosts,
    };
  } catch {
    return envConfig;
  }
}
