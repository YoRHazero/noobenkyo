import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

export function useDictionaryStats() {
  return useQuery({
    queryKey: ['dictionary_stats'],
    queryFn: () => unwrap(commands.dictionaryStats()),
  })
}
