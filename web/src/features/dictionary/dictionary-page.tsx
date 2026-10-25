import { useDictionarySearch } from '@/backend/dictionary/use-dictionary-search'
import { useStudySettings } from '@/backend/study/use-study-settings'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'
import { AgentButton } from '@/features/dictionary/agent/agent-button'
import { BrowseList } from '@/features/dictionary/browse/browse-list'
import { BrowseSearchBar } from '@/features/dictionary/browse/browse-search-bar'
import { rowsPerPage, useBrowseFilter } from '@/features/dictionary/browse/use-browse-filter'
import { ExportButton } from '@/features/dictionary/export/export-button'
import { ImportButton } from '@/features/dictionary/import/import-button'

/**
 * The dictionary: search and browse it by word, open a sense to edit or
 * delete it. Exporting takes the rows the filter shows, and so does filling
 * in sentences by an agent.
 */
export function DictionaryPage() {
  const browse = useBrowseFilter()
  const search = useDictionarySearch(browse.filter, browse.offset, rowsPerPage)
  // N, to tell mastered senses apart.
  const settings = useStudySettings()
  const error = search.error ?? settings.error

  return (
    <Page
      title="词库"
      actions={
        <div className="flex gap-2">
          <ImportButton />
          <ExportButton filter={browse.filter} total={search.data?.total ?? 0} />
          <AgentButton filter={browse.filter} />
        </div>
      }
    >
      <BrowseSearchBar browse={browse} />
      {error ? (
        <ErrorText error={error} />
      ) : search.data && settings.data ? (
        <BrowseList
          page={search.data}
          pageIndex={browse.page}
          maxStage={settings.data.quiz.max_stage}
          narrowed={browse.narrowed}
          onPage={browse.setPage}
        />
      ) : (
        <p className="text-sm text-muted-foreground">加载中…</p>
      )}
    </Page>
  )
}
