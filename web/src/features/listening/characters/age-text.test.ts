import { describe, expect, it } from 'vitest'
import { ageText } from '@/features/listening/characters/age-text'

describe('ageText', () => {
  it('writes ages the way they are said', () => {
    expect([1, 9, 10, 11, 17, 20, 21, 60, 99, 100].map(ageText)).toEqual([
      '一',
      '九',
      '十',
      '十一',
      '十七',
      '二十',
      '二十一',
      '六十',
      '九十九',
      '一百',
    ])
  })
})
