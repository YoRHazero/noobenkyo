import type { Language, LineView } from '@/bindings'
import { voiceLanguages, type VoiceModes } from '@/features/listening/theater/playback/voice-modes'

/** Lines lacking audio their voice's mode needs, by language. All zero: it can play. */
export function missingAudio(
  lines: readonly LineView[],
  modes: VoiceModes,
): Record<Language, number> {
  const missing: Record<Language, number> = { ja: 0, zh: 0 }
  for (const line of lines) {
    for (const language of voiceLanguages(modes, line.voice)) {
      if (!line.audio.includes(language)) {
        missing[language] += 1
      }
    }
  }
  return missing
}

export function canPlay(missing: Record<Language, number>): boolean {
  return missing.ja === 0 && missing.zh === 0
}
