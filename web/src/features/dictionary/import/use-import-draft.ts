import { useState } from 'react'
import type { ImportOptions } from '@/bindings'
import type { RowFilter } from '@/features/dictionary/import/preview/row-kinds'

export type ImportView = 'raw' | 'table'

/**
 * What the dialog holds while the user prepares an import. It lives above
 * the editor and the report, so going back from the report finds it as left.
 */
export function useImportDraft() {
  const [text, setText] = useState('')
  /** `null` until the user changes one, meaning the default options. */
  const [options, setOptions] = useState<ImportOptions | null>(null)
  const [view, setView] = useState<ImportView>('raw')
  const [page, setPage] = useState(0)
  /** The rows the table is narrowed to; `null` for every row. */
  const [filter, setFilter] = useState<RowFilter | null>(null)
  return {
    text,
    setText,
    options,
    setOptions,
    view,
    setView,
    page,
    setPage,
    filter,
    setFilter,
  }
}

export type ImportDraft = ReturnType<typeof useImportDraft>
