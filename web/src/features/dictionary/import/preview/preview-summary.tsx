import { useState } from 'react'
import { FilterChip } from '@/features/dictionary/import/preview/filter-chip'
import { KindList } from '@/features/dictionary/import/preview/kind-list'
import type { PreviewRow } from '@/features/dictionary/import/preview/preview-rows'
import {
  filterLabel,
  linesOf,
  skipKinds,
  warningKinds,
  type RowFilter,
} from '@/features/dictionary/import/preview/row-kinds'
import { SummaryCategory } from '@/features/dictionary/import/preview/summary-category'
import { SummaryFigure } from '@/features/dictionary/import/preview/summary-figure'

type Props = {
  /** Non-blank lines of the text. */
  lines: number
  rows: PreviewRow[]
  filter: RowFilter | null
  onFilter: (filter: RowFilter | null) => void
  onCut: (label: string, lines: number[]) => void
  onDelete: (lines: number[]) => void
  actionsDisabled: boolean
}

/**
 * The counts above the table. Warnings and skipped open into their kinds,
 * to show only those rows, to cut warned lines or to delete skipped ones.
 * Categories with no rows are left out.
 */
export function PreviewSummary({
  lines,
  rows,
  filter,
  onFilter,
  onCut,
  onDelete,
  actionsDisabled,
}: Props) {
  const [open, setOpen] = useState<'warning' | 'skipped' | null>(null)
  const imported = rows.filter((row) => row.kind === 'imported').length
  const warnings = linesOf(rows, 'warning').length
  const skipped = linesOf(rows, 'skipped').length
  const toggle = (category: 'warning' | 'skipped') => setOpen(open === category ? null : category)

  return (
    <div className="flex flex-col gap-2 text-sm">
      <div className="flex items-center gap-1">
        <SummaryFigure label="总计" count={lines} />
        <SummaryFigure label="可导入" count={imported} />
        {warnings > 0 && (
          <SummaryCategory
            label="警告"
            count={warnings}
            tone="text-warning"
            open={open === 'warning'}
            onToggle={() => toggle('warning')}
          />
        )}
        {skipped > 0 && (
          <SummaryCategory
            label="跳过"
            count={skipped}
            tone="text-destructive"
            open={open === 'skipped'}
            onToggle={() => toggle('skipped')}
          />
        )}
        <span className="ml-auto">
          {filter && <FilterChip label={filterLabel(filter)} onClear={() => onFilter(null)} />}
        </span>
      </div>
      {open === 'warning' && warnings > 0 && (
        <KindList
          category="warning"
          kinds={warningKinds}
          rows={rows}
          filter={filter}
          onFilter={onFilter}
          onCut={onCut}
          actionsDisabled={actionsDisabled}
        />
      )}
      {open === 'skipped' && skipped > 0 && (
        <KindList
          category="skipped"
          kinds={skipKinds}
          rows={rows}
          filter={filter}
          onFilter={onFilter}
          onDelete={onDelete}
          actionsDisabled={actionsDisabled}
        />
      )}
    </div>
  )
}
