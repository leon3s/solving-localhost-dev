/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  env: {
    API_URL: process.env.API_URL,
    AUTH_URL: process.env.AUTH_URL,
    CURRENT_URL: process.env.CURRENT_URL,
  },
}

export default nextConfig
