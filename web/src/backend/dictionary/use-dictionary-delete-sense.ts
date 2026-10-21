import { commands, type SenseKey } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useDictionaryDeleteSense() {
  return useChange((key: SenseKey) => unwrap(commands.dictionaryDeleteSense(key)))
}
