import { describe, expect, it } from 'vitest'
import { cutLines } from '@/features/dictionary/import/cut/cut-lines'

describe('cutLines', () => {
  it('takes the lines out in order and leaves the rest ending in a line break', () => {
    expect(cutLines('a\r\nb\nc\nd', [3, 1])).toEqual({ cut: ['a', 'c'], rest: 'b\nd\n' })
  })

  it('counts blank lines as the dictionary does', () => {
    expect(cutLines('a\n\nb\n', [3])).toEqual({ cut: ['b'], rest: 'a\n\n' })
  })

  it('leaves nothing when every line is cut', () => {
    expect(cutLines('a\nb', [1, 2])).toEqual({ cut: ['a', 'b'], rest: '' })
  })
})
