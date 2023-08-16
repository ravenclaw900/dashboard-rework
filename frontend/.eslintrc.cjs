module.exports = {
    env: {
        node: true,
        browser: true,
    },
    extends: [
        "eslint:recommended",
        "airbnb-base",
        "airbnb-typescript/base",
        "prettier",
        "plugin:jsx-a11y/recommended",
        "plugin:@typescript-eslint/recommended-type-checked",
        "plugin:@typescript-eslint/stylistic-type-checked",
        "plugin:solid/typescript",
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        project: true,
        tsconfigRootDir: __dirname,
    },
    plugins: ["@typescript-eslint", "solid", "jsx-a11y"],
    root: true,
    // "import/no-extraneous-dependencies": ["error", { devDependencies: true }],
    // Ignore non source files
    ignorePatterns: ["dist", "uno.config.ts", "vite.config.ts", ".eslintrc.cjs"],

    // Rule overrides
    rules: {
        // Never import files with extensions
        "import/extensions": ["error", "never"],
        // Allow variable shadowing
        "@typescript-eslint/no-shadow": "off",
    },
};
