import { describe, expect, it } from 'vitest'
import { parsePositiveInteger } from '@/features/dictionary/agent/positive-integer'

describe('parsePositiveInteger', () => {
  it('takes whole numbers from 1, around blanks', () => {
    expect(parsePositiveInteger('20')).toBe(20)
    expect(parsePositiveInteger(' 3 ')).toBe(3)
    expect(parsePositiveInteger('007')).toBe(7)
  })

  it('refuses zero, signs, fractions, words and empty text', () => {
    for (const text of ['0', '-1', '+1', '1.5', '1e3', '三', '']) {
      expect(parsePositiveInteger(text)).toBeNull()
    }
  })

  it('refuses what a 32-bit count cannot hold', () => {
    expect(parsePositiveInteger('4294967295')).toBe(4294967295)
    expect(parsePositiveInteger('4294967296')).toBeNull()
  })
})
