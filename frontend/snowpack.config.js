// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    public: { url: "/", static: true },
    src: { url: "/dist" },
  },
  plugins: [
    "@snowpack/plugin-react-refresh",
    "@snowpack/plugin-dotenv",
    "@snowpack/plugin-typescript",
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
    "assets": "./src/assets"
  }
}