/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
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
  publicRuntimeConfig: {
    apiUrl: `${process.env.API_URL}/api`
  },
  serverRuntimeConfig: {
    // This url is used to fetch internal data on next server side
    internalApiUrl: `${process.env.INTERNAL_API_URL || process.env.API_URL}/api`
  }
}

module.exports = nextConfig
