import { writeTextFile } from '@tauri-apps/plugin-fs'

/** Writes an exported text to the path the user chose. */
export async function writeEntryFile(path: string, text: string): Promise<void> {
  await writeTextFile(path, text).catch((error: unknown) => {
    throw new Error(`写入文件失败：${String(error)}`)
  })
}
