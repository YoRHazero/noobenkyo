import { describe, expect, it } from 'vitest'
import { countKana, fitsKanaLimit } from '@/features/study/kana-limit'

describe('kana limit', () => {
  it('ignores romaji that is still being typed', () => {
    expect(countKana('あs')).toBe(1)
    expect(countKana('あした')).toBe(3)
  })

  it('checks the limit', () => {
    expect(fitsKanaLimit('あした', 3)).toBe(true)
    expect(fitsKanaLimit('あしたs', 3)).toBe(true)
    expect(fitsKanaLimit('あしたに', 3)).toBe(false)
    expect(fitsKanaLimit('あしたに', null)).toBe(true)
  })
})
