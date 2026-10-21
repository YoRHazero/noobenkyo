import { useMemo, useRef, useState } from 'react'
import { flushSync } from 'react-dom'
import { Scissors } from 'lucide-react'
import type { ImportOptions } from '@/bindings'
import { useDictionaryPreviewEntries } from '@/backend/dictionary/use-dictionary-preview-entries'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogClose, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { CutDialog, type Cutting } from '@/features/dictionary/import/cut/cut-dialog'
import { cutLines } from '@/features/dictionary/import/cut/cut-lines'
import { ImportOptionsFields } from '@/features/dictionary/import/import-options-fields'
import { previewRows } from '@/features/dictionary/import/preview/preview-rows'
import { PreviewView } from '@/features/dictionary/import/preview/preview-view'
import { linesOf } from '@/features/dictionary/import/preview/row-kinds'
import { RawTextView } from '@/features/dictionary/import/raw-text/raw-text-view'
import { selectLine } from '@/features/dictionary/import/raw-text/select-line'
import { ReadFileButton } from '@/features/dictionary/import/read-file-button'
import type { ImportDraft, ImportView } from '@/features/dictionary/import/use-import-draft'

type Props = {
  draft: ImportDraft
  options: ImportOptions
  importing: boolean
  error: Error | null
  onImport: () => void
}

/**
 * Preparing an import: the options, the text (pasted or read from a file),
 * and the same text previewed as a table. Both views import the same text.
 * Warned lines can be cut from either view, skipped ones deleted from the table.
 */
export function ImportEditor({ draft, options, importing, error, onImport }: Props) {
  const textBox = useRef<HTMLTextAreaElement>(null)
  const preview = useDictionaryPreviewEntries(draft.text, options)
  const rows = useMemo(
    () => (preview.data ? previewRows(preview.data, draft.text) : []),
    [preview.data, draft.text],
  )
  const warned = linesOf(rows, 'warning')
  const [cutting, setCutting] = useState<Cutting | null>(null)
  // A preview of the text before its last change would cut the wrong lines.
  const actionsDisabled = importing || preview.isPlaceholderData

  /** Back to the raw text with the line selected, so it can be fixed. */
  const locate = (line: number) => {
    flushSync(() => draft.setView('raw'))
    if (textBox.current) {
      selectLine(textBox.current, line)
    }
  }
  /** The text without some of its lines, every row shown again. */
  const shorten = (rest: string) => {
    draft.setText(rest)
    draft.setFilter(null)
    draft.setPage(0)
  }

  return (
    <>
      <DialogHeader>
        <DialogTitle>导入</DialogTitle>
      </DialogHeader>
      <ImportOptionsFields options={options} disabled={importing} onChange={draft.setOptions} />
      <div className="flex items-center gap-3">
        <Tabs value={draft.view} onValueChange={(view) => draft.setView(view as ImportView)}>
          <TabsList>
            <TabsTrigger value="raw">原始文本</TabsTrigger>
            <TabsTrigger value="table">表格</TabsTrigger>
          </TabsList>
        </Tabs>
        {warned.length > 0 && (
          <Button
            variant="outline"
            disabled={actionsDisabled}
            onClick={() => setCutting({ label: '警告', lines: warned })}
          >
            <Scissors />
            剪切警告…
          </Button>
        )}
        <div className="ml-auto">
          <ReadFileButton disabled={importing} onRead={draft.setText} />
        </div>
      </div>
      <div className="min-h-0 flex-1">
        <RawTextView
          textBox={textBox}
          hidden={draft.view !== 'raw'}
          text={draft.text}
          disabled={importing}
          onChange={draft.setText}
        />
        {draft.view === 'table' && (
          <PreviewView
            preview={preview.data}
            error={preview.error}
            rows={rows}
            page={draft.page}
            filter={draft.filter}
            actionsDisabled={actionsDisabled}
            onPage={draft.setPage}
            onFilter={(filter) => {
              draft.setFilter(filter)
              draft.setPage(0)
            }}
            onCut={(label, lines) => setCutting({ label, lines })}
            onDelete={(lines) => shorten(cutLines(draft.text, lines).rest)}
            onLocate={locate}
          />
        )}
      </div>
      <DialogFooter className="items-center">
        <div className="mr-auto">
          {importing ? (
            <span className="text-sm text-muted-foreground">导入中…</span>
          ) : (
            <ErrorText error={error} />
          )}
        </div>
        <DialogClose asChild>
          <Button variant="outline" disabled={importing}>
            取消
          </Button>
        </DialogClose>
        <Button disabled={importing || draft.text.trim() === ''} onClick={onImport}>
          导入
        </Button>
      </DialogFooter>
      <CutDialog
        cutting={cutting}
        text={draft.text}
        options={options}
        onClose={() => setCutting(null)}
        onCut={(rest) => {
          shorten(rest)
          setCutting(null)
        }}
      />
    </>
  )
}
