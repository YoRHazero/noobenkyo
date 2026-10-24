import { writeTextFile as write } from '@tauri-apps/plugin-fs'
import { pluginError } from '@/files/plugin-error'

/** Writes a text to the path the user chose. */
export async function writeTextFile(path: string, text: string): Promise<void> {
  await write(path, text).catch(pluginError('写入文件'))
}
