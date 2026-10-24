import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  name: string
  setting: string
}

export function useListeningCreateWorld() {
  return useChange(({ name, setting }: Input) =>
    unwrap(commands.listeningCreateWorld(name, setting)),
  )
}
