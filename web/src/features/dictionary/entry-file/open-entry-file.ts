import { open } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
import { decodeUtf8 } from '@/features/dictionary/entry-file/decode-utf8'
import { openFilters } from '@/features/dictionary/entry-file/entry-file-filters'

/** Asks for an entry file and reads it as text; `null` if the user cancels. */
export async function openEntryFile(): Promise<string | null> {
  const path = await open({ multiple: false, directory: false, filters: openFilters }).catch(
    (error: unknown) => {
      throw new Error(`选择文件失败：${String(error)}`)
    },
  )
  if (path === null) {
    return null
  }
  const bytes = await readFile(path).catch((error: unknown) => {
    throw new Error(`读取文件失败：${String(error)}`)
  })
  return decodeUtf8(bytes)
}
