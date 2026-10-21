import { describe, expect, it } from 'vitest'
import type { ConvertedEntry } from '@/bindings'
import { rowStatus } from '@/features/dictionary/import/preview/row-status'

const entry: ConvertedEntry = {
  line: 1,
  writing: '上',
  content: { writing: '上', origin: '', stratum: '', reading: null },
}

describe('rowStatus', () => {
  it('names what is skipped', () => {
    expect(rowStatus({ kind: 'skipped', line: 1, entry })?.text).toBe('跳过：片假名词')
    expect(
      rowStatus({ kind: 'invalid', line: 1, writing: '', source: 'x', reason: '不是合法的 JSON' }),
    ).toEqual({ kind: 'skipped', text: '跳过：非法' })
  })

  it('names every warning of an imported line, in a fixed order', () => {
    const status = rowStatus({
      kind: 'imported',
      line: 1,
      entry,
      inflected: true,
      droppedSentences: [{ marked: '[[上]]x', reason: '多余的 x' }],
      unknownFields: ['pos'],
    })
    expect(status).toEqual({ kind: 'warning', text: '警告：活用形、非法例句、未知字段' })
  })

  it('has none for a line imported as written', () => {
    expect(
      rowStatus({
        kind: 'imported',
        line: 1,
        entry,
        inflected: false,
        droppedSentences: [],
        unknownFields: [],
      }),
    ).toBeNull()
  })
})
