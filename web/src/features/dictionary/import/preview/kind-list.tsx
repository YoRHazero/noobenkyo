import { Funnel, Scissors, Trash2 } from 'lucide-react'
import { ConfirmButton } from '@/components/confirm-button'
import { Button } from '@/components/ui/button'
import { cn } from '@/components/ui/cn'
import type { PreviewRow } from '@/features/dictionary/import/preview/preview-rows'
import {
  filterLabel,
  linesOf,
  type RowFilter,
} from '@/features/dictionary/import/preview/row-kinds'

type Props = {
  /** The whole category, listed first as "全部". */
  category: RowFilter
  kinds: RowFilter[]
  rows: PreviewRow[]
  filter: RowFilter | null
  onFilter: (filter: RowFilter | null) => void
  /** Cutting a kind's lines; without it the kinds have no scissors. */
  onCut?: (label: string, lines: number[]) => void
  /** Deleting lines, the whole category's included; without it there is no bin. */
  onDelete?: (lines: number[]) => void
  /** While the lines of the rows may not be the text's. */
  actionsDisabled: boolean
}

/**
 * The kinds of a category with their line counts. The funnel shows only
 * those rows, and pressed again every row; the scissors cut them; the bin
 * deletes them on a second click. Kinds with no rows are left out.
 */
export function KindList({
  category,
  kinds,
  rows,
  filter,
  onFilter,
  onCut,
  onDelete,
  actionsDisabled,
}: Props) {
  const items = [
    { kind: category, label: '全部', cuts: false },
    ...kinds.map((kind) => ({ kind, label: filterLabel(kind), cuts: true })),
  ]
    .map((item) => ({ ...item, lines: linesOf(rows, item.kind) }))
    .filter((item) => item.lines.length > 0)

  return (
    <div className="grid w-fit grid-cols-[5rem_3.5rem_auto] items-center gap-x-2 rounded-md border px-3 py-1.5 text-sm">
      {items.map((item) => (
        <div key={item.kind} className="contents">
          <span className={cn(item.kind === category && 'text-muted-foreground')}>
            {item.label}
          </span>
          <span className="text-muted-foreground tabular-nums">{item.lines.length} 行</span>
          <span className="flex gap-0.5">
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={`只看${item.label}`}
              aria-pressed={filter === item.kind}
              className={cn(filter === item.kind && 'bg-accent text-foreground')}
              onClick={() => onFilter(filter === item.kind ? null : item.kind)}
            >
              <Funnel />
            </Button>
            {item.cuts && onCut && (
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={`剪切${item.label}`}
                disabled={actionsDisabled}
                onClick={() => onCut(item.label, item.lines)}
              >
                <Scissors />
              </Button>
            )}
            {onDelete && (
              <ConfirmButton
                label={<Trash2 />}
                ariaLabel={`删除${item.label}`}
                confirmLabel="确认删除"
                size="icon-sm"
                disabled={actionsDisabled}
                onConfirm={() => onDelete(item.lines)}
              />
            )}
          </span>
        </div>
      ))}
    </div>
  )
}
