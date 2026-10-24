/** What the theater does once one recording ends. */
export type AfterItem =
  /** The line's next language. */
  | { kind: 'step'; step: number }
  /** The line is over: on to the next one. */
  | { kind: 'line'; lineIndex: number }
  /** The line is over and playing stops here. */
  | { kind: 'pause'; reason: 'auto-off' | 'timer' }
  /** The episode's last line is over. */
  | { kind: 'episode-end' }

type Where = {
  /** How many languages the current line plays in. */
  steps: number
  step: number
  lineIndex: number
  lines: number
  auto: boolean
  /** The timer ran out while this line played. */
  timerUp: boolean
}

/**
 * A line always plays to its end. Then the timer, then 自动, decide whether
 * to go on; the end of the episode is for the caller to handle.
 */
export function afterItem({ steps, step, lineIndex, lines, auto, timerUp }: Where): AfterItem {
  if (step + 1 < steps) {
    return { kind: 'step', step: step + 1 }
  }
  if (timerUp) {
    return { kind: 'pause', reason: 'timer' }
  }
  if (!auto) {
    return { kind: 'pause', reason: 'auto-off' }
  }
  if (lineIndex + 1 < lines) {
    return { kind: 'line', lineIndex: lineIndex + 1 }
  }
  return { kind: 'episode-end' }
}
