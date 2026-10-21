import { describe, expect, it } from 'vitest'
import type { SearchRow } from '@/bindings'
import { wordGroups } from '@/features/dictionary/browse/word-groups'

const row = (writing: string, origin: string, kana: string | null): SearchRow => ({
  word: { writing, origin },
  kana,
  sense: null,
})

describe('wordGroups', () => {
  it('groups runs of the same word, telling words apart by origin', () => {
    const rows = [
      row('明日', '', 'あした'),
      row('明日', '', 'あす'),
      row('コート', 'coat', null),
      row('コート', 'court', null),
    ]
    expect(wordGroups(rows).map((group) => [group.word.origin, group.rows.length])).toEqual([
      ['', 2],
      ['coat', 1],
      ['court', 1],
    ])
  })
})
