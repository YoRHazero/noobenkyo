import { describe, expect, it } from 'vitest'
import { parseSeed, randomSeed, voiceDesign } from '@/features/listening/characters/voice-design'

describe('voice design', () => {
  it('needs a prompt that is not blank', () => {
    expect(voiceDesign(null, 7)).toBeNull()
    expect(voiceDesign('  \n', 7)).toBeNull()
    expect(voiceDesign('明亮的声音。', 7)).toEqual({ prompt: '明亮的声音。', seed: 7 })
  })

  it('draws seeds that fit in 32 bits', () => {
    for (let i = 0; i < 100; i++) {
      const seed = randomSeed()
      expect(Number.isInteger(seed)).toBe(true)
      expect(seed).toBeGreaterThanOrEqual(0)
      expect(seed).toBeLessThan(2 ** 32)
    }
  })

  it('takes a typed seed that fits in 32 bits', () => {
    expect(parseSeed(' 42 ')).toBe(42)
    expect(parseSeed('0')).toBe(0)
    expect(parseSeed('4294967295')).toBe(2 ** 32 - 1)
    expect(parseSeed('4294967296')).toBeNull()
    expect(parseSeed('')).toBeNull()
    expect(parseSeed('-1')).toBeNull()
    expect(parseSeed('1.5')).toBeNull()
    expect(parseSeed('1e3')).toBeNull()
  })
})
