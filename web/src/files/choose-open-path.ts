import { open, type DialogFilter } from '@tauri-apps/plugin-dialog'
import { pluginError } from '@/files/plugin-error'

/** Asks for one file to open; `null` if the user cancels. */
export function chooseOpenPath(filters: DialogFilter[]): Promise<string | null> {
  return open({ multiple: false, directory: false, filters }).catch(pluginError('选择文件'))
}
