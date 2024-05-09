/** @type {import('next').NextConfig} */
const nextConfig = {
  swcMinify: true,
  env: {
    API_URL: process.env.API_URL,
    DEFAULT_REDIRECT: process.env.DEFAULT_REDIRECT,
  },
}

export default nextConfig
