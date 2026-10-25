import { create } from 'zustand'
import { createJSONStorage, persist } from 'zustand/middleware'

/** A point in the window, in CSS pixels. */
export type Point = { x: number; y: number }

/** Something elsewhere in the app that the pet reacts to. */
export type PetCue =
  /** An agent prompt was copied; the paper ball flies from where the button was. */
  | { kind: 'request'; from: Point }
  /** An AI action the user just took failed. */
  | { kind: 'failed' }

/** The side of the window she stands by, and how far her feet are from it, in CSS pixels. */
export type PetPlace = { edge: 'left' | 'right'; distance: number }

type PetState = {
  /** Where she stands; `null` until she has been moved. */
  place: PetPlace | null
  /** Cues waiting for her, oldest first, at most one of each kind. */
  cues: PetCue[]
  cue: (cue: PetCue) => void
  /** Removes a cue she has acted on. */
  drop: (cue: PetCue) => void
  clear: () => void
  setPlace: (place: PetPlace) => void
}

/**
 * The pet's state that other features reach: the cues they send her and
 * where she stands, which is remembered between runs. What she is doing at
 * the moment stays in the pet itself.
 */
export const usePet = create<PetState>()(
  persist(
    (set) => ({
      place: null,
      cues: [],
      // A second cue of a kind already waiting takes its place in the queue.
      cue: (cue) =>
        set((state) => {
          const waiting = state.cues.findIndex((other) => other.kind === cue.kind)
          return {
            cues:
              waiting === -1
                ? [...state.cues, cue]
                : state.cues.map((other, i) => (i === waiting ? cue : other)),
          }
        }),
      drop: (cue) => set((state) => ({ cues: state.cues.filter((other) => other !== cue) })),
      clear: () => set({ cues: [] }),
      setPlace: (place) => set({ place }),
    }),
    {
      name: 'pet',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({ place: state.place }),
    },
  ),
)

/** An agent prompt was copied from the button at `from`. */
export function petRequested(from: DOMRect) {
  usePet.getState().cue({
    kind: 'request',
    from: { x: from.left + from.width / 2, y: from.top + from.height / 2 },
  })
}

/** An AI action the user just took failed. */
export function petFailed() {
  usePet.getState().cue({ kind: 'failed' })
}
