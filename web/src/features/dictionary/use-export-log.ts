import { commands } from '@/bindings'
import { dateStamp } from '@/components/date-stamp'
import { chooseJsonlToSave } from '@/components/file-dialog'
import { unwrap } from '@/components/result'
import { useDictionaryMutation } from '@/features/dictionary/use-dictionary-mutation'

/** Asks where to save and writes the log; resolves to the record count, `null` if cancelled. */
export function useExportLog() {
  return useDictionaryMutation(async () => {
    const path = await chooseJsonlToSave(`noobenkyo-log-${dateStamp()}.jsonl`)
    return path === null ? null : unwrap(commands.exportLog(path))
  })
}
