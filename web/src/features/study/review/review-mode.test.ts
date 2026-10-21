import { describe, expect, it } from 'vitest'
import type { SessionOptions } from '@/bindings'
import { sessionOptions } from '@/features/study/review/review-mode'

const settings: SessionOptions = { size: 30, start_steps: 1, order: 'Urgent' }

describe('sessionOptions', () => {
  it('takes the settings for a group, and drops the size for the endless mode', () => {
    expect(sessionOptions(settings, 'group')).toEqual(settings)
    expect(sessionOptions(settings, 'endless')).toEqual({ ...settings, size: null })
  })
})
