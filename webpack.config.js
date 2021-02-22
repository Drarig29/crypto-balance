const { ESBuildPlugin } = require('esbuild-loader');
const { join } = require('path');

module.exports = {
  entry: './frontend/index.jsx',
  output: {
    path: join(__dirname, 'static/assets'),
    filename: 'bundle.js',
  },
  mode: 'production',
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        loader: 'esbuild-loader',
        options: {
          loader: 'jsx',
          target: 'es2015',
        },
      },
    ],
  },
  plugins: [
    new ESBuildPlugin(),
  ],
};