import { describe, expect, it } from 'vitest'
import {
  feetBelow,
  feetBeside,
  feetX,
  handAbove,
  nearWall,
  placeOf,
  sideTowards,
  snap,
} from '@/features/pet/geometry'
import { sheets } from '@/features/pet/sheets'

const feet = { x: 300, y: 700 }

describe('places', () => {
  it('measures from the nearer side, so she keeps to it when the window is resized', () => {
    expect(placeOf(100, 1000)).toEqual({ edge: 'left', distance: 100 })
    expect(placeOf(920, 1000)).toEqual({ edge: 'right', distance: 80 })
    expect(feetX({ edge: 'right', distance: 80 }, 1400)).toBe(1320)
    expect(feetX({ edge: 'left', distance: 100 }, 1400)).toBe(100)
  })

  it('keeps her inside a window too small for the place', () => {
    const { width, anchor } = sheets.standing
    expect(feetX({ edge: 'left', distance: 900 }, 500)).toBe(500 - width * (1 - anchor.x))
    expect(feetX({ edge: 'right', distance: 900 }, 500)).toBe(width * anchor.x)
  })
})

describe('changing sheets', () => {
  it('picks her up and sets her down on the same feet', () => {
    const back = feetBelow(handAbove(feet), 0)
    expect(back.x).toBeCloseTo(feet.x)
    expect(back.y).toBeCloseTo(feet.y)
  })

  it('sits her by a wall and stands her up on the same feet', () => {
    for (const wall of ['left', 'right'] as const) {
      expect(feetBeside(wall, sideTowards(wall, feet))).toEqual(feet)
    }
  })

  it('finds the side she stands within one body width of', () => {
    const body = sheets.standing.width
    expect(nearWall({ x: body, y: 0 }, 1000)).toBe('left')
    expect(nearWall({ x: 1000 - body, y: 0 }, 1000)).toBe('right')
    expect(nearWall({ x: 500, y: 0 }, 1000)).toBeNull()
  })
})

describe('snap', () => {
  it('rounds to whole device pixels', () => {
    expect(snap(10.3, 2)).toBe(10.5)
    expect(snap(10.3, 1)).toBe(10)
  })
})
