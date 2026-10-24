import { save, type DialogFilter } from '@tauri-apps/plugin-dialog'
import { pluginError } from '@/files/plugin-error'

/** Asks where to save a file; `null` if the user cancels. */
export function chooseSavePath(
  defaultPath: string,
  filters: DialogFilter[],
): Promise<string | null> {
  return save({ defaultPath, filters }).catch(pluginError('选择位置'))
}
