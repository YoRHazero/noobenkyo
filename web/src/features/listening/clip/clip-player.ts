/**
 * The one player for listening to a single recording outside the theater: a
 * reference, a line, a file about to be uploaded. Starting a clip stops the
 * one before, so two never talk over each other.
 */

export type ClipState = { key: string; status: 'loading' | 'playing' } | null

let state: ClipState = null
let audio: HTMLAudioElement | null = null
let url: string | null = null
/** Bumped on every start and stop, so a load that finishes late is dropped. */
let generation = 0
const listeners = new Set<() => void>()

function publish(next: ClipState) {
  state = next
  listeners.forEach((listener) => listener())
}

function release() {
  audio?.pause()
  audio?.removeAttribute('src')
  if (url !== null) {
    URL.revokeObjectURL(url)
    url = null
  }
}

/** Loads and plays a clip; rejects when it cannot be loaded or played. */
export async function playClip(key: string, load: () => Promise<Blob | null>): Promise<void> {
  stopClip()
  const mine = generation
  publish({ key, status: 'loading' })
  try {
    const blob = await load()
    if (mine !== generation) {
      return
    }
    if (blob === null) {
      throw new Error('无音频')
    }
    url = URL.createObjectURL(blob)
    audio ??= new Audio()
    audio.src = url
    audio.onended = () => {
      if (mine === generation) {
        stopClip()
      }
    }
    await audio.play()
    if (mine === generation) {
      publish({ key, status: 'playing' })
    }
  } catch (error) {
    if (mine === generation) {
      stopClip()
    }
    throw error
  }
}

export function stopClip() {
  generation += 1
  release()
  if (state !== null) {
    publish(null)
  }
}

export function subscribeClip(listener: () => void): () => void {
  listeners.add(listener)
  return () => listeners.delete(listener)
}

export function clipState(): ClipState {
  return state
}
