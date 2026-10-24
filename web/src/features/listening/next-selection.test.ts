import { describe, expect, it } from 'vitest'
import { nextSelection } from '@/features/listening/next-selection'

describe('nextSelection', () => {
  it('moves to the next name', () => {
    expect(nextSelection(['a', 'b', 'c'], 'b')).toBe('c')
  })

  it('moves back when the last name is removed', () => {
    expect(nextSelection(['a', 'b', 'c'], 'c')).toBe('b')
  })

  it('leaves nothing selected when nothing is left', () => {
    expect(nextSelection(['a'], 'a')).toBeNull()
  })
})
