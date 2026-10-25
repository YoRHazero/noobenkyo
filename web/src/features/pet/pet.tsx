import { useCallback, useEffect, useEffectEvent, useRef, useState } from 'react'
import { useInboxList } from '@/backend/ai/use-inbox-list'
import {
  afterClip,
  firstMode,
  onCue,
  onInbox,
  opensInbox,
  type Mode,
} from '@/features/pet/behaviour'
import {
  feetBelow,
  feetBeside,
  feetX,
  firstPlace,
  handAbove,
  nearWall,
  placeOf,
  sideTowards,
  snap,
  type Wall,
} from '@/features/pet/geometry'
import { firstFrame, looks, msUntil, type Look } from '@/features/pet/looks'
import { PaperBall } from '@/features/pet/paper-ball'
import { paperBall, sheets } from '@/features/pet/sheets'
import { usePet, type PetCue, type Point } from '@/stores/pet'

/** Standing, she yawns or sways after a random wait in this range, in seconds. */
const idle = { min: 20, max: 60 }
const idleModes = ['yawn', 'sway'] as const
/** Seconds without the mouse or keyboard before she sleeps by a side of the window. */
const sleepAfter = 180
/** How far, in CSS pixels, the mouse moves to wake her. */
const wakeDistance = 30
/** How far, in CSS pixels, a press on her moves before it is a drag, not a click. */
const clickSlop = 4
/** Falling speed gained each second, in CSS pixels a second. */
const gravity = 2600
/** About how long, in seconds, her hand takes to catch up with the cursor when picked up. */
const catchUp = 0.08
/** About how long, in seconds, she takes to shuffle into the corner as she sits down. */
const shuffle = 0.4
/** The request frame where she picks the ball up, and it stops being drawn on its own. */
const pickUpFrame = 3

function againstWall(mode: Mode) {
  return mode === 'sleeping' || mode === 'asleep' || mode === 'waking'
}

function savePlace(feet: Point) {
  usePet.getState().setPlace(placeOf(feet.x, window.innerWidth))
}

/** Keeps the page's text from being selected while she is dragged across it. */
function lockSelection(locked: boolean) {
  if (locked) {
    window.getSelection()?.removeAllRanges()
  }
  document.documentElement.classList.toggle('select-none', locked)
}

type Props = {
  /** Opens the inbox list, for a click on her while she reads her phone. */
  onOpenInbox: () => void
}

/**
 * The pet at the bottom of the window. She plays one mode at a time
 * (behaviour.ts decides what follows what, looks.ts how each is drawn):
 * standing she now and then yawns or sways, and sleeps by a side of the
 * window when the app is left alone; she reads her phone while the inbox has
 * something; cues from the store throw her a paper ball or trouble her; she
 * can be dragged by her raised hand, and falls back down when let go.
 *
 * Her position is a ref moved on every animation frame, not state: the
 * anchor of the current sheet (her hand when held, her side against the wall
 * by a wall, else her feet). On the floor it comes from the stored place.
 */
export function Pet({ onOpenInbox }: Props) {
  const inbox = useInboxList()
  const waiting = (inbox.data?.length ?? 0) > 0
  const [mode, setMode] = useState<Mode>(() => firstMode(waiting))
  const [frame, setFrame] = useState(() => firstFrame(firstMode(waiting)))
  // Counts every start of a mode, so a clip started again plays from the top.
  const [run, setRun] = useState(0)
  const [seenWaiting, setSeenWaiting] = useState(waiting)
  // The paper ball in flight or on the floor, and where it was thrown from.
  const [thrown, setThrown] = useState<{ id: number; from: Point } | null>(null)
  const pet = useRef<HTMLDivElement>(null)
  const body = useRef<Point>({
    x: feetX(usePet.getState().place ?? firstPlace, window.innerWidth),
    y: window.innerHeight,
  })
  const wall = useRef<Wall>('left')
  // Held: where the cursor is, and where her hand is from it (it eases to 0
  // so she does not jump); how far the cursor has gone since the last frame.
  const pointer = useRef<Point>({ x: 0, y: 0 })
  const grab = useRef<Point>({ x: 0, y: 0 })
  const travel = useRef(0)
  // When the mouse or keyboard was last used.
  const lastInput = useRef(0)
  // Where she was pressed while reading her phone, until it is a click or a drag.
  const press = useRef<Point | null>(null)
  const throws = useRef(0)

  const go = useCallback((next: Mode) => {
    setFrame(firstFrame(next))
    setMode(next)
    setRun((old) => old + 1)
  }, [])

  if (waiting !== seenWaiting) {
    setSeenWaiting(waiting)
    const next = onInbox(mode, waiting)
    if (next) {
      go(next)
    }
  }

  /** Plays a cue now. A paper ball is thrown, and she goes on until it hits. */
  const act = useCallback(
    (cue: PetCue) => {
      usePet.getState().drop(cue)
      if (cue.kind === 'failed') {
        go('troubled')
        return
      }
      throws.current += 1
      setThrown({ id: throws.current, from: cue.from })
    },
    [go],
  )

  const onCues = useEffectEvent(() => {
    const cue = usePet.getState().cues.find((cue) => onCue(mode, cue.kind) !== 'wait')
    if (!cue) {
      return
    }
    if (onCue(mode, cue.kind) === 'wake') {
      go('waking')
      return
    }
    act(cue)
  })

  const finish = useEffectEvent((ended: Mode) => {
    if (ended === 'waking') {
      body.current = feetBeside(wall.current, body.current)
      savePlace(body.current)
    }
    const after = afterClip(ended, usePet.getState().cues, waiting)
    if ('mode' in after) {
      go(after.mode)
      return
    }
    // She stands while a ball flies to her.
    if (after.cue.kind === 'request') {
      go('standing')
    }
    act(after.cue)
  })

  useEffect(() => {
    // Cues left from before she was last on screen are stale.
    usePet.getState().clear()
    lastInput.current = performance.now()
    for (const { url } of [...Object.values(sheets), paperBall]) {
      const image = new Image()
      image.src = url
    }
    return () => {
      lockSelection(false)
      usePet.getState().clear()
    }
  }, [])

  useEffect(
    () =>
      usePet.subscribe((state, before) => {
        if (state.cues.some((cue) => !before.cues.includes(cue))) {
          onCues()
        }
      }),
    [],
  )

  useEffect(() => {
    let moved = 0
    // More input can come before she is redrawn awake: wake her once.
    let woken = false
    const wake = () => {
      if (!woken) {
        woken = true
        go('waking')
      }
    }
    const onMove = (event: PointerEvent) => {
      lastInput.current = performance.now()
      if (mode === 'asleep') {
        moved += Math.hypot(event.movementX, event.movementY)
        if (moved >= wakeDistance) {
          wake()
        }
      }
    }
    const onKey = () => {
      lastInput.current = performance.now()
      if (mode === 'asleep') {
        wake()
      }
    }
    window.addEventListener('pointermove', onMove)
    window.addEventListener('keydown', onKey)
    window.addEventListener('wheel', onKey)
    return () => {
      window.removeEventListener('pointermove', onMove)
      window.removeEventListener('keydown', onKey)
      window.removeEventListener('wheel', onKey)
    }
  }, [mode, go])

  useEffect(() => {
    const look: Look = looks[mode]
    if (look.kind !== 'loop') {
      return
    }
    const frames = sheets[look.sheet].frames
    const timer = setInterval(() => setFrame((old) => (old + 1) % frames), 1000 / look.fps)
    return () => clearInterval(timer)
  }, [mode])

  useEffect(() => {
    const look: Look = looks[mode]
    if (look.kind !== 'clip') {
      return
    }
    let step = 0
    const next = () => {
      step += 1
      const shown = look.steps[step]
      if (!shown) {
        finish(mode)
        return
      }
      setFrame(shown.frame)
      timer = window.setTimeout(next, shown.ms)
    }
    let timer = window.setTimeout(next, look.steps[0]?.ms ?? 0)
    return () => clearTimeout(timer)
  }, [mode, run])

  useEffect(() => {
    if (mode !== 'standing') {
      return
    }
    const wait = idle.min + Math.random() * (idle.max - idle.min)
    const timer = setTimeout(
      () => go(idleModes[Math.floor(Math.random() * idleModes.length)] ?? 'yawn'),
      wait * 1000,
    )
    const check = setInterval(() => {
      if (performance.now() - lastInput.current < sleepAfter * 1000) {
        return
      }
      const near = nearWall(body.current, window.innerWidth)
      if (near) {
        wall.current = near
        body.current = sideTowards(near, body.current)
        go('sleeping')
      }
    }, 1000)
    return () => {
      clearTimeout(timer)
      clearInterval(check)
    }
  }, [mode, run, go])

  useEffect(() => {
    const sheet = sheets[looks[mode].sheet]
    let last = performance.now()
    let speed = 0
    let request = 0
    const tick = (now: number) => {
      const dt = Math.min(0.05, (now - last) / 1000)
      last = now
      const state = body.current
      if (mode === 'held') {
        const ease = Math.exp(-dt / catchUp)
        grab.current.x *= ease
        grab.current.y *= ease
        state.x = pointer.current.x + grab.current.x
        state.y = pointer.current.y + grab.current.y
      } else if (mode === 'falling') {
        speed += gravity * dt
        state.y += speed * dt
        if (state.y >= window.innerHeight) {
          state.y = window.innerHeight
          savePlace(state)
          go('landing')
          return
        }
      } else if (againstWall(mode)) {
        const target = wall.current === 'left' ? 0 : window.innerWidth
        state.x += (target - state.x) * (1 - Math.exp(-dt / shuffle))
        state.y = window.innerHeight
      } else {
        state.x = feetX(usePet.getState().place ?? firstPlace, window.innerWidth)
        state.y = window.innerHeight
      }
      const element = pet.current
      if (element) {
        // Against the right side, the sleep sheet (drawn against a wall on her left) is mirrored.
        const mirrored = againstWall(mode) && wall.current === 'right'
        const anchorX = mirrored ? 1 - sheet.anchor.x : sheet.anchor.x
        const ratio = window.devicePixelRatio || 1
        const left = snap(state.x - sheet.width * anchorX, ratio)
        const top = snap(state.y - sheet.height * sheet.anchor.y, ratio)
        element.style.transform = `translate(${left}px, ${top}px)${mirrored ? ' scaleX(-1)' : ''}`
        element.style.visibility = 'visible'
      }
      request = requestAnimationFrame(tick)
    }
    request = requestAnimationFrame(tick)
    return () => cancelAnimationFrame(request)
  }, [mode, go])

  /** Picks her up by the hand, with the cursor at (`x`, `y`). */
  const pickUp = (x: number, y: number) => {
    const feet = againstWall(mode) ? feetBeside(wall.current, body.current) : body.current
    const hand = mode === 'held' ? body.current : handAbove(feet)
    body.current = hand
    grab.current = { x: hand.x - x, y: hand.y - y }
    pointer.current = { x, y }
    travel.current = 0
    go('held')
  }

  const onPointerDown = (event: React.PointerEvent) => {
    event.currentTarget.setPointerCapture(event.pointerId)
    lockSelection(true)
    if (opensInbox(mode)) {
      press.current = { x: event.clientX, y: event.clientY }
      return
    }
    pickUp(event.clientX, event.clientY)
  }

  const onPointerMove = (event: React.PointerEvent) => {
    const pressed = press.current
    if (pressed) {
      if (Math.hypot(event.clientX - pressed.x, event.clientY - pressed.y) > clickSlop) {
        press.current = null
        pickUp(pressed.x, pressed.y)
      }
      return
    }
    if (mode !== 'held') {
      return
    }
    const dx = event.clientX - pointer.current.x
    const dy = event.clientY - pointer.current.y
    pointer.current = { x: event.clientX, y: event.clientY }
    // The sway moves on with the cursor and holds when it stops.
    travel.current += Math.hypot(dx, dy)
    const { stride } = looks.held
    if (travel.current >= stride) {
      const steps = Math.floor(travel.current / stride)
      travel.current -= steps * stride
      setFrame((old) => (old + steps) % sheets.dragged.frames)
    }
  }

  const onPointerUp = () => {
    lockSelection(false)
    if (press.current) {
      press.current = null
      onOpenInbox()
      return
    }
    if (mode === 'held') {
      body.current = feetBelow(body.current, frame)
      go('falling')
    }
  }

  /** The ball reaches her head: if she cannot be hit now, it waits as a cue again. */
  const hit = () => {
    if (onCue(mode, 'request') !== 'play') {
      if (thrown) {
        usePet.getState().cue({ kind: 'request', from: thrown.from })
      }
      return false
    }
    go('request')
    return true
  }

  const sheet = sheets[looks[mode].sheet]
  return (
    <>
      {/* Not given pointer events of its own: under a modal dialog, clicks pass through her. */}
      <div
        ref={pet}
        aria-hidden
        className="invisible fixed top-0 left-0 z-[60] cursor-grab touch-none select-none active:cursor-grabbing"
        style={{
          width: sheet.width,
          height: sheet.height,
          backgroundImage: `url(${sheet.url})`,
          backgroundSize: `${sheet.width * sheet.frames}px ${sheet.height}px`,
          backgroundPosition: `${-frame * sheet.width}px 0`,
          backgroundRepeat: 'no-repeat',
        }}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerUp}
      />
      {thrown && (
        <PaperBall
          key={thrown.id}
          from={thrown.from}
          feet={() => body.current}
          onHit={hit}
          pickedUp={msUntil('request', pickUpFrame) / 1000}
          onDone={() => setThrown(null)}
        />
      )}
    </>
  )
}
