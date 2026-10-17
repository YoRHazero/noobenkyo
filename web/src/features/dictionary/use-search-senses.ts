import { keepPreviousData, useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/components/result'
import { dictionaryKeys } from '@/features/dictionary/dictionary-keys'

export const sensesPerPage = 20

export function useSearchSenses(query: string, page: number) {
  return useQuery({
    queryKey: dictionaryKeys.senses(query, page),
    queryFn: () => unwrap(commands.searchSenses(query, page * sensesPerPage, sensesPerPage)),
    placeholderData: keepPreviousData,
  })
}
