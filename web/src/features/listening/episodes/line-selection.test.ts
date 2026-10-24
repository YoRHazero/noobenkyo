import { describe, expect, it } from 'vitest'
import { clickLine, noSelection } from '@/features/listening/episodes/line-selection'

const order = [11, 12, 13, 14, 15]

describe('clickLine', () => {
  it('ticks and unticks one line', () => {
    const ticked = clickLine(noSelection, order, 12, false)
    expect([...ticked.ids]).toEqual([12])
    expect([...clickLine(ticked, order, 12, false).ids]).toEqual([])
  })

  it('ticks a range with Shift, either way', () => {
    const ticked = clickLine(noSelection, order, 13, false)
    expect([...clickLine(ticked, order, 15, true).ids].sort()).toEqual([13, 14, 15])
    expect([...clickLine(ticked, order, 11, true).ids].sort()).toEqual([11, 12, 13])
  })

  it('treats Shift without an anchor as a plain click', () => {
    expect([...clickLine(noSelection, order, 14, true).ids]).toEqual([14])
  })
})
