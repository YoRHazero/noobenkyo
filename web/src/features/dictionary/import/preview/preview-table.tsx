import { useState } from 'react'
import { cn } from '@/components/ui/cn'
import { Pager } from '@/features/dictionary/pager'
import { previewColumns } from '@/features/dictionary/import/preview/preview-columns'
import { PreviewRowView } from '@/features/dictionary/import/preview/preview-row'
import type { PreviewRow } from '@/features/dictionary/import/preview/preview-rows'

const rowsPerPage = 50

type Props = {
  rows: PreviewRow[]
  page: number
  onPage: (page: number) => void
  onLocate: (line: number) => void
}

/** A page of preview rows; each row opens to show its detail. */
export function PreviewTable({ rows, page, onPage, onLocate }: Props) {
  const [opened, setOpened] = useState<ReadonlySet<number>>(new Set())
  const pages = Math.max(1, Math.ceil(rows.length / rowsPerPage))
  const current = Math.min(page, pages - 1)
  const shown = rows.slice(current * rowsPerPage, (current + 1) * rowsPerPage)

  const toggle = (line: number) => {
    const next = new Set(opened)
    if (!next.delete(line)) {
      next.add(line)
    }
    setOpened(next)
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col gap-2">
      <div
        className={cn(
          previewColumns,
          'rounded-md bg-muted px-3 py-2 text-sm font-medium text-muted-foreground',
        )}
      >
        <span />
        <span>行</span>
        <span>写法</span>
        <span>读音</span>
        <span>释义</span>
        <span>例句</span>
        <span>状态</span>
      </div>
      <div className="flex min-h-0 flex-1 flex-col gap-1.5 overflow-auto">
        {shown.map((row) => (
          <PreviewRowView
            key={row.line}
            row={row}
            opened={opened.has(row.line)}
            onToggle={() => toggle(row.line)}
            onLocate={() => onLocate(row.line)}
          />
        ))}
        {shown.length === 0 && <p className="py-6 text-center text-sm text-muted-foreground">无</p>}
      </div>
      <Pager page={current} pages={pages} onPage={onPage} />
    </div>
  )
}
