import { filterLabel, rowIs, warningKinds } from '@/features/dictionary/import/preview/row-kinds'
import { isWarning, type PreviewRow } from '@/features/dictionary/import/preview/preview-rows'

export type RowStatus = {
  kind: 'warning' | 'skipped'
  /** "跳过：片假名词", "警告：活用形、未知字段" … */
  text: string
}

/**
 * The status of a row; `null` for a line imported as written. What exactly
 * is left out, and why, is in the row's detail.
 */
export function rowStatus(row: PreviewRow): RowStatus | null {
  switch (row.kind) {
    case 'skipped':
      return { kind: 'skipped', text: '跳过：片假名词' }
    case 'invalid':
      return { kind: 'skipped', text: '跳过：非法' }
    case 'imported': {
      if (!isWarning(row)) {
        return null
      }
      const what = warningKinds.filter((kind) => rowIs(row, kind)).map(filterLabel)
      return { kind: 'warning', text: `警告：${what.join('、')}` }
    }
  }
}
