// 0.0.0.0:3001 is the default 
const BACKEND_URL = process.env.BACKEND_URL || "http://0.0.0.0:3001"

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination: `${BACKEND_URL}/api/:path*` // Proxy to Backend
      },
      // Proxy the files from backpack API. Yes I know this will screw up 404 pages on non nested invalid URL's. Cry more, i'll fix later
      {
        source: "/:path",
        destination: `${process.env.BACKEND_URL || "http://0.0.0.0:3001"}/:path` // Proxy to Backend
      }
    ]
  },
  async redirects() {
    return [
      {
        source: "/user/settings",
        destination: "/user/settings/profile",
        permanent: true
      }
    ]
  },
  webpack(config) {
    config.module.rules.push({
      test: /\.svg$/,
      use: ["@svgr/webpack"]
    })

    return config
  },
  env: {
    API_URL: `${BACKEND_URL}/api`
  }
}

module.exports = nextConfig
