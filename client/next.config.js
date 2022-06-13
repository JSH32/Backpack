// require('dotenv').config()
// console.log("hio")
// console.log(process.env.API_URL)

// Use proxied URL in development
const API_URL = process.env.NODE_ENV === "production"
  ? process.env.API_URL
  : `http://localhost:${process.env.PORT}`

const rewrites = async () => {
  const rewrites = [{
    source: "/user/settings",
    destination: "/user/settings/profile"
  }]

  // Proxy to Backend in development
  if (process.env.NODE_ENV !== "production")
    rewrites.push({
      source: "/api/:path*",
      destination: `${process.env.API_URL}/api/:path*`
    })

  return rewrites
}

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  rewrites,
  webpack(config) {
    config.module.rules.push({
      test: /\.svg$/,
      use: ["@svgr/webpack"]
    })

    return config
  },
  publicRuntimeConfig: {
    apiUrl: `${API_URL}/api`
  },
  serverRuntimeConfig: {
    // This url is used to fetch internal data on next server side
    internalApiUrl: `${process.env.INTERNAL_API_URL || API_URL}/api`
  }
}

module.exports = nextConfig
