const esbuild = require('esbuild');
const isDev = process.argv[2] === '--dev';

esbuild
    .build({
        entryPoints: ['frontend/index.jsx'],
        bundle: true,
        outfile: 'static/dist/bundle.js',
        loader: { '.js': 'jsx' },
        watch: isDev,
        minify: !isDev,
        sourcemap: true,
        logLevel: 'info',
    })
    .catch(() => process.exit(1));