import { describe, expect, it } from 'vitest'
import type { Content, ImportPreview } from '@/bindings'
import { isWarning, previewRows } from '@/features/dictionary/import/preview/preview-rows'
import { linesOf } from '@/features/dictionary/import/preview/row-kinds'

const content = (writing: string): Content => ({
  writing,
  origin: '',
  stratum: '',
  reading: null,
})

const text = [
  '{"writing": "諦める"}',
  '',
  'not json',
  '{"writing": "パソコン"}',
  '{"x": 1',
  '{"writing": "食べた"}',
  '{"writing": "上", "pos": "名词"}',
  '{"writing": "下"}',
].join('\n')

const preview: ImportPreview = {
  lines: 7,
  entries: [
    { line: 1, writing: '諦める', content: content('諦める') },
    { line: 6, writing: '食べた', content: content('食べた') },
    { line: 7, writing: '上', content: content('上') },
    { line: 8, writing: '下', content: content('下') },
  ],
  skipped_katakana: [{ line: 4, writing: 'パソコン', content: content('パソコン') }],
  inflections: [6],
  problems: [
    {
      line: 1,
      writing: '諦める',
      dropped: { kind: 'sentence', marked: '[[諦めない]]で' },
      reason: '目标词里的汉字 諦 没有注音',
    },
    { line: 3, writing: '', dropped: { kind: 'entry' }, reason: '不是合法的 JSON' },
    { line: 5, writing: '', dropped: { kind: 'entry' }, reason: '不是合法的 JSON' },
  ],
  ignored_fields: [{ name: 'pos', lines: [7] }],
}

describe('previewRows', () => {
  const rows = previewRows(preview, text)

  it('gives one row per line, in line order', () => {
    expect(rows.map((row) => [row.line, row.kind])).toEqual([
      [1, 'imported'],
      [3, 'invalid'],
      [4, 'skipped'],
      [5, 'invalid'],
      [6, 'imported'],
      [7, 'imported'],
      [8, 'imported'],
    ])
  })

  it('attaches dropped sentences and unknown fields to their line', () => {
    expect(rows[0]).toMatchObject({
      droppedSentences: [{ marked: '[[諦めない]]で', reason: '目标词里的汉字 諦 没有注音' }],
      unknownFields: [],
    })
    expect(rows[5]).toMatchObject({ droppedSentences: [], unknownFields: ['pos'] })
    expect(rows[3]).toMatchObject({ source: '{"x": 1' })
  })

  it('marks warnings, and finds the lines of every kind', () => {
    expect(rows.map(isWarning)).toEqual([true, false, false, false, true, true, false])
    expect(linesOf(rows, 'warning')).toEqual([1, 6, 7])
    expect(linesOf(rows, 'inflection')).toEqual([6])
    expect(linesOf(rows, 'sentence')).toEqual([1])
    expect(linesOf(rows, 'unknownField')).toEqual([7])
    expect(linesOf(rows, 'skipped')).toEqual([3, 4, 5])
    expect(linesOf(rows, 'katakana')).toEqual([4])
    expect(linesOf(rows, 'invalid')).toEqual([3, 5])
  })
})
