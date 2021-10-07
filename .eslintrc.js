module.exports = {
    env: {
        browser: true
    },
    extends: [
        'eslint:recommended',
        'plugin:react/recommended'
    ],
    parser: 'babel-eslint',
    plugins: [
        'jsx-a11y'
    ],
    rules: {
        quotes: ['error', 'single'],
        semi: ['error', 'always']
    }
  }