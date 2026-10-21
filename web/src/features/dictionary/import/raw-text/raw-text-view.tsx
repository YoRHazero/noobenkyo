import { useMemo, useRef, useState, type KeyboardEvent, type RefObject } from 'react'
import { flushSync } from 'react-dom'
import { EntryTextEditor } from '@/features/dictionary/import/raw-text/entry-text-editor'
import { FindBar } from '@/features/dictionary/import/raw-text/find-bar'
import { findMatches } from '@/features/dictionary/import/raw-text/find-matches'
import { scrollToMark } from '@/features/dictionary/import/raw-text/scroll-to-mark'

type Props = {
  textBox: RefObject<HTMLTextAreaElement | null>
  /** Hidden rather than unmounted, so the undo history and the query survive the table view. */
  hidden: boolean
  text: string
  disabled: boolean
  onChange: (text: string) => void
}

/**
 * The raw text view: the find bar, then the editable text with the matches
 * marked. Finding only marks and scrolls; the cursor stays where it was.
 */
export function RawTextView({ textBox, hidden, text, disabled, onChange }: Props) {
  const findInput = useRef<HTMLInputElement>(null)
  const currentMark = useRef<HTMLElement>(null)
  const [query, setQuery] = useState('')
  const [chosen, setChosen] = useState(0)
  const matches = useMemo(() => findMatches(text, query), [text, query])
  // Editing the text can leave fewer matches than the chosen index.
  const current = Math.min(chosen, matches.length - 1)

  /** Moves to a match, then scrolls to it once it is marked. */
  const go = (nextQuery: string, index: number) => {
    flushSync(() => {
      setQuery(nextQuery)
      setChosen(index)
    })
    if (textBox.current && currentMark.current) {
      scrollToMark(textBox.current, currentMark.current)
    }
  }
  const step = (by: 1 | -1) => {
    if (matches.length > 0) {
      go(query, (current + by + matches.length) % matches.length)
    }
  }
  /** Cmd+F (Ctrl+F) anywhere in the view goes to the find bar. */
  const keyDown = (event: KeyboardEvent) => {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'f') {
      event.preventDefault()
      findInput.current?.focus()
      findInput.current?.select()
    }
  }

  return (
    <div hidden={hidden} className="flex h-full flex-col gap-3" onKeyDown={keyDown}>
      <FindBar
        ref={findInput}
        query={query}
        matches={matches.length}
        current={current}
        onQuery={(nextQuery) => go(nextQuery, 0)}
        onStep={step}
      />
      <div className="min-h-0 flex-1">
        <EntryTextEditor
          textBox={textBox}
          currentMark={currentMark}
          text={text}
          disabled={disabled}
          matches={matches}
          current={current}
          onChange={onChange}
        />
      </div>
    </div>
  )
}
