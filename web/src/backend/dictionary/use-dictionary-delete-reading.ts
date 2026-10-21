import { commands, type ReadingKey } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useDictionaryDeleteReading() {
  return useChange((key: ReadingKey) => unwrap(commands.dictionaryDeleteReading(key)))
}
