import { commands, type ImportOptions } from '@/bindings'
import { chooseJsonlToOpen } from '@/components/file-dialog'
import { unwrap } from '@/components/result'
import { useDictionaryMutation } from '@/features/dictionary/use-dictionary-mutation'

/** Asks for a file and imports it; resolves to `null` if the dialog is cancelled. */
export function useImportEntries() {
  return useDictionaryMutation(async (options: ImportOptions) => {
    const path = await chooseJsonlToOpen()
    return path === null ? null : unwrap(commands.importEntries(path, options))
  })
}
