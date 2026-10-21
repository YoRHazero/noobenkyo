import { commands, type WordKey } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useDictionaryDeleteWord() {
  return useChange((key: WordKey) => unwrap(commands.dictionaryDeleteWord(key)))
}
