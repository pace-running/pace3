/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    swcMinify: true,
    eslint: {
        dirs: ['src']
    },
    typescript: {
        ignoreBuildErrors: false
    }
}

module.exports = nextConfig
