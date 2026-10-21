import { useRef, type Ref, type RefObject } from 'react'
import { cn } from '@/components/ui/cn'
import type { TextRange } from '@/features/dictionary/import/line-range'
import { MatchBackdrop } from '@/features/dictionary/import/raw-text/match-backdrop'

type Props = {
  textBox: RefObject<HTMLTextAreaElement | null>
  /** The mark of the current match, to scroll to. */
  currentMark: Ref<HTMLElement>
  text: string
  disabled: boolean
  matches: TextRange[]
  /** Index of the current match; -1 for none. */
  current: number
  onChange: (text: string) => void
}

/** Shared by the text box and its backdrop, so the marks line up with the text. */
const layout = 'px-3 py-2 font-mono text-sm leading-6'

/**
 * The entry text, one JSON object per line, with the find bar's matches
 * marked. Lines do not wrap, so each line of the text is one line on screen,
 * as the preview numbers them.
 */
export function EntryTextEditor({
  textBox,
  currentMark,
  text,
  disabled,
  matches,
  current,
  onChange,
}: Props) {
  const backdrop = useRef<HTMLDivElement>(null)

  /** Keeps the backdrop under the text it marks. */
  const follow = () => {
    if (textBox.current && backdrop.current) {
      const { scrollLeft, scrollTop } = textBox.current
      backdrop.current.style.transform = `translate(${-scrollLeft}px, ${-scrollTop}px)`
    }
  }

  return (
    <div
      className={cn(
        'relative h-full overflow-hidden rounded-md border border-input shadow-xs transition-[color,box-shadow] focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50 dark:bg-input/30',
        disabled && 'cursor-not-allowed opacity-50',
      )}
    >
      <MatchBackdrop
        ref={backdrop}
        currentMark={currentMark}
        layout={layout}
        text={text}
        matches={matches}
        current={current}
      />
      <textarea
        ref={textBox}
        aria-label="原始文本"
        wrap="off"
        spellCheck={false}
        value={text}
        disabled={disabled}
        placeholder={
          'JSON Lines，一行一个条目。例：\n{"writing": "明日", "kana": "あした", "glosses": ["明天"]}'
        }
        className={cn(
          layout,
          'relative block size-full resize-none bg-transparent outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed',
        )}
        onScroll={follow}
        onChange={(event) => onChange(event.target.value)}
      />
    </div>
  )
}
