import { commands, type SenseKey } from '@/bindings'
import { unwrap } from '@/components/result'
import { useDictionaryMutation } from '@/features/dictionary/use-dictionary-mutation'

export function useDeleteSense() {
  return useDictionaryMutation((key: SenseKey) => unwrap(commands.deleteSense(key)))
}
