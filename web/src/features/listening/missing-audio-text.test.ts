import { describe, expect, it } from 'vitest'
import { missingAudioText } from '@/features/listening/missing-audio-text'

describe('missingAudioText', () => {
  it('names only the languages that miss audio', () => {
    expect(missingAudioText(3, 5)).toBe('缺音频：日语 3 行；中文 5 行')
    expect(missingAudioText(0, 2)).toBe('缺音频：中文 2 行')
    expect(missingAudioText(0, 0)).toBeNull()
  })
})
