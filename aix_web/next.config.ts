import type { NextConfig } from "next";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const aixCorePkgPath = path.join(__dirname, "../aix_core/pkg");

const nextConfig: NextConfig = {
  turbopack: {},
  experimental: {
    externalDir: true,
    turbo: {
      resolveAlias: {
        aix_core: aixCorePkgPath,
      },
    },
  },
  transpilePackages: ["aix_core"],
  webpack: (config) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };
    return config;
  },
};

export default nextConfig;
