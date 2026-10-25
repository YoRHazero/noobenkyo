import { beforeEach, describe, expect, it } from 'vitest'
import { usePet } from '@/stores/pet'

const ball = (x: number) => ({ kind: 'request' as const, from: { x, y: 0 } })

describe('pet cues', () => {
  beforeEach(() => usePet.getState().clear())

  it('queues cues of different kinds in order', () => {
    usePet.getState().cue(ball(1))
    usePet.getState().cue({ kind: 'failed' })
    expect(usePet.getState().cues.map((cue) => cue.kind)).toEqual(['request', 'failed'])
  })

  it('keeps one cue of a kind, the newest, where the first one was', () => {
    usePet.getState().cue(ball(1))
    usePet.getState().cue({ kind: 'failed' })
    usePet.getState().cue(ball(2))
    expect(usePet.getState().cues).toEqual([ball(2), { kind: 'failed' }])
  })

  it('drops the cue acted on', () => {
    const first = ball(1)
    usePet.getState().cue(first)
    usePet.getState().cue({ kind: 'failed' })
    usePet.getState().drop(first)
    expect(usePet.getState().cues).toEqual([{ kind: 'failed' }])
  })
})
