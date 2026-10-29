import { fileURLToPath, URL } from 'node:url'
import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import { defineConfig } from 'vitest/config'

// Tauri expects a fixed port and handles the screen itself.
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) },
  },
  // no-chan ships TypeScript source that uses import.meta.glob, so Vite
  // compiles it with the app instead of pre-bundling it.
  optimizeDeps: { exclude: ['no-chan'] },
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: { ignored: ['**/crates/**'] },
  },
  test: {
    environment: 'node',
  },
})
