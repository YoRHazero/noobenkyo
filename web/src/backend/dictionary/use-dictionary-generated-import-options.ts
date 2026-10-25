import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'

/** The options entries an agent delivers are opened with. */
export function useDictionaryGeneratedImportOptions() {
  return useQuery({
    queryKey: ['dictionary_generated_import_options'],
    queryFn: () => commands.dictionaryGeneratedImportOptions(),
  })
}
