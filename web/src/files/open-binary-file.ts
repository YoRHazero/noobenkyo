import type { DialogFilter } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
import { chooseOpenPath } from '@/files/choose-open-path'
import { pluginError } from '@/files/plugin-error'

/** A file's name, to show which one was chosen, and its bytes. */
export type BinaryFile = {
  name: string
  bytes: Uint8Array
}

/** Asks for a file and reads its bytes; `null` if the user cancels. */
export async function openBinaryFile(filters: DialogFilter[]): Promise<BinaryFile | null> {
  const path = await chooseOpenPath(filters)
  if (path === null) {
    return null
  }
  const bytes = await readFile(path).catch(pluginError('读取文件'))
  return { name: path.split(/[\\/]/).pop() ?? path, bytes }
}
