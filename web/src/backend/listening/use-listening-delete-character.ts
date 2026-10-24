import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useListeningDeleteCharacter() {
  return useChange((name: string) => unwrap(commands.listeningDeleteCharacter(name)))
}
