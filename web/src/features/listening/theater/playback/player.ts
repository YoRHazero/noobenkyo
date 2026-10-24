import type { EpisodeDetail, Language, LineId } from '@/bindings'
import { missingAudioText } from '@/features/listening/missing-audio-text'
import { afterItem } from '@/features/listening/theater/playback/after-item'
import { canPlay, missingAudio } from '@/features/listening/theater/playback/missing-audio'
import {
  pauseClock,
  resumeClock,
  startClock,
  timeIsUp,
  type TimerChoice,
} from '@/features/listening/theater/playback/play-timer'
import type { PlayerSnapshot } from '@/features/listening/theater/playback/player-snapshot'
import {
  RecordingBuffer,
  type Recording,
} from '@/features/listening/theater/playback/recording-buffer'
import { voiceLanguages, type VoiceModes } from '@/features/listening/theater/playback/voice-modes'

/** How long an episode's title card stays up before its first line. */
const cardMs = 2000

type Wiring = {
  load: (line: LineId, language: Language) => Promise<Blob | null>
  /** The episode after this one, for going on; `null` after the last. */
  nextEpisode: (uuid: string) => string | null
  /** Asks for another episode to be loaded and handed back to `show`. */
  requestEpisode: (uuid: string) => void
}

/**
 * Plays a world's episodes line by line through one `<audio>`: each line in
 * the languages of its voice's mode, then on to the next line, the next
 * episode, until the timer, 自动 or the last episode stops it. It never
 * touches the screen; the theater reads its snapshots and calls its actions.
 */
export class Player {
  private snapshot: PlayerSnapshot = {
    episode: null,
    lineIndex: 0,
    steps: [],
    step: 0,
    playing: false,
    auto: true,
    card: false,
    blocked: false,
    notice: null,
    clock: startClock('none'),
  }
  private listeners = new Set<() => void>()
  private audio = new Audio()
  private url: string | null = null
  private buffer: RecordingBuffer
  private modes: VoiceModes = { cast: {}, other: 'ja-zh' }
  /** Bumped whenever what should play changes, so late loads are dropped. */
  private generation = 0
  private cardTimer: ReturnType<typeof setTimeout> | null = null
  /** The current recording is loaded into the `<audio>`, so a pause can resume it. */
  private loaded = false
  /** The current line played to its end and playing stopped there. */
  private lineDone = false
  /** Set when the screen went away; the next `show` starts afresh. */
  private detached = false

  constructor(private wiring: Wiring) {
    this.buffer = new RecordingBuffer(wiring.load)
    this.audio.addEventListener('ended', () => this.ended())
  }

  subscribe = (listener: () => void) => {
    this.listeners.add(listener)
    return () => {
      this.listeners.delete(listener)
    }
  }

  getSnapshot = () => this.snapshot

  private publish(change: Partial<PlayerSnapshot>) {
    this.snapshot = { ...this.snapshot, ...change }
    this.listeners.forEach((listener) => listener())
  }

  // ---- from the theater

  setModes(modes: VoiceModes) {
    this.modes = modes
  }

  /**
   * Puts an episode on stage: its title card, then its first line. The same
   * episode again only refreshes its lines, unless the screen went away.
   */
  show(episode: EpisodeDetail) {
    const current = this.snapshot.episode
    if (!this.detached && current?.uuid === episode.uuid) {
      this.publish({
        episode,
        lineIndex: Math.min(this.snapshot.lineIndex, Math.max(0, episode.lines.length - 1)),
      })
      return
    }
    this.detached = false
    this.halt()
    this.publish({ episode, lineIndex: 0, steps: [], step: 0, card: true, notice: null })
    this.cardTimer = setTimeout(
      () => {
        this.cardTimer = null
        this.publish({ card: false })
        this.startLine(0)
      },
      // In the background, timers crawl: go straight on.
      document.hidden ? 0 : cardMs,
    )
  }

  /** The screen is going away: silence, and forget what was loaded. */
  detach() {
    this.detached = true
    this.halt()
    this.buffer.clear()
    this.publish({ playing: false, clock: pauseClock(this.snapshot.clock, performance.now()) })
  }

  // ---- actions

  toggle() {
    const { playing, card, blocked, lineIndex, step } = this.snapshot
    if (card) {
      return
    }
    if (blocked) {
      this.publish({ blocked: false })
      this.playStep(step)
      return
    }
    if (playing) {
      this.generation += 1
      this.audio.pause()
      this.stop(null)
      return
    }
    if (this.lineDone) {
      this.goOn(lineIndex + 1)
    } else if (this.loaded) {
      this.publish({
        playing: true,
        notice: null,
        clock: resumeClock(this.snapshot.clock, performance.now()),
      })
      this.audio.play().catch((error: unknown) => this.refused(error))
    } else if (this.snapshot.steps.length > 0 && this.snapshot.notice === null) {
      // Paused while the recording was still loading.
      this.playStep(step)
    } else {
      this.startLine(lineIndex)
    }
  }

  next() {
    if (!this.snapshot.card) {
      this.goOn(this.snapshot.lineIndex + 1)
    }
  }

  previous() {
    if (!this.snapshot.card) {
      this.startLine(Math.max(0, this.snapshot.lineIndex - 1))
    }
  }

  replay() {
    if (!this.snapshot.card) {
      this.startLine(this.snapshot.lineIndex)
    }
  }

  jumpTo(lineIndex: number) {
    if (!this.snapshot.card) {
      this.startLine(lineIndex)
    }
  }

  setAuto(auto: boolean) {
    this.publish({ auto })
  }

  /** A new choice starts counting from now. */
  setTimer(choice: TimerChoice) {
    const clock = startClock(choice)
    this.publish({
      clock: this.snapshot.playing ? resumeClock(clock, performance.now()) : clock,
    })
  }

  // ---- the play loop

  /** The line at `lineIndex`, or what comes after the episode when it is past the end. */
  private goOn(lineIndex: number) {
    const episode = this.snapshot.episode
    if (episode === null) {
      return
    }
    if (lineIndex < episode.lines.length) {
      this.startLine(lineIndex)
      return
    }
    const next = this.wiring.nextEpisode(episode.uuid)
    if (next === null) {
      this.stop('已是最后一集')
    } else {
      this.halt()
      this.wiring.requestEpisode(next)
    }
  }

  private startLine(lineIndex: number) {
    const episode = this.snapshot.episode
    const line = episode?.lines[lineIndex]
    if (!episode || !line) {
      return
    }
    this.lineDone = false
    const missing = missingAudio(episode.lines, this.modes)
    if (!canPlay(missing)) {
      this.halt()
      this.publish({ lineIndex, steps: voiceLanguages(this.modes, line.voice), step: 0 })
      this.stop(missingAudioText(missing.ja, missing.zh))
      return
    }
    this.publish({ lineIndex, steps: voiceLanguages(this.modes, line.voice), step: 0 })
    this.playStep(0)
  }

  private async playStep(step: number) {
    const { episode, lineIndex, steps } = this.snapshot
    const line = episode?.lines[lineIndex]
    const language = steps[step]
    if (!episode || !line || !language) {
      return
    }
    const generation = ++this.generation
    this.audio.pause()
    this.loaded = false
    this.publish({
      step,
      playing: true,
      blocked: false,
      notice: null,
      clock: resumeClock(this.snapshot.clock, performance.now()),
    })
    const current: Recording = { line: line.id, language }
    const upcoming = this.upcoming(step)
    this.buffer.keep(upcoming ? [current, upcoming] : [current])
    const blob = await this.buffer.get(current).catch((error: unknown) => {
      if (generation === this.generation) {
        this.stop(error instanceof Error ? error.message : String(error))
      }
      return undefined
    })
    if (generation !== this.generation || blob === undefined) {
      return
    }
    if (blob === null) {
      this.stop('缺音频')
      return
    }
    this.setSource(blob)
    this.loaded = true
    this.audio.play().catch((error: unknown) => {
      if (generation === this.generation) {
        this.refused(error)
      }
    })
  }

  /** The recording after the current one in this episode, to load ahead. */
  private upcoming(step: number): Recording | null {
    const { episode, lineIndex, steps } = this.snapshot
    const line = episode?.lines[lineIndex]
    const nextLanguage = steps[step + 1]
    if (line && nextLanguage) {
      return { line: line.id, language: nextLanguage }
    }
    const nextLine = episode?.lines[lineIndex + 1]
    const first = nextLine && voiceLanguages(this.modes, nextLine.voice)[0]
    return nextLine && first ? { line: nextLine.id, language: first } : null
  }

  private ended() {
    const { episode, lineIndex, steps, step, auto, clock } = this.snapshot
    if (!episode) {
      return
    }
    const now = performance.now()
    const after = afterItem({
      steps: steps.length,
      step,
      lineIndex,
      lines: episode.lines.length,
      auto,
      timerUp: timeIsUp(clock, now),
    })
    switch (after.kind) {
      case 'step':
        this.playStep(after.step)
        return
      case 'line':
        this.startLine(after.lineIndex)
        return
      case 'pause':
        this.lineDone = true
        this.stop(null)
        if (after.reason === 'timer') {
          this.publish({ clock: startClock('none') })
        }
        return
      case 'episode-end':
        this.lineDone = true
        if (clock.choice === 'episode') {
          this.stop(null)
          this.publish({ clock: startClock('none') })
        } else {
          this.goOn(episode.lines.length)
        }
    }
  }

  /** Playing stops where it is, saying why when it stopped by itself. */
  private stop(notice: string | null) {
    this.publish({
      playing: false,
      notice,
      clock: pauseClock(this.snapshot.clock, performance.now()),
    })
  }

  /** The webview would not play without a click: wait for one. */
  private refused(error: unknown) {
    if (error instanceof DOMException && error.name === 'NotAllowedError') {
      this.stop(null)
      this.publish({ blocked: true })
    } else {
      this.stop(error instanceof Error ? error.message : String(error))
    }
  }

  /** Silences everything in progress: the card's countdown, a load, the sound. */
  private halt() {
    this.generation += 1
    if (this.cardTimer !== null) {
      clearTimeout(this.cardTimer)
      this.cardTimer = null
    }
    this.audio.pause()
    this.loaded = false
  }

  private setSource(blob: Blob) {
    if (this.url !== null) {
      URL.revokeObjectURL(this.url)
    }
    this.url = URL.createObjectURL(blob)
    this.audio.src = this.url
  }
}
