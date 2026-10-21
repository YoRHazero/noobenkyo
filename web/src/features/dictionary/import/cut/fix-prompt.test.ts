import { describe, expect, it } from 'vitest'
import { fixPrompt } from '@/features/dictionary/import/cut/fix-prompt'

describe('fixPrompt', () => {
  it('ends with the warnings, then the lines in a code block', () => {
    const prompt = fixPrompt(
      ['第 1 行 食べた：疑似活用形'],
      ['{"writing": "食べた", "kana": "たべた", "glosses": ["吃了"]}'],
    )
    expect(prompt.split('\n').slice(-7)).toEqual([
      '警告（行号是"原文"里的行号）：',
      '第 1 行 食べた：疑似活用形',
      '',
      '原文：',
      '```',
      '{"writing": "食べた", "kana": "たべた", "glosses": ["吃了"]}',
      '```',
    ])
  })

  it('gives an example line that is valid JSON', () => {
    const example = fixPrompt([], [])
      .split('\n')
      .find((line) => line.startsWith('{"writing": "諦める"'))
    expect(() => JSON.parse(example ?? '')).not.toThrow()
  })
})
