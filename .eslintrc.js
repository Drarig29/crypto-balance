module.exports = {
    env: {
        browser: true
    },
    extends: [
        'eslint:recommended',
        'plugin:react/recommended'
    ],
    parser: '@babel/eslint-parser',
    plugins: [
        'jsx-a11y'
    ],
    settings: {
        react: {
            version: 'detect'
        }
    },
    rules: {
        'react/prop-types': 0,
        quotes: ['error', 'single'],
        semi: ['error', 'always']
    }
}