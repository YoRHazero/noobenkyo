import { keepPreviousData, useQuery } from '@tanstack/react-query'
import { commands, type ImportOptions } from '@/bindings'

/** Keeps showing the last preview while a new one is on its way. */
export function useDictionaryPreviewEntries(text: string, options: ImportOptions) {
  return useQuery({
    queryKey: ['dictionary_preview_entries', text, options],
    queryFn: () => commands.dictionaryPreviewEntries(text, options),
    placeholderData: keepPreviousData,
  })
}
