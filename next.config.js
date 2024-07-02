/** @type {import('next').NextConfig} */
const nextConfig = {
    output: 'export',
    images: {
        loader: 'imgix',
        path: '/',
    },
};

module.exports = nextConfig;
