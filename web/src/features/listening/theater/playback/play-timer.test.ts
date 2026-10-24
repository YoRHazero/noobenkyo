import { describe, expect, it } from 'vitest'
import {
  clockText,
  pauseClock,
  played,
  remainingMs,
  resumeClock,
  startClock,
  timeIsUp,
} from '@/features/listening/theater/playback/play-timer'

describe('play timer', () => {
  it('counts only the time spent playing', () => {
    let clock = resumeClock(startClock(15), 1_000)
    clock = pauseClock(clock, 61_000)
    // A ten-minute pause.
    clock = resumeClock(clock, 661_000)
    expect(played(clock, 721_000)).toBe(120_000)
    expect(remainingMs(clock, 721_000)).toBe(780_000)
  })

  it('is up once the minutes are played', () => {
    const clock = resumeClock(startClock(15), 0)
    expect(timeIsUp(clock, 899_999)).toBe(false)
    expect(timeIsUp(clock, 900_000)).toBe(true)
  })

  it('never runs out without minutes', () => {
    const clock = resumeClock(startClock('episode'), 0)
    expect(remainingMs(clock, 10_000_000)).toBeNull()
    expect(timeIsUp(clock, 10_000_000)).toBe(false)
  })

  it('shows minutes and seconds left', () => {
    expect(clockText(1_112_000)).toBe('18:32')
    expect(clockText(500)).toBe('0:01')
  })
})
