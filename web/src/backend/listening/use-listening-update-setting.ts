import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  name: string
  setting: string
}

export function useListeningUpdateSetting() {
  return useChange(({ name, setting }: Input) =>
    unwrap(commands.listeningUpdateSetting(name, setting)),
  )
}
