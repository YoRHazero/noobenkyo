import { describe, expect, it } from 'vitest'
import { afterItem } from '@/features/listening/theater/playback/after-item'

const base = { steps: 2, step: 1, lineIndex: 3, lines: 8, auto: true, timerUp: false }

describe('afterItem', () => {
  it('finishes the line before anything else', () => {
    expect(afterItem({ ...base, step: 0, timerUp: true, auto: false })).toEqual({
      kind: 'step',
      step: 1,
    })
  })

  it('stops when the timer ran out, then when 自动 is off', () => {
    expect(afterItem({ ...base, timerUp: true })).toEqual({ kind: 'pause', reason: 'timer' })
    expect(afterItem({ ...base, auto: false })).toEqual({ kind: 'pause', reason: 'auto-off' })
  })

  it('goes on to the next line, then to the end of the episode', () => {
    expect(afterItem(base)).toEqual({ kind: 'line', lineIndex: 4 })
    expect(afterItem({ ...base, lineIndex: 7 })).toEqual({ kind: 'episode-end' })
  })
})
