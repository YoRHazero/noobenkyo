import { useEffect, useEffectEvent, useRef } from 'react'
import { curve, snap } from '@/features/pet/geometry'
import { paperBall } from '@/features/pet/sheets'
import type { Point } from '@/stores/pet'

/** Where it hits her and where it lands, from her feet, in CSS pixels: measured on the frames. */
const hitAt = { x: 0, y: -97 }
const restAt = { x: -16, y: -5 }
/** Seconds in the air on the way to her, then bouncing off her head. */
const flight = 0.45
const bounce = 0.45

type Props = {
  /** Where it is thrown from: the button the prompt was copied with. */
  from: Point
  /** Where her feet are now; it aims at her head and lands beside them. */
  feet: () => Point
  /** It reaches her head; false if she cannot be hit now, and it is gone. */
  onHit: () => boolean
  /** Seconds from the hit until she picks it up, when it stops being drawn here. */
  pickedUp: number
  onDone: () => void
}

/**
 * The paper ball of a copied prompt: it is lobbed from the button to her
 * head, bounces off and lies beside her feet until she picks it up in her
 * own frames.
 */
export function PaperBall({ from, feet, onHit, pickedUp, onDone }: Props) {
  const ball = useRef<HTMLDivElement>(null)
  const aim = useEffectEvent(() => feet())
  const hit = useEffectEvent(() => onHit())
  const done = useEffectEvent(() => onDone())

  useEffect(() => {
    const start = performance.now()
    let struck = false
    let request = 0
    const tick = (now: number) => {
      const t = (now - start) / 1000
      const at = aim()
      const head = { x: at.x + hitAt.x, y: at.y + hitAt.y }
      const rest = { x: at.x + restAt.x, y: at.y + restAt.y }
      let point: Point
      if (t < flight) {
        const over = { x: (from.x + head.x) / 2, y: Math.min(from.y, head.y) - 120 }
        point = curve(from, over, head, t / flight)
      } else {
        if (!struck) {
          struck = true
          if (!hit()) {
            done()
            return
          }
        }
        const since = t - flight
        if (since >= pickedUp) {
          done()
          return
        }
        const over = { x: (head.x + rest.x) / 2 - 10, y: head.y - 40 }
        point = curve(head, over, rest, Math.min(1, since / bounce))
      }
      const element = ball.current
      if (element) {
        const ratio = window.devicePixelRatio || 1
        const half = paperBall.size / 2
        const spin = Math.min(t, flight + bounce) * 540
        element.style.transform = `translate(${snap(point.x - half, ratio)}px, ${snap(point.y - half, ratio)}px) rotate(${spin}deg)`
        element.style.visibility = 'visible'
      }
      request = requestAnimationFrame(tick)
    }
    request = requestAnimationFrame(tick)
    return () => cancelAnimationFrame(request)
  }, [from, pickedUp])

  return (
    <div
      ref={ball}
      aria-hidden
      className="pointer-events-none invisible fixed top-0 left-0 z-[61]"
      style={{
        width: paperBall.size,
        height: paperBall.size,
        backgroundImage: `url(${paperBall.url})`,
        backgroundSize: 'contain',
      }}
    />
  )
}
