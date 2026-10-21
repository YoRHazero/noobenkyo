import type { ReactNode, Ref } from 'react'
import { cn } from '@/components/ui/cn'
import type { TextRange } from '@/features/dictionary/import/line-range'

type Props = {
  /** The laid-out text, which the text box moves as it scrolls. */
  ref: Ref<HTMLDivElement>
  /** The mark of the current match, to scroll to. */
  currentMark: Ref<HTMLElement>
  /** The text box's font, line height and padding. */
  layout: string
  text: string
  matches: TextRange[]
  /** Index of the current match; -1 for none. */
  current: number
}

/**
 * The text laid out exactly as in the text box, but invisible, with the
 * matches marked. It sits behind the transparent text box, so only the marks
 * show, under the text they mark.
 */
export function MatchBackdrop({ ref, currentMark, layout, text, matches, current }: Props) {
  const parts: ReactNode[] = []
  let at = 0
  matches.forEach((match, index) => {
    parts.push(text.slice(at, match.start))
    parts.push(
      <mark
        key={match.start}
        ref={index === current ? currentMark : undefined}
        className={cn(
          'rounded-sm text-transparent',
          index === current
            ? 'bg-yellow-400 dark:bg-yellow-500/70'
            : 'bg-yellow-200 dark:bg-yellow-500/30',
        )}
      >
        {text.slice(match.start, match.end)}
      </mark>,
    )
    at = match.end
  })

  return (
    <div aria-hidden className="pointer-events-none absolute inset-0 overflow-hidden">
      <div ref={ref} className={cn(layout, 'relative whitespace-pre text-transparent')}>
        {matches.length > 0 && [...parts, text.slice(at)]}
      </div>
    </div>
  )
}
