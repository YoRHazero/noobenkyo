import { describe, expect, it } from 'vitest'
import {
  deleteLine,
  insertLine,
  replaceHeader,
  replaceLine,
  scriptParts,
  scriptText,
  type ScriptParts,
} from '@/features/listening/script/script-edits'

const line = (speaker: string) => ({ speaker, ja: `{${speaker}|か}`, zh: speaker })

const parts: ScriptParts = {
  title: '初遇',
  summary: '两人相遇。',
  lines: [line('甲'), line('乙'), line('丙')],
}

const speakers = (text: string) =>
  (JSON.parse(text) as ScriptParts).lines.map((line) => line.speaker)

describe('scriptParts', () => {
  it('keeps the Japanese as written and drops the ruby', () => {
    const preview = {
      title: '初遇',
      summary: '两人相遇。',
      lines: [
        {
          speaker: '旁白',
          ja: '{春|はる}',
          ja_ruby: [{ kind: 'ruby' as const, writing: '春', reading: 'はる' }],
          zh: '春天',
        },
      ],
    }
    expect(scriptParts(preview)).toEqual({
      title: '初遇',
      summary: '两人相遇。',
      lines: [{ speaker: '旁白', ja: '{春|はる}', zh: '春天' }],
    })
  })
})

describe('scriptText', () => {
  it('writes the standard format with two-space indents', () => {
    const text = scriptText({
      title: 't',
      summary: 's',
      lines: [{ speaker: 'a', ja: 'b', zh: 'c' }],
    })
    expect(text).toBe(
      '{\n  "title": "t",\n  "summary": "s",\n  "lines": [\n    {\n      "speaker": "a",\n      "ja": "b",\n      "zh": "c"\n    }\n  ]\n}',
    )
  })
})

describe('edits', () => {
  it('replaces the title and summary only', () => {
    const edited = JSON.parse(replaceHeader(parts, { title: '重逢', summary: '又见面了。' }))
    expect(edited).toEqual({ ...parts, title: '重逢', summary: '又见面了。' })
  })

  it('replaces one line', () => {
    expect(speakers(replaceLine(parts, 1, line('丁')))).toEqual(['甲', '丁', '丙'])
  })

  it('deletes one line', () => {
    expect(speakers(deleteLine(parts, 0))).toEqual(['乙', '丙'])
  })

  it('inserts before a line, first, or last', () => {
    expect(speakers(insertLine(parts, 1, line('丁')))).toEqual(['甲', '丁', '乙', '丙'])
    expect(speakers(insertLine(parts, 0, line('丁')))).toEqual(['丁', '甲', '乙', '丙'])
    expect(speakers(insertLine(parts, 3, line('丁')))).toEqual(['甲', '乙', '丙', '丁'])
  })
})
