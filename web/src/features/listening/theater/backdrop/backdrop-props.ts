import type { LineView } from '@/bindings'

/** What the theater knows; a backdrop may use any of it, or none. */
export type BackdropProps = {
  world: string
  /** The uuid of the episode on stage. */
  episode: string | null
  /** The line being played; `null` while the title card is up. */
  line: LineView | null
  /** Whether sound is coming out. */
  playing: boolean
}
