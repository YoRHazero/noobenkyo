import { open, save } from '@tauri-apps/plugin-dialog'

const jsonLines = [{ name: 'JSON Lines', extensions: ['jsonl'] }]

/** Asks for a .jsonl file to read; `null` if cancelled. */
export async function chooseJsonlToOpen(): Promise<string | null> {
  const path = await open({ multiple: false, directory: false, filters: jsonLines })
  return typeof path === 'string' ? path : null
}

/** Asks where to write a .jsonl file; `null` if cancelled. */
export async function chooseJsonlToSave(defaultName: string): Promise<string | null> {
  return save({ defaultPath: defaultName, filters: jsonLines })
}
