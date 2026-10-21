import { useMemo, useState } from 'react'
import type { Completeness, ContentFilter, ProgressState } from '@/bindings'
import { useDebounced } from '@/features/dictionary/browse/use-debounced'

export const rowsPerPage = 50

/** Layers only a sense has; a progress condition leaves out the others. */
const senseLayers: Completeness[] = ['NoSentence', 'WithSentence']

/**
 * The dictionary page's filter and page. Typing waits a moment before it
 * searches; any change of the filter goes back to the first page.
 */
export function useBrowseFilter() {
  const [text, setText] = useState('')
  const settledText = useDebounced(text, 250)
  const [completeness, setCompleteness] = useState<Completeness[]>([])
  const [progress, setProgress] = useState<ProgressState>('Any')
  const [page, setPage] = useState(0)

  const filter: ContentFilter = useMemo(
    () => ({ text: settledText.trim() || null, completeness, progress }),
    [settledText, completeness, progress],
  )
  return {
    filter,
    /** Whether anything narrows the rows down. */
    narrowed: filter.text !== null || completeness.length > 0 || progress !== 'Any',
    text,
    setText: (next: string) => {
      setText(next)
      setPage(0)
    },
    completeness,
    setCompleteness: (next: Completeness[]) => {
      setCompleteness(next)
      setPage(0)
    },
    progress,
    setProgress: (next: ProgressState) => {
      setProgress(next)
      if (next !== 'Any') {
        setCompleteness(completeness.filter((layer) => senseLayers.includes(layer)))
      }
      setPage(0)
    },
    page,
    setPage,
    offset: page * rowsPerPage,
  }
}

export type BrowseFilter = ReturnType<typeof useBrowseFilter>
