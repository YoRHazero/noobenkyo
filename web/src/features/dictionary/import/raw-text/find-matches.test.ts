import { describe, expect, it } from 'vitest'
import { findMatches } from '@/features/dictionary/import/raw-text/find-matches'

describe('findMatches', () => {
  it('finds every match, ignoring case', () => {
    expect(findMatches('"pos": 1\n"POS": 2', 'pos')).toEqual([
      { start: 1, end: 4 },
      { start: 10, end: 13 },
    ])
  })

  it('takes the query literally', () => {
    expect(findMatches('a.b axb', '.')).toEqual([{ start: 1, end: 2 }])
    expect(findMatches('[[上]]', '[[')).toEqual([{ start: 0, end: 2 }])
  })

  it('finds nothing for an empty query', () => {
    expect(findMatches('abc', '')).toEqual([])
  })
})
