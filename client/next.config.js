/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination: `${process.env.API_URL || "http://0.0.0.0:3001/api"}/:path*` // Proxy to Backend
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
    API_URL: process.env.API_URL || "http://0.0.0.0:3001/api" // 0.0.0.0:3001 is the default
  }
}

module.exports = nextConfig
