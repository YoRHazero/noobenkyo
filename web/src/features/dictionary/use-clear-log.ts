import { commands } from '@/bindings'
import { unwrap } from '@/components/result'
import { useDictionaryMutation } from '@/features/dictionary/use-dictionary-mutation'

export function useClearLog() {
  return useDictionaryMutation(() => unwrap(commands.clearLog()))
}
