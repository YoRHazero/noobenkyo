import { useState } from 'react'
import type { SenseSummary } from '@/bindings'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

type Props = {
  sense: SenseSummary
  deleting: boolean
  onDelete: () => void
}

/** One sense in the browser; deleting asks for a second click. */
export function SenseRow({ sense, deleting, onDelete }: Props) {
  const [confirming, setConfirming] = useState(false)
  const { key } = sense
  return (
    <li className="flex items-start justify-between gap-4 border-b px-3 py-3 last:border-b-0">
      <div className="flex flex-col gap-1">
        <div className="flex items-baseline gap-3">
          <span lang="ja" className="text-lg">
            {key.writing}
          </span>
          <span lang="ja" className="text-muted-foreground">
            {key.kana}
          </span>
          {key.origin && <span className="text-xs text-muted-foreground">{key.origin}</span>}
        </div>
        <p className="text-sm">
          {key.primary_gloss}
          {sense.secondary_glosses.length > 0 && (
            <span className="text-muted-foreground">；{sense.secondary_glosses.join('；')}</span>
          )}
        </p>
        <div className="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
          {sense.labels.map((label) => (
            <Badge key={label} variant="secondary">
              {label}
            </Badge>
          ))}
          <span>
            档位 {sense.stage} · 正确 {sense.passes} · 错误 {sense.lapses} · 例句{' '}
            {sense.sentence_count}
          </span>
        </div>
      </div>
      <Button
        variant={confirming ? 'destructive' : 'ghost'}
        size="sm"
        disabled={deleting}
        onClick={() => (confirming ? onDelete() : setConfirming(true))}
        onBlur={() => setConfirming(false)}
      >
        {confirming ? '确认删除' : '删除'}
      </Button>
    </li>
  )
}
