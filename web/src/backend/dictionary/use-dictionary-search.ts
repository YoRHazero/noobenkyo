import { keepPreviousData, useQuery } from '@tanstack/react-query'
import { commands, type ContentFilter } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Keeps showing the last page while the next one is on its way. */
export function useDictionarySearch(filter: ContentFilter, offset: number, limit: number) {
  return useQuery({
    queryKey: ['dictionary_search', filter, offset, limit],
    queryFn: () => unwrap(commands.dictionarySearch(filter, offset, limit)),
    placeholderData: keepPreviousData,
  })
}
