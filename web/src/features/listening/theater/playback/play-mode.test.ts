import { describe, expect, it } from 'vitest'
import { modeLanguages } from '@/features/listening/theater/playback/play-mode'

describe('play modes', () => {
  it('plays the languages in the mode’s order', () => {
    expect(modeLanguages('zh-ja')).toEqual(['zh', 'ja'])
    expect(modeLanguages('ja')).toEqual(['ja'])
  })
})
