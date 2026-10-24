import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  world: string
  character: string
}

export function useListeningRemoveCast() {
  return useChange(({ world, character }: Input) =>
    unwrap(commands.listeningRemoveCast(world, character)),
  )
}
