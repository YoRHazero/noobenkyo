import { describe, expect, it } from 'vitest'
import { afterClip, firstMode, onCue, onInbox, opensInbox } from '@/features/pet/behaviour'
import type { PetCue } from '@/stores/pet'

const ball: PetCue = { kind: 'request', from: { x: 0, y: 0 } }
const failed: PetCue = { kind: 'failed' }

describe('onCue', () => {
  it('plays over idling and the phone', () => {
    for (const mode of ['standing', 'yawn', 'sway', 'phoneOut', 'reading', 'phoneAway'] as const) {
      expect(onCue(mode, 'request')).toBe('play')
      expect(onCue(mode, 'failed')).toBe('play')
    }
  })

  it('wakes her when she sleeps', () => {
    expect(onCue('sleeping', 'failed')).toBe('wake')
    expect(onCue('asleep', 'request')).toBe('wake')
  })

  it('waits while she is moved, lands or wakes', () => {
    for (const mode of ['held', 'falling', 'landing', 'waking'] as const) {
      expect(onCue(mode, 'request')).toBe('wait')
    }
  })

  it('plays again from the start over the same cue, and waits for another', () => {
    expect(onCue('request', 'request')).toBe('play')
    expect(onCue('request', 'failed')).toBe('wait')
    expect(onCue('troubled', 'failed')).toBe('play')
    expect(onCue('troubled', 'request')).toBe('wait')
  })
})

describe('afterClip', () => {
  it('stops on the last frame of sitting down and of taking the phone out', () => {
    expect(afterClip('sleeping', [failed], false)).toEqual({ mode: 'asleep' })
    expect(afterClip('phoneOut', [], true)).toEqual({ mode: 'reading' })
  })

  it('takes the oldest waiting cue first', () => {
    expect(afterClip('landing', [failed, ball], true)).toEqual({ cue: failed })
  })

  it('goes to the phone if the inbox has something, else stands', () => {
    expect(afterClip('troubled', [], true)).toEqual({ mode: 'phoneOut' })
    expect(afterClip('phoneAway', [], false)).toEqual({ mode: 'standing' })
  })
})

describe('onInbox', () => {
  it('takes the phone out only while she stands', () => {
    expect(onInbox('standing', true)).toBe('phoneOut')
    expect(onInbox('yawn', true)).toBeNull()
    expect(onInbox('falling', true)).toBeNull()
  })

  it('puts the phone away when the inbox empties', () => {
    expect(onInbox('reading', false)).toBe('phoneAway')
    expect(onInbox('phoneOut', false)).toBe('phoneAway')
    expect(onInbox('standing', false)).toBeNull()
  })
})

describe('appearing and clicks', () => {
  it('appears already reading if the inbox has something', () => {
    expect(firstMode(true)).toBe('reading')
    expect(firstMode(false)).toBe('standing')
  })

  it('opens the inbox only while she has her phone out', () => {
    expect(opensInbox('reading')).toBe(true)
    expect(opensInbox('phoneOut')).toBe(true)
    expect(opensInbox('standing')).toBe(false)
  })
})
