import { describe, expect, it } from 'vitest'
import { editedText, editorStatus, openInsert } from '@/features/listening/script/script-editing'
import type { ScriptParts } from '@/features/listening/script/script-edits'

const line = { speaker: '甲', ja: 'あ', zh: '啊' }

describe('editorStatus', () => {
  it('is closed without an editor', () => {
    expect(editorStatus(null)).toBe('closed')
  })

  it('is open until the draft differs beyond surrounding blanks', () => {
    const editing = { kind: 'line' as const, index: 0, original: line, problems: [] }
    expect(editorStatus({ ...editing, draft: { ...line, zh: ' 啊 ' } })).toBe('open')
    expect(editorStatus({ ...editing, draft: { ...line, zh: '哦' } })).toBe('changed')
  })

  it('counts a new line as changed once anything is typed', () => {
    const editing = openInsert(0)
    expect(editorStatus(editing)).toBe('open')
    expect(editorStatus({ ...editing, draft: { ...editing.draft, ja: 'い' } })).toBe('changed')
  })

  it('compares the title and summary of the header', () => {
    const header = { title: '初遇', summary: '相遇。' }
    const editing = { kind: 'header' as const, original: header, problems: [] }
    expect(editorStatus({ ...editing, draft: header })).toBe('open')
    expect(editorStatus({ ...editing, draft: { ...header, summary: '重逢。' } })).toBe('changed')
  })
})

describe('editedText', () => {
  it('puts the new line where the editor was opened', () => {
    const parts: ScriptParts = { title: 't', summary: 's', lines: [line] }
    const next = { speaker: '乙', ja: 'い', zh: '咦' }
    const edited = JSON.parse(editedText(parts, { ...openInsert(1), draft: next })) as ScriptParts
    expect(edited.lines).toEqual([line, next])
  })
})
