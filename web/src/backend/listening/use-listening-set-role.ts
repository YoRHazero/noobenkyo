import { commands, type Role } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  world: string
  character: string
  role: Role
}

/** Changes the kind of role in place, or a main character's description. */
export function useListeningSetRole() {
  return useChange(({ world, character, role }: Input) =>
    unwrap(commands.listeningSetRole(world, character, role)),
  )
}
