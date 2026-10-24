import type { CastMember, Language } from '@/bindings'
import {
  modeLanguages,
  startMode,
  type PlayMode,
} from '@/features/listening/theater/playback/play-mode'

/**
 * The mode of each voice in the cast, and the one for every other voice: a
 * character taken out of the world or deleted, whose references are unknown.
 */
export type VoiceModes = {
  cast: Record<string, PlayMode>
  other: PlayMode
}

/** Modes for the cast: the remembered ones, the start mode for the rest. */
export function castModes(
  cast: readonly CastMember[],
  remembered: Partial<VoiceModes>,
): VoiceModes {
  const modes: Record<string, PlayMode> = {}
  for (const member of cast) {
    modes[member.character] = remembered.cast?.[member.character] ?? startMode
  }
  return { cast: modes, other: remembered.other ?? startMode }
}

/** The languages a line plays in, by its voice's mode. */
export function voiceLanguages(modes: VoiceModes, voice: string): Language[] {
  return modeLanguages(modes.cast[voice] ?? modes.other)
}
