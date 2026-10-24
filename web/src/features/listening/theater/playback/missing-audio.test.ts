import { describe, expect, it } from 'vitest'
import type { LineView } from '@/bindings'
import { canPlay, missingAudio } from '@/features/listening/theater/playback/missing-audio'

const line = (voice: string, audio: LineView['audio']): LineView => ({
  id: 1,
  speaker: voice,
  voice,
  ja: '',
  ja_ruby: [],
  zh: '',
  audio,
})

describe('missingAudio', () => {
  it('counts only what each voice’s mode needs', () => {
    const lines = [line('美咲', ['ja']), line('田中', ['ja']), line('健二', [])]
    const modes = { cast: { 美咲: 'ja-zh', 田中: 'ja' } as const, other: 'zh' as const }
    const missing = missingAudio(lines, modes)
    expect(missing).toEqual({ ja: 0, zh: 2 })
    expect(canPlay(missing)).toBe(false)
    expect(canPlay(missingAudio(lines.slice(1, 2), modes))).toBe(true)
  })
})
