import { commands } from '@/bindings'
import { dateStamp } from '@/components/date-stamp'
import { chooseJsonlToSave } from '@/components/file-dialog'
import { unwrap } from '@/components/result'
import { useDictionaryMutation } from '@/features/dictionary/use-dictionary-mutation'

/** Asks where to save and exports; resolves to `null` if the dialog is cancelled. */
export function useExportEntries() {
  return useDictionaryMutation(async (keepProgress: boolean) => {
    const path = await chooseJsonlToSave(`noobenkyo-${dateStamp()}.jsonl`)
    return path === null ? null : unwrap(commands.exportEntries(path, keepProgress))
  })
}
