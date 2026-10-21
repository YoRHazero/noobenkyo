import { describe, expect, it } from 'vitest'
import { countKana, fitsKanaLimit } from '@/features/study/quiz/kana-limit'

describe('kana limit', () => {
  it('does not count romaji still being typed', () => {
    expect(countKana('あs')).toBe(1)
    expect(countKana('あした')).toBe(3)
  })

  it('keeps a place for pending romaji', () => {
    expect(fitsKanaLimit('あした', 3)).toBe(true)
    expect(fitsKanaLimit('あしs', 3)).toBe(true)
    expect(fitsKanaLimit('あしたs', 3)).toBe(false)
    expect(fitsKanaLimit('あしたに', 3)).toBe(false)
    expect(fitsKanaLimit('あしたに', null)).toBe(true)
  })
})
