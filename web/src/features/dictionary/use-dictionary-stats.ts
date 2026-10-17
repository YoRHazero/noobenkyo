import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/components/result'
import { dictionaryKeys } from '@/features/dictionary/dictionary-keys'

export function useDictionaryStats() {
  return useQuery({
    queryKey: dictionaryKeys.stats,
    queryFn: () => unwrap(commands.dictionaryStats()),
  })
}
