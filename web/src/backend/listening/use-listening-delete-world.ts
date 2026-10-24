import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

/** Deletes the world with its episodes and their audio. */
export function useListeningDeleteWorld() {
  return useChange((name: string) => unwrap(commands.listeningDeleteWorld(name)))
}
