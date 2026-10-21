import { commands, type SenseKey, type SensePatch } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  key: SenseKey
  patch: SensePatch
}

export function useDictionaryReplaceSense() {
  return useChange(({ key, patch }: Input) => unwrap(commands.dictionaryReplaceSense(key, patch)))
}
