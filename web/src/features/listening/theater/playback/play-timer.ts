/** When playing stops by itself: never, after some minutes of playing, or at the episode's end. */
export type TimerChoice = 'none' | 15 | 30 | 45 | 60 | 'episode'

export const timerChoices: readonly TimerChoice[] = ['none', 15, 30, 45, 60, 'episode']

export function timerName(choice: TimerChoice): string {
  return choice === 'none' ? '不限' : choice === 'episode' ? '放完这一集' : `${choice} 分钟`
}

/**
 * Only time spent playing counts: a pause stops the clock. `since` is when
 * the current stretch of playing began, `null` while paused.
 */
export type PlayClock = {
  choice: TimerChoice
  playedMs: number
  since: number | null
}

export function startClock(choice: TimerChoice): PlayClock {
  return { choice, playedMs: 0, since: null }
}

export function played(clock: PlayClock, now: number): number {
  return clock.playedMs + (clock.since === null ? 0 : Math.max(0, now - clock.since))
}

export function resumeClock(clock: PlayClock, now: number): PlayClock {
  return clock.since === null ? { ...clock, since: now } : clock
}

export function pauseClock(clock: PlayClock, now: number): PlayClock {
  return { ...clock, playedMs: played(clock, now), since: null }
}

/** Milliseconds left on a timed choice; `null` for the others. */
export function remainingMs(clock: PlayClock, now: number): number | null {
  return typeof clock.choice === 'number'
    ? Math.max(0, clock.choice * 60_000 - played(clock, now))
    : null
}

/** Checked when a line ends, so a line is never cut off. */
export function timeIsUp(clock: PlayClock, now: number): boolean {
  return remainingMs(clock, now) === 0
}

/** 18:32 */
export function clockText(ms: number): string {
  const seconds = Math.ceil(ms / 1000)
  return `${Math.floor(seconds / 60)}:${String(seconds % 60).padStart(2, '0')}`
}
