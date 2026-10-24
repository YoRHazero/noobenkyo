import { describe, expect, it } from 'vitest'
import type { CastMember } from '@/bindings'
import { castModes, voiceLanguages } from '@/features/listening/theater/playback/voice-modes'

const cast: CastMember[] = [
  { character: '幸子', role: { kind: 'narrator' }, languages: ['ja', 'zh'] },
  { character: '田中', role: { kind: 'minor' }, languages: ['ja'] },
]

describe('voice modes', () => {
  it('starts every voice with both languages, whatever references it has', () => {
    expect(castModes(cast, {})).toEqual({ cast: { 幸子: 'ja-zh', 田中: 'ja-zh' }, other: 'ja-zh' })
  })

  it('keeps remembered modes as they were chosen', () => {
    const modes = castModes(cast, { cast: { 幸子: 'zh-ja', 田中: 'zh' }, other: 'ja' })
    expect(modes).toEqual({ cast: { 幸子: 'zh-ja', 田中: 'zh' }, other: 'ja' })
  })

  it('plays a voice outside the cast in the other mode', () => {
    const modes = castModes(cast, { other: 'zh' })
    expect(voiceLanguages(modes, '健二')).toEqual(['zh'])
    expect(voiceLanguages(modes, '田中')).toEqual(['ja', 'zh'])
  })
})
