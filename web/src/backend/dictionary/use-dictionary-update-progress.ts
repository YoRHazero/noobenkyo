import { commands, type SenseKey } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  key: SenseKey
  stage: number
  passes: number
  lapses: number
}

export function useDictionaryUpdateProgress() {
  return useChange(({ key, stage, passes, lapses }: Input) =>
    unwrap(commands.dictionaryUpdateProgress(key, stage, passes, lapses)),
  )
}
