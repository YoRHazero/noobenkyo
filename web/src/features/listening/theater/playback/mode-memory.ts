import { playModes, type PlayMode } from '@/features/listening/theater/playback/play-mode'
import type { VoiceModes } from '@/features/listening/theater/playback/voice-modes'

/**
 * The modes chosen in each world, kept in this browser only: the backend
 * never sees them. Storage may be unavailable or hold anything, so reading
 * keeps only valid modes and failures are ignored.
 */
const key = (world: string) => `noobenkyo.listening.modes.${world}`

const isMode = (value: unknown): value is PlayMode => playModes.includes(value as PlayMode)

export function rememberedModes(world: string): Partial<VoiceModes> {
  try {
    const stored: unknown = JSON.parse(localStorage.getItem(key(world)) ?? '{}')
    if (typeof stored !== 'object' || stored === null) {
      return {}
    }
    const { cast, other } = stored as { cast?: unknown; other?: unknown }
    const modes: Record<string, PlayMode> = {}
    if (typeof cast === 'object' && cast !== null) {
      for (const [voice, mode] of Object.entries(cast)) {
        if (isMode(mode)) {
          modes[voice] = mode
        }
      }
    }
    return isMode(other) ? { cast: modes, other } : { cast: modes }
  } catch {
    return {}
  }
}

export function rememberModes(world: string, modes: VoiceModes) {
  try {
    localStorage.setItem(key(world), JSON.stringify(modes))
  } catch {
    // Not remembering only means choosing again next time.
  }
}
