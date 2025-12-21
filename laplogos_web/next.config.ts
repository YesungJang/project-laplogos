import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  experimental: {
    externalDir: true,
  },
  transpilePackages: ["laplogos_core"],
  webpack: (config, { isServer }) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: false,
    };
    if (config.module?.rules) {
      config.module.rules.push({
        test: /\.wasm$/,
        type: "asset/resource",
      });
    }
    if (isServer) {
      const externals = config.externals ?? [];
      type ExternalsData = { context?: string; request?: string };
      type ExternalsCallback = (err?: Error | null, result?: string) => void;
      type ExternalsFunction = (data: ExternalsData, callback: ExternalsCallback) => void;

      const handler: ExternalsFunction = ({ request }, cb) => {
        if (request === "laplogos_core") {
          return cb(null, "commonjs laplogos_core");
        }
        return cb();
      };

      if (Array.isArray(externals)) {
        config.externals = [...externals, handler];
      } else if (typeof externals === "function") {
        const externalFn = externals as ExternalsFunction;
        config.externals = (data: ExternalsData, cb: ExternalsCallback) => {
          if (data.request === "laplogos_core") {
            return cb(null, "commonjs laplogos_core");
          }
          return externalFn(data, cb);
        };
      }
    }
    return config;
  },
};

export default nextConfig;
