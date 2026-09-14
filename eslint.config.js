import js from '@eslint/js'
import { defineConfig, globalIgnores } from 'eslint/config'
import prettier from 'eslint-config-prettier'
import svelte from 'eslint-plugin-svelte'
import globals from 'globals'
import ts from 'typescript-eslint'
import svelteConfig from './svelte.config.js'

export default defineConfig(
  globalIgnores(['build/', '.svelte-kit/', 'static/']),
  js.configs.recommended,
  ts.configs.recommended,
  svelte.configs.recommended,
  prettier,
  svelte.configs.prettier,
  {
    // Type-aware rules are limited to .ts because typescript-eslint resolves
    // imports through tsc, which cannot read .svelte modules.
    files: ['src/**/*.ts'],
    extends: [ts.configs.recommendedTypeChecked],
    languageOptions: {
      parserOptions: {
        project: './tsconfig.json',
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
    languageOptions: {
      parserOptions: {
        parser: ts.parser,
        extraFileExtensions: ['.svelte'],
        svelteConfig,
      },
    },
  },
  { languageOptions: { globals: globals.browser } },
  { files: ['*.js'], languageOptions: { globals: globals.node } },
)
