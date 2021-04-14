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
        resolve: {
          extensions: ['.js', '.jsx'],
        },
        loader: 'esbuild-loader',
        options: {
          loader: 'jsx',
          target: 'es2015',
        },
      },
      {
        test: /\.css$/i,
        use: [
          'style-loader',
          'css-loader',
          {
            loader: 'esbuild-loader',
            options: {
              loader: 'css',
              minify: true
            }
          }
        ]
      }
    ],
  },
  plugins: [
    new ESBuildPlugin(),
  ],
};