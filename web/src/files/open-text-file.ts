import type { DialogFilter } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
import { chooseOpenPath } from '@/files/choose-open-path'
import { decodeUtf8 } from '@/files/decode-utf8'
import { pluginError } from '@/files/plugin-error'

/** Asks for a text file and reads it as UTF-8; `null` if the user cancels. */
export async function openTextFile(filters: DialogFilter[]): Promise<string | null> {
  const path = await chooseOpenPath(filters)
  if (path === null) {
    return null
  }
  return decodeUtf8(await readFile(path).catch(pluginError('读取文件')))
}
