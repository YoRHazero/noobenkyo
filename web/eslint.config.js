import js from '@eslint/js'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import globals from 'globals'
import tseslint from 'typescript-eslint'

const features = ['dictionary', 'study', 'settings']

/** Imports outside the current directory go through `@/`, so paths can be checked. */
const noParentImports = { regex: '^\\.\\./', message: '跨目录引用请用 @/ 路径' }

/** Path rules: features never import each other or the app shell; components hold no business concepts. */
const pathRules = [
  {
    files: ['src/**/*.{ts,tsx}'],
    rules: { 'no-restricted-imports': ['error', { patterns: [noParentImports] }] },
  },
  ...features.map((feature) => ({
    files: [`src/features/${feature}/**/*.{ts,tsx}`],
    rules: {
      'no-restricted-imports': [
        'error',
        {
          patterns: [
            noParentImports,
            {
              regex: `^@/features/(?!${feature}/)`,
              message: 'features 之间互不引用，共用的放进 components',
            },
            { regex: '^@/app/', message: 'features 不能引用 app' },
          ],
        },
      ],
    },
  })),
  {
    files: ['src/components/**/*.{ts,tsx}'],
    rules: {
      'no-restricted-imports': [
        'error',
        {
          patterns: [
            noParentImports,
            {
              regex: '^@/(features|app)/',
              message: 'components 不含业务概念，不能引用 features 或 app',
            },
          ],
        },
      ],
    },
  },
]

export default tseslint.config(
  { ignores: ['dist', 'src/bindings.ts'] },
  {
    extends: [js.configs.recommended, ...tseslint.configs.strict],
    files: ['**/*.{ts,tsx}'],
    languageOptions: { ecmaVersion: 2022, globals: globals.browser },
    plugins: { 'react-hooks': reactHooks, 'react-refresh': reactRefresh },
    rules: {
      ...reactHooks.configs.recommended.rules,
      'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
    },
  },
  {
    files: ['src/components/ui/**/*.tsx'],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
  ...pathRules,
)
