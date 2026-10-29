import js from '@eslint/js'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import globals from 'globals'
import tseslint from 'typescript-eslint'

const features = ['dictionary', 'study', 'settings', 'listening', 'models']

/** Imports outside the current directory go through `@/`, so paths can be checked. */
const noParentImports = { regex: '^\\.\\./', message: '跨目录引用请用 @/ 路径' }

/** Commands are called only through backend; elsewhere bindings gives types only. */
const bindingsTypesOnly = {
  regex: '^@/bindings$',
  allowTypeImports: true,
  message: '命令只能通过 backend 调用，这里只能引用 bindings 里的类型',
}

/**
 * The one direction of dependency: app → features → backend → bindings;
 * app and features may use components, files and stores; components see
 * bindings' types only; files and stores use no other layer.
 */
const layers = [
  { files: ['src/**/*.{ts,tsx}'], patterns: [] },
  { files: ['src/app/**/*.{ts,tsx}'], patterns: [bindingsTypesOnly] },
  ...features.map((feature) => ({
    files: [`src/features/${feature}/**/*.{ts,tsx}`],
    patterns: [
      bindingsTypesOnly,
      {
        regex: `^@/features/(?!${feature}/)`,
        message: 'features 之间互不引用，共用的界面部件放进 components，共用的前端状态放进 stores',
      },
      { regex: '^@/app/', message: 'features 不能引用 app' },
    ],
  })),
  {
    files: ['src/backend/**/*.{ts,tsx}'],
    patterns: [
      {
        regex: '^@/(app|features|components|files|stores)/',
        message:
          'backend 只包装 bindings 里的命令，不能引用 app、features、components、files、stores',
      },
    ],
  },
  {
    files: ['src/components/**/*.{ts,tsx}'],
    patterns: [
      bindingsTypesOnly,
      {
        regex: '^@/(app|features|backend|files|stores)/',
        message:
          'components 不含业务概念，数据从参数传进来，不能引用 app、features、backend、files、stores',
      },
    ],
  },
  {
    files: ['src/files/**/*.{ts,tsx}'],
    patterns: [
      {
        regex: '^@/(app|features|components|backend|bindings|stores)(/|$)',
        message: 'files 只包装 Tauri 的 dialog、fs 插件，不引用其他任何一层',
      },
    ],
  },
  {
    files: ['src/stores/**/*.{ts,tsx}'],
    patterns: [
      {
        regex: '^@/(app|features|components|backend|files|bindings)(/|$)',
        message: 'stores 只放跨 feature 的纯前端状态，不引用其他任何一层',
      },
    ],
  },
].map(({ files, patterns }) => ({
  files,
  rules: {
    '@typescript-eslint/no-restricted-imports': [
      'error',
      { patterns: [noParentImports, ...patterns] },
    ],
  },
}))

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
  ...layers,
)
