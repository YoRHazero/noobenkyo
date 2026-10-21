import { useEffect, useState } from 'react'
import type { SearchPage, SenseKey } from '@/bindings'
import { WordGroupView } from '@/features/dictionary/browse/word-group-view'
import { senseRowKey } from '@/features/dictionary/browse/row-key'
import { rowsPerPage } from '@/features/dictionary/browse/use-browse-filter'
import { wordGroups } from '@/features/dictionary/browse/word-groups'
import { Pager } from '@/features/dictionary/pager'

type Props = {
  page: SearchPage
  /** 0-based. */
  pageIndex: number
  maxStage: number
  /** Whether the filter narrows the rows down, for the empty case. */
  narrowed: boolean
  onPage: (page: number) => void
}

/** A page of rows grouped by word, with the open senses' details. */
export function BrowseList({ page, pageIndex, maxStage, narrowed, onPage }: Props) {
  const [opened, setOpened] = useState<ReadonlySet<string>>(new Set())
  const pages = Math.max(1, Math.ceil(page.total / rowsPerPage))
  // Deleting the last rows of the last page leaves it empty: go back one.
  useEffect(() => {
    if (pageIndex >= pages) {
      onPage(pages - 1)
    }
  }, [pageIndex, pages, onPage])

  const toggle = (key: string) => {
    const next = new Set(opened)
    if (!next.delete(key)) {
      next.add(key)
    }
    setOpened(next)
  }
  /** Keeps an edited sense open when its primary gloss, and so its key, changed. */
  const rekey = (from: string, to: SenseKey) => {
    const next = new Set(opened)
    next.delete(from)
    next.add(senseRowKey(to))
    setOpened(next)
  }

  if (page.total === 0) {
    return (
      <p className="py-6 text-center text-sm text-muted-foreground">
        {narrowed ? '无' : '词库为空'}
      </p>
    )
  }
  return (
    <div className="flex flex-col gap-2">
      <p className="text-sm text-muted-foreground">共 {page.total} 行</p>
      <div className="flex flex-col gap-1.5">
        {wordGroups(page.rows).map((group) => (
          <WordGroupView
            key={JSON.stringify([group.word.writing, group.word.origin])}
            group={group}
            maxStage={maxStage}
            opened={opened}
            onToggle={toggle}
            onRekey={rekey}
          />
        ))}
      </div>
      <Pager page={Math.min(pageIndex, pages - 1)} pages={pages} onPage={onPage} />
    </div>
  )
}
