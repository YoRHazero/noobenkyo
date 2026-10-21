import { useQuery } from '@tanstack/react-query'
import { commands, type ImportOptions } from '@/bindings'

export function useDictionaryCheckEntries(text: string, options: ImportOptions) {
  return useQuery({
    queryKey: ['dictionary_check_entries', text, options],
    queryFn: () => commands.dictionaryCheckEntries(text, options),
  })
}
