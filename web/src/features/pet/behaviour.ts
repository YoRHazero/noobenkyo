import type { PetCue } from '@/stores/pet'

/** What she is doing. Each mode is drawn from one sheet. */
export type Mode =
  | 'standing'
  | 'yawn'
  | 'sway'
  /** Sitting down against a side of the window. */
  | 'sleeping'
  | 'asleep'
  | 'waking'
  /** Taking her phone out. */
  | 'phoneOut'
  | 'reading'
  | 'phoneAway'
  | 'held'
  | 'falling'
  /** Sitting where she landed, then getting up. */
  | 'landing'
  | 'request'
  | 'troubled'

/** The clip each cue plays. */
export const cueMode = { request: 'request', failed: 'troubled' } as const satisfies Record<
  PetCue['kind'],
  Mode
>

/**
 * What a cue does when it comes while she is in `mode`: plays at once (over
 * idling and her phone, or from the start if it is already playing), wakes
 * her and waits, or waits until she stands again (after a fall, waking or
 * another cue).
 */
export function onCue(mode: Mode, kind: PetCue['kind']): 'play' | 'wake' | 'wait' {
  switch (mode) {
    case 'standing':
    case 'yawn':
    case 'sway':
    case 'phoneOut':
    case 'reading':
    case 'phoneAway':
      return 'play'
    case 'sleeping':
    case 'asleep':
      return 'wake'
    case 'request':
    case 'troubled':
      return mode === cueMode[kind] ? 'play' : 'wait'
    case 'held':
    case 'falling':
    case 'landing':
    case 'waking':
      return 'wait'
  }
}

/**
 * What follows a clip that has played to its end: sitting down and taking
 * the phone out stop on their last frame; anything else comes back to her
 * feet, where a waiting cue goes first, then her phone if the inbox has
 * something.
 */
export function afterClip(
  ended: Mode,
  cues: PetCue[],
  inbox: boolean,
): { mode: Mode } | { cue: PetCue } {
  if (ended === 'sleeping') {
    return { mode: 'asleep' }
  }
  if (ended === 'phoneOut') {
    return { mode: 'reading' }
  }
  const cue = cues[0]
  if (cue) {
    return { cue }
  }
  return { mode: inbox ? 'phoneOut' : 'standing' }
}

/** What the inbox filling up or emptying does while she is in `mode`; `null` for nothing. */
export function onInbox(mode: Mode, inbox: boolean): Mode | null {
  if (inbox && mode === 'standing') {
    return 'phoneOut'
  }
  if (!inbox && (mode === 'phoneOut' || mode === 'reading')) {
    return 'phoneAway'
  }
  return null
}

/** How she is when she appears: already reading her phone if the inbox has something. */
export function firstMode(inbox: boolean): Mode {
  return inbox ? 'reading' : 'standing'
}

/** Whether a click on her opens the inbox: while she has her phone out. */
export function opensInbox(mode: Mode) {
  return mode === 'phoneOut' || mode === 'reading'
}
