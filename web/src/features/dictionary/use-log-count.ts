import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/components/result'
import { dictionaryKeys } from '@/features/dictionary/dictionary-keys'

export function useLogCount() {
  return useQuery({
    queryKey: dictionaryKeys.logCount,
    queryFn: () => unwrap(commands.logCount()),
  })
}
