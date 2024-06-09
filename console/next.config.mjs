/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  env: {
    API_URL: process.env.API_URL,
    AUTH_URL: process.env.AUTH_URL,
    APP_URL: process.env.APP_URL,
  },
}

export default nextConfig
