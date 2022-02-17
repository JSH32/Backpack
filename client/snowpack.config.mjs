/* eslint-disable @typescript-eslint/explicit-module-boundary-types */

// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
import proxy from "http2-proxy"

export default {
  mount: {
    public: { url: "/", static: true },
    src: { url: "/dist" }
  },
  devOptions: {
    hostname: "localhost",
    port: 3000
  },
  optimize: {
    // See issue: https://github.com/withastro/snowpack/issues/3403
    bundle: false,
    minify: true
  },
  routes: [
    {
      src: "/api/.*",
      dest: (req, res) => proxy.web(req, res, {
        hostname: "localhost",
        port: 3001
      })
    },
    {
      match: "routes",
      src: ".*",
      dest: "/index.html"
    }
  ],
  plugins: [
    "@snowpack/plugin-react-refresh",
    "@snowpack/plugin-dotenv",
    "@snowpack/plugin-typescript",
    "snowpack-plugin-relative-css-urls",
    "snowpack-plugin-svgr",
    "@snowpack/plugin-sass",
    "@snowpack/plugin-dotenv"
  ],
  alias: {
    "components": "./src/components",
    "routes": "./src/routes",
    "assets": "./src/assets",
    "store": "./src/store.ts",
    "api": "./src/api.ts",
    "bpkutil": "./src/bpkutil.ts"
  },
  packageOptions: {
    knownEntryPoints: [
      "@chakra-ui/hooks/use-animation-state",
      "framesync"
    ]
  },
  buildOptions: {
    /* ... */
  }
}
