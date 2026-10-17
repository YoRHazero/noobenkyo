import { commands } from '@/bindings'
import { chooseJsonlToOpen } from '@/components/file-dialog'
import { unwrap } from '@/components/result'
import { useDictionaryMutation } from '@/features/dictionary/use-dictionary-mutation'

/** Asks for a log file and adds its records; resolves to `null` if cancelled. */
export function useImportLog() {
  return useDictionaryMutation(async () => {
    const path = await chooseJsonlToOpen()
    return path === null ? null : unwrap(commands.importLog(path))
  })
}
