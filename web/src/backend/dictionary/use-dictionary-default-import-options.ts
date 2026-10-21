import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'

export function useDictionaryDefaultImportOptions() {
  return useQuery({
    queryKey: ['dictionary_default_import_options'],
    queryFn: () => commands.dictionaryDefaultImportOptions(),
  })
}
