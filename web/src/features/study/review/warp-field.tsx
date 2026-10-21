import { useEffect, useRef } from 'react'
import styles from '@/features/study/review/endless-effect.module.css'

type Point = { x: number; y: number }

type Star = {
  angle: number
  /** From the origin, in pixels. */
  distance: number
  speed: number
  width: number
  hue: number
}

const starCount = 260

/**
 * Points of light streaming out from the origin ever faster, like going to
 * warp speed. It only runs while active, and not at all with reduced motion.
 */
export function WarpField({ active, origin }: { active: boolean; origin: Point }) {
  const canvas = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const element = canvas.current
    const context = element?.getContext('2d')
    if (!active || !element || !context) {
      return
    }
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
      return
    }
    const ratio = window.devicePixelRatio || 1
    const resize = () => {
      element.width = window.innerWidth * ratio
      element.height = window.innerHeight * ratio
      context.setTransform(ratio, 0, 0, ratio, 0, 0)
    }
    resize()
    window.addEventListener('resize', resize)

    const reach = Math.hypot(window.innerWidth, window.innerHeight)
    const star = (distance: number): Star => ({
      angle: Math.random() * Math.PI * 2,
      distance,
      speed: 0.6 + Math.random() * 1.8,
      width: 0.6 + Math.random() * 1.6,
      hue: 190 + Math.random() * 130,
    })
    const stars = Array.from({ length: starCount }, () => star(Math.random() * reach * 0.6))

    let boost = 0.4
    let last = performance.now()
    let frame = 0
    const draw = (now: number) => {
      // Frames as a multiple of 60 fps, so a slow frame does not jump.
      const step = Math.min(3, (now - last) / 16.7)
      last = now
      boost = Math.min(6, boost * (1 + 0.012 * step))
      context.clearRect(0, 0, window.innerWidth, window.innerHeight)
      for (const s of stars) {
        const from = s.distance
        s.distance += s.speed * boost * step * (1 + s.distance / 160)
        if (s.distance > reach) {
          Object.assign(s, star(Math.random() * 40))
          continue
        }
        const cos = Math.cos(s.angle)
        const sin = Math.sin(s.angle)
        context.strokeStyle = `hsla(${s.hue}, 90%, 78%, ${Math.min(1, s.distance / 220)})`
        context.lineWidth = s.width * (1 + s.distance / reach)
        context.beginPath()
        context.moveTo(origin.x + cos * from, origin.y + sin * from)
        context.lineTo(origin.x + cos * s.distance, origin.y + sin * s.distance)
        context.stroke()
      }
      frame = requestAnimationFrame(draw)
    }
    frame = requestAnimationFrame(draw)
    return () => {
      cancelAnimationFrame(frame)
      window.removeEventListener('resize', resize)
    }
  }, [active, origin.x, origin.y])

  return <canvas ref={canvas} className={styles.stars} aria-hidden />
}
