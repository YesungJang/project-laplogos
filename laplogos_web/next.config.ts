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
      const handler = (ctx: unknown, req: string, cb: (err?: Error | null, result?: string) => void) => {
        if (req === "laplogos_core") {
          return cb(null, "commonjs laplogos_core");
        }
        return cb();
      };

      if (Array.isArray(externals)) {
        config.externals = [...externals, handler];
      } else if (typeof externals === "function") {
        config.externals = (
          ctx: unknown,
          req: string,
          cb: (err?: Error | null, result?: string) => void
        ) => {
          if (req === "laplogos_core") {
            return cb(null, "commonjs laplogos_core");
          }
          return externals(ctx, req, cb);
        };
      }
    }
    return config;
  },
};

export default nextConfig;
