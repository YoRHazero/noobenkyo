import { describe, expect, it } from 'vitest'
import { lineRange, lineText } from '@/features/dictionary/import/line-range'

const text = 'first\n\nthird\r\nlast'

describe('lineRange', () => {
  it('counts blank lines and leaves out a trailing CR', () => {
    expect(lineRange(text, 1)).toEqual({ start: 0, end: 5 })
    expect(lineRange(text, 2)).toEqual({ start: 6, end: 6 })
    expect(lineText(text, 3)).toBe('third')
    expect(lineText(text, 4)).toBe('last')
  })

  it('has nothing past the last line', () => {
    expect(lineRange(text, 5)).toBeNull()
    expect(lineText(text, 5)).toBe('')
  })
})
