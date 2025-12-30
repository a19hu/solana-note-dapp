import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  webpack: (config) => {
    config.externals.push({
      "pino-pretty": "commonjs pino-pretty",
      lokijs: "commonjs lokijs",
      encoding: "commonjs encoding",
    });
    return config;
  },
};

export default nextConfig;
