import { describe, expect, it } from 'vitest'
import { parseNumberList } from '@/features/settings/parse-number-list'

describe('parseNumberList', () => {
  it('accepts commas and spaces', () => {
    expect(parseNumberList('1, 2，4  7')).toEqual([1, 2, 4, 7])
    expect(parseNumberList('')).toEqual([])
  })

  it('rejects non-integers', () => {
    expect(parseNumberList('1, 2.5')).toBeNull()
    expect(parseNumberList('1, x')).toBeNull()
    expect(parseNumberList('-1')).toBeNull()
  })
})
