import { save } from '@tauri-apps/plugin-dialog'
import { dateStamp } from '@/features/dictionary/entry-file/date-stamp'
import { saveFilters } from '@/features/dictionary/entry-file/entry-file-filters'

/** Asks where to save an export; `null` if the user cancels. */
export function chooseExportPath(): Promise<string | null> {
  return save({ defaultPath: `noobenkyo-${dateStamp()}.jsonl`, filters: saveFilters })
}
