/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    swcMinify: true,
    env: {
        NEXT_PUBLIC_API_URL: process.env.API_URL
    },
    eslint: {
        dirs: ['src']
    },
    typescript: {
        ignoreBuildErrors: false
    }
}

module.exports = nextConfig
