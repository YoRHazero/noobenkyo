import type { EpisodeDetail, Language } from '@/bindings'
import type { PlayClock } from '@/features/listening/theater/playback/play-timer'

/** Everything the theater shows; the player hands out a new one on every change. */
export type PlayerSnapshot = {
  /** The episode on stage; `null` until the first one has loaded. */
  episode: EpisodeDetail | null
  lineIndex: number
  /** The languages the current line plays in, fixed when the line started. */
  steps: Language[]
  step: number
  /** Sound is coming out, or about to once loaded. */
  playing: boolean
  auto: boolean
  /** The episode's title card is up. */
  card: boolean
  /** The webview would not start sound without a click. */
  blocked: boolean
  /** Why playing stopped by itself, when it did. */
  notice: string | null
  clock: PlayClock
}
