import { describe, expect, it } from 'vitest'
import {
  modeAllowed,
  modeLanguages,
  startMode,
} from '@/features/listening/theater/playback/play-mode'

describe('play modes', () => {
  it('plays the languages in the mode’s order', () => {
    expect(modeLanguages('zh-ja')).toEqual(['zh', 'ja'])
    expect(modeLanguages('ja')).toEqual(['ja'])
  })

  it('allows only languages the voice has references in', () => {
    expect(modeAllowed('ja', ['ja'])).toBe(true)
    expect(modeAllowed('ja-zh', ['ja'])).toBe(false)
  })

  it('starts with both when there are both, else with the one there is', () => {
    expect(startMode(['ja', 'zh'])).toBe('ja-zh')
    expect(startMode(['zh'])).toBe('zh')
    expect(startMode(['ja'])).toBe('ja')
  })
})
