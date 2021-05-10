// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */

const httpProxy = require('http-proxy')
const proxy = httpProxy.createServer({target: 'http://localhost:8081'})

module.exports = {
  mount: {
    public: { url: "/", static: true },
    src: { url: "/dist" },
  },
  "routes": [
    {
      src: '/api/.*',
      dest: (req, res) => proxy.web(req, res)
    },
    {
      "match": "routes", 
      "src": ".*", 
      "dest": "/index.html"
    }
  ],
  plugins: [
    "@snowpack/plugin-react-refresh",
    "@snowpack/plugin-dotenv",
    "@snowpack/plugin-typescript",
    "snowpack-plugin-relative-css-urls",
    "snowpack-plugin-svgr",
    "@snowpack/plugin-sass",
    [
      "@snowpack/plugin-run-script", {
        "cmd": "eslint src --ext .js,jsx,.ts,.tsx",
        "watch": "esw -w --clear src --ext .js,jsx,.ts,.tsx"
      }
    ]
  ],
  alias: {
    "components": "./src/components",
    "routes": "./src/routes",
    "assets": "./src/assets",
    "api": "./src/api.ts",
  }
}