import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  world: string
  text: string
}

/** Stores the script as a new episode of the world; resolves to its uuid. */
export function useListeningImportScript() {
  return useChange(({ world, text }: Input) => unwrap(commands.listeningImportScript(world, text)))
}
