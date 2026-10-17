// Runs the Tauri CLI for the app in crates/core, where tauri.conf.json lives.
import { fileURLToPath } from 'node:url'
import { run } from '@tauri-apps/cli'

process.env.TAURI_APP_PATH ??= fileURLToPath(new URL('../crates/core', import.meta.url))
process.env.TAURI_FRONTEND_PATH ??= fileURLToPath(new URL('.', import.meta.url))
await run(process.argv.slice(2), 'tauri')
