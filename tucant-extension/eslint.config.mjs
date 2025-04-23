// @ts-check

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config({
    ignores: ["bootstrap.bundle.min.js"],
    extends: [
        eslint.configs.recommended,
        tseslint.configs.strict,
        tseslint.configs.stylistic
    ]
});