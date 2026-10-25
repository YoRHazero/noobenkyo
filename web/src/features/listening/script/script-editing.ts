import {
  insertLine,
  replaceHeader,
  replaceLine,
  type HeaderDraft,
  type LineDraft,
  type ScriptParts,
} from '@/features/listening/script/script-edits'

/**
 * The one editor open in the preview, with what is typed in it and the
 * problems its last confirm met. Only one at a time: an insert or a delete
 * renumbers the lines an open editor points at.
 */
export type Editing =
  | { kind: 'header'; original: HeaderDraft; draft: HeaderDraft; problems: string[] }
  | { kind: 'line'; index: number; original: LineDraft; draft: LineDraft; problems: string[] }
  | { kind: 'insert'; at: number; draft: LineDraft; problems: string[] }

/**
 * Whether an editor is open, and whether what is in it differs from what it
 * opened with: a changed editor is an edit not written back yet.
 */
export type EditorStatus = 'closed' | 'open' | 'changed'

const differs = <T extends object>(draft: T, original: T) =>
  (Object.keys(draft) as (keyof T)[]).some(
    (key) => String(draft[key]).trim() !== String(original[key]).trim(),
  )

const emptyLine: LineDraft = { speaker: '', ja: '', zh: '' }

export function editorStatus(editing: Editing | null): EditorStatus {
  if (editing === null) {
    return 'closed'
  }
  const original = editing.kind === 'insert' ? emptyLine : editing.original
  return differs(editing.draft, original) ? 'changed' : 'open'
}

/** The script with what the editor holds put in. */
export function editedText(parts: ScriptParts, editing: Editing): string {
  switch (editing.kind) {
    case 'header':
      return replaceHeader(parts, editing.draft)
    case 'line':
      return replaceLine(parts, editing.index, editing.draft)
    case 'insert':
      return insertLine(parts, editing.at, editing.draft)
  }
}

export function openInsert(at: number): Extract<Editing, { kind: 'insert' }> {
  return { kind: 'insert', at, draft: emptyLine, problems: [] }
}
