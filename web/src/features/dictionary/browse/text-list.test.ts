import { describe, expect, it } from 'vitest'
import { joinList, splitList } from '@/features/dictionary/browse/text-list'

describe('splitList', () => {
  it('splits at either semicolon and drops blanks', () => {
    expect(splitList(' 明天；明日;; ')).toEqual(['明天', '明日'])
    expect(splitList('')).toEqual([])
  })

  it('reads back what joinList writes', () => {
    expect(splitList(joinList(['放弃', '死心']))).toEqual(['放弃', '死心'])
  })
})
