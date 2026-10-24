import { describe, expect, it } from 'vitest'
import {
  parseAge,
  promptPrefill,
  replacePrefill,
} from '@/features/listening/characters/prompt-prefill'

describe('prompt prefill', () => {
  it('opens with the age and gender', () => {
    expect(promptPrefill('female', 20)).toBe('二十岁的女性。')
    expect(promptPrefill('male', 17)).toBe('十七岁的男性。')
  })

  it('replaces only the opening sentence', () => {
    const prompt = '二十岁的女性。声音明亮。'
    expect(replacePrefill(prompt, '二十岁的女性。', '十七岁的女性。')).toBe(
      '十七岁的女性。声音明亮。',
    )
  })

  it('leaves a prompt that no longer opens with it', () => {
    expect(replacePrefill('声音明亮。', '二十岁的女性。', '十七岁的女性。')).toBe('声音明亮。')
    expect(replacePrefill('', '二十岁的女性。', '十七岁的女性。')).toBe('')
  })

  it('takes whole ages from 1 to 100 only', () => {
    expect(['17', ' 1 ', '100', '0', '101', '1.5', '', 'a'].map(parseAge)).toEqual([
      17,
      1,
      100,
      null,
      null,
      null,
      null,
      null,
    ])
  })
})
