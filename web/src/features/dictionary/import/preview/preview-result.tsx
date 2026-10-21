import type { ImportPreview } from '@/bindings'
import type { PreviewRow } from '@/features/dictionary/import/preview/preview-rows'
import { PreviewSummary } from '@/features/dictionary/import/preview/preview-summary'
import { PreviewTable } from '@/features/dictionary/import/preview/preview-table'
import { rowIs, type RowFilter } from '@/features/dictionary/import/preview/row-kinds'

type Props = {
  preview: ImportPreview
  rows: PreviewRow[]
  page: number
  filter: RowFilter | null
  actionsDisabled: boolean
  onPage: (page: number) => void
  onFilter: (filter: RowFilter | null) => void
  onCut: (label: string, lines: number[]) => void
  onDelete: (lines: number[]) => void
  onLocate: (line: number) => void
}

/** The summary of a preview, then its rows, narrowed as chosen. */
export function PreviewResult({ preview, rows, page, filter, actionsDisabled, ...on }: Props) {
  if (preview.lines === 0) {
    return <p className="text-sm text-muted-foreground">无内容</p>
  }
  const shown = filter ? rows.filter((row) => rowIs(row, filter)) : rows
  return (
    <div className="flex h-full flex-col gap-3">
      <PreviewSummary
        lines={preview.lines}
        rows={rows}
        filter={filter}
        onFilter={on.onFilter}
        onCut={on.onCut}
        onDelete={on.onDelete}
        actionsDisabled={actionsDisabled}
      />
      <PreviewTable rows={shown} page={page} onPage={on.onPage} onLocate={on.onLocate} />
    </div>
  )
}
