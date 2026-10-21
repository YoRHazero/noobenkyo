import { useQuery } from '@tanstack/react-query'
import { commands, type SenseKey } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

export function useDictionarySenseDetail(key: SenseKey) {
  return useQuery({
    queryKey: ['dictionary_sense_detail', key],
    queryFn: () => unwrap(commands.dictionarySenseDetail(key)),
  })
}
