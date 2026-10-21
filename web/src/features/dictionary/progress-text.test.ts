import { describe, expect, it } from 'vitest'
import { progressText } from '@/features/dictionary/progress-text'
import { senseState } from '@/features/dictionary/sense-state'

describe('senseState', () => {
  it('follows Design.md', () => {
    expect(senseState(0, 0, 7)).toBe('untouched')
    expect(senseState(0, 1, 7)).toBe('learning')
    expect(senseState(3, 2, 7)).toBe('learning')
    expect(senseState(7, 2, 7)).toBe('mastered')
  })
})

describe('progressText', () => {
  it('names the state with the stage out of N, then the counts', () => {
    expect(progressText({ stage: 3, passes: 2, lapses: 1, modified_at: null }, 7)).toBe(
      '学习中（3/7）；正确：2；错误：1',
    )
  })
})
