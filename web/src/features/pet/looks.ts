import type { Mode } from '@/features/pet/behaviour'
import type { SheetName } from '@/features/pet/sheets'

/** One step of a clip: a frame and how long it shows. */
export type Shown = { frame: number; ms: number }

/** How a mode is drawn from its sheet. */
export type Look = { sheet: SheetName } &
  /** One frame, until something else happens. */
  (
    | { kind: 'still'; frame: number }
    /** Played once; what follows is `afterClip`'s choice. */
    | { kind: 'clip'; steps: Shown[] }
    /** The frames in turn, `fps` a second, until she lands. */
    | { kind: 'loop'; fps: number }
    /** A frame on each `stride` CSS pixels the cursor moves; it holds when the cursor stops. */
    | { kind: 'drag'; stride: number }
  )

/**
 * Each mode's frames and timing. Clips that come back to her feet end on the
 * standing frame their sheet draws, so they hand over to the standing image
 * without a jump.
 */
export const looks = {
  standing: { sheet: 'standing', kind: 'still', frame: 0 },
  // The big yawn, frames 2 and 3, lasts about a second.
  yawn: {
    sheet: 'yawn',
    kind: 'clip',
    steps: [
      { frame: 0, ms: 80 },
      { frame: 1, ms: 250 },
      { frame: 2, ms: 350 },
      { frame: 3, ms: 650 },
      { frame: 4, ms: 400 },
      { frame: 5, ms: 120 },
    ],
  },
  // Frame 1 is upright with her hands on the headphones, 2 leans left, 3
  // leans right: she sways twice each way.
  sway: {
    sheet: 'sway',
    kind: 'clip',
    steps: [
      { frame: 0, ms: 80 },
      { frame: 1, ms: 300 },
      { frame: 2, ms: 350 },
      { frame: 1, ms: 300 },
      { frame: 3, ms: 350 },
      { frame: 1, ms: 300 },
      { frame: 2, ms: 350 },
      { frame: 1, ms: 300 },
      { frame: 3, ms: 350 },
      { frame: 1, ms: 250 },
      { frame: 4, ms: 300 },
      { frame: 5, ms: 120 },
    ],
  },
  sleeping: {
    sheet: 'sleep',
    kind: 'clip',
    steps: [
      { frame: 0, ms: 120 },
      { frame: 1, ms: 700 },
      { frame: 2, ms: 450 },
      { frame: 3, ms: 600 },
    ],
  },
  asleep: { sheet: 'sleep', kind: 'still', frame: 4 },
  waking: {
    sheet: 'sleep',
    kind: 'clip',
    steps: [
      { frame: 4, ms: 200 },
      { frame: 3, ms: 250 },
      { frame: 2, ms: 250 },
      { frame: 1, ms: 300 },
      { frame: 0, ms: 120 },
    ],
  },
  phoneOut: {
    sheet: 'phone',
    kind: 'clip',
    steps: [
      { frame: 0, ms: 100 },
      { frame: 1, ms: 300 },
      { frame: 2, ms: 300 },
      { frame: 3, ms: 250 },
    ],
  },
  reading: { sheet: 'phone', kind: 'still', frame: 4 },
  phoneAway: {
    sheet: 'phone',
    kind: 'clip',
    steps: [
      { frame: 4, ms: 150 },
      { frame: 3, ms: 250 },
      { frame: 2, ms: 300 },
      { frame: 1, ms: 300 },
      { frame: 0, ms: 100 },
    ],
  },
  held: { sheet: 'dragged', kind: 'drag', stride: 24 },
  falling: { sheet: 'falling', kind: 'loop', fps: 12 },
  // She sits for a moment where she landed, then gets up.
  landing: {
    sheet: 'duckSit',
    kind: 'clip',
    steps: [
      { frame: 0, ms: 450 },
      { frame: 1, ms: 150 },
      { frame: 2, ms: 150 },
      { frame: 3, ms: 150 },
      { frame: 4, ms: 150 },
      { frame: 5, ms: 120 },
    ],
  },
  // Hit on the head, she holds it, looks down at the ball, picks it up
  // (frame 3), looks at it and puts it in her pocket.
  request: {
    sheet: 'request',
    kind: 'clip',
    steps: [
      { frame: 0, ms: 250 },
      { frame: 1, ms: 550 },
      { frame: 2, ms: 500 },
      { frame: 3, ms: 400 },
      { frame: 4, ms: 650 },
      { frame: 5, ms: 450 },
      { frame: 6, ms: 120 },
    ],
  },
  // She frowns, scratches her head (frames 2 and 3, back and forth), then
  // smiles awkwardly.
  troubled: {
    sheet: 'troubled',
    kind: 'clip',
    steps: [
      { frame: 0, ms: 80 },
      { frame: 1, ms: 400 },
      { frame: 2, ms: 180 },
      { frame: 3, ms: 180 },
      { frame: 2, ms: 180 },
      { frame: 3, ms: 180 },
      { frame: 2, ms: 180 },
      { frame: 3, ms: 250 },
      { frame: 4, ms: 450 },
      { frame: 5, ms: 120 },
    ],
  },
} satisfies Record<Mode, Look>

/** The frame a mode starts on. */
export function firstFrame(mode: Mode): number {
  const look: Look = looks[mode]
  if (look.kind === 'still') {
    return look.frame
  }
  if (look.kind === 'clip') {
    return look.steps[0]?.frame ?? 0
  }
  return 0
}

/** Milliseconds from the start of a clip until it first shows `frame`. */
export function msUntil(mode: Mode, frame: number): number {
  const look: Look = looks[mode]
  if (look.kind !== 'clip') {
    return 0
  }
  const at = look.steps.findIndex((shown) => shown.frame === frame)
  return look.steps.slice(0, at === -1 ? 0 : at).reduce((sum, shown) => sum + shown.ms, 0)
}
