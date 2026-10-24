import { chooseSavePath } from '@/files/choose-save-path'
import { dateStamp } from '@/features/dictionary/entry-file/date-stamp'
import { saveFilters } from '@/features/dictionary/entry-file/entry-file-filters'

/** Asks where to save an export; `null` if the user cancels. */
export function chooseExportPath(): Promise<string | null> {
  return chooseSavePath(`noobenkyo-${dateStamp()}.jsonl`, saveFilters)
}
