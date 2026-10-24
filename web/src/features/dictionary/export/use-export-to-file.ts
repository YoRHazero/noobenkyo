import { useMutation } from '@tanstack/react-query'
import type { ContentFilter } from '@/bindings'
import { useDictionaryExport } from '@/backend/dictionary/use-dictionary-export'
import { writeTextFile } from '@/files/write-text-file'
import { chooseExportPath } from '@/features/dictionary/entry-file/choose-export-path'

type Input = {
  filter: ContentFilter
  keepProgress: boolean
}

export type Exported = {
  path: string
  entries: number
}

/**
 * Asks where to save first, so cancelling exports nothing; then exports the
 * rows and writes them there. Resolves to `null` when the user cancels.
 */
export function useExportToFile() {
  const exportEntries = useDictionaryExport()
  return useMutation({
    mutationFn: async ({ filter, keepProgress }: Input): Promise<Exported | null> => {
      const path = await chooseExportPath()
      if (path === null) {
        return null
      }
      const exported = await exportEntries.mutateAsync({ filter, limit: null, keepProgress })
      await writeTextFile(path, exported.text)
      return { path, entries: exported.entries }
    },
  })
}
