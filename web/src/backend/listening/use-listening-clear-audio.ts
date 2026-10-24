import { commands, type Language, type LineId } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  lines: LineId[]
  language: Language
}

/** Resolves to how many recordings were dropped. */
export function useListeningClearAudio() {
  return useChange(({ lines, language }: Input) =>
    unwrap(commands.listeningClearAudio(lines, language)),
  )
}
