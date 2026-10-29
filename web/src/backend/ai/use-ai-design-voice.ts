import { useMutation } from '@tanstack/react-query'
import { commands, type Language } from '@/bindings'
import { wavBytes } from '@/backend/audio-bytes'
import { unwrap } from '@/backend/unwrap'

type Input = {
  prompt: string
  language: Language
  text: string
  /** A whole number from 0 to 2³² − 1. */
  seed: number
}

/**
 * Makes up a voice saying `text` and resolves to its WAV. Nothing is stored
 * until the user keeps it, so no cached result is refetched.
 */
export function useAiDesignVoice() {
  return useMutation({
    mutationFn: ({ prompt, language, text, seed }: Input) =>
      unwrap(commands.aiDesignVoice(prompt, language, text, seed)).then(wavBytes),
  })
}
