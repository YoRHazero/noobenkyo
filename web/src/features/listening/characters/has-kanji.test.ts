import { describe, expect, it } from 'vitest'
import { hasKanji } from '@/features/listening/characters/has-kanji'

describe('hasKanji', () => {
  it('finds kanji', () => {
    expect(hasKanji('美咲')).toBe(true)
    expect(hasKanji('人々')).toBe(true)
  })

  it('finds none in kana or latin letters', () => {
    expect(hasKanji('アリス・リデル')).toBe(false)
    expect(hasKanji('Alice')).toBe(false)
  })
})
