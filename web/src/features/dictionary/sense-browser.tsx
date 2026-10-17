import { useDeferredValue, useState } from 'react'
import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { SenseRow } from '@/features/dictionary/sense-row'
import { useDeleteSense } from '@/features/dictionary/use-delete-sense'
import { sensesPerPage, useSearchSenses } from '@/features/dictionary/use-search-senses'

/** Searching senses by writing, kana or gloss, and deleting unwanted ones. */
export function SenseBrowser() {
  const [query, setQuery] = useState('')
  const [page, setPage] = useState(0)
  const deferredQuery = useDeferredValue(query)
  const senses = useSearchSenses(deferredQuery, page)
  const deleteSense = useDeleteSense()
  const total = senses.data?.total ?? 0
  const pages = Math.max(1, Math.ceil(total / sensesPerPage))

  return (
    <Section title="义项" description="不想学的义项直接删除。">
      <Input
        value={query}
        placeholder="搜索写法、假名或释义"
        onChange={(event) => {
          setQuery(event.target.value)
          setPage(0)
        }}
      />
      <ErrorText error={senses.error ?? deleteSense.error} />
      <ul className="rounded-md border">
        {senses.data?.rows.map((sense) => (
          <SenseRow
            key={`${sense.key.writing}|${sense.key.origin}|${sense.key.kana}|${sense.key.primary_gloss}`}
            sense={sense}
            deleting={deleteSense.isPending}
            onDelete={() => deleteSense.mutate(sense.key)}
          />
        ))}
        {senses.data?.rows.length === 0 && (
          <li className="px-3 py-6 text-center text-sm text-muted-foreground">没有义项</li>
        )}
      </ul>
      <div className="flex items-center justify-end gap-3 text-sm">
        <span className="text-muted-foreground tabular-nums">
          共 {total} 条 · {page + 1} / {pages}
        </span>
        <Button variant="outline" size="sm" disabled={page === 0} onClick={() => setPage(page - 1)}>
          上一页
        </Button>
        <Button
          variant="outline"
          size="sm"
          disabled={page + 1 >= pages}
          onClick={() => setPage(page + 1)}
        >
          下一页
        </Button>
      </div>
    </Section>
  )
}
