import type { MouseEvent } from 'react'
import { ChevronDown, ChevronRight } from 'lucide-react'
import { cn } from '@/components/ui/cn'
import { previewColumns } from '@/features/dictionary/import/preview/preview-columns'
import { InvalidRowDetail } from '@/features/dictionary/import/preview/invalid-row-detail'
import { PreviewRowDetail } from '@/features/dictionary/import/preview/preview-row-detail'
import type { PreviewRow } from '@/features/dictionary/import/preview/preview-rows'
import { rowStatus } from '@/features/dictionary/import/preview/row-status'
import { StatusCell } from '@/features/dictionary/import/preview/status-cell'

type Props = {
  row: PreviewRow
  opened: boolean
  onToggle: () => void
  /** Back to the raw text, with this line selected. */
  onLocate: () => void
}

const frame = {
  warning: 'border-warning ring-1 ring-inset ring-warning',
  skipped: 'border-destructive ring-1 ring-inset ring-destructive',
}

/** One line of the text in a box: orange for a warning, red when skipped. */
export function PreviewRowView({ row, opened, onToggle, onLocate }: Props) {
  const status = rowStatus(row)
  const Chevron = opened ? ChevronDown : ChevronRight
  const locate = (event: MouseEvent) => {
    event.stopPropagation()
    onLocate()
  }

  return (
    <div className={cn('rounded-md border text-sm', status && frame[status.kind])}>
      <div
        className={cn(previewColumns, 'cursor-pointer px-3 py-2 hover:bg-accent/40')}
        onClick={onToggle}
      >
        <span className="pt-0.5 text-muted-foreground">
          <Chevron className="size-3.5" />
        </span>
        <button
          type="button"
          className="self-start text-left font-semibold text-muted-foreground tabular-nums underline underline-offset-4 hover:text-foreground"
          onClick={locate}
        >
          {row.line}
        </button>
        <RowCells row={row} />
        <StatusCell status={status} />
      </div>
      {opened && (
        <div className="border-t px-3 py-3 pl-[4.5rem]">
          {row.kind === 'invalid' ? (
            <InvalidRowDetail source={row.source} reason={row.reason} />
          ) : (
            <PreviewRowDetail
              entry={row.entry}
              droppedSentences={row.kind === 'imported' ? row.droppedSentences : []}
              unknownFields={row.kind === 'imported' ? row.unknownFields : []}
            />
          )}
        </div>
      )}
    </div>
  )
}

/** 写法, 读音, 释义 and 例句. */
function RowCells({ row }: { row: PreviewRow }) {
  if (row.kind === 'invalid') {
    return (
      <>
        <span className="truncate" lang="ja">
          {row.writing || <span className="font-mono text-muted-foreground">{row.source}</span>}
        </span>
        <span />
        <span />
        <span />
      </>
    )
  }
  const { content } = row.entry
  const sense = content.reading?.sense ?? null
  const dropped = row.kind === 'imported' ? row.droppedSentences.length : 0
  return (
    <>
      <span className="truncate">
        <span lang="ja">{content.writing}</span>
        {content.origin && (
          <span className="ml-2 text-xs text-muted-foreground">{content.origin}</span>
        )}
      </span>
      <span className="truncate" lang="ja">
        {content.reading?.kana}
      </span>
      <span className="truncate">
        {sense && (
          <>
            {sense.glosses[0]}
            {sense.glosses.length > 1 && (
              <span className="text-muted-foreground">；{sense.glosses.slice(1).join('；')}</span>
            )}
          </>
        )}
      </span>
      <span className={cn('tabular-nums', dropped > 0 && 'text-warning')}>
        {sense &&
          row.kind === 'imported' &&
          `${sense.sentences.length}/${sense.sentences.length + dropped}`}
      </span>
    </>
  )
}
