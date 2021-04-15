const esbuild = require('esbuild');

const preactCompatPlugin = {
    name: "preact-compat",
    setup(build) {
        const path = require("path");
        const preact = path.join(process.cwd(), "node_modules", "preact", "compat", "dist", "compat.module.js");

        build.onResolve({ filter: /^(react-dom|react)$/ }, () => {
            return { path: preact };
        });
    }
}

const isDev = process.argv[2] === '--dev';

esbuild
    .build({
        entryPoints: ['frontend/index.jsx'],
        bundle: true,
        outfile: 'static/dist/bundle.js',
        loader: { '.js': 'jsx' },
        jsxFactory: 'h',
        jsxFragment: 'Fragment',
        watch: isDev,
        minify: !isDev,
        sourcemap: true,
        logLevel: 'info',
        plugins: [preactCompatPlugin],
    })
    .catch(() => process.exit(1));