import { commands, type Language, type LineId } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  line: LineId
  language: Language
  text: string
}

/**
 * Replaces one language of a line, dropping its audio in that language.
 * Resolves to the problems that rejected the text, empty otherwise.
 */
export function useListeningUpdateLine() {
  return useChange(({ line, language, text }: Input) =>
    unwrap(commands.listeningUpdateLine(line, language, text)),
  )
}
