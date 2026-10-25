import type { Point } from '@/stores/pet'
import draggedJson from '@/features/pet/sprites/dragged.json'
import draggedUrl from '@/features/pet/sprites/dragged.png'
import duckSitJson from '@/features/pet/sprites/duck-sit.json'
import duckSitUrl from '@/features/pet/sprites/duck-sit.png'
import fallingJson from '@/features/pet/sprites/falling.json'
import fallingUrl from '@/features/pet/sprites/falling.png'
import paperBallJson from '@/features/pet/sprites/paper-ball.json'
import paperBallUrl from '@/features/pet/sprites/paper-ball.png'
import phoneJson from '@/features/pet/sprites/phone.json'
import phoneUrl from '@/features/pet/sprites/phone.png'
import requestJson from '@/features/pet/sprites/request.json'
import requestUrl from '@/features/pet/sprites/request.png'
import sleepJson from '@/features/pet/sprites/sleep.json'
import sleepUrl from '@/features/pet/sprites/sleep.png'
import standingJson from '@/features/pet/sprites/standing.json'
import standingUrl from '@/features/pet/sprites/standing.png'
import swayJson from '@/features/pet/sprites/sway.json'
import swayUrl from '@/features/pet/sprites/sway.png'
import troubledJson from '@/features/pet/sprites/troubled.json'
import troubledUrl from '@/features/pet/sprites/troubled.png'
import yawnJson from '@/features/pet/sprites/yawn.json'
import yawnUrl from '@/features/pet/sprites/yawn.png'

/** What scripts/pet/grid.py writes next to each strip of frames. */
type Cut = {
  frames: number
  width: number
  height: number
  /** CSS pixels per image pixel. */
  display: number
  anchor: Point
  feet?: Point[]
}

/** One strip of frames, sized in CSS pixels. */
export type Sheet = {
  url: string
  frames: number
  width: number
  height: number
  /** The point that stays put from frame to frame, as fractions of a frame. */
  anchor: Point
  /** Where her feet are in each frame, as fractions of a frame, for a sheet held by the hand. */
  feet: Point[]
}

function sheet(url: string, cut: Cut): Sheet {
  return {
    url,
    frames: cut.frames,
    width: cut.width * cut.display,
    height: cut.height * cut.display,
    anchor: cut.anchor,
    feet: cut.feet ?? [],
  }
}

/**
 * Anchored at her feet, except `dragged` (her raised hand) and `sleep` (her
 * side against the wall, drawn against a wall on her left).
 */
export const sheets = {
  standing: sheet(standingUrl, standingJson),
  dragged: sheet(draggedUrl, draggedJson),
  falling: sheet(fallingUrl, fallingJson),
  duckSit: sheet(duckSitUrl, duckSitJson),
  yawn: sheet(yawnUrl, yawnJson),
  sway: sheet(swayUrl, swayJson),
  sleep: sheet(sleepUrl, sleepJson),
  request: sheet(requestUrl, requestJson),
  phone: sheet(phoneUrl, phoneJson),
  troubled: sheet(troubledUrl, troubledJson),
}

export type SheetName = keyof typeof sheets

/** The paper ball, drawn at the scale of the request sheet. */
export const paperBall = {
  url: paperBallUrl,
  size: paperBallJson.width * paperBallJson.display,
}
