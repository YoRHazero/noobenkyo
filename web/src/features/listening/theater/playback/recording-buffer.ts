import type { Language, LineId } from '@/bindings'

/** One recording to play: a line in one language. */
export type Recording = {
  line: LineId
  language: Language
}

const keyOf = (recording: Recording) => `${recording.line}:${recording.language}`

/**
 * The recordings being played and about to be: loading the next one while
 * the current one plays hides the IPC. Only those two are kept, since audio
 * is large.
 */
export class RecordingBuffer {
  private loads = new Map<string, Promise<Blob | null>>()

  constructor(private load: (line: LineId, language: Language) => Promise<Blob | null>) {}

  get(recording: Recording): Promise<Blob | null> {
    const key = keyOf(recording)
    let pending = this.loads.get(key)
    if (pending === undefined) {
      pending = this.load(recording.line, recording.language)
      // A failed load is tried again next time instead of failing forever.
      pending.catch(() => this.loads.delete(key))
      this.loads.set(key, pending)
    }
    return pending
  }

  /** Drops everything but these, and starts loading them. */
  keep(recordings: readonly Recording[]) {
    const kept = new Set(recordings.map(keyOf))
    for (const key of this.loads.keys()) {
      if (!kept.has(key)) {
        this.loads.delete(key)
      }
    }
    recordings.forEach((recording) => void this.get(recording).catch(() => null))
  }

  clear() {
    this.loads.clear()
  }
}
