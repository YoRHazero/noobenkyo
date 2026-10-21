import type { KeyboardEvent, Ref } from 'react'
import { ChevronDown, ChevronUp, Search } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { cn } from '@/components/ui/cn'
import { Input } from '@/components/ui/input'

type Props = {
  ref: Ref<HTMLInputElement>
  query: string
  matches: number
  /** Index of the current match; -1 for none. */
  current: number
  onQuery: (query: string) => void
  /** To the next match (1) or the previous one (-1), wrapping around. */
  onStep: (step: 1 | -1) => void
}

/**
 * Finds in the raw text: Enter for the next match, Shift+Enter for the
 * previous one, Escape to clear. The count and the arrows show only while
 * there is a query.
 */
export function FindBar({ ref, query, matches, current, onQuery, onStep }: Props) {
  const keyDown = (event: KeyboardEvent) => {
    if (event.nativeEvent.isComposing) {
      return
    }
    if (event.key === 'Enter') {
      event.preventDefault()
      onStep(event.shiftKey ? -1 : 1)
    } else if (event.key === 'Escape') {
      onQuery('')
    }
  }

  return (
    <div className="flex items-center gap-1">
      <div className="relative">
        <Search className="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          ref={ref}
          value={query}
          placeholder="查找"
          aria-label="查找"
          data-own-escape
          className="h-8 w-60 pl-8"
          onChange={(event) => onQuery(event.target.value)}
          onKeyDown={keyDown}
        />
      </div>
      {query !== '' && (
        <>
          <span
            className={cn(
              'px-1 text-sm tabular-nums',
              matches === 0 ? 'text-destructive' : 'text-muted-foreground',
            )}
          >
            {current + 1}/{matches}
          </span>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="上一个"
            disabled={matches === 0}
            onClick={() => onStep(-1)}
          >
            <ChevronUp />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="下一个"
            disabled={matches === 0}
            onClick={() => onStep(1)}
          >
            <ChevronDown />
          </Button>
        </>
      )}
    </div>
  )
}
