import { describe, expect, it } from 'vitest'
import {
  addPicked,
  removePicked,
  togglePicked,
  withoutPicks,
} from '@/features/listening/generate/transfer'

describe('moving episodes between the boxes', () => {
  it('adds the picked ones after the chosen, in the episodes order', () => {
    expect(addPicked(['a', 'b', 'c', 'd'], ['c'], new Set(['d', 'a']))).toEqual(['c', 'a', 'd'])
  })

  it('does not add one twice', () => {
    expect(addPicked(['a', 'b'], ['a'], new Set(['a', 'b']))).toEqual(['a', 'b'])
  })

  it('takes the picked ones out and keeps the order of the rest', () => {
    expect(removePicked(['c', 'a', 'd'], new Set(['a']))).toEqual(['c', 'd'])
  })

  it('picks and unpicks', () => {
    const once = togglePicked(new Set(), 'a')
    expect([...once]).toEqual(['a'])
    expect([...togglePicked(once, 'a')]).toEqual([])
  })

  it('forgets the picks that moved', () => {
    expect([...withoutPicks(new Set(['a', 'b']), ['a'])]).toEqual(['b'])
  })
})
