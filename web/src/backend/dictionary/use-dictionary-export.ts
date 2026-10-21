import { useMutation } from '@tanstack/react-query'
import { commands, type ContentFilter } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

type Input = {
  filter: ContentFilter
  limit: number | null
  keepProgress: boolean
}

/** Exporting reads only, so nothing is invalidated. */
export function useDictionaryExport() {
  return useMutation({
    mutationFn: ({ filter, limit, keepProgress }: Input) =>
      unwrap(commands.dictionaryExport(filter, limit, keepProgress)),
  })
}
