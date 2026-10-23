import { isWarning, type PreviewRow } from '@/features/dictionary/import/preview/preview-rows'

/** The kinds of warning; one row can be several at once. */
export type WarningKind = 'inflection' | 'originLanguage' | 'sentence' | 'unknownField'
export type SkipKind = 'katakana' | 'invalid'
/** What the table can be narrowed to: a whole category, or one kind of it. */
export type RowFilter = 'warning' | 'skipped' | WarningKind | SkipKind

/** In the order statuses and the summary name them. */
export const warningKinds: WarningKind[] = [
  'inflection',
  'originLanguage',
  'sentence',
  'unknownField',
]
export const skipKinds: SkipKind[] = ['katakana', 'invalid']

const labels: Record<RowFilter, string> = {
  warning: '警告',
  skipped: '跳过',
  inflection: '活用形',
  originLanguage: '语种缺词源',
  sentence: '非法例句',
  unknownField: '未知字段',
  katakana: '片假名词',
  invalid: '非法',
}

/** "警告", "活用形" … as the summary and the statuses write it. */
export function filterLabel(filter: RowFilter): string {
  return labels[filter]
}

/** Whether the row belongs to the category or kind. */
export function rowIs(row: PreviewRow, filter: RowFilter): boolean {
  switch (filter) {
    case 'warning':
      return isWarning(row)
    case 'skipped':
      return row.kind !== 'imported'
    case 'inflection':
      return row.kind === 'imported' && row.inflected
    case 'originLanguage':
      return row.kind === 'imported' && row.droppedLanguage !== null
    case 'sentence':
      return row.kind === 'imported' && row.droppedSentences.length > 0
    case 'unknownField':
      return row.kind === 'imported' && row.unknownFields.length > 0
    case 'katakana':
      return row.kind === 'skipped'
    case 'invalid':
      return row.kind === 'invalid'
  }
}

/** The lines of the rows that belong to the category or kind, in order. */
export function linesOf(rows: PreviewRow[], filter: RowFilter): number[] {
  return rows.filter((row) => rowIs(row, filter)).map((row) => row.line)
}
