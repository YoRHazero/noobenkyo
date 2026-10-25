import { describe, expect, it } from 'vitest'
import { fillRangeText } from '@/features/dictionary/agent/fill-range-text'

describe('fillRangeText', () => {
  it('names only the senses without sentences when nothing else narrows', () => {
    expect(fillRangeText(null, 'Any', 12)).toBe('范围：没例句（12 行）')
  })

  it('names the keyword and progress in front', () => {
    expect(fillRangeText('食', 'Untouched', 37)).toBe(
      '范围：关键词「食」、进度：未学习、没例句（37 行）',
    )
  })
})
