import type { ConvertedEntry, ImportPreview } from '@/bindings'
import { lineText } from '@/features/dictionary/import/line-range'

export type DroppedSentence = {
  marked: string
  reason: string
}

/** A language written without an origin, left out of the word. */
export type DroppedLanguage = {
  value: string
  reason: string
}

/** One line of the text in the preview. */
export type PreviewRow =
  /** Merged; a warning when part of the line is left out, or it looks inflected. */
  | {
      kind: 'imported'
      line: number
      entry: ConvertedEntry
      inflected: boolean
      droppedLanguage: DroppedLanguage | null
      droppedSentences: DroppedSentence[]
      unknownFields: string[]
    }
  /** A pure katakana word, skipped by the import option. */
  | { kind: 'skipped'; line: number; entry: ConvertedEntry }
  /** Not a valid entry; `source` is the line as written, for lines without a writing. */
  | { kind: 'invalid'; line: number; writing: string; source: string; reason: string }

/** One row per non-blank line, in line order. */
export function previewRows(preview: ImportPreview, text: string): PreviewRow[] {
  const droppedLanguages = new Map<number, DroppedLanguage>()
  const droppedSentences = groupByLine<DroppedSentence>()
  const unknownFields = groupByLine<string>()
  const rows: PreviewRow[] = []
  for (const problem of preview.problems) {
    switch (problem.dropped.kind) {
      case 'entry':
        rows.push({
          kind: 'invalid',
          line: problem.line,
          writing: problem.writing,
          source: lineText(text, problem.line).trim(),
          reason: problem.reason,
        })
        break
      case 'originLanguage':
        droppedLanguages.set(problem.line, {
          value: problem.dropped.value,
          reason: problem.reason,
        })
        break
      case 'sentence':
        droppedSentences.add(problem.line, {
          marked: problem.dropped.marked,
          reason: problem.reason,
        })
        break
      case 'sense':
        // Only merging finds these; a preview never has them.
        break
    }
  }
  for (const field of preview.ignored_fields) {
    for (const line of field.lines) {
      unknownFields.add(line, field.name)
    }
  }
  const inflections = new Set(preview.inflections)
  for (const entry of preview.entries) {
    rows.push({
      kind: 'imported',
      line: entry.line,
      entry,
      inflected: inflections.has(entry.line),
      droppedLanguage: droppedLanguages.get(entry.line) ?? null,
      droppedSentences: droppedSentences.get(entry.line),
      unknownFields: unknownFields.get(entry.line),
    })
  }
  for (const entry of preview.skipped_katakana) {
    rows.push({ kind: 'skipped', line: entry.line, entry })
  }
  return rows.sort((a, b) => a.line - b.line)
}

/** An imported row that looks inflected or leaves part of its line out. */
export function isWarning(row: PreviewRow): boolean {
  return (
    row.kind === 'imported' &&
    (row.inflected ||
      row.droppedLanguage !== null ||
      row.droppedSentences.length > 0 ||
      row.unknownFields.length > 0)
  )
}

function groupByLine<T>() {
  const groups = new Map<number, T[]>()
  return {
    add: (line: number, item: T) => groups.set(line, [...(groups.get(line) ?? []), item]),
    get: (line: number): T[] => groups.get(line) ?? [],
  }
}
