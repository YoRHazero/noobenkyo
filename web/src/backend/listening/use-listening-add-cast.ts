import { commands, type Role } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  world: string
  character: string
  role: Role
}

export function useListeningAddCast() {
  return useChange(({ world, character, role }: Input) =>
    unwrap(commands.listeningAddCast(world, character, role)),
  )
}
