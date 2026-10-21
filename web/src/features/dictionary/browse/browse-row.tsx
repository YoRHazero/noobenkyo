import type { SearchRow, SenseKey } from '@/bindings'
import { ChevronDown, ChevronRight } from 'lucide-react'
import { cn } from '@/components/ui/cn'
import { browseColumns } from '@/features/dictionary/browse/browse-columns'
import { DeleteReadingButton } from '@/features/dictionary/browse/delete-reading-button'
import { SenseDetailPanel } from '@/features/dictionary/browse/sense-detail-panel'
import { senseState, senseStateLabel } from '@/features/dictionary/sense-state'

type Props = {
  /** A row with a reading: a sense, or a reading with nothing below it. */
  row: SearchRow
  maxStage: number
  opened: boolean
  onToggle: () => void
  onRekey: (to: SenseKey) => void
}

/**
 * One line under a word. A sense opens to its detail; a reading with nothing
 * below it has only its bin.
 */
export function BrowseRow({ row, maxStage, opened, onToggle, onRekey }: Props) {
  const { word, sense } = row
  const kana = row.kana ?? ''
  if (!sense) {
    return (
      <div className={cn(browseColumns, 'border-t px-3 py-1.5')}>
        <span />
        <span lang="ja">{kana}</span>
        <span />
        <span />
        <span />
        <span className="flex justify-end">
          <DeleteReadingButton reading={{ writing: word.writing, origin: word.origin, kana }} />
        </span>
      </div>
    )
  }

  const key: SenseKey = { ...word, kana, primary_gloss: sense.primary_gloss }
  const state = senseState(sense.stage, sense.passes, maxStage)
  const Chevron = opened ? ChevronDown : ChevronRight
  return (
    <div className="border-t">
      <div
        aria-expanded={opened}
        className={cn(browseColumns, 'cursor-pointer px-3 py-2 hover:bg-accent/40')}
        onClick={onToggle}
      >
        <span className="text-muted-foreground">
          <Chevron className="size-3.5" />
        </span>
        <span lang="ja" className="truncate">
          {kana}
        </span>
        <span className="truncate">
          {sense.primary_gloss}
          {sense.secondary_glosses.length > 0 && (
            <span className="text-muted-foreground">；{sense.secondary_glosses.join('；')}</span>
          )}
        </span>
        <span className="truncate text-muted-foreground">{sense.labels.join('、')}</span>
        <span className={cn('tabular-nums', sense.sentence_count === 0 && 'text-muted-foreground')}>
          {sense.sentence_count} 句
        </span>
        <span className={cn(state === 'untouched' && 'text-muted-foreground')}>
          {senseStateLabel[state]}
        </span>
      </div>
      {opened && (
        <div className="border-t px-3 py-3 pl-[2.75rem]">
          <SenseDetailPanel senseKey={key} maxStage={maxStage} onRekey={onRekey} />
        </div>
      )}
    </div>
  )
}
