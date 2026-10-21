import type { SenseKey } from '@/bindings'
import { BrowseRow } from '@/features/dictionary/browse/browse-row'
import { DeleteWordButton } from '@/features/dictionary/browse/delete-word-button'
import { rowKey } from '@/features/dictionary/browse/row-key'
import type { WordGroup } from '@/features/dictionary/browse/word-groups'

type Props = {
  group: WordGroup
  maxStage: number
  opened: ReadonlySet<string>
  onToggle: (key: string) => void
  onRekey: (from: string, to: SenseKey) => void
}

/**
 * A word in a box: its writing and origin, then one line per row below it.
 * A word with nothing below it is the heading alone, with its bin.
 */
export function WordGroupView({ group, maxStage, opened, onToggle, onRekey }: Props) {
  const { word, rows } = group
  const wordOnly = rows.length === 1 && rows[0]?.kana === null
  return (
    <div className="rounded-md border text-sm">
      <div className="flex min-h-11 items-center gap-2 px-3 py-1.5">
        <span lang="ja" className="text-base font-medium">
          {word.writing}
        </span>
        {word.origin && <span className="text-xs text-muted-foreground">{word.origin}</span>}
        {wordOnly && (
          <span className="ml-auto">
            <DeleteWordButton word={word} />
          </span>
        )}
      </div>
      {!wordOnly &&
        rows.map((row) => {
          const key = rowKey(row)
          return (
            <BrowseRow
              key={key}
              row={row}
              maxStage={maxStage}
              opened={opened.has(key)}
              onToggle={() => onToggle(key)}
              onRekey={(to) => onRekey(key, to)}
            />
          )
        })}
    </div>
  )
}
