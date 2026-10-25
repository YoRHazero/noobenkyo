import { sheets } from '@/features/pet/sheets'
import type { PetPlace, Point } from '@/stores/pet'

/** A side of the window she can sleep against. */
export type Wall = 'left' | 'right'

/** Where she stands until she is first moved: by the right side, like the inbox. */
export const firstPlace: PetPlace = { edge: 'right', distance: 80 }

/** Where her feet are, for `place` in a window `width` wide, kept so she stays inside. */
export function feetX(place: PetPlace, width: number): number {
  const { width: frame, anchor } = sheets.standing
  const x = place.edge === 'left' ? place.distance : width - place.distance
  return Math.max(frame * anchor.x, Math.min(width - frame * (1 - anchor.x), x))
}

/** The place for feet at `x` in a window `width` wide: measured from the nearer side. */
export function placeOf(x: number, width: number): PetPlace {
  return x < width / 2 ? { edge: 'left', distance: x } : { edge: 'right', distance: width - x }
}

/** Where her hand is when the first dragged frame has its feet at `feet`. */
export function handAbove(feet: Point): Point {
  const { width, height, anchor, feet: frames } = sheets.dragged
  const first = frames[0] ?? anchor
  return { x: feet.x + width * (anchor.x - first.x), y: feet.y + height * (anchor.y - first.y) }
}

/** Where her feet are in dragged frame `frame` with her hand at `hand`. */
export function feetBelow(hand: Point, frame: number): Point {
  const { width, height, anchor, feet: frames } = sheets.dragged
  const feet = frames[frame] ?? anchor
  return { x: hand.x + width * (feet.x - anchor.x), y: hand.y + height * (feet.y - anchor.y) }
}

/** Where her side towards `wall` is, standing with her feet at `feet`. */
export function sideTowards(wall: Wall, feet: Point): Point {
  const { width, anchor } = sheets.standing
  return {
    x: wall === 'left' ? feet.x - width * anchor.x : feet.x + width * (1 - anchor.x),
    y: feet.y,
  }
}

/** Where her feet are, standing with her side towards `wall` at `side`. */
export function feetBeside(wall: Wall, side: Point): Point {
  const { width, anchor } = sheets.standing
  return {
    x: wall === 'left' ? side.x + width * anchor.x : side.x - width * (1 - anchor.x),
    y: side.y,
  }
}

/** The side of a window `width` wide she stands within one body width of, if any. */
export function nearWall(feet: Point, width: number): Wall | null {
  const body = sheets.standing.width
  if (sideTowards('left', feet).x <= body) {
    return 'left'
  }
  if (width - sideTowards('right', feet).x <= body) {
    return 'right'
  }
  return null
}

/**
 * Rounds a position to whole device pixels: moved by a fraction of a pixel,
 * an image is resampled and blurs.
 */
export function snap(value: number, ratio: number) {
  return Math.round(value * ratio) / ratio
}

/** The point `t` (0 to 1) along a quadratic curve. */
export function curve(from: Point, through: Point, to: Point, t: number): Point {
  const along = (a: number, b: number, c: number) =>
    (1 - t) * (1 - t) * a + 2 * (1 - t) * t * b + t * t * c
  return { x: along(from.x, through.x, to.x), y: along(from.y, through.y, to.y) }
}
